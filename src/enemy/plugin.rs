use crate::enemy::{fsm::update_enemy_fsm, spawn::spawn_jester_in_room};
use bevy::prelude::*;

use super::spawn::spawn_jimbo_in_room;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jester_in_room)
            .add_systems(Startup, spawn_jester_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Update, update_enemy_fsm);
    }
}
