use bevy::prelude::*;

use crate::input::resources::InputSettings;

use super::systems::keyboard::{cursor_grab_system, handle_keyboard_input, handle_shoot_input};
use super::systems::weapon_switch::weapon_switch_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputSettings>().add_systems(
            Update,
            (
                handle_keyboard_input,
                handle_shoot_input,
                cursor_grab_system,
                weapon_switch_system,
            ),
        );
    }
}
