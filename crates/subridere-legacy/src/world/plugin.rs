use crate::world::room::types::RoomMap;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoomMap::default());
    }
}
