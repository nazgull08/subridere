use bevy::prelude::*;
use crate::enemy::component::{Enemy, EnemyMemory};

/// Сбрасывает target_position, если враг застрял дольше 2 секунд
pub fn enemy_stuck_detection_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut EnemyMemory), With<Enemy>>,
) {
    for (tf, mut memory) in &mut query {
        let delta = tf.translation - memory.last_position;

        if delta.length_squared() < 0.01 {
            memory.stuck_timer.tick(time.delta());
        } else {
            memory.last_position = tf.translation;
            memory.stuck_timer.reset();
        }

        if memory.stuck_timer.finished() {
            tracing::info!("Enemy stuck, resetting target");
            memory.target_position = None;
            memory.stuck_timer.reset();
        }
    }
}
