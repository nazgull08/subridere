use bevy::prelude::*;

use crate::app::AppState;
use crate::player::component::Player;

use super::component::Health;

/// Регенерация здоровья
pub fn regenerate_health(mut query: Query<&mut Health>, time: Res<Time>) {
    for mut health in &mut query {
        if health.regen > 0.0 && health.current < health.max {
            health.current = (health.current + health.regen * time.delta_secs()).min(health.max);
        }
    }
}

/// Проверка смерти игрока
pub fn check_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(health) = player_query.single() else {
        return;
    };

    if health.is_dead() {
        info!("💀 Player died! HP: {}", health.current);
        next_state.set(AppState::Dead);
    }
}
