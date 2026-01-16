use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::app::AppState;

use super::spawn::{despawn_main_menu, spawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), (spawn_main_menu, show_cursor))
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(OnEnter(AppState::InGame), hide_cursor);

        info!("✅ Main Menu plugin initialized");
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
        info!("👁️ Cursor shown");
    }
}

fn hide_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
        info!("🔒 Cursor hidden and locked");
    }
}
