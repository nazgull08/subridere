use bevy::prelude::*;


use crate::input::resources::InputSettings;

use super::systems::keyboard::{cursor_grab_system, handle_keyboard_input, handle_mouse_input};
use super::systems::movement::movement_system;
use super::systems::dash::dash_system;
use super::systems::jump::jump_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ресурсы
            .init_resource::<InputSettings>()
            
            // Системы ввода (выполняются первыми)
            .add_systems(Update, (
                handle_keyboard_input,
                handle_mouse_input,
                cursor_grab_system
            ).in_set(InputSet::Input))
            
            // Системы логики движения (выполняются после ввода)
            .add_systems(Update, (
                movement_system,
                jump_system,
                dash_system,
            ).in_set(InputSet::Movement).after(InputSet::Input));
    }
}

/// Наборы систем для правильного порядка выполнения
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSet {
    Input,      // Обработка ввода
    Movement,   // Логика движения
}
