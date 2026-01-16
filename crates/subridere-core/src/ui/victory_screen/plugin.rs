use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::app::AppState;

use super::spawn::{despawn_victory_screen, spawn_victory_screen};

pub struct VictoryScreenPlugin;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Victory),
            (spawn_victory_screen, show_cursor),
        )
        .add_systems(OnExit(AppState::Victory), despawn_victory_screen);

        info!("✅ Victory Screen plugin initialized");
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
