use bevy::prelude::*;
use crate::enemy::component::*;
use crate::player::component::Player;

/// Обновляет память врага: 
/// - если игрок в sight — пишем в pursue_target (и сбрасываем patrol_target),
/// - иначе — ничего не меняем (patrol_target подхватится в transition).
pub fn target_generation_system(
    mut enemies: Query<(&Transform, &SightRange, &mut EnemyMemory), With<Enemy>>,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    for (enemy_tf, sight, mut memory) in &mut enemies {
        // находим ближайшего игрока в радиусе
        if let Some((entity, _tf)) = players
            .iter()
            .filter_map(|(e, tf)| {
                let d = enemy_tf.translation.distance(tf.translation);
                (d <= sight.0).then_some((e, d))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            // захватили цель
            if memory.pursue_target != Some(entity) {
                tracing::info!(?entity, "Enemy acquired target");
            }
            memory.pursue_target = Some(entity);
            memory.patrol_target = None;
        } else if memory.pursue_target.is_some() {
            tracing::info!("Enemy lost target");
            memory.pursue_target = None;
            // patrol_target не трогаем — transition создаст новую, когда нужно
        }
    }
}
