// fighting/melee/state.rs

use bevy::prelude::*;

use crate::fighting::components::{
    ArmCombatState, AttackPhase, AttackTimings, CurrentAttackTimings, PlayerCombatState,
};
use crate::fighting::melee::{LeftAttackIntent, RightAttackIntent};
use crate::player::component::Player;

/// Система обработки боевых состояний (обе руки независимо)
pub fn process_combat_state(
    mut commands: Commands,
    time: Res<Time>,
    timings: Res<CurrentAttackTimings>,
    mut query: Query<
        (
            Entity,
            &mut PlayerCombatState,
            Option<&RightAttackIntent>,
            Option<&LeftAttackIntent>,
        ),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();
    let timings = &timings.0;

    for (entity, mut combat, right_intent, left_intent) in &mut query {
        // Правая рука — независимо
        if right_intent.is_some() {
            commands.entity(entity).remove::<RightAttackIntent>();
        }
        process_arm(
            &mut combat.right,
            right_intent.is_some(),
            "RIGHT",
            dt,
            timings,
        );

        // Левая рука — независимо
        if left_intent.is_some() {
            commands.entity(entity).remove::<LeftAttackIntent>();
        }
        process_arm(&mut combat.left, left_intent.is_some(), "LEFT", dt, timings);
    }
}

fn process_arm(
    arm_state: &mut ArmCombatState,
    has_intent: bool,
    side_name: &str,
    dt: f32,
    timings: &AttackTimings,
) {
    match arm_state {
        ArmCombatState::Ready => {
            if has_intent {
                info!("⚔️ {} ARM: ATTACK START → Windup", side_name);
                *arm_state = ArmCombatState::Attacking {
                    phase: AttackPhase::Windup,
                    phase_timer: 0.0,
                    damage_dealt: false,
                };
            }
        }

        ArmCombatState::Attacking {
            phase, phase_timer, ..
        } => {
            *phase_timer += dt;

            match phase {
                AttackPhase::Windup => {
                    if *phase_timer >= timings.windup {
                        info!("⚔️ {} ARM: Windup → Active (hitbox ON)", side_name);
                        *phase = AttackPhase::Active;
                        *phase_timer = 0.0;
                    }
                }

                AttackPhase::Active => {
                    if *phase_timer >= timings.active {
                        info!("⚔️ {} ARM: Active → Recovery (hitbox OFF)", side_name);
                        *phase = AttackPhase::Recovery;
                        *phase_timer = 0.0;
                    }
                }

                AttackPhase::Recovery => {
                    if *phase_timer >= timings.recovery {
                        info!("⚔️ {} ARM: Recovery → Ready", side_name);
                        *arm_state = ArmCombatState::Ready;
                    }
                }
            }
        }
    }
}

/// Проверка: находится ли рука в активной фазе
pub fn is_arm_in_active_phase(arm_state: &ArmCombatState) -> bool {
    matches!(
        arm_state,
        ArmCombatState::Attacking {
            phase: AttackPhase::Active,
            ..
        }
    )
}

/// Проверка: хотя бы одна рука в активной фазе
pub fn is_any_arm_active(combat: &PlayerCombatState) -> bool {
    is_arm_in_active_phase(&combat.right) || is_arm_in_active_phase(&combat.left)
}
