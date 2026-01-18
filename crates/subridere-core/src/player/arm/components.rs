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
    pub hand_rotation: Quat,
}

impl IkTarget {
    pub fn right() -> Self {
        Self {
            side: ArmSide::Right,
            position: Vec3::new(0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(0.4, -0.5, 0.1),
            hand_rotation: Quat::IDENTITY,
        }
    }

    pub fn left() -> Self {
        Self {
            side: ArmSide::Left,
            position: Vec3::new(-0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(-0.4, -0.5, 0.1),
            hand_rotation: Quat::IDENTITY,
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
    pub hand_rotation: Quat,
}

impl ArmPose {
    // ───────────────────────────────────────────────────────────────
    // ХЕЛПЕР ДЛЯ СОЗДАНИЯ РОТАЦИИ
    // ───────────────────────────────────────────────────────────────

    /// Создаёт Quat из градусов (pitch, yaw, roll)
    fn rot(pitch: f32, yaw: f32, roll: f32) -> Quat {
        Quat::from_euler(
            EulerRot::XYZ,
            pitch.to_radians(),
            yaw.to_radians(),
            roll.to_radians(),
        )
    }

    // ───────────────────────────────────────────────────────────────
    // БАЗОВЫЕ ПОЗЫ
    // ───────────────────────────────────────────────────────────────

    /// Idle - руки опущены по бокам, слегка согнуты
    pub fn idle_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(0.4, -0.5, 0.1),
            hand_rotation: Quat::IDENTITY,
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
            hand_rotation: Quat::IDENTITY,
        }
    }

    /// Fists Light Active - прямой удар вперёд
    pub fn fists_punch_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.08, -0.20, -1.10),
            elbow_hint: Vec3::new(0.20, -0.5, -0.5),
            hand_rotation: Quat::IDENTITY,
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
            hand_rotation: Quat::IDENTITY,
        }
    }

    /// Fists Heavy Windup - рука поднята и взведена
    pub fn fists_heavy_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.05, -0.15),
            elbow_hint: Vec3::new(0.45, -0.1, 0.35),
            hand_rotation: Quat::IDENTITY,
        }
    }

    /// Fists Heavy Active - удар сверху вниз-вперёд
    pub fn fists_heavy_active_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.05, -0.35, -1.40),
            elbow_hint: Vec3::new(0.15, -0.5, -0.6),
            hand_rotation: Quat::IDENTITY,
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // SWORD (меч) — ГОРИЗОНТАЛЬНЫЙ СЛЭШ
    // ═══════════════════════════════════════════════════════════════

    // ───────────────────────────────────────────────────────────────
    // SWORD IDLE
    // ───────────────────────────────────────────────────────────────

    /// Sword Idle - меч наготове, направлен вперёд-вверх
    pub fn sword_idle_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.30, -0.55),
            elbow_hint: Vec3::new(0.4, -0.4, 0.1),
            // Кисть нейтральная, меч чуть наклонён
            hand_rotation: Self::rot(0.0, 0.0, 0.0),
        }
    }

    // ───────────────────────────────────────────────────────────────
    // SWORD LIGHT (горизонтальный слэш справа налево)
    // ───────────────────────────────────────────────────────────────

    /// Sword Light Windup - меч отведён вправо, кисть развёрнута
    pub fn sword_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.55, -0.25, -0.40),
            elbow_hint: Vec3::new(0.6, -0.3, 0.3),
            // Кисть повёрнута вправо, меч "взведён" для слэша
            hand_rotation: Self::rot(0.0, 60.0, -90.0),
        }
    }

    /// Sword Light Active - слэш влево, кисть вращается
    pub fn sword_slash_right() -> Self {
        Self {
            hand_offset: Vec3::new(-0.30, -0.30, -1.00),
            elbow_hint: Vec3::new(0.10, -0.4, -0.4),
            // Кисть повёрнута влево, завершение слэша
            hand_rotation: Self::rot(0.0, -60.0, 90.0),
        }
    }

    // ───────────────────────────────────────────────────────────────
    // SWORD HEAVY (вертикальный удар сверху)
    // ───────────────────────────────────────────────────────────────

    /// Sword Heavy Charging - меч поднимается над головой
    pub fn sword_heavy_charging_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.40, 0.05, -0.30),
            elbow_hint: Vec3::new(0.35, 0.1, 0.2),
            // Кисть начинает подниматься
            hand_rotation: Self::rot(-20.0, 0.0, 0.0),
        }
    }

    /// Sword Heavy Windup - меч занесён над головой
    pub fn sword_heavy_windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.35, 0.25, -0.15),
            elbow_hint: Vec3::new(0.30, 0.3, 0.35),
            // Кисть отогнута назад, меч над головой
            hand_rotation: Self::rot(-45.0, 0.0, 0.0),
        }
    }

    /// Sword Heavy Active - вертикальный удар вниз-вперёд
    pub fn sword_heavy_slash_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.05, -0.45, -1.00),
            elbow_hint: Vec3::new(0.15, -0.5, -0.5),
            // Кисть резко вниз, завершение удара
            hand_rotation: Self::rot(30.0, 0.0, 0.0),
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
            hand_rotation: Self::rot(-30.0, 0.0, 0.0),
        }
    }

    /// Recovery - возврат к idle (универсальная)
    pub fn recovery_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.30, -0.50),
            elbow_hint: Vec3::new(0.4, -0.5, 0.0),
            hand_rotation: Quat::IDENTITY,
        }
    }

    // ───────────────────────────────────────────────────────────────
    // УТИЛИТЫ
    // ───────────────────────────────────────────────────────────────

    /// Зеркальная версия для левой руки
    pub fn mirror(&self) -> ArmPose {
        // Для зеркалирования Quat: инвертируем Y и Z компоненты
        let mirrored_rotation = Quat::from_xyzw(
            self.hand_rotation.x,
            -self.hand_rotation.y,
            -self.hand_rotation.z,
            self.hand_rotation.w,
        );

        ArmPose {
            hand_offset: Vec3::new(-self.hand_offset.x, self.hand_offset.y, self.hand_offset.z),
            elbow_hint: Vec3::new(-self.elbow_hint.x, self.elbow_hint.y, self.elbow_hint.z),
            hand_rotation: mirrored_rotation,
        }
    }

    /// Линейная интерполяция между позами
    pub fn lerp(&self, other: &ArmPose, t: f32) -> ArmPose {
        ArmPose {
            hand_offset: self.hand_offset.lerp(other.hand_offset, t),
            elbow_hint: self.elbow_hint.lerp(other.elbow_hint, t),
            hand_rotation: self.hand_rotation.slerp(other.hand_rotation, t),
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
