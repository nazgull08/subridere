use bevy::prelude::*;

use crate::app::AppState; // ← ДОБАВИТЬ
use crate::enemies::worm::{
    attack::{worm_execute_lunge_system, worm_prepare_visual_feedback},
    damage::{worm_collision_damage_system, worm_damage_cooldown_system},
    death::{animate_blood_pool, fade_corpse_segments, spawn_blood_pool_visuals},
    detection::worm_detect_targets,
    health::{worm_death_system, worm_projectile_damage_system},
    movement::worm_move_forward,
    particles::{spawn_dust_on_landing, update_blood_particles, update_dust_particles},
    rotation::worm_rotate_to_target,
    state::worm_update_state,
};

/// Plugin for all enemy-related systems
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                worm_detect_targets,
                worm_update_state,
                worm_execute_lunge_system,
                worm_prepare_visual_feedback,
                worm_damage_cooldown_system,
                worm_collision_damage_system,
                worm_rotate_to_target,
                worm_move_forward,
                spawn_dust_on_landing,
                update_dust_particles,
                worm_projectile_damage_system,
                update_blood_particles,
                worm_death_system,
                spawn_blood_pool_visuals,
                animate_blood_pool,
                fade_corpse_segments,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    }
}
