use bevy::prelude::*;

use crate::input::resources::InputSettings;
use crate::ui::inventory::inventory_closed;

use super::systems::keyboard::{handle_keyboard_input, handle_shoot_input};
use super::systems::weapon_switch::weapon_switch_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputSettings>().add_systems(
            Update,
            (
                handle_keyboard_input.run_if(inventory_closed),
                handle_shoot_input.run_if(inventory_closed),
                weapon_switch_system.run_if(inventory_closed),
            ),
        );
    }
}
