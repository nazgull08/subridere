use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;
    let forward = transform.forward();
    let right = transform.right();

    if keys.pressed(KeyCode::W) {
        direction += forward;
    }
    if keys.pressed(KeyCode::S) {
        direction -= forward;
    }
    if keys.pressed(KeyCode::A) {
        direction -= right;
    }
    if keys.pressed(KeyCode::D) {
        direction += right;
    }

    direction.y = 0.0;

    if direction.length_squared() > 0.0 {
        transform.translation += direction.normalize() * time.delta_seconds() * 3.0;
    }
}
