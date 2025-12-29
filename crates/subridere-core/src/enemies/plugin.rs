use bevy::prelude::*;

use crate::enemies::worm::{
    attack::{worm_execute_lunge_system, worm_prepare_visual_feedback}, damage::{worm_collision_damage_system, worm_damage_cooldown_system}, detection::worm_detect_targets, movement::worm_move_forward, particles::{spawn_dust_on_landing, update_dust_particles, update_blood_particles}, rotation::worm_rotate_to_target, state::worm_update_state, health::{worm_death_system, worm_projectile_damage_system},
death::{spawn_blood_pool_visuals, animate_blood_pool, fade_corpse_segments},
};

/// Plugin for all enemy-related systems
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // 1. Detect nearby players
                worm_detect_targets,
                
                // 2. Update state machine (handles timers)
                worm_update_state,
                
                // 3. Execute lunge attack if in Lunging state
                worm_execute_lunge_system,
                worm_prepare_visual_feedback,

                worm_damage_cooldown_system,
                worm_collision_damage_system,
                
                // 4. Rotate towards target (skips during attack)
                worm_rotate_to_target,
                
                // 5. Move forward (skips during attack)
                worm_move_forward,
                spawn_dust_on_landing,
                update_dust_particles,

                worm_projectile_damage_system,
                update_blood_particles,
                worm_death_system,
                spawn_blood_pool_visuals,
                animate_blood_pool,
                fade_corpse_segments
            )
                .chain(),
        );
    }
}
