// src/enemy/fsm/target_generation.rs
use bevy::prelude::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use crate::enemy::component::*;
use crate::player::component::Player;
use crate::world::room::types::RoomMap;

/* ---------- публичный «фасад» ---------- */

pub fn target_selection_system(
    mut enemies: Query<(Entity,
                        &Transform,
                        &SightRange,
                        &mut EnemyMemory,
                        &mut EnemyState), With<Enemy>>,
    players:   Query<&Transform,           With<Player>>,
    room_map:  Res<RoomMap>,
) {
    for (e, tf, sight, mut memory, mut state) in &mut enemies {
        // 1) Пытаемся обнаружить игрока
        detect_player(tf, sight, &mut memory, &mut state, &players);

        // 2) Если цели нет — генерируем патруль
        if memory.target_position.is_none() {
            refresh_patrol_target(tf, &mut memory, &room_map);
        }

        // 3) Проверяем, не убежал ли игрок слишком далеко
        forget_player_if_far(tf, sight, &mut memory, &players);
    }
}

/* ---------- мелкие чистые функции ---------- */

/// Если игрок в радиусе — ставим его позицию целью и переводим FSM в MovingToTarget
fn detect_player(
    enemy_tf: &Transform,
    sight: &SightRange,
    memory: &mut EnemyMemory,
    state: &mut EnemyState,
    players: &Query<&Transform, With<Player>>,
) {
    for player_tf in players {
        if enemy_tf.translation.distance(player_tf.translation) <= sight.0 {
            memory.target_position = Some(player_tf.translation);
            if !matches!(*state, EnemyState::Attack(_) | EnemyState::Dead) {
                *state = EnemyState::MovingToTarget;
            }
            return; // нашли кого-то — выходим
        }
    }
}

/// Выбирает случайную точку внутри случайной комнаты
fn refresh_patrol_target(
    enemy_tf: &Transform,
    memory: &mut EnemyMemory,
    room_map: &RoomMap,
) {
    let mut rng = rand::thread_rng();
    if let Some((coord, _)) = room_map.rooms.iter().choose(&mut rng) {
        let room_size = Vec3::new(10.0, 5.0, 10.0); // подстрой под свои данные
        let origin = coord.as_vec3() * room_size;
        let offset = Vec3::new(
            rng.gen_range(-room_size.x / 2.0..room_size.x / 2.0),
            0.0,
            rng.gen_range(-room_size.z / 2.0..room_size.z / 2.0),
        );
        memory.target_position = Some(origin + offset);
        tracing::debug!("Patrol target set to {:?}", memory.target_position);
    } else {
        // fallback: чуть вперёд
        memory.target_position = Some(enemy_tf.translation + Vec3::X * 2.0);
    }
}

/// Сбрасываем цель, если старый «замеченный» игрок ушёл далеко
fn forget_player_if_far(
    enemy_tf: &Transform,
    sight: &SightRange,
    memory: &mut EnemyMemory,
    players: &Query<&Transform, With<Player>>,
) {
    if let Some(target) = memory.target_position {
        // проверяем, не является ли target позицией какого-нибудь игрока
        for player_tf in players {
            if player_tf.translation == target {
                // это игрок; проверим дистанцию
                if enemy_tf.translation.distance(player_tf.translation) > sight.0 * 1.5 {
                    memory.target_position = None;
                    tracing::info!("Lost sight of player → back to patrol");
                }
                break;
            }
        }
    }
}
