use crate::camera::flycam::fly_camera_input;
use bevy::prelude::*;

use super::controller::grab_mouse;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grab_mouse)
            .add_systems(Update, fly_camera_input);
    }
}
