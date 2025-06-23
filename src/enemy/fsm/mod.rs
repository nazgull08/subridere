use bevy::prelude::*;
use crate::enemy::component::*;

pub mod plugin;
pub mod walk;
pub mod idle;

pub fn update_enemy_fsm_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EnemyState, &mut StateTimer, &mut Transform, Option<&TargetPos>), With<Enemy>>,
    mut commands: Commands,
) {
    for (entity, mut state, mut timer, transform, target) in &mut query {
        timer.0.tick(time.delta());

        match *state {
            EnemyState::Idle => {
                if timer.0.finished() {
                    // Выбрать новую цель
                    let offset = Vec3::new(
                        fastrand::f32() * 6.0 - 3.0,
                        0.0,
                        fastrand::f32() * 6.0 - 3.0,
                    );
                    let target_pos = transform.translation + offset;
                    commands.entity(entity).insert(TargetPos(target_pos));
                    *state = EnemyState::Walk;
                    timer.0 = Timer::from_seconds(10.0, TimerMode::Once); // запас на движение
                }
            }

            EnemyState::Walk => {
                let Some(target) = target else { continue };
                let dist = transform.translation.distance(target.0);
                if dist < 0.5 || timer.0.finished() {
                    // Достигли точки или вышло время — возвращаемся в Idle
                    commands.entity(entity).remove::<TargetPos>();
                    *state = EnemyState::Idle;
                    timer.0 = Timer::from_seconds(5.0, TimerMode::Once);
                }
            }
        }
    }
}
