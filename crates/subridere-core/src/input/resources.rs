use bevy::prelude::*;

/// Настройки управления
#[derive(Resource)]
pub struct InputSettings {
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
    pub key_bindings: KeyBindings,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.002,
            invert_y: false,
            key_bindings: KeyBindings::default(),
        }
    }
}

/// Привязки клавиш
#[derive(Clone)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub crouch: KeyCode,
    pub run: KeyCode,
    pub dash: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            crouch: KeyCode::ControlLeft,
            run: KeyCode::ShiftLeft,
            dash: KeyCode::AltLeft,
        }
    }
}
