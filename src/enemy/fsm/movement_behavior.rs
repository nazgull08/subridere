// src/enemy/fsm/movement_behavior.rs

use crate::enemy::component::*;
use bevy::prelude::*;

pub fn enemy_movement_behavior_system(
    mut commands: Commands,
    query: Query<(Entity, &EnemyState, &Transform, &EnemyMemory), With<Enemy>>,
) {
    for (entity, state, tf, memory) in &query {
        if *state != EnemyState::MovingToTarget {
            continue;
        }

        let Some(target) = memory.target_position else {
            continue;
        };

        let dir = (target - tf.translation).with_y(0.0);
        let dist2 = dir.length_squared();
        if dist2 < 0.25 {
            tracing::info!(?entity, "Target reached");
            continue;
        }

        let velocity = dir.normalize_or_zero() * 3.0;

        commands.entity(entity).insert(SteeringIntent {
            desired_velocity: velocity,
        });
    }
}
