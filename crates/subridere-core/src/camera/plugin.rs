use bevy::prelude::*;

use crate::app::AppState;
use crate::camera::flycam::fly_camera_input;
use crate::ui::game_menu::game_menu_closed;
use crate::ui::system_menu::system_menu_closed;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            fly_camera_input // ← УБРАЛИ grab_mouse
                .run_if(in_state(AppState::InGame))
                .run_if(game_menu_closed)
                .run_if(system_menu_closed),
        );
    }
}
