use crate::enemy::spawn::jester::spawn_jester_in_room;
use bevy::prelude::*;

use super::{fsm::update_enemy_fsm_system, movement::walk_movement_system, spawn::jimbo::spawn_jimbo_in_room, system::update_enemy_animation_on_state_change};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jester_in_room)
            .add_systems(Startup, spawn_jester_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Update, update_enemy_fsm_system)
            .add_systems(Update, update_enemy_animation_on_state_change)
            .add_systems(Update, walk_movement_system);

    }
}
