use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyKind {
    Jester,
    Jimbo,
}
