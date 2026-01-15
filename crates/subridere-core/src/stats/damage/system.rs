use super::component::{Damage, DamageType};
use crate::{
    audio::player::events::PlayerDamageEvent,
    player::component::Player,
    stats::{computed::ComputedStats, health::Health},
    ui::hud::hitflash::HitFlashEvent,
};
use bevy::prelude::*;

/// Применить урон с учётом защиты
pub fn apply_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &Damage, Option<&ComputedStats>)>,
    players: Query<&Transform, With<Player>>,
    mut ev_flash: EventWriter<HitFlashEvent>,
    mut ev_audio: EventWriter<PlayerDamageEvent>,
) {
    for (entity, mut health, damage, computed) in &mut query {
        // Рассчитываем итоговый урон с учётом защиты
        let final_damage = calculate_damage(damage, computed);

        // Наносим урон
        health.damage(final_damage);

        // Эффекты для игрока
        if players.get(entity).is_ok() {
            ev_flash.write(HitFlashEvent);
            ev_audio.write(PlayerDamageEvent);
        }

        // Убираем компонент урона
        commands.entity(entity).remove::<Damage>();
    }
}

/// Рассчитать итоговый урон с учётом защиты
fn calculate_damage(damage: &Damage, computed: Option<&ComputedStats>) -> f32 {
    let Some(stats) = computed else {
        // Нет статов - полный урон
        return damage.amount.max(0.0);
    };

    let reduction = match damage.damage_type {
        DamageType::Physical => stats.physical_defense,
        DamageType::Magical => stats.magic_resist,
    };

    // Плоское снижение, минимум 0
    (damage.amount - reduction).max(0.0)
}
