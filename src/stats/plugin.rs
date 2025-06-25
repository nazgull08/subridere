use bevy::prelude::*;

use super::damage::system::apply_damage;
use super::health::system::regenerate_health;
use super::mana::system::regenerate_mana;
use super::stamina::system::regenerate_stamina;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (regenerate_health, regenerate_mana, regenerate_stamina, apply_damage)
        );
    }
}
