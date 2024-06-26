use std::num::NonZeroUsize;

use recordkeeper_macros::SaveBin;
use thiserror::Error;

use crate::{
    character::CHARACTER_MAX,
    dlc::{CraftItemData, CRAFTED_ITEM_ID},
    SaveData,
};

pub const ITEM_ACCESSORY_MAX: usize = 1500;
pub const GEM_CATEGORY_MAX: usize = 20;

pub mod edit;

#[derive(SaveBin, Debug)]
pub struct Inventory {
    pub(crate) chronological_id_max: u32,

    #[loc(0x28)]
    /// `ITM_Cylinder`
    pub cylinders: Box<[ItemSlot; 16]>,
    /// `ITM_Gem`
    pub gems: Box<[ItemSlot; 300]>,
    /// `ITM_Collection`
    pub collectibles: Box<[ItemSlot; 1500]>,
    /// `ITM_Info`, discussion info dialogues
    pub infos: Box<[ItemSlot; 800]>,
    /// `ITM_Accessory`
    pub accessories: Box<[ItemSlot; ITEM_ACCESSORY_MAX]>,
    /// `ITM_Precious`
    pub key_items: Box<[ItemSlot; 200]>,
    /// `ITM_Exchange` (unused item type)
    pub exchange: Box<[ItemSlot; 16]>,
    /// `ITM_Extra`
    pub extra: Box<[ItemSlot; 64]>,
}

/// An item slot in the player's inventory.
///
/// To edit item slots, use the [`edit::ItemEditor`] struct.
#[derive(SaveBin, Debug, Clone, Copy)]
#[size(16)]
pub struct ItemSlot {
    item_id: u16,
    slot_index: u16,
    item_type: u32,
    chronological_id: u32,
    #[loc(0xc)]
    amount: u16,
    flags: u8,
}

pub enum SlotFlags {
    /// The slot has an item inside
    Active = 1,
    /// The player has marked the item as favorite
    Favorite = 1 << 1,
    /// The small circle icon for "unchecked" items
    New = 1 << 2,
    /// Whether the item has crafted accessory data associated to it
    HasCraftData = 1 << 3,
}

#[derive(SaveBin, Debug)]
pub struct DlcManualSlot {
    item_id: u16,
    inventory_slot_index: u16,
    item_type: u16,
}

/// Highest levels the player has unlocked for each gem category (in the form of item IDs).
///
/// Unfortunately, the library cannot automatically update this when items are changed, as it
/// requires knowledge of game data. (in particular, the ID ranges of each gem category)
#[derive(SaveBin, Debug)]
pub struct GemLevels {
    /// Gem Item IDs (12000+) for each category
    item_ids: [u16; GEM_CATEGORY_MAX],
}

/// Forged accessory data (unused/removed feature)
///
/// This is a removed feature, but its data still lingers in the latest version. The mechanic
/// would allegedly make some character-exclusive accessories forgeable, allowing their effect
/// to be changed and their stats upgraded.
///
/// **This is not to be confused with Masha accessory crafting**. This is, however, not a
/// coincidence: the code responsible for updating this section now updates the Masha section instead.
#[derive(SaveBin, Debug)]
pub struct LegacyForge {
    /// One entry per character (ID - 1). The single accessory associated to an entry is the
    /// one with the respective `UseChr` in `ITM_Accessory`.
    pub data: [LegacyForgeData; CHARACTER_MAX],
}

/// See [`LegacyForge`]
#[derive(SaveBin, Debug)]
#[size(4)]
pub struct LegacyForgeData {
    pub enhance_id: u16,
    pub valid: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ItemType {
    Cylinder = 1,
    Gem = 2,
    Collection = 3,
    Info = 4,
    Accessory = 5,
    Collectopedia = 6,
    Precious = 7,
    Exchange = 8,
    Extra = 9,
}

#[derive(Error, Debug)]
#[error("unknown item type {0}")]
pub struct TypeFromIntError(u32);

impl Inventory {
    pub fn slots(&self, item_type: ItemType) -> &[ItemSlot] {
        match item_type {
            ItemType::Cylinder => &*self.cylinders,
            ItemType::Gem => &*self.gems,
            ItemType::Collection => &*self.collectibles,
            ItemType::Info => &*self.infos,
            ItemType::Accessory => &*self.accessories,
            ItemType::Precious => &*self.key_items,
            ItemType::Exchange => &*self.exchange,
            ItemType::Extra => &*self.extra,
            t => panic!("unsupported item type {t:?}"),
        }
    }

    pub fn slots_mut(&mut self, item_type: ItemType) -> &mut [ItemSlot] {
        match item_type {
            ItemType::Cylinder => &mut *self.cylinders,
            ItemType::Gem => &mut *self.gems,
            ItemType::Collection => &mut *self.collectibles,
            ItemType::Info => &mut *self.infos,
            ItemType::Accessory => &mut *self.accessories,
            ItemType::Precious => &mut *self.key_items,
            ItemType::Exchange => &mut *self.exchange,
            ItemType::Extra => &mut *self.extra,
            t => panic!("unsupported item type {t:?}"),
        }
    }

    pub(crate) fn split_slots_mut(
        &mut self,
        id_a: u16,
        id_b: u16,
    ) -> (&mut ItemSlot, &mut ItemSlot) {
        let (ty_a, ty_b) = (
            ItemType::get_by_item_id(id_a),
            ItemType::get_by_item_id(id_b),
        );
        let (idx_a, idx_b) = (
            self.slots(ty_a)
                .iter()
                .position(|s| s.item_id() == id_a)
                .expect("item a not found"),
            self.slots(ty_b)
                .iter()
                .position(|s| s.item_id() == id_b)
                .expect("item b not found"),
        );
        if ty_a == ty_b {
            let slots = self.slots_mut(ty_a);
            if idx_a > idx_b {
                let (before_a, after_a) = slots.split_at_mut(idx_a);
                (&mut after_a[0], &mut before_a[idx_b])
            } else {
                let (before_b, after_b) = slots.split_at_mut(idx_b);
                (&mut before_b[idx_a], &mut after_b[0])
            }
        } else {
            let (slots_a, slots_b) = (
                self.slots_mut(ty_a) as *mut [ItemSlot],
                self.slots_mut(ty_b) as *mut [ItemSlot],
            );
            // SAFETY: the two slot arrays are different, so we are just splitting
            // the borrow. The indexing is still performed safely with bound checks.
            unsafe { (&mut (&mut *slots_a)[idx_a], &mut (&mut *slots_b)[idx_b]) }
        }
    }
}

impl ItemSlot {
    /// Returns the slot's positional index.
    pub fn index(&self) -> u16 {
        self.slot_index
    }

    /// Returns the slot's item ID.
    pub fn item_id(&self) -> u16 {
        self.item_id
    }

    /// Returns the slot's item amount.
    pub fn amount(&self) -> u16 {
        self.amount
    }

    /// Returns the slot's item type.
    ///
    /// ## Panics
    /// Panics if the item type is invalid or if the slot is empty.
    pub fn item_type(&self) -> ItemType {
        assert!(self.is_valid(), "empty slot");
        ItemType::try_from(self.item_type).unwrap()
    }

    /// Returns whether the slot is occupied by a valid item.
    pub fn is_valid(&self) -> bool {
        self.flags & (SlotFlags::Active as u8) != 0
    }

    /// Returns whether the slot hosts a crafted accessory. (DLC3)
    pub fn is_crafted_accessory(&self) -> bool {
        self.is_valid()
            && self.item_type() == ItemType::Accessory
            && self.item_id() == CRAFTED_ITEM_ID
    }

    /// Returns the accessory crafting data for the item slot, if present.
    pub fn craft_data<'s>(&self, save: &'s SaveData) -> Option<&'s CraftItemData> {
        save.accessory_crafting.get_data(self.slot_index as usize)
    }

    pub(crate) fn chronological_id(&self) -> u32 {
        self.chronological_id
    }

    pub(crate) fn set_chronological_id(&mut self, chronological_id: u32) {
        self.chronological_id = chronological_id;
    }
}

impl ItemType {
    pub fn get_by_item_id(item_id: u16) -> Self {
        //match item_id {}
        todo!()
    }

    pub fn lang_id(self) -> &'static str {
        match self {
            Self::Cylinder => "cylinder",
            Self::Gem => "gem",
            Self::Collection => "collection",
            Self::Collectopedia => "collepedia",
            Self::Info => "info",
            Self::Accessory => "accessory",
            Self::Precious => "precious",
            Self::Exchange => "exchange",
            Self::Extra => "extra",
        }
    }
}

impl GemLevels {
    /// Returns the item ID that represents the gem category (`ITM_Gem.Category`).
    ///
    /// The ID points to a row in `ITM_Gem`.
    pub fn get_item(&self, category: usize) -> Option<u16> {
        self.item_ids
            .get(category.checked_sub(1)?)
            .copied()
            .and_then(|n| (n != 0).then_some(n))
    }

    /// Updates the item ID that represents a gem category (`ITM_Gem.Category`).
    ///
    /// The ID should point to a row in `ITM_Gem`, and should represent the highest level the
    /// player has unlocked for the gem. It is possible to use the ID of a gem with
    /// a different category, which results in the game showing a duplicate entry.
    pub fn set_item(&mut self, category: NonZeroUsize, item_id: u16) {
        self.item_ids[category.get() - 1] = item_id;
    }
}

impl TryFrom<u32> for ItemType {
    type Error = TypeFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Cylinder,
            2 => Self::Gem,
            3 => Self::Collection,
            4 => Self::Info,
            5 => Self::Accessory,
            6 => Self::Collectopedia,
            7 => Self::Precious,
            8 => Self::Exchange,
            9 => Self::Extra,
            i => return Err(TypeFromIntError(i)),
        })
    }
}
