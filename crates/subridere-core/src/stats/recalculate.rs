//! Система пересчёта статов при изменении Attributes или StatModifiers.

use bevy::prelude::*;

use super::attributes::Attributes;
use super::computed::ComputedStats;
use super::formulas::*;
use super::health::Health;
use super::mana::Mana;
use super::modifiers::{ModifierTarget, StatModifiers, apply_modifiers};
use super::stamina::Stamina;

/// Пересчитать все статы при изменении атрибутов или модификаторов
pub fn recalculate_stats(
    mut query: Query<
        (
            &Attributes,
            &StatModifiers,
            &mut ComputedStats,
            &mut Health,
            &mut Mana,
            &mut Stamina,
        ),
        Or<(Changed<Attributes>, Changed<StatModifiers>)>,
    >,
) {
    for (attrs, mods, mut computed, mut health, mut mana, mut stamina) in &mut query {
        recalculate(
            &attrs,
            &mods,
            &mut computed,
            &mut health,
            &mut mana,
            &mut stamina,
        );
    }
}

/// Внутренняя функция пересчёта (можно вызывать напрямую)
pub fn recalculate(
    attrs: &Attributes,
    mods: &StatModifiers,
    computed: &mut ComputedStats,
    health: &mut Health,
    mana: &mut Mana,
    stamina: &mut Stamina,
) {
    // === 1. Эффективные первичные атрибуты ===
    computed.might = apply_mods_for(attrs.might as f32, ModifierTarget::Might, mods);
    computed.fortitude = apply_mods_for(attrs.fortitude as f32, ModifierTarget::Fortitude, mods);
    computed.agility = apply_mods_for(attrs.agility as f32, ModifierTarget::Agility, mods);
    computed.arcana = apply_mods_for(attrs.arcana as f32, ModifierTarget::Arcana, mods);
    computed.resolve = apply_mods_for(attrs.resolve as f32, ModifierTarget::Resolve, mods);

    // === 2. Производные ресурсы ===
    let base_hp = calc_max_health(computed.fortitude);
    computed.max_health = apply_mods_for(base_hp, ModifierTarget::MaxHealth, mods);

    let base_mana = calc_max_mana(computed.arcana);
    computed.max_mana = apply_mods_for(base_mana, ModifierTarget::MaxMana, mods);

    let base_stamina = calc_max_stamina(computed.resolve);
    computed.max_stamina = apply_mods_for(base_stamina, ModifierTarget::MaxStamina, mods);

    // Регенерация
    let base_hp_regen = calc_health_regen(computed.fortitude);
    computed.health_regen = apply_mods_for(base_hp_regen, ModifierTarget::HealthRegen, mods);

    let base_mana_regen = calc_mana_regen(computed.arcana);
    computed.mana_regen = apply_mods_for(base_mana_regen, ModifierTarget::ManaRegen, mods);

    let base_stam_regen = calc_stamina_regen(computed.resolve);
    computed.stamina_regen = apply_mods_for(base_stam_regen, ModifierTarget::StaminaRegen, mods);

    // === 3. Боевые статы ===
    let base_melee = calc_melee_damage(computed.might);
    computed.melee_damage = apply_mods_for(base_melee, ModifierTarget::MeleeDamage, mods);

    let base_magic = calc_magic_damage(computed.arcana);
    computed.magic_damage = apply_mods_for(base_magic, ModifierTarget::MagicDamage, mods);

    let base_atk_speed = calc_attack_speed(computed.agility);
    computed.attack_speed = apply_mods_for(base_atk_speed, ModifierTarget::AttackSpeed, mods);

    let base_defense = calc_physical_defense(computed.fortitude);
    computed.physical_defense = apply_mods_for(base_defense, ModifierTarget::PhysicalDefense, mods);

    let base_magic_res = calc_magic_resist(computed.arcana);
    computed.magic_resist = apply_mods_for(base_magic_res, ModifierTarget::MagicResist, mods);

    // === 4. Движение ===
    let base_move = calc_move_speed(computed.agility);
    computed.move_speed = apply_mods_for(base_move, ModifierTarget::MoveSpeed, mods);

    let base_dodge = calc_dodge_frames(computed.agility);
    computed.dodge_frames = apply_mods_for(base_dodge, ModifierTarget::DodgeFrames, mods);

    // === 5. Утилиты ===
    let base_carry = calc_carry_capacity(computed.might);
    computed.carry_capacity = apply_mods_for(base_carry, ModifierTarget::CarryCapacity, mods);

    let base_knockback = calc_knockback_resist(computed.fortitude);
    computed.knockback_resist =
        apply_mods_for(base_knockback, ModifierTarget::KnockbackResist, mods).clamp(0.0, 1.0);

    let base_status = calc_status_resist(computed.resolve);
    computed.status_resist =
        apply_mods_for(base_status, ModifierTarget::StatusResist, mods).clamp(0.0, 1.0);

    // === 6. Синхронизация с ресурсами (сохраняем процент) ===
    sync_resource(&mut health.current, health.max, computed.max_health);
    health.max = computed.max_health;
    health.regen = computed.health_regen;

    sync_resource(&mut mana.current, mana.max, computed.max_mana);
    mana.max = computed.max_mana;
    mana.regen = computed.mana_regen;

    sync_resource(&mut stamina.current, stamina.max, computed.max_stamina);
    stamina.max = computed.max_stamina;
    stamina.regen = computed.stamina_regen;
}

/// Применить модификаторы к базовому значению для конкретной цели
fn apply_mods_for(base: f32, target: ModifierTarget, mods: &StatModifiers) -> f32 {
    apply_modifiers(base, mods.get_for_target(target).map(|m| m.op))
}

/// Синхронизировать текущее значение ресурса с новым максимумом
/// Сохраняет процент заполнения
fn sync_resource(current: &mut f32, old_max: f32, new_max: f32) {
    if old_max > 0.0 {
        let ratio = *current / old_max;
        *current = new_max * ratio;
    } else {
        *current = new_max;
    }
}
