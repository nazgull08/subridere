use bevy::prelude::*;
use crate::enemy::kind::EnemyKind;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyState {
    Idle,
    Walk,
}

#[derive(Component)]
pub struct TargetPos(pub Vec3);

#[derive(Component)]
pub struct MeleeAttack {
    pub damage: f32,
}

/// Флаг: текущая активная анимация (можно просто строкой или enum)
#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum AnimationKind {
    Idle,
    Walk,
}

#[derive(Component)]
pub struct StateTimer(pub Timer);
