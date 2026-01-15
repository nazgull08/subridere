use bevy::prelude::*;

/// Базовые атрибуты персонажа — игрок распределяет очки сюда.
/// Диапазон каждого: 0-30
#[derive(Component, Clone, Copy, Debug)]
pub struct Attributes {
    pub might: u8,     // сила: ближний урон, оглушение, грузоподъёмность
    pub fortitude: u8, // стойкость: HP, физ.защита, сопротивление отбросу
    pub agility: u8,   // ловкость: скорость, скорость атаки, i-frames
    pub arcana: u8,    // магия: маг.урон, мана, разнообразие заклинаний
    pub resolve: u8,   // воля: стамина, реген стамины, сопротивление статусам

    pub unspent_points: u8,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            might: 3,
            fortitude: 3,
            agility: 3,
            arcana: 3,
            resolve: 3,
            unspent_points: 0,
        }
    }
}

impl Attributes {
    pub const MAX_VALUE: u8 = 30;

    /// Создать с кастомными значениями
    pub fn new(might: u8, fortitude: u8, agility: u8, arcana: u8, resolve: u8) -> Self {
        Self {
            might: might.min(Self::MAX_VALUE),
            fortitude: fortitude.min(Self::MAX_VALUE),
            agility: agility.min(Self::MAX_VALUE),
            arcana: arcana.min(Self::MAX_VALUE),
            resolve: resolve.min(Self::MAX_VALUE),
            unspent_points: 0,
        }
    }

    /// Можно ли повысить атрибут?
    pub fn can_increase(&self, attr: AttributeType) -> bool {
        self.unspent_points > 0 && self.get(attr) < Self::MAX_VALUE
    }

    /// Повысить атрибут на 1
    pub fn increase(&mut self, attr: AttributeType) -> bool {
        if !self.can_increase(attr) {
            return false;
        }
        *self.get_mut(attr) += 1;
        self.unspent_points -= 1;
        true
    }

    /// Добавить очки для распределения
    pub fn add_points(&mut self, points: u8) {
        self.unspent_points = self.unspent_points.saturating_add(points);
    }

    pub fn get(&self, attr: AttributeType) -> u8 {
        match attr {
            AttributeType::Might => self.might,
            AttributeType::Fortitude => self.fortitude,
            AttributeType::Agility => self.agility,
            AttributeType::Arcana => self.arcana,
            AttributeType::Resolve => self.resolve,
        }
    }

    fn get_mut(&mut self, attr: AttributeType) -> &mut u8 {
        match attr {
            AttributeType::Might => &mut self.might,
            AttributeType::Fortitude => &mut self.fortitude,
            AttributeType::Agility => &mut self.agility,
            AttributeType::Arcana => &mut self.arcana,
            AttributeType::Resolve => &mut self.resolve,
        }
    }

    /// Сумма всех атрибутов (для отладки/UI)
    pub fn total(&self) -> u8 {
        self.might + self.fortitude + self.agility + self.arcana + self.resolve
    }
}

/// Для удобства работы с атрибутами
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttributeType {
    Might,
    Fortitude,
    Agility,
    Arcana,
    Resolve,
}

impl AttributeType {
    pub const ALL: [AttributeType; 5] = [
        AttributeType::Might,
        AttributeType::Fortitude,
        AttributeType::Agility,
        AttributeType::Arcana,
        AttributeType::Resolve,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            AttributeType::Might => "Might",
            AttributeType::Fortitude => "Fortitude",
            AttributeType::Agility => "Agility",
            AttributeType::Arcana => "Arcana",
            AttributeType::Resolve => "Resolve",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            AttributeType::Might => "MIG",
            AttributeType::Fortitude => "FOR",
            AttributeType::Agility => "AGI",
            AttributeType::Arcana => "ARC",
            AttributeType::Resolve => "RES",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AttributeType::Might => "Melee damage, stagger power, carry capacity",
            AttributeType::Fortitude => "Max HP, physical defense, knockback resistance",
            AttributeType::Agility => "Move speed, attack speed, dodge i-frames",
            AttributeType::Arcana => "Magic damage, max mana, spell variety",
            AttributeType::Resolve => "Max stamina, stamina regen, status resistance",
        }
    }
}
