use bevy::prelude::*;

use crate::app::AppState;
use crate::ui::game_menu::GameMenuState;

use super::spawn::{despawn_system_menu, spawn_system_menu};
use super::state::SystemMenuState;

pub struct SystemMenuPlugin;

impl Plugin for SystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SystemMenuState>()
            .add_systems(
                Update,
                handle_escape_input.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(SystemMenuState::Open), spawn_system_menu)
            .add_systems(OnExit(SystemMenuState::Open), despawn_system_menu);

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

    // Если game_menu открыт — не открываем system_menu
    if *game_menu_state.get() == GameMenuState::Open {
        return;
    }

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
