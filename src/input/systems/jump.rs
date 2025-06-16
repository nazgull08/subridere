use bevy::prelude::*;

use crate::input::component::{MovementInput, MovementStats, MovementState, PlayerControlled};

/// Система прыжков
pub fn jump_system(
    mut query: Query<(
        &MovementInput,
        &MovementStats,
        &mut MovementState,
    ), With<PlayerControlled>>,
) {
    for (input, stats, mut state) in &mut query {
        // Прыжок возможен только на земле и при нажатии кнопки
        if input.jump && state.is_grounded && !state.is_crouching {
            state.velocity.y = stats.jump_force;
            state.is_grounded = false; // Принудительно убираем флаг земли
            
            println!("🦘 Прыжок! Скорость Y: {}", stats.jump_force);
        }
    }
}
