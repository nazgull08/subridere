use bevy::prelude::*;

use crate::player::component::Player;
use crate::stats::{Attributes, ComputedStats, Experience, Level};

use super::components::*;

/// Sync level and XP display
pub fn sync_level_display(
    player_query: Query<(&Level, &Experience, &Attributes), With<Player>>,
    mut level_text: Query<&mut Text, With<LevelText>>,
    mut xp_text: Query<&mut Text, (With<XpText>, Without<LevelText>)>,
    mut xp_bar: Query<&mut Node, With<XpProgressBar>>,
    mut points_text: Query<
        &mut Text,
        (
            With<AttributePointsText>,
            Without<LevelText>,
            Without<XpText>,
        ),
    >,
) {
    let Ok((level, exp, attrs)) = player_query.single() else {
        return;
    };

    // Level text
    if let Ok(mut text) = level_text.single_mut() {
        text.0 = format!("Level {}", level.current);
    }

    // XP text
    if let Ok(mut text) = xp_text.single_mut() {
        text.0 = format!("{}/{}", exp.current, exp.to_next_level);
    }

    // XP bar
    if let Ok(mut node) = xp_bar.single_mut() {
        node.width = Val::Percent(exp.progress() * 100.0);
    }

    // Attribute points
    if let Ok(mut text) = points_text.single_mut() {
        text.0 = format!("Attribute Points: {}", attrs.unspent_points);
    }
}

/// Sync attribute values
pub fn sync_attributes_display(
    player_query: Query<&Attributes, With<Player>>,
    mut value_texts: Query<(&AttributeValueText, &mut Text)>,
    mut bars: Query<(&AttributeBar, &mut Node)>,
    mut buttons: Query<(&IncreaseAttributeButton, &mut Visibility)>, // Changed
) {
    let Ok(attrs) = player_query.single() else {
        return;
    };

    // Values
    for (attr_text, mut text) in &mut value_texts {
        text.0 = format!("{}", attrs.get(attr_text.0));
    }

    // Bars (percentage of max 30)
    for (attr_bar, mut node) in &mut bars {
        let value = attrs.get(attr_bar.0) as f32;
        let percent = (value / Attributes::MAX_VALUE as f32) * 100.0;
        node.width = Val::Percent(percent);
    }

    // Hide buttons if no points available
    let can_spend = attrs.unspent_points > 0;
    for (btn, mut vis) in &mut buttons {
        let can_increase = can_spend && attrs.get(btn.0) < Attributes::MAX_VALUE;
        *vis = if can_increase {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Sync derived stats
pub fn sync_stats_display(
    player_query: Query<&ComputedStats, With<Player>>,
    mut stat_texts: Query<(&StatText, &mut Text)>,
) {
    let Ok(stats) = player_query.single() else {
        return;
    };

    for (stat, mut text) in &mut stat_texts {
        text.0 = match stat.0 {
            StatType::MaxHealth => format!("{:.0}", stats.max_health),
            StatType::MaxMana => format!("{:.0}", stats.max_mana),
            StatType::MaxStamina => format!("{:.0}", stats.max_stamina),
            StatType::HealthRegen => format!("{:.1}/s", stats.health_regen),
            StatType::ManaRegen => format!("{:.1}/s", stats.mana_regen),
            StatType::StaminaRegen => format!("{:.1}/s", stats.stamina_regen),
            StatType::MeleeDamage => format!("{:.1}", stats.melee_damage),
            StatType::MagicDamage => format!("{:.1}", stats.magic_damage),
            StatType::AttackSpeed => format!("{:.2}", stats.attack_speed),
            StatType::MoveSpeed => format!("{:.2}", stats.move_speed),
            StatType::PhysDefense => format!("{:.1}", stats.physical_defense),
            StatType::MagicResist => format!("{:.1}", stats.magic_resist),
        };
    }
}
