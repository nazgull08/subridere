use bevy::prelude::*;

use crate::app::AppState;

use super::spawn::{despawn_main_menu, spawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);

        info!("✅ Main Menu plugin initialized");
    }
}
