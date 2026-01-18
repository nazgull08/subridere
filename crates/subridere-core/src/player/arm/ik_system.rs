// crates/subridere-core/src/player/arm/ik_system.rs
//
// IK системы для рук игрока

use bevy::prelude::*;
use block_bodies_core::solve_arm_ik;

use super::components::*;
use crate::fighting::components::{
    ArmCombatState, AttackPhase, AttackTimings, CurrentAttackTimings, PlayerCombatState,
};
use crate::player::component::Player;

// ═══════════════════════════════════════════════════════════════════
// IK TARGET UPDATE
// ═══════════════════════════════════════════════════════════════════

pub fn update_ik_target_from_combat(
    player_query: Query<&PlayerCombatState, With<Player>>,
    timings: Res<CurrentAttackTimings>,
    mut ik_targets: Query<&mut IkTarget>,
) {
    let Ok(combat) = player_query.single() else {
        return;
    };

    let timings = &timings.0;

    for mut ik_target in &mut ik_targets {
        // Получаем состояние нужной руки
        let arm_state = match ik_target.side {
            ArmSide::Right => &combat.right,
            ArmSide::Left => &combat.left,
        };

        // Вычисляем позу (всегда как для правой)
        let pose = compute_arm_pose(arm_state, timings);

        // Зеркалим если левая рука
        let pose = match ik_target.side {
            ArmSide::Right => pose,
            ArmSide::Left => pose.mirror(),
        };

        ik_target.position = ik_target.position.lerp(pose.hand_offset, 0.3);
        ik_target.elbow_hint = ik_target.elbow_hint.lerp(pose.elbow_hint, 0.3);
    }
}

fn compute_arm_pose(state: &ArmCombatState, timings: &AttackTimings) -> ArmPose {
    match state {
        ArmCombatState::Ready => ArmPose::idle_right(),

        ArmCombatState::Attacking {
            phase, phase_timer, ..
        } => match phase {
            AttackPhase::Windup => {
                let progress = (*phase_timer / timings.windup).clamp(0.0, 1.0);
                let eased = ease_out_quad(progress);
                ArmPose::idle_right().lerp(&ArmPose::windup_right(), eased)
            }

            AttackPhase::Active => {
                let progress = (*phase_timer / timings.active).clamp(0.0, 1.0);
                let eased = ease_out_quad(progress);
                ArmPose::windup_right().lerp(&ArmPose::punch_right(), eased)
            }

            AttackPhase::Recovery => {
                let progress = (*phase_timer / timings.recovery).clamp(0.0, 1.0);
                let eased = ease_in_out_quad(progress);
                ArmPose::punch_right().lerp(&ArmPose::idle_right(), eased)
            }
        },
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

            // Конвертируем мировую ротацию в локальную относительно shoulder
            let local_upper_rot = shoulder_rot.inverse() * ik_result.upper_rotation;
            upper_transform.rotation = local_upper_rot;

            // Forearm
            let Ok(upper_children) = children_query.get(upper_arm_entity) else {
                continue;
            };

            for forearm_entity in upper_children.iter() {
                let Ok(mut forearm_transform) = forearm_query.get_mut(forearm_entity) else {
                    continue;
                };

                forearm_transform.rotation = ik_result.lower_rotation;
                forearm_transform.translation = Vec3::new(0.0, 0.0, -config.upper_arm_length);

                // Hand
                let Ok(forearm_children) = children_query.get(forearm_entity) else {
                    continue;
                };

                for hand_entity in forearm_children.iter() {
                    if let Ok(mut hand_transform) = hand_query.get_mut(hand_entity) {
                        hand_transform.translation = Vec3::new(0.0, 0.0, -config.forearm_length);
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
