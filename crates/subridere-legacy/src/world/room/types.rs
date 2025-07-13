use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct WallFlags {
    pub front: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Default, Clone)]
pub struct DoorFlags {
    pub front: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Default, Clone)]
pub struct RoomMetadata {
    pub wall_flags: WallFlags,
    pub door_flags: DoorFlags,
    pub has_light: bool,
    pub entity: Option<Entity>,
}

#[derive(Debug, Resource, Default)]
pub struct RoomMap {
    pub rooms: HashMap<IVec3, RoomMetadata>,
}
