use super::components::{WormAI, WormHead, WormState};
use crate::audio::worm::events::WormBiteEvent;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Executes the lunge attack - applies impulse and transitions to Recovering
pub fn worm_execute_lunge_system(
    mut heads: Query<(&Transform, &mut WormState, &WormAI), With<WormHead>>,
    mut impulses: Query<&mut ExternalImpulse, With<WormHead>>,
    mut bite_event: EventWriter<WormBiteEvent>,
) {
    for ((head_transform, mut state, ai), mut impulse) in heads.iter_mut().zip(impulses.iter_mut())
    {
        if let WormState::Lunging { target, target_pos } = *state {
            // Calculate direction to target (запомненная позиция)
            let to_target = target_pos - head_transform.translation;
            let horizontal_dir = Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero();

            if horizontal_dir.length_squared() < 0.001 {
                // Invalid direction - skip lunge
                info!("⚠️ Lunge canceled - invalid direction");
                *state = WormState::Recovering {
                    recovery_timer: ai.jump_recovery_time,
                };
                continue;
            }

            // Calculate impulse vector
            let horizontal_impulse = horizontal_dir * ai.jump_force;
            let vertical_impulse = Vec3::Y * ai.jump_height;
            let total_impulse = horizontal_impulse + vertical_impulse;

            // Apply impulse
            impulse.impulse = total_impulse;

            bite_event.send(WormBiteEvent);

            info!(
                "🚀 LUNGE! impulse=[{:.0},{:.0},{:.0}] to target at [{:.1},{:.1},{:.1}]",
                total_impulse.x,
                total_impulse.y,
                total_impulse.z,
                target_pos.x,
                target_pos.y,
                target_pos.z
            );

            // Immediately transition to Recovering
            *state = WormState::Recovering {
                recovery_timer: ai.jump_recovery_time,
            };
        }
    }
}

/// Visual feedback during PrepareAttack - red glow that intensifies
pub fn worm_prepare_visual_feedback(
    heads: Query<(&WormState, &WormAI, &MeshMaterial3d<StandardMaterial>), With<WormHead>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (state, ai, material_handle) in &heads {
        if let WormState::PrepareAttack { prepare_timer, .. } = state {
            // Calculate progress (0.0 at start → 1.0 when ready to jump)
            let progress = 1.0 - (prepare_timer / ai.jump_prepare_time);

            // Get material and update emissive
            if let Some(material) = materials.get_mut(&material_handle.0) {
                // Red glow that intensifies (0 → 8)
                let intensity = 8.0 * progress;
                material.emissive = LinearRgba::rgb(intensity, 0.0, 0.0);
            }
        } else {
            // Reset emissive when not preparing
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.emissive = LinearRgba::rgb(0.0, 0.0, 0.0);
            }
        }
    }
}
