use super::components::{WormAI, WormHead, WormState};
use crate::player::component::Player;
use bevy::prelude::*;

/// Updates worm state machine with attack logic
pub fn worm_update_state(
    mut heads: Query<(&Transform, &mut WormState, &WormAI), With<WormHead>>,
    targets: Query<&Transform, (With<Player>, Without<WormHead>)>,
    time: Res<Time>,
) {
    for (head_transform, mut state, ai) in &mut heads {
        let new_state = match &*state {
            // ============================================
            // IDLE - ищем цель
            // ============================================
            WormState::Idle => {
                if let Some(target_entity) = ai.target {
                    if let Ok(_) = targets.get(target_entity) {
                        info!("🔄 Idle -> Chase");
                        WormState::Chase {
                            target: target_entity,
                        }
                    } else {
                        WormState::Idle
                    }
                } else {
                    WormState::Idle
                }
            }

            // ============================================
            // CHASE - преследуем цель
            // ============================================
            WormState::Chase { target: _ } => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = targets.get(target_entity) {
                        let distance = head_transform
                            .translation
                            .distance(target_transform.translation);

                        // Достаточно близко для прыжка?
                        if distance <= ai.jump_range {
                            info!("🔄 Chase -> PrepareAttack (distance: {:.1}m)", distance);
                            WormState::PrepareAttack {
                                target: target_entity,
                                prepare_timer: ai.jump_prepare_time,
                                target_pos: target_transform.translation, // ✅ Запоминаем!
                            }
                        } else {
                            // Продолжаем преследовать
                            WormState::Chase {
                                target: target_entity,
                            }
                        }
                    } else {
                        // Потеряли цель
                        info!("🔄 Chase -> Idle (lost target)");
                        WormState::Idle
                    }
                } else {
                    // Нет цели
                    info!("🔄 Chase -> Idle (no target)");
                    WormState::Idle
                }
            }

            // ============================================
            // PREPARE ATTACK - готовимся к прыжку
            // ============================================
            WormState::PrepareAttack {
                target,
                prepare_timer,
                target_pos,
            } => {
                let new_timer = prepare_timer - time.delta_secs();

                if new_timer <= 0.0 {
                    // Время вышло - ПРЫГАЕМ!
                    info!("🔄 PrepareAttack -> Lunging");
                    WormState::Lunging {
                        target: *target,
                        target_pos: *target_pos,
                    }
                } else {
                    // Продолжаем готовиться
                    WormState::PrepareAttack {
                        target: *target,
                        prepare_timer: new_timer,
                        target_pos: *target_pos,
                    }
                }
            }

            // ============================================
            // LUNGING - в полёте (кратковременно)
            // ============================================
            // Это состояние мгновенно переходит в Recovering
            // после применения импульса (в другой системе)
            WormState::Lunging { .. } => {
                // Эта система не меняет Lunging
                // Переход в Recovering делает система execute_lunge
                state.clone()
            }

            // ============================================
            // RECOVERING - восстанавливаемся после прыжка
            // ============================================
            WormState::Recovering { recovery_timer } => {
                let new_timer = recovery_timer - time.delta_secs();

                if new_timer <= 0.0 {
                    // Восстановились - возвращаемся к преследованию
                    if ai.target.is_some() {
                        info!("🔄 Recovering -> Chase");
                        WormState::Chase {
                            target: ai.target.unwrap(),
                        }
                    } else {
                        info!("🔄 Recovering -> Idle");
                        WormState::Idle
                    }
                } else {
                    // Продолжаем восстанавливаться
                    WormState::Recovering {
                        recovery_timer: new_timer,
                    }
                }
            }
        };

        // Применяем новое состояние
        *state = new_state;
    }
}
