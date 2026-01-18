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
    /// Idle - руки опущены по бокам, слегка согнуты
    pub fn idle_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.30, -0.35, -0.60),
            elbow_hint: Vec3::new(0.4, -0.5, 0.1),
        }
    }

    pub fn windup_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.40, -0.25, -0.35), // отводим назад
            elbow_hint: Vec3::new(0.5, -0.4, 0.4),
        }
    }

    pub fn punch_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.08, -0.20, -1.10), // ДАЛЕКО вперёд (было -0.80)
            elbow_hint: Vec3::new(0.20, -0.5, -0.5),
        }
    }

    pub fn recovery_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.25, -0.30, -0.50),
            elbow_hint: Vec3::new(0.4, -0.5, 0.0),
        }
    }

    /// Питьё зелья - рука у рта
    pub fn drinking_right() -> Self {
        Self {
            hand_offset: Vec3::new(0.10, -0.10, -0.30),
            elbow_hint: Vec3::new(0.4, -0.4, 0.1),
        }
    }

    /// Зеркальная версия для левой руки
    pub fn mirror(&self) -> ArmPose {
        ArmPose {
            hand_offset: Vec3::new(-self.hand_offset.x, self.hand_offset.y, self.hand_offset.z),
            elbow_hint: Vec3::new(-self.elbow_hint.x, self.elbow_hint.y, self.elbow_hint.z),
        }
    }

    pub fn lerp(&self, other: &ArmPose, t: f32) -> ArmPose {
        ArmPose {
            hand_offset: self.hand_offset.lerp(other.hand_offset, t),
            elbow_hint: self.elbow_hint.lerp(other.elbow_hint, t),
        }
    }
}
