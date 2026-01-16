use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::app::AppState;
use crate::ui::game_menu::state::GameMenuState;
use crate::ui::system_menu::state::SystemMenuState;

/// Централизованное управление курсором
pub fn update_cursor_state(
    mut windows: Query<&mut Window>,
    app_state: Res<State<AppState>>,
    game_menu_state: Res<State<GameMenuState>>,
    system_menu_state: Res<State<SystemMenuState>>,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };

    // Курсор скрыт только в игре без открытых меню
    let should_hide = *app_state.get() == AppState::InGame
        && *game_menu_state.get() == GameMenuState::Closed
        && *system_menu_state.get() == SystemMenuState::Closed;

    if should_hide {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    } else {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
