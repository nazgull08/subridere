use super::components::{WormAI, WormHead, WormState};
use crate::player::component::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

// Detection - правильная версия
pub fn worm_detect_targets(
    mut heads: Query<(&Transform, &mut WormAI), With<WormHead>>,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    for (head_transform, mut ai) in &mut heads {
        let mut closest: Option<(Entity, f32)> = None;

        // Find closest player
        for (player_entity, player_transform) in &players {
            let distance = head_transform
                .translation
                .distance(player_transform.translation);

            if distance <= ai.detection_range {
                match closest {
                    Some((_, current_closest)) if distance < current_closest => {
                        closest = Some((player_entity, distance));
                    }
                    None => {
                        closest = Some((player_entity, distance));
                    }
                    _ => {}
                }
            }
        }

        ai.target = closest.map(|(e, _)| e);
    }
}

// State updates
pub fn worm_update_state(
    mut heads: Query<(&Transform, &mut WormState, &WormAI), With<WormHead>>,
    targets: Query<&Transform, (With<Player>, Without<WormHead>)>,
) {
    for (head_transform, mut state, ai) in &mut heads {
        let new_state = if let Some(target_entity) = ai.target {
            if let Ok(target_transform) = targets.get(target_entity) {
                let distance = head_transform
                    .translation
                    .distance(target_transform.translation);

                if distance <= ai.attack_range {
                    WormState::Attack {
                        target: target_entity,
                    }
                } else {
                    WormState::Chase {
                        target: target_entity,
                    }
                }
            } else {
                WormState::Idle
            }
        } else {
            WormState::Idle
        };

        if !matches!(&*state, s if std::mem::discriminant(s) == std::mem::discriminant(&new_state))
        {
            info!("Worm state: {:?} -> {:?}", *state, new_state);
            *state = new_state;
        }
    }
}

pub fn worm_move_system(
    heads: Query<(&Transform, &WormState), (With<WormHead>, Without<Player>)>,
    mut controllers: Query<&mut KinematicCharacterController, With<WormHead>>,
    targets: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for ((head_transform, state), mut controller) in heads.iter().zip(controllers.iter_mut()) {
        if let WormState::Chase { target } = state {
            if let Ok(target_transform) = targets.get(*target) {
                let to_target = target_transform.translation - head_transform.translation;
                let direction_xz = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();

                let move_speed = 2.0;
                controller.translation = Some(direction_xz * move_speed * time.delta_secs());
            }
        }
    }
}

pub fn worm_rotate_system(
    mut heads: Query<(&mut Transform, &WormState), (With<WormHead>, Without<Player>)>,
    targets: Query<&Transform, With<Player>>,
) {
    for (mut head_transform, state) in &mut heads {
        if let WormState::Chase { target } = state {
            if let Ok(target_transform) = targets.get(*target) {
                let to_target = target_transform.translation - head_transform.translation;
                let direction_xz = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();

                if direction_xz.length_squared() > 0.001 {
                    let target_rotation = Quat::from_rotation_arc(Vec3::X, direction_xz);
                    head_transform.rotation = head_transform.rotation.lerp(target_rotation, 0.1);
                }
            }
        }
    }
}

// Attack
pub fn worm_attack_behavior(
    heads: Query<(&Transform, &WormState), (With<WormHead>, Without<Player>)>,
    targets: Query<&Transform, With<Player>>,
) {
    for (head_transform, state) in &heads {
        if let WormState::Attack { target } = state {
            if let Ok(target_transform) = targets.get(*target) {
                info!(
                    "Worm attacking target at distance: {:.2}",
                    head_transform
                        .translation
                        .distance(target_transform.translation)
                );
            }
        }
    }
}
