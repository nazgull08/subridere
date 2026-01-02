use super::components::{WormAI, WormHead};
use crate::player::component::Player;
use bevy::prelude::*;

/// Detects nearest player within detection range
pub fn worm_detect_targets(
    mut heads: Query<(&Transform, &mut WormAI), With<WormHead>>,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    for (head_transform, mut ai) in &mut heads {
        let mut closest: Option<(Entity, f32)> = None;

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

        // Log detection changes
        if let Some((entity, distance)) = closest {
            if ai.target != Some(entity) {
                info!("🎯 Worm detected player at {:.1}m", distance);
            }
        } else if ai.target.is_some() {
            info!("❌ Worm lost target");
        }

        ai.target = closest.map(|(e, _)| e);
    }
}
