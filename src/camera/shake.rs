// src/camera/shake.rs
use bevy::prelude::*;
use rand::Rng;

use super::flycam::FlyCamera;

#[derive(Event)]
pub struct CameraShakeEvent;

#[derive(Component)]
pub struct CameraShake {
    pub timer: Timer,
}


pub fn start_camera_shake(
    mut commands: Commands,
    mut evr: EventReader<CameraShakeEvent>,
    query: Query<Entity, With<FlyCamera>>,
) {
    for _ in evr.read() {
        for entity in &query {
            println!("inserting entity CameraShake");
            commands.entity(entity).insert(CameraShake {
                timer: Timer::from_seconds(0.3, TimerMode::Once),
            });
        }
    }
}

pub fn apply_camera_shake(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut CameraShake)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut transform, mut shake) in &mut query {
        shake.timer.tick(time.delta());

        let strength = 0.1 * (1.0 - shake.timer.fraction()); // затухающая амплитуда
        let offset_x = rng.gen_range(-strength..=strength);
        let offset_y = rng.gen_range(-strength..=strength);

        transform.translation.x += offset_x;
        transform.translation.y += offset_y;

        if shake.timer.finished() {
            commands.entity(entity).remove::<CameraShake>();
        }
        println!("Shaking camera");
    }
}
