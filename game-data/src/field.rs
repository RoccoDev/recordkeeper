use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable, Id, TextTable},
    IdInt, LanguageData,
};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct FieldRegistry {
    maps: Box<[Map]>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldLang {
    pub locations: FilterTable,
    pub com_spots: TextTable,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Map {
    pub id: MapId,
    pub locations: Box<[Location]>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct MapId {
    pub id: IdInt,
    pub name_id: IdInt,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct Location {
    pub id: IdInt,
    pub name_id: IdInt,
    pub location_type: LocationType,
    pub map_jump: Option<NonZeroU16>,
    pub map_point: Option<MapPoint>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct MapPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum LocationType {
    Region,
    Location,
    Landmark,
    RestSpot,
    SecretArea,
    Colony,
    /// Invisible landmarks, usually only active for a
    /// specific story sequence.
    RespawnPoint,
}

impl FieldRegistry {
    pub fn new(maps: impl IntoIterator<Item = Map>) -> Self {
        Self {
            maps: maps.into_iter().collect(),
        }
    }

    pub fn get_map_by_id(&self, id: IdInt) -> Option<&Map> {
        id.checked_sub(1).and_then(|i| self.maps.get(i as usize))
    }

    pub fn maps(&self) -> &[Map] {
        &self.maps
    }
}

impl Filterable for MapId {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.field.locations.get(self.name_id)
    }
}

impl Id for MapId {
    fn id(&self) -> IdInt {
        self.id
    }
}

impl Filterable for Map {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        self.id.get_filter(language)
    }
}

impl Id for Map {
    fn id(&self) -> IdInt {
        self.id.id()
    }
}

impl Filterable for Location {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.field.locations.get(self.name_id)
    }
}

impl Id for Location {
    fn id(&self) -> IdInt {
        self.id
    }
}
