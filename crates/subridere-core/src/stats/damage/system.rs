use super::component::{Damage, DamageType};
use crate::{
    audio::player::events::PlayerDamageEvent, player::component::Player,
    stats::health::component::Health, ui::hud::hitflash::HitFlashEvent,
};
use bevy::prelude::*;

pub fn apply_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &Damage)>,
    players: Query<&Transform, With<Player>>,
    mut ev_flash: EventWriter<HitFlashEvent>,
    mut ev_audio: EventWriter<PlayerDamageEvent>,
) {
    for (entity, mut health, damage) in &mut query {
        health.current -= damage.amount;

        if players.get(entity).is_ok() {
            ev_flash.write(HitFlashEvent);
            ev_audio.write(PlayerDamageEvent);
        }

        commands.entity(entity).remove::<Damage>();
    }
}
