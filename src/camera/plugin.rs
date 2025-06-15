use bevy::prelude::*;
use crate::camera::flycam::{FlyCamera, fly_camera_input};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fly_camera)
           .add_systems(Update, fly_camera_input);
    }
}

fn spawn_fly_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCamera::default(),
        Name::new("FlyCam"),
    ));
}
