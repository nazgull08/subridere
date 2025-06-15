use bevy::prelude::*;

/// Отмечает любую сущность, к которой применима логика передвижения, гравитации, состояний и т.п.
#[derive(Component)]
pub struct Unit;

/// Состояние: стоит ли на земле
#[derive(Component, Default)]
pub struct Grounded(pub bool);
