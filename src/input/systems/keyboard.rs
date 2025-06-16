use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;

use crate::input::{
    component::{MovementInput, PlayerControlled},
    resources::InputSettings,
};

/// Система обработки клавиатурного ввода
pub fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<InputSettings>,
    mut query: Query<&mut MovementInput, With<PlayerControlled>>,
) {
    for mut input in &mut query {
        let bindings = &settings.key_bindings;
        
        // Направление движения
        let mut direction = Vec2::ZERO;
        
        if keys.pressed(bindings.move_forward) {
            direction.y += 1.0;
        }
        if keys.pressed(bindings.move_backward) {
            direction.y -= 1.0;
        }
        if keys.pressed(bindings.move_left) {
            direction.x -= 1.0;
        }
        if keys.pressed(bindings.move_right) {
            direction.x += 1.0;
        }
        
        // Нормализуем диагональное движение
        if direction.length() > 1.0 {
            direction = direction.normalize();
        }
        
        input.direction = direction;
        
        // Остальные действия
        input.jump = keys.just_pressed(bindings.jump);
        input.crouch = keys.pressed(bindings.crouch);
        input.run = keys.pressed(bindings.run);
        input.dash = keys.just_pressed(bindings.dash);
    }
}

/// Система обработки мыши
pub fn handle_mouse_input(
    mut mouse_motion: EventReader<MouseMotion>,
    settings: Res<InputSettings>,
    mut query: Query<&mut MovementInput, With<PlayerControlled>>,
) {
    let mut delta = Vec2::ZERO;
    
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }
    
    if delta != Vec2::ZERO {
        for mut input in &mut query {
            input.mouse_delta = delta * settings.mouse_sensitivity;
            
            // Инвертировать Y если нужно
            if settings.invert_y {
                input.mouse_delta.y = -input.mouse_delta.y;
            }
        }
    }
}

/// Система для захвата курсора мыши
pub fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            // Освободить курсор при нажатии Escape
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        } else if keys.just_pressed(KeyCode::Tab) {
            // Захватить курсор при нажатии Tab
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
    }
}
