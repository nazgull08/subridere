use bevy::prelude::*;

use crate::input::component::PlayerControlled;
use crate::input::resources::InputSettings;
use crate::unit::component::{DashIntent, JumpIntent, MoveIntent, ShootIntent};

/// Processes keyboard input and generates intent components.
pub fn handle_keyboard_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<InputSettings>,
    query: Query<Entity, With<PlayerControlled>>, // ← ensure included
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

pub fn handle_shoot_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    player_query: Query<Entity, With<PlayerControlled>>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        if let (Ok(camera), Ok(player_entity)) = (camera_query.single(), player_query.single()) {
            let direction = camera.forward();
            commands
                .entity(player_entity)
                .insert(ShootIntent(*direction));
            println!("🔫 ShootIntent created");
        }
    }
}
