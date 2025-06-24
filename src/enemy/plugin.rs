use bevy::prelude::*;

use super::{
    fsm::{movement_behavior::enemy_movement_behavior_system, target_generation::target_generation_system, transition::enemy_state_transition_system}, spawn::jimbo::spawn_jimbo_in_room, system::update_enemy_animation_on_state_change,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Update, enemy_state_transition_system)
            .add_systems(Update, target_generation_system)
            .add_systems(Update, enemy_movement_behavior_system)
            .add_systems(Update, update_enemy_animation_on_state_change);
    }
}
