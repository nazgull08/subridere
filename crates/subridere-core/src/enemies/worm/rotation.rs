use super::components::{WormAI, WormHead, WormState};
use crate::player::component::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const ROTATION_TORQUE_STRENGTH: f32 = 250.0;
const ALIGNED_THRESHOLD: f32 = 0.15;

pub fn is_facing_target(head_transform: &Transform, target_pos: Vec3) -> bool {
    let to_target = target_pos - head_transform.translation;
    let target_dir = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();

    if target_dir.length_squared() < 0.001 {
        return true;
    }

    let forward = Vec3::NEG_X;
    let current_rotation = head_transform.rotation;
    let current_forward = current_rotation * forward;
    let current_dir = Vec3::new(current_forward.x, 0.0, current_forward.z).normalize_or_zero();

    if current_dir.length_squared() < 0.001 {
        return false;
    }

    let dot = current_dir.dot(target_dir).clamp(-1.0, 1.0);
    let angle = dot.acos();

    angle < ALIGNED_THRESHOLD
}

pub fn worm_rotate_to_target(
    heads: Query<(&Transform, &WormState, &WormAI), With<WormHead>>,
    mut forces: Query<&mut ExternalForce, With<WormHead>>,
    targets: Query<&Transform, (With<Player>, Without<WormHead>)>,
    time: Res<Time>,
) {
    for ((head_transform, state, ai), mut force) in heads.iter().zip(forces.iter_mut()) {
        // ✅ Don't rotate during attack states
        match state {
            WormState::PrepareAttack { .. }
            | WormState::Lunging { .. }
            | WormState::Recovering { .. } => {
                force.torque.y = 0.0;
                continue;
            }
            _ => {}
        }

        if let WormState::Chase { target } = state {
            if let Ok(target_transform) = targets.get(*target) {
                let target_pos = target_transform.translation;

                if is_facing_target(head_transform, target_pos) {
                    force.torque.y = 0.0;
                    continue;
                }

                let to_target = target_pos - head_transform.translation;
                let target_dir = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();

                if target_dir.length_squared() < 0.001 {
                    continue;
                }

                let forward = Vec3::NEG_X;
                let current_rotation = head_transform.rotation;
                let current_forward = current_rotation * forward;
                let current_dir =
                    Vec3::new(current_forward.x, 0.0, current_forward.z).normalize_or_zero();

                if current_dir.length_squared() < 0.001 {
                    continue;
                }

                let cross = current_dir.cross(target_dir);
                let dot = current_dir.dot(target_dir).clamp(-1.0, 1.0);
                let angle = dot.acos();

                force.torque.y = cross.y * angle * ROTATION_TORQUE_STRENGTH;

                let elapsed = time.elapsed_secs() as i32;
                if elapsed % 2 == 0 && time.delta_secs() < 0.02 {
                    /*
                                        info!(
                                            "↻ angle={:.1}° torque={:.0} | fwd=[{:.2},{:.2}] tgt=[{:.2},{:.2}]",
                                            angle.to_degrees(),
                                            force.torque.y,
                                            current_dir.x,
                                            current_dir.z,
                                            target_dir.x,
                                            target_dir.z
                                        );
                    */
                }
            }
        }
    }
}
