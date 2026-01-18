// crates/subridere-core/src/player/arm/components.rs

use bevy::prelude::*;

// ═══════════════════════════════════════════════════════════════════
// ARM SIDE
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmSide {
    Left,
    Right,
}

// ═══════════════════════════════════════════════════════════════════
// ARM PARTS
// ═══════════════════════════════════════════════════════════════════

#[derive(Component)]
pub struct Shoulder {
    pub side: ArmSide,
}

#[derive(Component)]
pub struct UpperArm {
    pub side: ArmSide,
    pub length: f32,
}

#[derive(Component)]
pub struct Forearm {
    pub side: ArmSide,
    pub length: f32,
}

#[derive(Component)]
pub struct Hand {
    pub side: ArmSide,
}

#[derive(Component)]
pub struct WeaponSocket {
    pub side: ArmSide,
}

#[derive(Component)]
pub struct MeleeHitbox {
    pub side: ArmSide,
}

// ═══════════════════════════════════════════════════════════════════
// IK TARGET
// ═══════════════════════════════════════════════════════════════════

#[derive(Component)]
pub struct IkTarget {
    pub side: ArmSide,
    pub position: Vec3,
    pub elbow_hint: Vec3,
}

impl IkTarget {
    pub fn right() -> Self {
        Self {
            side: ArmSide::Right,
            position: Vec3::new(0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(0.4, -0.5, 0.1),
        }
    }

    pub fn left() -> Self {
        Self {
            side: ArmSide::Left,
            position: Vec3::new(-0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(-0.4, -0.5, 0.1),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ARM CONFIG
// ═══════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct ArmConfig {
    pub shoulder_offset_right: Vec3,
    pub shoulder_offset_left: Vec3,
    pub upper_arm_length: f32,
    pub forearm_length: f32,
    pub upper_arm_size: Vec3,
    pub forearm_size: Vec3,
    pub hand_size: Vec3,
}

impl Default for ArmConfig {
    fn default() -> Self {
        Self {
            shoulder_offset_right: Vec3::new(0.45, -0.10, -0.30),
            shoulder_offset_left: Vec3::new(-0.45, -0.10, -0.30),

            upper_arm_length: 0.40,
            forearm_length: 0.40,

            upper_arm_size: Vec3::new(0.14, 0.14, 0.40),
            forearm_size: Vec3::new(0.12, 0.12, 0.35),
            hand_size: Vec3::new(0.20, 0.16, 0.20),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ARM POSES
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
pub struct ArmPose {
    pub hand_offset: Vec3,
    pub elbow_hint: Vec3,
}

impl ArmPose {
    // ───────────────────────────────────────────────────────────────
    // БАЗОВЫЕ ПОЗЫ
    // ───────────────────────────────────────────────────────────────

    /// Idle - руки опущены по бокам, слегка согнуты
    pub fn idle_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(0.4, -0.5, 0.1),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // FISTS (кулаки)
    // ═══════════════════════════════════════════════════════════════

    // ───────────────────────────────────────────────────────────────
    // FISTS LIGHT (прямой удар)
    // ───────────────────────────────────────────────────────────────

    /// Fists Light Windup - рука отводится назад
    pub fn fists_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.40, -0.25, -0.35),
            elbow_hint: Vec3::new(0.5, -0.4, 0.4),
        }
    }

    /// Fists Light Active - прямой удар вперёд
    pub fn fists_punch_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.08, -0.20, -1.10),
            elbow_hint: Vec3::new(0.20, -0.5, -0.5),
        }
    }

    // ───────────────────────────────────────────────────────────────
    // FISTS HEAVY (оверхенд — удар сверху)
    // ───────────────────────────────────────────────────────────────

    /// Fists Heavy Charging - рука поднимается и отводится назад
    pub fn fists_heavy_charging_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.20, -0.15, -0.25),
            elbow_hint: Vec3::new(0.4, -0.2, 0.3),
        }
    }

    /// Fists Heavy Windup - рука поднята и взведена
    pub fn fists_heavy_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.05, -0.15),
            elbow_hint: Vec3::new(0.45, -0.1, 0.35),
        }
    }

    /// Fists Heavy Active - удар сверху вниз-вперёд
    pub fn fists_heavy_active_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.05, -0.35, -1.40),
            elbow_hint: Vec3::new(0.15, -0.5, -0.6),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // SWORD (меч)
    // ═══════════════════════════════════════════════════════════════

    // ───────────────────────────────────────────────────────────────
    // SWORD IDLE
    // ───────────────────────────────────────────────────────────────

    /// Sword Idle - меч держится наготове, чуть впереди
    pub fn sword_idle_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.30, -0.55),
            elbow_hint: Vec3::new(0.4, -0.4, 0.1),
        }
    }

    // ───────────────────────────────────────────────────────────────
    // SWORD LIGHT (горизонтальный слэш справа налево)
    // ───────────────────────────────────────────────────────────────

    /// Sword Light Windup - меч отведён вправо
    pub fn sword_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.50, -0.20, -0.30),
            elbow_hint: Vec3::new(0.6, -0.3, 0.3),
        }
    }

    /// Sword Light Active - слэш влево-вперёд
    pub fn sword_slash_right() -> Self {
        Self {
            hand_offset: Vec3::new(-0.10, -0.25, -0.90),
            elbow_hint: Vec3::new(0.15, -0.4, -0.4),
        }
    }

    // ───────────────────────────────────────────────────────────────
    // SWORD HEAVY (вертикальный удар сверху)
    // ───────────────────────────────────────────────────────────────

    /// Sword Heavy Charging - меч поднимается над головой
    pub fn sword_heavy_charging_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.15, 0.00, -0.30),
            elbow_hint: Vec3::new(0.35, 0.1, 0.2),
        }
    }

    /// Sword Heavy Windup - меч занесён над головой
    pub fn sword_heavy_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.10, 0.15, -0.20),
            elbow_hint: Vec3::new(0.30, 0.2, 0.3),
        }
    }

    /// Sword Heavy Active - вертикальный удар вниз-вперёд
    pub fn sword_heavy_slash_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.05, -0.40, -1.10),
            elbow_hint: Vec3::new(0.15, -0.5, -0.5),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // TODO: STAFF, HAMMER, SHIELD...
    // ═══════════════════════════════════════════════════════════════

    // ───────────────────────────────────────────────────────────────
    // СПЕЦИАЛЬНЫЕ ПОЗЫ
    // ───────────────────────────────────────────────────────────────

    /// Питьё зелья - рука у рта
    pub fn drinking_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.10, -0.10, -0.30),
            elbow_hint: Vec3::new(0.4, -0.4, 0.1),
        }
    }

    /// Recovery - возврат к idle (универсальная)
    pub fn recovery_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.30, -0.50),
            elbow_hint: Vec3::new(0.4, -0.5, 0.0),
        }
    }

    // ───────────────────────────────────────────────────────────────
    // УТИЛИТЫ
    // ───────────────────────────────────────────────────────────────

    /// Зеркальная версия для левой руки
    pub fn mirror(&self) -> ArmPose {
        ArmPose {
            hand_offset: Vec3::new(-self.hand_offset.x, self.hand_offset.y, self.hand_offset.z),
            elbow_hint: Vec3::new(-self.elbow_hint.x, self.elbow_hint.y, self.elbow_hint.z),
        }
    }

    /// Линейная интерполяция между позами
    pub fn lerp(&self, other: &ArmPose, t: f32) -> ArmPose {
        ArmPose {
            hand_offset: self.hand_offset.lerp(other.hand_offset, t),
            elbow_hint: self.elbow_hint.lerp(other.elbow_hint, t),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// LEGACY ALIASES (для совместимости)
// ═══════════════════════════════════════════════════════════════════

impl ArmPose {
    /// Alias: windup_right → fists_windup_right
    pub fn windup_right() -> Self {
        Self::fists_windup_right()
    }

    /// Alias: punch_right → fists_punch_right
    pub fn punch_right() -> Self {
        Self::fists_punch_right()
    }

    /// Alias: heavy_charging_right → fists_heavy_charging_right
    pub fn heavy_charging_right() -> Self {
        Self::fists_heavy_charging_right()
    }

    /// Alias: heavy_windup_right → fists_heavy_windup_right
    pub fn heavy_windup_right() -> Self {
        Self::fists_heavy_windup_right()
    }

    /// Alias: heavy_active_right → fists_heavy_active_right
    pub fn heavy_active_right() -> Self {
        Self::fists_heavy_active_right()
    }
}
