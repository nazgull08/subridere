use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyKind {
    Jester,
    Jimbo,
}

/// Состояние врага (верхнеуровневая FSM)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyState {
    Idle,
    Walk,
    Attack(EnemyAttackState),
    Dead,
}

/// Подсостояния атаки (вложенная FSM внутри Attack)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyAttackState {
    Approach,
    Bite,
    Slash,
    Cooldown,
}

#[derive(Component, Debug, Default)]
pub struct EnemyMemory {
    pub patrol_target: Option<Vec3>,
    pub pursue_target: Option<Entity>
}

/// Цель атаки (например, игрок)
#[derive(Component)]
pub struct AggroTarget(pub Entity);

/// Радиус обнаружения цели (в метрах)
#[derive(Component)]
pub struct SightRange(pub f32);

/// Параметры ближней атаки
#[derive(Component)]
pub struct MeleeAttack {
    pub bite_damage: f32,
    pub slash_damage: f32,
}

/// Текущий тип анимации (idle, walk, attack и т. д.)
#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationKind {
    Idle,
    Walk,
    BiteAttack,
    SlashAttack,
}

/// Таймер для текущего состояния
#[derive(Component)]
pub struct StateTimer(pub Timer);
