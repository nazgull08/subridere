use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum EnemyKind {
    Jester,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyState {
    Idle,
    Walk,
    Attack,
}

#[derive(Component)]
pub struct TargetPos(pub Vec3); // Куда идти

#[derive(Component)]
pub struct MeleeAttack {
    pub damage: f32,
}
