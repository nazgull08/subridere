use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::ItemId;

/// Контейнер всех активных модификаторов на сущности
#[derive(Component, Default, Clone, Debug)]
pub struct StatModifiers {
    modifiers: Vec<StatModifier>,
    next_id: u32,
}

/// Один модификатор
#[derive(Clone, Debug)]
pub struct StatModifier {
    pub id: u32,
    pub source: ModifierSource,
    pub target: ModifierTarget,
    pub op: ModifierOp,
}

/// Откуда пришёл модификатор
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModifierSource {
    Equipment(ItemId),
    Perk(u32),
    Buff(u32),
    Innate,
}

/// Что модифицируем
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModifierTarget {
    // Первичные
    Might,
    Fortitude,
    Agility,
    Arcana,
    Resolve,

    // Ресурсы
    MaxHealth,
    MaxMana,
    MaxStamina,
    HealthRegen,
    ManaRegen,
    StaminaRegen,

    // Боевые
    MeleeDamage,
    MagicDamage,
    AttackSpeed,
    PhysicalDefense,
    MagicResist,

    // Движение
    MoveSpeed,
    DodgeFrames,

    // Утилиты
    CarryCapacity,
    KnockbackResist,
    StatusResist,
}

/// Тип операции
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ModifierOp {
    /// Плоское добавление: +10
    Flat(f32),
    /// Процентное добавление: +10% → 0.1
    Percent(f32),
    /// Умножение: ×1.5 (применяется после Flat и Percent)
    Multiply(f32),
}

impl StatModifiers {
    /// Добавить модификатор, возвращает id
    pub fn add(&mut self, source: ModifierSource, target: ModifierTarget, op: ModifierOp) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.modifiers.push(StatModifier {
            id,
            source,
            target,
            op,
        });
        id
    }

    /// Добавить несколько модификаторов от одного источника
    pub fn add_many(
        &mut self,
        source: ModifierSource,
        mods: impl IntoIterator<Item = (ModifierTarget, ModifierOp)>,
    ) {
        for (target, op) in mods {
            self.add(source.clone(), target, op);
        }
    }

    /// Удалить по id
    pub fn remove(&mut self, id: u32) -> bool {
        let len_before = self.modifiers.len();
        self.modifiers.retain(|m| m.id != id);
        self.modifiers.len() != len_before
    }

    /// Удалить все от источника (например, при снятии предмета)
    pub fn remove_by_source(&mut self, source: &ModifierSource) {
        self.modifiers.retain(|m| &m.source != source);
    }

    /// Удалить модификаторы, не прошедшие фильтр
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&StatModifier) -> bool,
    {
        self.modifiers.retain(|m| f(m));
    }

    /// Есть ли модификаторы от источника?
    pub fn has_source(&self, source: &ModifierSource) -> bool {
        self.modifiers.iter().any(|m| &m.source == source)
    }

    /// Получить все модификаторы для цели
    pub fn get_for_target(&self, target: ModifierTarget) -> impl Iterator<Item = &StatModifier> {
        self.modifiers.iter().filter(move |m| m.target == target)
    }

    /// Все модификаторы (для отладки/UI)
    pub fn all(&self) -> &[StatModifier] {
        &self.modifiers
    }

    /// Количество модификаторов
    pub fn len(&self) -> usize {
        self.modifiers.len()
    }

    /// Пусто ли?
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Очистить все модификаторы
    pub fn clear(&mut self) {
        self.modifiers.clear();
    }
}

/// Применить все модификаторы к базовому значению
///
/// Порядок: (base + flat) * (1 + percent) * multiply
pub fn apply_modifiers(base: f32, modifiers: impl Iterator<Item = ModifierOp>) -> f32 {
    let mut flat = 0.0;
    let mut percent = 0.0;
    let mut multiply = 1.0;

    for op in modifiers {
        match op {
            ModifierOp::Flat(v) => flat += v,
            ModifierOp::Percent(v) => percent += v,
            ModifierOp::Multiply(v) => multiply *= v,
        }
    }

    (base + flat) * (1.0 + percent) * multiply
}
