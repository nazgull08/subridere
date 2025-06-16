use bevy::prelude::*;

use crate::input::component::{MovementInput, MovementStats, MovementState, PlayerControlled};

/// Система дэша/рывка
pub fn dash_system(
    mut query: Query<(
        &MovementInput,
        &MovementStats,
        &mut MovementState,
    ), With<PlayerControlled>>,
) {
    for (input, stats, mut state) in &mut query {
        // Дэш возможен при нажатии кнопки и если кулдаун прошел
        if input.dash && state.can_dash {
            // Определяем направление дэша
            let dash_direction = if input.direction.length() > 0.0 {
                // Дэш в направлении движения
                Vec3::new(input.direction.x, 0.0, -input.direction.y).normalize()
            } else {
                // Дэш вперед, если не двигаемся
                Vec3::new(0.0, 0.0, -1.0)
            };
            
            // Применяем силу дэша
            let dash_velocity = dash_direction * stats.dash_force;
            
            // Заменяем горизонтальную скорость на скорость дэша
            state.velocity.x = dash_velocity.x;
            state.velocity.z = dash_velocity.z;
            
            // Небольшой подъем, если на земле
            if state.is_grounded {
                state.velocity.y = stats.jump_force * 0.3;
            }
            
            // Запускаем кулдаун
            state.dash_timer = stats.dash_cooldown;
            state.can_dash = false;
            
            println!("💨 Дэш! Направление: {:?}, Сила: {}", dash_direction, stats.dash_force);
        }
    }
}
