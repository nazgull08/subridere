use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;

use crate::input::{
    component::MovementInput,
    resources::InputSettings,
};
use crate::unit::component::{JumpIntent, DashIntent, MoveIntent};

/// Processes keyboard input and generates intent components.
pub fn handle_keyboard_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<InputSettings>,
    query: Query<Entity, With<crate::input::component::PlayerControlled>>, // ← ensure included
) {
    let bindings = &settings.key_bindings;

    for entity in &query {
        let mut direction = Vec3::ZERO;

        if keys.pressed(bindings.move_forward) {
            direction.z -= 1.0;
        }
        if keys.pressed(bindings.move_backward) {
            direction.z += 1.0;
        }
        if keys.pressed(bindings.move_left) {
            direction.x -= 1.0;
        }
        if keys.pressed(bindings.move_right) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 1.0 {
            direction = direction.normalize();
        }

        if direction != Vec3::ZERO {
            commands.entity(entity).insert(MoveIntent(direction));
        }

        if keys.just_pressed(bindings.jump) {
            commands.entity(entity).insert(JumpIntent);
        }

        if keys.just_pressed(bindings.dash) {
            let dash_dir = if direction != Vec3::ZERO {
                direction
            } else {
                Vec3::NEG_Z
            };
            commands.entity(entity).insert(DashIntent(dash_dir));
        }
    }
}

/// Updates mouse delta used for look rotation.
pub fn handle_mouse_input(
    mut mouse_motion: EventReader<MouseMotion>,
    settings: Res<InputSettings>,
    mut query: Query<&mut MovementInput, With<crate::input::component::PlayerControlled>>,
) {
    let mut delta = Vec2::ZERO;

    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    if delta != Vec2::ZERO {
        for mut input in &mut query {
            input.mouse_delta = delta * settings.mouse_sensitivity;

            if settings.invert_y {
                input.mouse_delta.y = -input.mouse_delta.y;
            }
        }
    }
}

/// Toggles mouse grab with Escape/Tab keys.
pub fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        } else if keys.just_pressed(KeyCode::Tab) {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
    }
}
