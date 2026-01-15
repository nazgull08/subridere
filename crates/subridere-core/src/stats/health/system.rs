use super::component::Health;
use bevy::prelude::*;

/// Регенерация здоровья
pub fn regenerate_health(mut query: Query<&mut Health>, time: Res<Time>) {
    for mut health in &mut query {
        if health.regen > 0.0 && health.current < health.max {
            health.current = (health.current + health.regen * time.delta_secs()).min(health.max);
        }
    }
}
