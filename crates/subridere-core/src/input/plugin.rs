use bevy::prelude::*;

use crate::app::AppState;
use crate::input::resources::InputSettings;
use crate::ui::game_menu::game_menu_closed;
use crate::ui::system_menu::system_menu_closed;

use super::cursor::update_cursor_state;
use super::systems::keyboard::{handle_keyboard_input};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputSettings>()
            // Cursor management - всегда активно
            .add_systems(Update, update_cursor_state)
            // Game input - только в игре без меню
            .add_systems(
                Update,
                (
                    handle_keyboard_input,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(game_menu_closed)
                    .run_if(system_menu_closed),
            );
    }
}
