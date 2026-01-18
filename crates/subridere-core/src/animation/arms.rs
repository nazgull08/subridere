// animation/arms.rs

use bevy::prelude::*;

use crate::fighting::components::{
    AttackPhase, AttackTimings, CombatState, CurrentAttackTimings, PlayerCombatState,
};
use crate::player::body::components::{ArmPart, ArmSide};
use crate::player::component::Player;

/// Базовая позиция правой руки
const BASE_POSITION_RIGHT: Vec3 = Vec3::new(1.0, -0.8, -2.0);
/// Насколько рука отводится назад при замахе (Windup)
const WINDUP_BACK: f32 = 0.5;
/// Насколько рука выдвигается вперёд при ударе (Active)
const PUNCH_FORWARD: f32 = 1.5;
/// Насколько рука смещается к центру при ударе
const PUNCH_INWARD: f32 = 0.5;

/// Анимирует правую руку при атаке (Souls-like phases)
pub fn animate_arm_swing(
    player_query: Query<&PlayerCombatState, With<Player>>,
    timings: Res<CurrentAttackTimings>,
    mut arms: Query<(&mut Transform, &ArmPart)>,
) {
    let Ok(combat) = player_query.single() else {
        return;
    };

    let timings = &timings.0;

    for (mut transform, arm_part) in &mut arms {
        if arm_part.side != ArmSide::Right {
            continue;
        }

        let target_position = compute_arm_position(&combat.state, timings);
        transform.translation = transform.translation.lerp(target_position, 0.4);
    }
}

fn compute_arm_position(state: &CombatState, timings: &AttackTimings) -> Vec3 {
    match state {
        CombatState::Ready => BASE_POSITION_RIGHT,

        CombatState::Attacking {
            phase, phase_timer, ..
        } => match phase {
            AttackPhase::Windup => {
                let progress = (*phase_timer / timings.windup).clamp(0.0, 1.0);
                let amount = progress.sqrt();

                Vec3::new(
                    BASE_POSITION_RIGHT.x,
                    BASE_POSITION_RIGHT.y,
                    BASE_POSITION_RIGHT.z + WINDUP_BACK * amount,
                )
            }

            AttackPhase::Active => {
                let progress = (*phase_timer / timings.active).clamp(0.0, 1.0);
                let amount = progress.sqrt();

                let z = WINDUP_BACK - (WINDUP_BACK + PUNCH_FORWARD) * amount;
                let x_offset = PUNCH_INWARD * amount;

                Vec3::new(
                    BASE_POSITION_RIGHT.x - x_offset,
                    BASE_POSITION_RIGHT.y,
                    BASE_POSITION_RIGHT.z + z,
                )
            }

            AttackPhase::Recovery => {
                let progress = (*phase_timer / timings.recovery).clamp(0.0, 1.0);
                let amount = 1.0 - (1.0 - progress).powi(2);

                let punch_z = BASE_POSITION_RIGHT.z - PUNCH_FORWARD;
                let z = punch_z + (BASE_POSITION_RIGHT.z - punch_z) * amount;
                let x_offset = PUNCH_INWARD * (1.0 - amount);

                Vec3::new(BASE_POSITION_RIGHT.x - x_offset, BASE_POSITION_RIGHT.y, z)
            }
        },
    }
}
