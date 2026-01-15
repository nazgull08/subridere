use bevy::prelude::*;

use crate::camera::flycam::fly_camera_input;
use crate::ui::game_menu::game_menu_closed;
use crate::ui::inventory::inventory_closed;

use super::controller::grab_mouse;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (grab_mouse, fly_camera_input)
                .run_if(inventory_closed)
                .run_if(game_menu_closed),
        );
    }
}
