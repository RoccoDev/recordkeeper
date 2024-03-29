use std::{cmp::Ordering, num::NonZeroUsize};

use recordkeeper_macros::SaveBin;

use crate::{
    character::{
        class::ClassAccessory,
        slot::{Slot, SlotMut},
        CHARACTER_MAX,
    },
    chrono::ChronologicalOrder,
    flags::FlagType,
    SaveData,
};

pub const DLC4_ENEMYPEDIA_MAX_EACH: usize = 200;

#[derive(SaveBin, Debug)]
pub struct Dlc4 {
    /// Number of victories for Enemypedia entries 0-199
    enemypedia_0_199: Box<[u8; DLC4_ENEMYPEDIA_MAX_EACH]>,

    /// Extra inventory, indexed by character ID
    extra_inventory: Box<[Dlc4ExtraInventory; CHARACTER_MAX]>,

    /// Number of victories for Enemypedia entries 200-399
    // lol
    #[loc(0x80c8)]
    enemypedia_200_399: Box<[u8; DLC4_ENEMYPEDIA_MAX_EACH]>,
}

#[derive(SaveBin, Debug)]
#[size(512)]
struct Dlc4ExtraInventory {
    /// Indexed by class ID
    battle_manual: Box<[ClassAccessory; 64]>,
}

pub struct CommunityChrono<'a> {
    save: &'a mut SaveData,
    flag_type: FlagType,
}

impl Dlc4 {
    /// Gets the current number of victories against an enemypedia enemy.
    ///
    /// The `index` parameter is `32F9A6F1` from `B4158056`, - 2190.
    ///
    /// ## Panics
    /// Panics if the index is out of bounds (`0 <= index < 400`)
    pub fn get_enemypedia_count(&self, index: usize) -> u8 {
        if index < 200 {
            self.enemypedia_0_199[index]
        } else {
            self.enemypedia_200_399[index - 200]
        }
    }

    /// Updates the current number of victories against an enemypedia enemy.
    ///
    /// The `index` parameter is `32F9A6F1` from `B4158056`, - 2190.
    ///
    /// ## Panics
    /// Panics if the index is out of bounds (`0 <= index < 400`)
    pub fn set_enemypedia_count(&mut self, index: usize, count: u8) {
        if index < 200 {
            self.enemypedia_0_199[index] = count;
        } else {
            self.enemypedia_200_399[index - 200] = count;
        }
    }

    /// Returns the contents of the battle manual slot for the character and class ID pair.
    ///
    /// The IDs start at 1, for example Matthew is #36 with class #37.
    pub fn battle_manual_slot(
        &self,
        chr_id: NonZeroUsize,
        class_id: NonZeroUsize,
    ) -> Slot<ClassAccessory> {
        Slot(self.extra_inventory[chr_id.get() - 1].battle_manual[class_id.get() - 1])
    }

    /// Returns a mutable battle manual slot for the character and class ID pair.
    ///
    /// The IDs start at 1, for example Matthew is #36 with class #37.
    pub fn battle_manual_slot_mut(
        &mut self,
        chr_id: NonZeroUsize,
        class_id: NonZeroUsize,
    ) -> SlotMut<ClassAccessory> {
        SlotMut(&mut self.extra_inventory[chr_id.get() - 1].battle_manual[class_id.get() - 1])
    }
}

impl<'a> CommunityChrono<'a> {
    pub fn new(save: &'a mut SaveData) -> Self {
        Self {
            save,
            flag_type: FlagType::Byte,
        }
    }

    /// Checks whether an NPC community entry is registered in the order.
    /// If it is not registered, then no task from that entry was completed.
    pub fn is_present(&self, flag: usize) -> bool {
        self.save
            .flags
            .get(self.flag_type, flag)
            .is_some_and(|f| f != 0)
    }

    /// Removes an NPC community entry from the order.
    pub fn delete(&mut self, flag: usize) {
        let val = self
            .save
            .flags
            .get(self.flag_type, flag)
            .expect("flag out of bounds");
        if val == self.save.dlc4_community_order_max.into() {
            // Prevent the max value from growing too large
            self.save.dlc4_community_order_max =
                self.save.dlc4_community_order_max.saturating_sub(1);
        }
        self.save.flags.set(self.flag_type, flag, 0);
    }
}

impl<'a> ChronologicalOrder for CommunityChrono<'a> {
    /// Compares the order of two NPC community entries. Parameters are flag IDs
    fn cmp_entries(&self, id_a: usize, id_b: usize) -> Ordering {
        self.save
            .flags
            .get(self.flag_type, id_a)
            .cmp(&self.save.flags.get(self.flag_type, id_b))
    }

    /// Swaps the order of two NPC community entries. Parameters are flag IDs
    fn swap(&mut self, id_a: usize, id_b: usize) {
        let val_a = self
            .save
            .flags
            .get(self.flag_type, id_a)
            .expect("id_a out of bounds");
        let val_b = self
            .save
            .flags
            .get(self.flag_type, id_b)
            .expect("id_b out of bounds");
        self.save.flags.set(self.flag_type, id_a, val_b);
        self.save.flags.set(self.flag_type, id_b, val_a);
    }

    /// Inserts a new NPC community entry at the end of the order. Parameter is
    /// a flag ID.
    fn insert(&mut self, id: usize) {
        if self.is_present(id) {
            return;
        }
        let val = self
            .save
            .dlc4_community_order_max
            .checked_add(1)
            .expect("community chrono reached max value");
        self.save.flags.set(self.flag_type, id, val as u32);
        self.save.dlc4_community_order_max = val;
    }
}
