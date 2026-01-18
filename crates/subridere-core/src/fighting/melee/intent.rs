// fighting/melee/intent.rs
//
// Intent компоненты для melee атак.
// Используют enum для поддержки разных состояний ввода.

use bevy::prelude::*;

/// Состояние ввода атаки
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackInputState {
    /// Кнопка только что нажата — начать зарядку/действие
    Pressed,
    /// Кнопка только что отпущена — выпустить атаку/завершить
    Released,
    // TODO: Held — для щита (пока кнопка удерживается)
}

/// Intent: ввод правой руки (ЛКМ)
#[derive(Component, Debug)]
pub struct RightAttackInput(pub AttackInputState);

/// Intent: ввод левой руки (ПКМ)
#[derive(Component, Debug)]
pub struct LeftAttackInput(pub AttackInputState);
