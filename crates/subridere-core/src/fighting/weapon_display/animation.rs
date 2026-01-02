use super::component::WeaponDisplay;
use bevy::prelude::*;

/// Animates weapon display: bob, rotation, and fade-in
pub fn animate_weapon_display(
    time: Res<Time>,
    mut weapons: Query<(
        &mut Transform,
        &mut WeaponDisplay,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut display, material_handle) in &mut weapons {
        // Update cooldown timer
        display.cooldown.tick(time.delta());

        // Bob animation (up/down floating)
        display.bob_timer += dt;
        let bob_offset = (display.bob_timer * 2.0).sin() * 0.1;

        // Slight horizontal sway
        let sway_offset = (display.bob_timer * 1.5).cos() * 0.05;

        // Apply position with bob and sway
        transform.translation = Vec3::new(
            display.base_position.x + sway_offset,
            display.base_position.y + bob_offset,
            display.base_position.z,
        );

        // Rotation animation
        transform.rotate_y(display.rotation_speed * dt);

        // Fade-in animation when forming
        if display.forming {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let progress = display.cooldown.fraction(); // 0.0 → 1.0

                // Fade alpha
                let target_alpha = 1.0;
                let current_alpha = progress * target_alpha;
                material.base_color.set_alpha(current_alpha);

                // Scale from 50% to 100%
                let scale = 0.5 + progress * 0.5;
                transform.scale = Vec3::splat(scale);

                // Stop forming when cooldown finished
                if display.cooldown.finished() {
                    display.forming = false;
                    material.base_color.set_alpha(1.0);
                    transform.scale = Vec3::ONE;
                }
            }
        }
    }
}

/// Hides weapon display when shooting
pub fn hide_weapon_on_shoot(mut weapons: Query<(&mut Visibility, &mut WeaponDisplay)>) {
    for (mut visibility, mut display) in &mut weapons {
        if display.forming && display.cooldown.elapsed_secs() < 0.01 {
            // Just started forming - hide it
            *visibility = Visibility::Hidden;
        } else if display.forming && display.cooldown.fraction() > 0.1 {
            // Show after 10% of cooldown
            *visibility = Visibility::Visible;
        }
    }
}
