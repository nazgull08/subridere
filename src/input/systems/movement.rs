use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::input::component::{MovementInput, MovementState, MovementStats, PlayerControlled};


/// Основная система движения
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(
        &MovementInput,
        &MovementStats,
        &mut MovementState,
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
    ), With<PlayerControlled>>,
) {
    for (input, stats, mut state, mut controller, output) in &mut query {
        // Проверяем, стоим ли на земле
        state.is_grounded = output.grounded;
        
        // Обновляем таймер дэша
        if state.dash_timer > 0.0 {
            state.dash_timer -= time.delta_secs();
        } else {
            state.can_dash = true;
        }
        
        // Вычисляем желаемую скорость
        let mut desired_velocity = Vec3::ZERO;
        
        if input.direction.length() > 0.0 {
            // Преобразуем 2D направление в 3D
            let direction_3d = Vec3::new(input.direction.x, 0.0, -input.direction.y);
            
            // Определяем скорость (ходьба/бег/приседание)
            let base_speed = if input.run && !input.crouch {
                stats.run_speed
            } else {
                stats.walk_speed
            };
            
            let final_speed = if input.crouch {
                base_speed * stats.crouch_speed_multiplier
            } else {
                base_speed
            };
            
            desired_velocity = direction_3d * final_speed;
        }
        
        // Применяем управление в воздухе
        let control_factor = if state.is_grounded {
            1.0
        } else {
            stats.air_control
        };
        
        // Плавно изменяем скорость
        let acceleration = if desired_velocity.length() > state.velocity.length() {
            stats.acceleration
        } else {
            stats.deceleration
        };
        
        // Обновляем горизонтальную скорость
        let horizontal_velocity = Vec3::new(state.velocity.x, 0.0, state.velocity.z);
        let target_horizontal = desired_velocity * control_factor;
        
        let new_horizontal = horizontal_velocity.lerp(
            target_horizontal,
            acceleration * time.delta_secs()
        );
        
        // Сохраняем вертикальную составляющую скорости
        state.velocity.x = new_horizontal.x;
        state.velocity.z = new_horizontal.z;
        
        // Применяем гравитацию
        if !state.is_grounded {
            state.velocity.y -= 20.0 * time.delta_secs(); // Гравитация
        } else if state.velocity.y < 0.0 {
            state.velocity.y = 0.0; // Останавливаем падение на земле
        }
        
        // Устанавливаем перемещение для контроллера
        controller.translation = Some(state.velocity * time.delta_secs());
        
        // Обновляем состояние приседания
        state.is_crouching = input.crouch;
    }
}
