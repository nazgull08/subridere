use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::ui::game_menu::GameMenuState;

use super::spawn::{despawn_system_menu, spawn_system_menu};
use super::state::SystemMenuState;

pub struct SystemMenuPlugin;

impl Plugin for SystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SystemMenuState>()
            .add_systems(Update, handle_escape_input)
            .add_systems(
                OnEnter(SystemMenuState::Open),
                (spawn_system_menu, show_cursor),
            )
            .add_systems(
                OnExit(SystemMenuState::Open),
                (despawn_system_menu, hide_cursor),
            );

        info!("✅ System Menu plugin initialized");
    }
}

fn handle_escape_input(
    keys: Res<ButtonInput<KeyCode>>,
    game_menu_state: Res<State<GameMenuState>>,
    system_menu_state: Res<State<SystemMenuState>>,
    mut next_system_menu: ResMut<NextState<SystemMenuState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    // Если game_menu открыт — не открываем system_menu (game_menu сам закроется)
    if *game_menu_state.get() == GameMenuState::Open {
        return;
    }

    // Toggle system menu
    match system_menu_state.get() {
        SystemMenuState::Closed => {
            info!("⚙️ Opening system menu");
            next_system_menu.set(SystemMenuState::Open);
        }
        SystemMenuState::Open => {
            info!("⚙️ Closing system menu");
            next_system_menu.set(SystemMenuState::Closed);
        }
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

fn hide_cursor(mut windows: Query<&mut Window>, game_menu_state: Res<State<GameMenuState>>) {
    // Не скрываем курсор если game_menu ещё открыт
    if *game_menu_state.get() == GameMenuState::Open {
        return;
    }

    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    }
}
