use bevy::prelude::*;

/// Боевое состояние игрока
#[derive(Component, Default)]
pub struct PlayerCombatState {
    pub state: CombatState,
}

#[derive(Default, Clone, Debug)]
pub enum CombatState {
    #[default]
    Ready,
    Attacking {
        timer: f32,
        duration: f32,
        damage_dealt: bool,
    },
    Cooldown {
        remaining: f32,
    },
}
