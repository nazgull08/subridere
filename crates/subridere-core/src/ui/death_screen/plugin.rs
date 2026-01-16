use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::app::AppState;

use super::spawn::{despawn_death_screen, spawn_death_screen};

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dead), (spawn_death_screen, show_cursor))
            .add_systems(OnExit(AppState::Dead), despawn_death_screen);

        info!("✅ Death Screen plugin initialized");
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
