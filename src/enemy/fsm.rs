use crate::enemy::component::*;
use bevy::prelude::*;

/// FSM-заглушка: враг всегда остаётся в Idle.
pub fn update_enemy_fsm(mut query: Query<&mut EnemyState, With<Enemy>>) {
    /*
    for mut state in &mut query {
        if *state != EnemyState::Idle {
            *state = EnemyState::Idle;
        }
    }*/
}
