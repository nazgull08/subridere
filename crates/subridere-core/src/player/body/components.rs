use bevy::prelude::*;

/// Корневой контейнер first-person рук
#[derive(Component)]
pub struct FirstPersonArms;

/// Сторона руки
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArmSide {
    Left,
    Right,
}

/// Маркер части руки
#[derive(Component)]
pub struct ArmPart {
    pub side: ArmSide,
}

/// Маркер кисти (сюда крепится оружие)
#[derive(Component)]
pub struct HandPart {
    pub side: ArmSide,
}

/// Слот для крепления оружия
#[derive(Component)]
pub struct WeaponMount;
