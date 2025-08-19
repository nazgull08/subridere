use bevy::prelude::*;

use super::{
    fsm::{
        attack::enemy_melee_attack_system,
        enemy_stuck_detection_system::enemy_stuck_detection_system,
        movement_behavior::enemy_movement_behavior_system,
        target_generation::target_selection_system, transition::enemy_state_transition_system,
    },
    spawn::jimbo::spawn_jimbo_in_room,
    system::{
        apply_steering_intents_system, debug_enemy_axes, rotate_enemy_towards_velocity_system,
        update_enemy_animation_on_state_change,
    },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Startup, spawn_jimbo_in_room)
            .add_systems(Update, enemy_state_transition_system)
            .add_systems(Update, target_selection_system)
            .add_systems(Update, enemy_movement_behavior_system)
            .add_systems(Update, enemy_stuck_detection_system)
            .add_systems(Update, apply_steering_intents_system)
            .add_systems(Update, rotate_enemy_towards_velocity_system)
            .add_systems(Update, enemy_melee_attack_system)
            .add_systems(Update, update_enemy_animation_on_state_change);
        //            .add_systems(Update, debug_enemy_axes);
    }
}
