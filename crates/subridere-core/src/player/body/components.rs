use bevy::prelude::*;

#[derive(Component)]
pub struct FirstPersonArms;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArmSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct ArmPart {
    pub side: ArmSide,
}

#[derive(Component)]
pub struct HandPart {
    pub side: ArmSide,
}

#[derive(Component)]
pub struct WeaponMount {
    pub side: ArmSide,
}

#[derive(Component)]
pub struct MeleeHitbox;
