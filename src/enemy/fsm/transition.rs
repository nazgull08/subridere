use std::time::Duration;
use bevy::prelude::*;
use crate::enemy::component::*;
use crate::unit::component::{AttackIntent, MoveIntent};

/// FSM-переходы: переключаем стейты и генерируем только атаки.
/// Патрульная цель хранится в EnemyMemory; переход в Attack происходит сразу, как только в памяти появился pursue_target.
pub fn enemy_state_transition_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut EnemyState,
        &mut StateTimer,
        &mut EnemyMemory,
        Option<&Transform>,
    )>,
) {
    for (entity, mut state, mut timer, mut memory, maybe_tf) in &mut q {
        timer.0.tick(time.delta());

        match *state {
            EnemyState::Idle => {
                if timer.0.finished() {
                    *state = EnemyState::MovingToTarget;
                    timer.0.set_duration(Duration::from_secs(4));
                    timer.0.reset();
                    tracing::info!(?entity, "Idle → Walk");
                }
            }

            EnemyState::MovingToTarget => {
            }


            EnemyState::Attack(EnemyAttackState::Bite | EnemyAttackState::Slash) => {
                if timer.0.finished() {
                    *state = EnemyState::Attack(EnemyAttackState::Cooldown);
                    timer.0.set_duration(Duration::from_secs(1));
                    timer.0.reset();
                    tracing::info!(?entity, "Attack → Cooldown");
                }
            }

            EnemyState::Attack(EnemyAttackState::Cooldown) => {
                if timer.0.finished() {
                    *state = EnemyState::Idle;
                    timer.0.set_duration(Duration::from_secs(2));
                    timer.0.reset();
                    tracing::info!(?entity, "Cooldown → Idle");
                }
            }

            EnemyState::Dead => {
                // Можно очистить интенты здесь
                commands.entity(entity).remove::<MoveIntent>();
                commands.entity(entity).remove::<AttackIntent>();
            }
        }
    }
}
