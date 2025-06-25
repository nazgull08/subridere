use bevy::prelude::*;
use crate::{player::component::Player, stats::health::component::Health, ui::hud::hitflash::HitFlashEvent};
use super::component::{Damage, DamageType};

pub fn apply_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &Damage)>,
    players: Query<&Transform, With<Player>>,
    mut ev_flash: EventWriter<HitFlashEvent>,
) {
    for (entity, mut health, damage) in &mut query {
        health.current -= damage.amount;

        if players.get(entity).is_ok() {
            ev_flash.write(HitFlashEvent);
        }

        commands.entity(entity).remove::<Damage>();
    }
}
