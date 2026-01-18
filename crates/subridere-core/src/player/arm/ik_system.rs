// crates/subridere-core/src/player/arm/ik_system.rs
//
// IK системы для рук игрока

use bevy::prelude::*;
use block_bodies_core::solve_arm_ik;

use super::components::*;
use crate::fighting::components::{
    ArmCombatState, AttackPhase, AttackTimings, AttackType, ChargeConfig, CurrentAttackTimings,
    PlayerCombatState, WeaponKind,
};
use crate::player::component::Player;

// ═══════════════════════════════════════════════════════════════════
// IK TARGET UPDATE
// ═══════════════════════════════════════════════════════════════════

pub fn update_ik_target_from_combat(
    player_query: Query<&PlayerCombatState, With<Player>>,
    timings: Res<CurrentAttackTimings>,
    charge_config: Res<ChargeConfig>,
    mut ik_targets: Query<&mut IkTarget>,
) {
    let Ok(combat) = player_query.single() else {
        return;
    };

    for mut ik_target in &mut ik_targets {
        // Получаем состояние нужной руки
        let (arm_state, weapon_kind) = match ik_target.side {
            ArmSide::Right => (&combat.right, timings.right_weapon),
            ArmSide::Left => (&combat.left, timings.left_weapon),
        };

        // Вычисляем позу (всегда как для правой)
        let pose = compute_arm_pose(
            arm_state,
            weapon_kind,
            &timings,
            &charge_config,
            ik_target.side,
        );

        // Зеркалим если левая рука
        let pose = match ik_target.side {
            ArmSide::Right => pose,
            ArmSide::Left => pose.mirror(),
        };

        // Интерполяция позиции и elbow_hint
        ik_target.position = ik_target.position.lerp(pose.hand_offset, 0.3);
        ik_target.elbow_hint = ik_target.elbow_hint.lerp(pose.elbow_hint, 0.3);
        
        // Интерполяция ротации кисти (slerp для плавности)
        ik_target.hand_rotation = ik_target.hand_rotation.slerp(pose.hand_rotation, 0.3);
    }
}

// ═══════════════════════════════════════════════════════════════════
// POSE COMPUTATION
// ═══════════════════════════════════════════════════════════════════

fn compute_arm_pose(
    state: &ArmCombatState,
    default_weapon: WeaponKind,
    timings: &CurrentAttackTimings,
    charge_config: &ChargeConfig,
    side: ArmSide,
) -> ArmPose {
    match state {
        ArmCombatState::Ready => idle_pose(default_weapon),

        ArmCombatState::Charging {
            charge_timer,
            weapon_kind,
        } => {
            let weapon = *weapon_kind;
            let t = (*charge_timer / charge_config.heavy_threshold).clamp(0.0, 1.0);
            idle_pose(weapon).lerp(&heavy_charging_pose(weapon), ease_out_quad(t))
        }

        ArmCombatState::Attacking {
            attack_type,
            phase,
            phase_timer,
            weapon_kind,
            ..
        } => {
            let weapon = *weapon_kind;
            let attack_timings = timings.get(side, *attack_type);

            match attack_type {
                AttackType::Light => {
                    compute_light_pose(weapon, *phase, *phase_timer, attack_timings)
                }
                AttackType::Heavy => {
                    compute_heavy_pose(weapon, *phase, *phase_timer, attack_timings)
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// POSE HELPERS BY WEAPON
// ═══════════════════════════════════════════════════════════════════

fn idle_pose(weapon: WeaponKind) -> ArmPose {
    match weapon {
        WeaponKind::Fists => ArmPose::idle_right(),
        WeaponKind::Sword => ArmPose::sword_idle_right(),
    }
}

fn heavy_charging_pose(weapon: WeaponKind) -> ArmPose {
    match weapon {
        WeaponKind::Fists => ArmPose::fists_heavy_charging_right(),
        WeaponKind::Sword => ArmPose::sword_heavy_charging_right(),
    }
}

// ═══════════════════════════════════════════════════════════════════
// LIGHT ATTACK POSES
// ═══════════════════════════════════════════════════════════════════

fn compute_light_pose(
    weapon: WeaponKind,
    phase: AttackPhase,
    phase_timer: f32,
    timings: &AttackTimings,
) -> ArmPose {
    match weapon {
        WeaponKind::Fists => compute_fists_light_pose(phase, phase_timer, timings),
        WeaponKind::Sword => compute_sword_light_pose(phase, phase_timer, timings),
    }
}

fn compute_fists_light_pose(
    phase: AttackPhase,
    phase_timer: f32,
    timings: &AttackTimings,
) -> ArmPose {
    match phase {
        AttackPhase::Windup => {
            let progress = (phase_timer / timings.windup).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::idle_right().lerp(&ArmPose::fists_windup_right(), eased)
        }
        AttackPhase::Active => {
            let progress = (phase_timer / timings.active).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::fists_windup_right().lerp(&ArmPose::fists_punch_right(), eased)
        }
        AttackPhase::Recovery => {
            let progress = (phase_timer / timings.recovery).clamp(0.0, 1.0);
            let eased = ease_in_out_quad(progress);
            ArmPose::fists_punch_right().lerp(&ArmPose::idle_right(), eased)
        }
    }
}

fn compute_sword_light_pose(
    phase: AttackPhase,
    phase_timer: f32,
    timings: &AttackTimings,
) -> ArmPose {
    match phase {
        AttackPhase::Windup => {
            let progress = (phase_timer / timings.windup).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::sword_idle_right().lerp(&ArmPose::sword_windup_right(), eased)
        }
        AttackPhase::Active => {
            let progress = (phase_timer / timings.active).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::sword_windup_right().lerp(&ArmPose::sword_slash_right(), eased)
        }
        AttackPhase::Recovery => {
            let progress = (phase_timer / timings.recovery).clamp(0.0, 1.0);
            let eased = ease_in_out_quad(progress);
            ArmPose::sword_slash_right().lerp(&ArmPose::sword_idle_right(), eased)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// HEAVY ATTACK POSES
// ═══════════════════════════════════════════════════════════════════

fn compute_heavy_pose(
    weapon: WeaponKind,
    phase: AttackPhase,
    phase_timer: f32,
    timings: &AttackTimings,
) -> ArmPose {
    match weapon {
        WeaponKind::Fists => compute_fists_heavy_pose(phase, phase_timer, timings),
        WeaponKind::Sword => compute_sword_heavy_pose(phase, phase_timer, timings),
    }
}

fn compute_fists_heavy_pose(
    phase: AttackPhase,
    phase_timer: f32,
    timings: &AttackTimings,
) -> ArmPose {
    match phase {
        AttackPhase::Windup => {
            let progress = (phase_timer / timings.windup).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::fists_heavy_charging_right().lerp(&ArmPose::fists_heavy_windup_right(), eased)
        }
        AttackPhase::Active => {
            let progress = (phase_timer / timings.active).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::fists_heavy_windup_right().lerp(&ArmPose::fists_heavy_active_right(), eased)
        }
        AttackPhase::Recovery => {
            let progress = (phase_timer / timings.recovery).clamp(0.0, 1.0);
            let eased = ease_in_out_quad(progress);
            ArmPose::fists_heavy_active_right().lerp(&ArmPose::idle_right(), eased)
        }
    }
}

fn compute_sword_heavy_pose(
    phase: AttackPhase,
    phase_timer: f32,
    timings: &AttackTimings,
) -> ArmPose {
    match phase {
        AttackPhase::Windup => {
            let progress = (phase_timer / timings.windup).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::sword_heavy_charging_right().lerp(&ArmPose::sword_heavy_windup_right(), eased)
        }
        AttackPhase::Active => {
            let progress = (phase_timer / timings.active).clamp(0.0, 1.0);
            let eased = ease_out_quad(progress);
            ArmPose::sword_heavy_windup_right().lerp(&ArmPose::sword_heavy_slash_right(), eased)
        }
        AttackPhase::Recovery => {
            let progress = (phase_timer / timings.recovery).clamp(0.0, 1.0);
            let eased = ease_in_out_quad(progress);
            ArmPose::sword_heavy_slash_right().lerp(&ArmPose::sword_idle_right(), eased)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// IK SOLVER APPLICATION
// ═══════════════════════════════════════════════════════════════════

pub fn apply_arm_ik(
    config: Res<ArmConfig>,
    ik_target_query: Query<&IkTarget>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    shoulder_query: Query<(Entity, &GlobalTransform, &Shoulder)>,
    mut upper_arm_query: Query<&mut Transform, (With<UpperArm>, Without<Forearm>, Without<Hand>)>,
    mut forearm_query: Query<&mut Transform, (With<Forearm>, Without<UpperArm>, Without<Hand>)>,
    mut hand_query: Query<&mut Transform, (With<Hand>, Without<UpperArm>, Without<Forearm>)>,
    children_query: Query<&Children>,
) {
    let Ok(camera_global) = camera_query.single() else {
        return;
    };

    for (shoulder_entity, shoulder_global, shoulder) in &shoulder_query {
        let Some(ik_target) = ik_target_query.iter().find(|t| t.side == shoulder.side) else {
            continue;
        };

        // === ПОЗИЦИИ ===
        let shoulder_pos = shoulder_global.translation();
        let hand_target_world = camera_global.transform_point(ik_target.position);
        let elbow_hint_world = camera_global.transform_point(ik_target.elbow_hint);

        // === IK SOLVE ===
        let ik_result = solve_arm_ik(
            shoulder_pos,
            config.upper_arm_length,
            config.forearm_length,
            hand_target_world,
            elbow_hint_world,
        );

        // === ПРИМЕНЕНИЕ РОТАЦИЙ ===
        let Ok(shoulder_children) = children_query.get(shoulder_entity) else {
            continue;
        };

        let (_, shoulder_rot, _) = shoulder_global.to_scale_rotation_translation();

        for upper_arm_entity in shoulder_children.iter() {
            let Ok(mut upper_transform) = upper_arm_query.get_mut(upper_arm_entity) else {
                continue;
            };

            let local_upper_rot = shoulder_rot.inverse() * ik_result.upper_rotation;
            upper_transform.rotation = local_upper_rot;

            let Ok(upper_children) = children_query.get(upper_arm_entity) else {
                continue;
            };

            for forearm_entity in upper_children.iter() {
                let Ok(mut forearm_transform) = forearm_query.get_mut(forearm_entity) else {
                    continue;
                };

                forearm_transform.rotation = ik_result.lower_rotation;
                forearm_transform.translation = Vec3::new(0.0, 0.0, -config.upper_arm_length);

                let Ok(forearm_children) = children_query.get(forearm_entity) else {
                    continue;
                };

                for hand_entity in forearm_children.iter() {
                    if let Ok(mut hand_transform) = hand_query.get_mut(hand_entity) {
                        hand_transform.translation = Vec3::new(0.0, 0.0, -config.forearm_length);
                        
                        // === ПРИМЕНЯЕМ РОТАЦИЮ КИСТИ ===
                        hand_transform.rotation = ik_target.hand_rotation;
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// EASING
// ═══════════════════════════════════════════════════════════════════

fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}
