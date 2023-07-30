use recordkeeper_macros::SaveBin;
use crate::save::character::{Character, CHARACTER_MAX};
use crate::save::enemy::{ENEMY_TOMBSTONE_MAX, EnemyTombstone};
use crate::save::flags::AllFlags;

mod character;
mod item;
mod flags;
mod enemy;

const SAVE_VERSION: u8 = 10;

#[derive(SaveBin)]
pub struct SaveData {
    #[no_getter]
    #[assert(0xb378fa6a)]
    _magic: u32,
    #[assert(SAVE_VERSION)]
    save_version: u8,

    #[loc(0x10)]
    play_time: PlayTime,
    #[loc(0x18)]
    timestamp: SaveTimestamp,
    gold: u32,

    /// Saved event flow ID for end-of-chapter saves
    #[loc(0x684)]
    saved_event_flow: u32,

    #[loc(0x68c)]
    map_id: u16,
    map_time: MapTime,

    #[loc(0x6a0)]
    player_pos: Pos,
    #[loc(0x6a8)]
    ship_pos: Pos,

    #[loc(0x710)]
    flags: AllFlags,

    #[loc(0xe3a0)]
    characters: [Character; CHARACTER_MAX],

    #[loc(0x183000)]
    enemy_tombstones: [EnemyTombstone; ENEMY_TOMBSTONE_MAX],
}

#[derive(SaveBin)]
pub struct PlayTime {
    raw: u32
}

#[derive(SaveBin)]
pub struct SaveTimestamp {
    time: u32,
    date: u32
}

#[derive(SaveBin)]
pub struct Pos {
    x: f32,
    y: f32,
    z: f32
}

#[derive(SaveBin)]
pub struct MapTime {
    hour: u16,
    minute: u16
}