use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::scene::SceneRoot;

pub fn spawn_test_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle = asset_server.load("models/jester.glb#Scene0");


    commands
        .spawn((
            SceneRoot(scene_handle),
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.4)),
            GlobalTransform::default(),
            Name::new("Jester"),
            NoFrustumCulling
        ))
        .with_children(|parent| {
            parent.spawn((AnimationPlayer::default(), Name::new("JesterAnimationPlayer")));
        });
}
