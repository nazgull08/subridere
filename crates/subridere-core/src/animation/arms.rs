use bevy::prelude::*;

use crate::fighting::components::{CombatState, PlayerCombatState};
use crate::player::body::components::{ArmPart, ArmSide};
use crate::player::component::Player;

/// Базовая позиция правой руки
const BASE_POSITION_RIGHT: Vec3 = Vec3::new(1.0, -0.8, -2.0);
/// Насколько рука выдвигается вперёд при ударе
const PUNCH_FORWARD: f32 = 1.5;
/// Насколько рука смещается к центру при ударе
const PUNCH_INWARD: f32 = 0.5;

/// Анимирует правую руку при атаке (боксёрский удар)
pub fn animate_arm_swing(
    player_query: Query<&PlayerCombatState, With<Player>>,
    mut arms: Query<(&mut Transform, &ArmPart)>,
) {
    let Ok(combat) = player_query.single() else {
        return;
    };

    for (mut transform, arm_part) in &mut arms {
        // Анимируем только правую руку
        if arm_part.side != ArmSide::Right {
            continue;
        }

        let target_position = match &combat.state {
            CombatState::Attacking {
                timer, duration, ..
            } => {
                // Прогресс атаки 0.0 → 1.0
                let progress = *timer / *duration;

                // Удар: быстро вперёд в первой половине, возврат во второй
                let punch_amount = if progress < 0.4 {
                    // 0.0 → 0.4: быстро вперёд (ease out)
                    (progress / 0.4).sqrt()
                } else {
                    // 0.4 → 1.0: плавный возврат
                    1.0 - ((progress - 0.4) / 0.6).powi(2)
                };

                Vec3::new(
                    BASE_POSITION_RIGHT.x - PUNCH_INWARD * punch_amount, // к центру
                    BASE_POSITION_RIGHT.y,
                    BASE_POSITION_RIGHT.z - PUNCH_FORWARD * punch_amount, // вперёд (-Z)
                )
            }
            _ => BASE_POSITION_RIGHT,
        };

        // Плавный переход
        transform.translation = transform.translation.lerp(target_position, 0.4);
    }
}
