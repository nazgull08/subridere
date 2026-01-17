use bevy::prelude::*;

use crate::player::body::components::ArmSide;

/// Запрос на проигрывание анимации
#[derive(Event, Clone, Debug)]
pub enum AnimationRequest {
    MeleeSwing { hand: ArmSide },
    // Будущее: DrinkPotion, Block, etc.
}

/// Анимация завершена
#[derive(Event, Clone, Debug)]
pub struct AnimationFinished {
    pub request: AnimationRequest,
}
