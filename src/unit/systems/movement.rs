use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::player::plugin::Player;
use crate::unit::component::Unit;

pub fn movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ctrl_query: Query<(&mut KinematicCharacterController, &Transform), (With<Player>, With<Unit>)>,
) {
    let (mut controller, transform) = match ctrl_query.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut dir = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { dir += *transform.forward(); }
    if keys.pressed(KeyCode::KeyS) { dir -= *transform.forward(); }
    if keys.pressed(KeyCode::KeyA) { dir -= *transform.right(); }
    if keys.pressed(KeyCode::KeyD) { dir += *transform.right(); }

    let speed = if keys.pressed(KeyCode::ShiftLeft) { 9.0 } else { 6.0 };

    controller.translation = Some(dir.normalize_or_zero() * speed * time.delta_secs());
}


pub fn jump_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut KinematicCharacterController, &KinematicCharacterControllerOutput), With<Player>>,
) {
    for (mut ctrl, output) in &mut q {
        if keys.just_pressed(KeyCode::Space) && output.grounded {
            ctrl.translation = Some(Vec3::Y * 0.4); // Или переменная скорость прыжка
        }
    }
}
