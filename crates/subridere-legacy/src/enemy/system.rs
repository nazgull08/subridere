use bevy::prelude::*;
use crate::enemy::component::*;
use crate::block_bodies::animation::component::AnimationCycle;
use crate::block_bodies::pose::BlockPose;
use crate::unit::component::{Unit, Velocity};

pub fn update_enemy_animation_on_state_change(
    mut commands: Commands,
    mut query: Query<(Entity, &EnemyState, Option<&mut AnimationCycle>, Option<&AnimationKind>), Changed<EnemyState>>,
) {
    for (entity, state, _maybe_cycle, current_kind) in &mut query {
        let desired = match state {
            EnemyState::Idle => AnimationKind::Idle,
            EnemyState::MovingToTarget => AnimationKind::Walk,
            EnemyState::Attack(attack_state) => match attack_state {
                EnemyAttackState::Bite => AnimationKind::BiteAttack,
                EnemyAttackState::Slash => AnimationKind::SlashAttack,
                _ => continue, // не меняем анимацию для Cooldown
            },
            EnemyState::Dead => continue,
        };

        if current_kind == Some(&desired) {
            continue;
        }

        let pose_duration = match desired {
            AnimationKind::BiteAttack => 0.2, // быстро!
            AnimationKind::SlashAttack => 0.4,
            AnimationKind::Walk => 0.4,
            AnimationKind::Idle => 0.5,
        };

        let poses = match desired {
            AnimationKind::Idle => load_poses("idle"),
            AnimationKind::Walk => load_poses("walk"),
            AnimationKind::BiteAttack => load_poses("bite"),
            AnimationKind::SlashAttack => load_poses("slash"),
        };

        commands.entity(entity)
            .insert(AnimationCycle::new(poses, pose_duration))
            .insert(desired);
    }
}

fn load_poses(tag: &str) -> Vec<BlockPose> {
    use std::fs;

    let dir = format!("assets/poses/jimbo/{}", tag);
    let mut files = fs::read_dir(&dir)
        .expect("directory exists")
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|p| p.extension().map(|ext| ext == "ron").unwrap_or(false))
        .collect::<Vec<_>>();

    files.sort(); // чтобы порядок был: prepare → attack → recover

    files.into_iter()
        .map(|p| BlockPose::from_ron_file(p.to_str().unwrap().to_string()).unwrap())
        .collect()
}

pub fn apply_steering_intents_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &SteeringIntent, &mut Velocity), (With<Enemy>, With<Unit>)>,
) {
    let dt = time.delta_secs();

    for (entity, intent, mut velocity) in &mut query {
        let desired = intent.desired_velocity;
        velocity.0.x = velocity.0.x.lerp(desired.x, 10.0 * dt);
        velocity.0.z = velocity.0.z.lerp(desired.z, 10.0 * dt);

        commands.entity(entity).remove::<SteeringIntent>();
    }
}

pub fn rotate_enemy_towards_velocity_system(
    mut query: Query<(&Velocity, &mut Transform), With<Enemy>>,
) {
    for (vel, mut tf) in &mut query {
        // Только горизонтальное направление
        let flat = Vec3::new(vel.0.x, 0.0, vel.0.z);

        // Не вращаемся, если скорость почти нулевая
        if flat.length_squared() < 0.01 {
            continue;
        }

        // Целевой поворот: вперёд = -Z в Bevy
        let target_dir = flat.normalize();
        let yaw = target_dir.z.atan2(target_dir.x);
        let target_rot = Quat::from_rotation_y(-yaw);

        // Проверяем, не слишком ли мал угол поворота
        let dot = tf.rotation.dot(target_rot);
        if dot > 0.99 {
            continue; // почти совпадает, не трогаем
        }

        // Плавное вращение
        tf.rotation = tf.rotation.slerp(target_rot, 0.3);
    }
}


pub fn debug_enemy_axes(
    query: Query<&Transform, With<Enemy>>,
    mut gizmos: Gizmos,
) {
    for tf in &query {
        let pos = tf.translation;
        gizmos.arrow(pos, pos + tf.forward() * 2.0, Color::srgb(1.0, 0.0, 0.0));     // forward (-Z)
        gizmos.arrow(pos, pos + tf.right() * 2.0, Color::srgb(0.0, 1.0, 0.0));     // right (+X)
        gizmos.arrow(pos, pos + tf.up() * 2.0, Color::srgb(0.0, 0.0, 1.0));         // up (+Y)
    }
}
