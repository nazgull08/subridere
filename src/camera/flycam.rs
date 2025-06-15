use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,
    pub sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 5.0,
            sensitivity: 0.15,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

pub fn fly_camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    // суммируем за кадр
    let mut mouse_delta = Vec2::ZERO;
    for ev in mouse_motion_events.read() {
        mouse_delta += ev.delta;
    }

    for (mut transform, mut cam) in &mut query {
        /* ---------- движение ---------- */
        let mut dir = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { dir += *transform.forward(); }
        if keys.pressed(KeyCode::KeyS) { dir -= *transform.forward(); }
        if keys.pressed(KeyCode::KeyA) { dir -= *transform.right();   }
        if keys.pressed(KeyCode::KeyD) { dir += *transform.right();   }
        if keys.pressed(KeyCode::Space)     { dir += Vec3::Y; }
        if keys.pressed(KeyCode::ShiftLeft) { dir -= Vec3::Y; }

        if keys.just_pressed(KeyCode::Minus) {
            cam.sensitivity -= 0.01;
            println!("↓ Sensitivity: {}", cam.sensitivity);
        }
        if keys.just_pressed(KeyCode::Equal) {
            cam.sensitivity += 0.01;
            println!("↑ Sensitivity: {}", cam.sensitivity);
        }

        transform.translation += dir.normalize_or_zero() * cam.speed * time.delta_secs();

        /* ---------- вращение ---------- */
        if mouse_delta.length_squared() > 0.0 {
            cam.yaw   -= mouse_delta.x * cam.sensitivity;
            cam.pitch -= mouse_delta.y * cam.sensitivity;
            cam.pitch = cam.pitch.clamp(-89.0, 89.0);

            transform.rotation =
                Quat::from_rotation_y(cam.yaw.to_radians()) *
                Quat::from_rotation_x(cam.pitch.to_radians());
        }
    }
}
