use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::player::component::Player;
use super::components::{WormAI, WormHead, WormState};
use super::rotation::is_facing_target;

const MOVE_FORCE_MULTIPLIER: f32 = 1.0;

pub fn worm_move_forward(
    heads: Query<(&Transform, &WormState, &WormAI, &Velocity), With<WormHead>>,
    mut forces: Query<&mut ExternalForce, With<WormHead>>,
    targets: Query<&Transform, (With<Player>, Without<WormHead>)>,
    time: Res<Time>,
) {
    for ((head_transform, state, ai, velocity), mut force) in heads.iter().zip(forces.iter_mut()) {
        
        // ✅ Don't move during attack states
        match state {
            WormState::PrepareAttack { .. } 
            | WormState::Lunging { .. }
            | WormState::Recovering { .. } => {
                force.force = Vec3::ZERO;
                continue;
            }
            _ => {}
        }
        
        if let WormState::Chase { target } = state {
            if let Ok(target_transform) = targets.get(*target) {
                let target_pos = target_transform.translation;
                
                if is_facing_target(head_transform, target_pos) {
                    let to_target = target_pos - head_transform.translation;
                    let direction = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();
                    
                    if direction.length_squared() > 0.001 {
                        force.force = direction * ai.move_force * MOVE_FORCE_MULTIPLIER;
                        
                        if to_target.y > 1.0 {
                            force.force.y += ai.move_force * 0.2;
                        }

                        let elapsed = time.elapsed_secs() as i32;
                        if elapsed % 2 == 0 && time.delta_secs() < 0.02 {
                            let forward = Vec3::NEG_X;
                            let current_forward = head_transform.rotation * forward;
                            let fwd_xz = Vec3::new(current_forward.x, 0.0, current_forward.z).normalize_or_zero();
                            /*
                            info!(
                                "→ Moving vel={:.1} force={:.0} | fwd=[{:.2},{:.2}] dir=[{:.2},{:.2}]",
                                velocity.linvel.length(),
                                force.force.length(),
                                fwd_xz.x,
                                fwd_xz.z,
                                direction.x,
                                direction.z
                            );
                        */
                        }
                    }
                } else {
                    force.force = Vec3::ZERO;
                }
            }
        } else {
            force.force = Vec3::ZERO;
        }
    }
}
