// fighting/melee/state.rs

use bevy::prelude::*;

use crate::fighting::components::{
    ArmCombatState, AttackPhase, AttackType, ChargeConfig, CurrentAttackTimings, PlayerCombatState,
};
use crate::fighting::melee::{LeftAttackIntent, RightAttackIntent};
use crate::player::component::Player;

/// Система обработки боевых состояний (обе руки независимо)
pub fn process_combat_state(
    mut commands: Commands,
    time: Res<Time>,
    timings: Res<CurrentAttackTimings>,
    charge_config: Res<ChargeConfig>,
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
            &timings,
            &charge_config,
        );

        // Левая рука — независимо
        if left_intent.is_some() {
            commands.entity(entity).remove::<LeftAttackIntent>();
        }
        process_arm(
            &mut combat.left,
            left_intent.is_some(),
            "LEFT",
            dt,
            &timings,
            &charge_config,
        );
    }
}

fn process_arm(
    arm_state: &mut ArmCombatState,
    has_intent: bool,
    side_name: &str,
    dt: f32,
    timings: &CurrentAttackTimings,
    charge_config: &ChargeConfig,
) {
    match arm_state {
        ArmCombatState::Ready => {
            if has_intent {
                info!("⚔️ {} ARM: ATTACK START → Light Windup", side_name);
                // Пока без зарядки — сразу light attack
                // TODO: в следующем шаге добавим Charging
                *arm_state = ArmCombatState::Attacking {
                    attack_type: AttackType::Light,
                    phase: AttackPhase::Windup,
                    phase_timer: 0.0,
                    damage_dealt: false,
                    charge_level: 0.0,
                };
            }
        }

        ArmCombatState::Charging { charge_timer } => {
            // TODO: будет реализовано в следующем шаге
            // Пока просто переходим в атаку
            *charge_timer += dt;
        }

        ArmCombatState::Attacking {
            attack_type,
            phase,
            phase_timer,
            ..
        } => {
            *phase_timer += dt;

            // Выбираем тайминги в зависимости от типа атаки
            let attack_timings = match attack_type {
                AttackType::Light => &timings.light,
                AttackType::Heavy => &timings.heavy,
            };

            match phase {
                AttackPhase::Windup => {
                    if *phase_timer >= attack_timings.windup {
                        info!("⚔️ {} ARM: Windup → Active (hitbox ON)", side_name);
                        *phase = AttackPhase::Active;
                        *phase_timer = 0.0;
                    }
                }

                AttackPhase::Active => {
                    if *phase_timer >= attack_timings.active {
                        info!("⚔️ {} ARM: Active → Recovery (hitbox OFF)", side_name);
                        *phase = AttackPhase::Recovery;
                        *phase_timer = 0.0;
                    }
                }

                AttackPhase::Recovery => {
                    if *phase_timer >= attack_timings.recovery {
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
