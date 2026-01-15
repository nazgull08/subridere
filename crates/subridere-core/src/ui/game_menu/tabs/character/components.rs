use bevy::prelude::*;

use crate::stats::AttributeType;

/// Level display
#[derive(Component)]
pub struct LevelText;

/// XP progress bar
#[derive(Component)]
pub struct XpProgressBar;

/// XP text (e.g. "340/500")
#[derive(Component)]
pub struct XpText;

/// Attribute points available
#[derive(Component)]
pub struct AttributePointsText;

/// Attribute value text
#[derive(Component)]
pub struct AttributeValueText(pub AttributeType);

/// Attribute progress bar
#[derive(Component)]
pub struct AttributeBar(pub AttributeType);

/// Increase attribute button
#[derive(Component)]
pub struct IncreaseAttributeButton(pub AttributeType);

/// Derived stat text
#[derive(Component)]
pub struct StatText(pub StatType);

#[derive(Clone, Copy, Debug)]
pub enum StatType {
    MaxHealth,
    MaxMana,
    MaxStamina,
    HealthRegen,
    ManaRegen,
    StaminaRegen,
    MeleeDamage,
    MagicDamage,
    AttackSpeed,
    MoveSpeed,
    PhysDefense,
    MagicResist,
}
