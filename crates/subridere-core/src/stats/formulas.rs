//! Формулы расчёта производных статов.
//!
//! Все константы и формулы собраны здесь для удобного балансирования.

// ============================================================================
// РЕСУРСЫ
// ============================================================================

/// HP = BASE + Fortitude × MULT
pub const HEALTH_BASE: f32 = 50.0;
pub const HEALTH_PER_FORTITUDE: f32 = 5.0;

/// Mana = BASE + Arcana × MULT
pub const MANA_BASE: f32 = 20.0;
pub const MANA_PER_ARCANA: f32 = 4.0;

/// Stamina = BASE + Resolve × MULT
pub const STAMINA_BASE: f32 = 50.0;
pub const STAMINA_PER_RESOLVE: f32 = 3.0;

// ============================================================================
// РЕГЕНЕРАЦИЯ
// ============================================================================

/// HP regen = Fortitude × MULT (в секунду)
pub const HEALTH_REGEN_PER_FORTITUDE: f32 = 0.1;

/// Mana regen = Arcana × MULT (в секунду)
pub const MANA_REGEN_PER_ARCANA: f32 = 0.5;

/// Stamina regen = BASE + Resolve × MULT (в секунду)
pub const STAMINA_REGEN_BASE: f32 = 5.0;
pub const STAMINA_REGEN_PER_RESOLVE: f32 = 0.5;

// ============================================================================
// БОЕВЫЕ СТАТЫ
// ============================================================================

/// Базовый урон ближнего боя = Might × MULT
pub const MELEE_DAMAGE_PER_MIGHT: f32 = 2.0;

/// Базовый магический урон = Arcana × MULT
pub const MAGIC_DAMAGE_PER_ARCANA: f32 = 2.0;

/// Скорость атаки = BASE + Agility × MULT
pub const ATTACK_SPEED_BASE: f32 = 0.7;
pub const ATTACK_SPEED_PER_AGILITY: f32 = 0.01;

/// Физическая защита = Fortitude × MULT (плоское снижение урона)
pub const DEFENSE_PER_FORTITUDE: f32 = 0.5;

/// Магическое сопротивление = Arcana × MULT
pub const MAGIC_RESIST_PER_ARCANA: f32 = 0.5;

// ============================================================================
// ДВИЖЕНИЕ
// ============================================================================

/// Скорость передвижения = BASE + Agility × MULT
pub const MOVE_SPEED_BASE: f32 = 0.85;
pub const MOVE_SPEED_PER_AGILITY: f32 = 0.005;

/// Длительность i-frames при уклонении = BASE + Agility × MULT
pub const DODGE_FRAMES_BASE: f32 = 0.15;
pub const DODGE_FRAMES_PER_AGILITY: f32 = 0.005;

// ============================================================================
// УТИЛИТЫ
// ============================================================================

/// Грузоподъёмность = BASE + Might × MULT
pub const CARRY_CAPACITY_BASE: f32 = 30.0;
pub const CARRY_CAPACITY_PER_MIGHT: f32 = 2.0;

/// Сопротивление отбросу = Fortitude × MULT (0-1, clamp)
pub const KNOCKBACK_RESIST_PER_FORTITUDE: f32 = 0.02;

/// Сопротивление статусам = Resolve × MULT (0-1, clamp)
pub const STATUS_RESIST_PER_RESOLVE: f32 = 0.02;

// ============================================================================
// ФУНКЦИИ РАСЧЁТА
// ============================================================================

/// Вычислить max health
#[inline]
pub fn calc_max_health(fortitude: f32) -> f32 {
    HEALTH_BASE + fortitude * HEALTH_PER_FORTITUDE
}

/// Вычислить max mana
#[inline]
pub fn calc_max_mana(arcana: f32) -> f32 {
    MANA_BASE + arcana * MANA_PER_ARCANA
}

/// Вычислить max stamina
#[inline]
pub fn calc_max_stamina(resolve: f32) -> f32 {
    STAMINA_BASE + resolve * STAMINA_PER_RESOLVE
}

/// Вычислить health regen
#[inline]
pub fn calc_health_regen(fortitude: f32) -> f32 {
    fortitude * HEALTH_REGEN_PER_FORTITUDE
}

/// Вычислить mana regen
#[inline]
pub fn calc_mana_regen(arcana: f32) -> f32 {
    arcana * MANA_REGEN_PER_ARCANA
}

/// Вычислить stamina regen
#[inline]
pub fn calc_stamina_regen(resolve: f32) -> f32 {
    STAMINA_REGEN_BASE + resolve * STAMINA_REGEN_PER_RESOLVE
}

/// Вычислить melee damage
#[inline]
pub fn calc_melee_damage(might: f32) -> f32 {
    might * MELEE_DAMAGE_PER_MIGHT
}

/// Вычислить magic damage
#[inline]
pub fn calc_magic_damage(arcana: f32) -> f32 {
    arcana * MAGIC_DAMAGE_PER_ARCANA
}

/// Вычислить attack speed
#[inline]
pub fn calc_attack_speed(agility: f32) -> f32 {
    ATTACK_SPEED_BASE + agility * ATTACK_SPEED_PER_AGILITY
}

/// Вычислить physical defense
#[inline]
pub fn calc_physical_defense(fortitude: f32) -> f32 {
    fortitude * DEFENSE_PER_FORTITUDE
}

/// Вычислить magic resist
#[inline]
pub fn calc_magic_resist(arcana: f32) -> f32 {
    arcana * MAGIC_RESIST_PER_ARCANA
}

/// Вычислить move speed multiplier
#[inline]
pub fn calc_move_speed(agility: f32) -> f32 {
    MOVE_SPEED_BASE + agility * MOVE_SPEED_PER_AGILITY
}

/// Вычислить dodge frames
#[inline]
pub fn calc_dodge_frames(agility: f32) -> f32 {
    DODGE_FRAMES_BASE + agility * DODGE_FRAMES_PER_AGILITY
}

/// Вычислить carry capacity
#[inline]
pub fn calc_carry_capacity(might: f32) -> f32 {
    CARRY_CAPACITY_BASE + might * CARRY_CAPACITY_PER_MIGHT
}

/// Вычислить knockback resist (clamped 0-1)
#[inline]
pub fn calc_knockback_resist(fortitude: f32) -> f32 {
    (fortitude * KNOCKBACK_RESIST_PER_FORTITUDE).clamp(0.0, 1.0)
}

/// Вычислить status resist (clamped 0-1)
#[inline]
pub fn calc_status_resist(resolve: f32) -> f32 {
    (resolve * STATUS_RESIST_PER_RESOLVE).clamp(0.0, 1.0)
}
