use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_startup_system(grab_mouse)
            .add_system(camera_movement)
            .add_system(handle_mouse_toggle);
    }
}

// --- Камера

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// --- Движение

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

// --- Захват мыши

fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn handle_mouse_toggle(
    mut windows: Query<&mut Window>,
    keys: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Escape) {
        if window.cursor.grab_mode != CursorGrabMode::None {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        } else {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }
}
