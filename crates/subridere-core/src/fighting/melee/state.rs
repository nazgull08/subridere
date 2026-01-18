// fighting/melee/state.rs

use bevy::prelude::*;

use crate::fighting::components::{
    ArmCombatState, AttackPhase, AttackType, ChargeConfig, CurrentAttackTimings, PlayerCombatState,
    WeaponKind,
};
use crate::fighting::melee::{AttackInputState, LeftAttackInput, RightAttackInput};
use crate::player::arm::ArmSide;
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
            Option<&RightAttackInput>,
            Option<&LeftAttackInput>,
        ),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();

    for (entity, mut combat, right_input, left_input) in &mut query {
        // Правая рука
        let (right_pressed, right_released) = extract_right_input(right_input);
        if right_input.is_some() {
            commands.entity(entity).remove::<RightAttackInput>();
        }
        process_arm(
            &mut combat.right,
            right_pressed,
            right_released,
            "RIGHT",
            ArmSide::Right,
            dt,
            &timings,
            &charge_config,
        );

        // Левая рука
        let (left_pressed, left_released) = extract_left_input(left_input);
        if left_input.is_some() {
            commands.entity(entity).remove::<LeftAttackInput>();
        }
        process_arm(
            &mut combat.left,
            left_pressed,
            left_released,
            "LEFT",
            ArmSide::Left,
            dt,
            &timings,
            &charge_config,
        );
    }
}

fn extract_right_input(input: Option<&RightAttackInput>) -> (bool, bool) {
    match input {
        Some(RightAttackInput(AttackInputState::Pressed)) => (true, false),
        Some(RightAttackInput(AttackInputState::Released)) => (false, true),
        None => (false, false),
    }
}

fn extract_left_input(input: Option<&LeftAttackInput>) -> (bool, bool) {
    match input {
        Some(LeftAttackInput(AttackInputState::Pressed)) => (true, false),
        Some(LeftAttackInput(AttackInputState::Released)) => (false, true),
        None => (false, false),
    }
}

fn process_arm(
    arm_state: &mut ArmCombatState,
    pressed: bool,
    released: bool,
    side_name: &str,
    side: ArmSide,
    dt: f32,
    timings: &CurrentAttackTimings,
    charge_config: &ChargeConfig,
) {
    // Получаем текущее оружие для этой руки
    let weapon_kind = timings.weapon(side);

    match arm_state {
        ArmCombatState::Ready => {
            if pressed {
                info!("⚔️ {} ARM: Ready → Charging ({:?})", side_name, weapon_kind);
                *arm_state = ArmCombatState::Charging {
                    charge_timer: 0.0,
                    weapon_kind,
                };
            }
        }

        ArmCombatState::Charging {
            charge_timer,
            weapon_kind: wk,
        } => {
            *charge_timer += dt;

            if released {
                // Определяем тип атаки по времени заряда
                let is_heavy = *charge_timer >= charge_config.heavy_threshold;
                let attack_type = if is_heavy {
                    AttackType::Heavy
                } else {
                    AttackType::Light
                };

                let charge_level = charge_config.charge_level(*charge_timer);

                let type_name = if is_heavy { "Heavy" } else { "Light" };
                info!(
                    "⚔️ {} ARM: Charging ({:.2}s) → {} Attack (charge: {:.0}%, {:?})",
                    side_name,
                    charge_timer,
                    type_name,
                    charge_level * 100.0,
                    wk
                );

                *arm_state = ArmCombatState::Attacking {
                    attack_type,
                    phase: AttackPhase::Windup,
                    phase_timer: 0.0,
                    damage_dealt: false,
                    charge_level,
                    weapon_kind: *wk,
                };
            }
        }

        ArmCombatState::Attacking {
            attack_type,
            phase,
            phase_timer,
            weapon_kind: wk,
            ..
        } => {
            *phase_timer += dt;

            // Получаем тайминги для этого оружия и типа атаки
            let attack_timings = timings.get(side, *attack_type);

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
