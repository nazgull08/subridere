use crate::enemies::worm::animation::animate_worm_trail;
use crate::enemies::worm::behavior::{
    worm_attack_behavior, worm_move_system, worm_rotate_system, worm_detect_targets, worm_update_state,
};
use bevy::prelude::*;

/// Plugin for all enemy-related systems
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                worm_detect_targets,
                worm_update_state,
                worm_move_system,
                worm_rotate_system,
                worm_attack_behavior,
                animate_worm_trail,
            )
                .chain(),
        );
    }
}
