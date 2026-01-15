use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use super::spawn::{despawn_game_menu, spawn_game_menu};
use super::state::{GameMenuState, game_menu_open};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenuState>()
            .add_systems(Update, toggle_game_menu_input)
            .add_systems(OnEnter(GameMenuState::Open), (spawn_game_menu, show_cursor))
            .add_systems(
                OnExit(GameMenuState::Open),
                (despawn_game_menu, hide_cursor),
            );

        info!("✅ Game Menu plugin initialized");
    }
}

fn toggle_game_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameMenuState>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameMenuState::Closed => {
                info!("🎮 Opening game menu");
                next_state.set(GameMenuState::Open);
            }
            GameMenuState::Open => {
                info!("🎮 Closing game menu");
                next_state.set(GameMenuState::Closed);
            }
        }
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

fn hide_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    }
}
