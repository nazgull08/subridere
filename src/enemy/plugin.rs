use crate::enemy::{fsm::update_enemy_fsm, spawn::spawn_jester_in_room};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jester_in_room)
            .add_systems(Startup, spawn_jester_in_room)
            .add_systems(Startup, spawn_jester_in_room)
            .add_systems(Update, update_enemy_fsm);
    }
}
