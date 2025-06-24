use bevy::prelude::*;
use crate::enemy::component::*;
use crate::unit::component::{MoveIntent, LookAtIntent};

/// Конвертирует память в интенты движения и поворота.
/// - Если pursue_target есть → идём к нему.
/// - Иначе, если patrol_target есть → идём к нему.
/// Останавливаемся, когда цель ближе 0.5 м.
pub fn enemy_movement_behavior_system(
    mut commands: Commands,
    query: Query<(Entity, &EnemyState, &Transform, &EnemyMemory), With<Enemy>>,
    transforms: Query<&Transform>,
) {
    for (entity, state, tf, memory) in &query {
        match *state {
            EnemyState::Walk | EnemyState::Attack(EnemyAttackState::Approach) => {
                println!("current transform {:?}", tf);
                // выбираем актуальную цель
                let opt = memory
                    .pursue_target
                    .and_then(|e| transforms.get(e).ok().map(|t| t.translation))
                    .or(memory.patrol_target);

                let target = match opt {
                    Some(v) => v,
                    None => continue,
                };

                // горизонтальный вектор
                let mut dir = (target - tf.translation).with_y(0.0);
                let dist2 = dir.length_squared();
                if dist2 < 0.25 {
                    // достали цель
                    tracing::info!(?entity, "Target reached");
                    continue;
                }

                // направляем и движемся
                let dir_norm = dir.normalize_or_zero();
                let mut look = target;
                look.y = tf.translation.y;

                commands.entity(entity)
                    .insert(MoveIntent(dir_norm))
                    .insert(LookAtIntent(look));
            }
            _ => {}
        }
    }
}
