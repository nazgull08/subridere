use bevy::prelude::*;
use bevy::scene::SceneRoot;

pub fn spawn_test_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle = asset_server.load("models/char3.glb#Scene0");

    commands.spawn((
        SceneRoot(scene_handle),
        Transform::from_xyz(0.0, 2.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        Name::new("TestModel"),
    ));
}

