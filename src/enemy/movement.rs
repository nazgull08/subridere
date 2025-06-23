use bevy::prelude::*;
use crate::enemy::component::*;

const WALK_SPEED: f32 = 4.0; // Увеличена скорость движения

pub fn walk_movement_system(
    time: Res<Time>,
    mut query: Query<(&EnemyState, &mut Transform, Option<&TargetPos>)>,
) {
    for (state, mut transform, maybe_target) in &mut query {
        if *state != EnemyState::Walk {
            continue;
        }

        let Some(target) = maybe_target else { continue };

        let to_target = target.0 - transform.translation;
        let distance = to_target.length();

        if distance > 0.05 {
            let step = WALK_SPEED * time.delta_secs();
            let movement = to_target.normalize() * step.min(distance);
            transform.translation += movement;

            // Плавный разворот к цели
            let direction = to_target.normalize();
            if direction.length_squared() > 0.01 {
                let target_rot = Quat::from_rotation_y(-direction.x.atan2(direction.z));
                transform.rotation = transform.rotation.slerp(target_rot, 0.1); // плавно поворачиваем
            }
        }
    }
}
