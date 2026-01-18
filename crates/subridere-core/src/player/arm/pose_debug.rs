// crates/subridere-core/src/player/arm/pose_debug.rs
//
// Система дебага поз для настройки анимаций
//
// Управление:
//   F10        — включить/выключить pose debug
//   F11        — следующая поза
//   Shift+F11  — предыдущая поза
//   F12        — переключить редактируемый параметр (offset/elbow/rotation)
//   ← →        — переключить ось (X/Y/Z)
//   ↑ ↓        — изменить значение
//   Shift+↑↓   — большой шаг
//   F9         — вывести все позы в консоль

use bevy::prelude::*;

use super::components::{ArmPose, ArmSide, IkTarget};

// ═══════════════════════════════════════════════════════════════════
// POSE ENUM
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugPose {
    #[default]
    SwordIdle,
    SwordWindup,
    SwordSlash,
    SwordHeavyCharging,
    SwordHeavyWindup,
    SwordHeavySlash,
    FistsIdle,
    FistsWindup,
    FistsPunch,
    FistsHeavyCharging,
    FistsHeavyWindup,
    FistsHeavyActive,
}

impl DebugPose {
    pub const ALL: &'static [DebugPose] = &[
        Self::SwordIdle,
        Self::SwordWindup,
        Self::SwordSlash,
        Self::SwordHeavyCharging,
        Self::SwordHeavyWindup,
        Self::SwordHeavySlash,
        Self::FistsIdle,
        Self::FistsWindup,
        Self::FistsPunch,
        Self::FistsHeavyCharging,
        Self::FistsHeavyWindup,
        Self::FistsHeavyActive,
    ];

    pub fn next(self) -> Self {
        let idx = Self::ALL.iter().position(|&p| p == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        let idx = Self::ALL.iter().position(|&p| p == self).unwrap_or(0);
        Self::ALL[(idx + Self::ALL.len() - 1) % Self::ALL.len()]
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::SwordIdle => "sword_idle_right",
            Self::SwordWindup => "sword_windup_right",
            Self::SwordSlash => "sword_slash_right",
            Self::SwordHeavyCharging => "sword_heavy_charging_right",
            Self::SwordHeavyWindup => "sword_heavy_windup_right",
            Self::SwordHeavySlash => "sword_heavy_slash_right",
            Self::FistsIdle => "idle_right (fists)",
            Self::FistsWindup => "fists_windup_right",
            Self::FistsPunch => "fists_punch_right",
            Self::FistsHeavyCharging => "fists_heavy_charging_right",
            Self::FistsHeavyWindup => "fists_heavy_windup_right",
            Self::FistsHeavyActive => "fists_heavy_active_right",
        }
    }

    pub fn fn_name(self) -> &'static str {
        match self {
            Self::SwordIdle => "sword_idle_right",
            Self::SwordWindup => "sword_windup_right",
            Self::SwordSlash => "sword_slash_right",
            Self::SwordHeavyCharging => "sword_heavy_charging_right",
            Self::SwordHeavyWindup => "sword_heavy_windup_right",
            Self::SwordHeavySlash => "sword_heavy_slash_right",
            Self::FistsIdle => "idle_right",
            Self::FistsWindup => "fists_windup_right",
            Self::FistsPunch => "fists_punch_right",
            Self::FistsHeavyCharging => "fists_heavy_charging_right",
            Self::FistsHeavyWindup => "fists_heavy_windup_right",
            Self::FistsHeavyActive => "fists_heavy_active_right",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// EDIT TARGET
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditTarget {
    #[default]
    HandOffset,
    ElbowHint,
    HandRotation,
}

impl EditTarget {
    pub fn next(self) -> Self {
        match self {
            Self::HandOffset => Self::ElbowHint,
            Self::ElbowHint => Self::HandRotation,
            Self::HandRotation => Self::HandOffset,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::HandOffset => "hand_offset",
            Self::ElbowHint => "elbow_hint",
            Self::HandRotation => "hand_rotation (degrees)",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// DEBUG STATE
// ═══════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct PoseDebugState {
    pub enabled: bool,
    pub current_pose: DebugPose,
    pub edit_target: EditTarget,
    pub axis: usize, // 0=X, 1=Y, 2=Z
    pub step_small: f32,
    pub step_large: f32,
    pub step_rotation: f32,

    // Редактируемые значения для каждой позы
    pub poses: PoseDebugData,
}

impl Default for PoseDebugState {
    fn default() -> Self {
        Self {
            enabled: false,
            current_pose: DebugPose::SwordWindup,
            edit_target: EditTarget::HandRotation,
            axis: 2, // Z (roll)
            step_small: 0.05,
            step_large: 0.15,
            step_rotation: 5.0,

            poses: PoseDebugData::from_defaults(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// POSE DATA STORAGE
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone)]
pub struct PoseValues {
    pub hand_offset: Vec3,
    pub elbow_hint: Vec3,
    pub rotation_degrees: Vec3, // pitch, yaw, roll
}

impl PoseValues {
    pub fn to_arm_pose(&self) -> ArmPose {
        ArmPose {
            hand_offset: self.hand_offset,
            elbow_hint: self.elbow_hint,
            hand_rotation: Quat::from_euler(
                EulerRot::XYZ,
                self.rotation_degrees.x.to_radians(),
                self.rotation_degrees.y.to_radians(),
                self.rotation_degrees.z.to_radians(),
            ),
        }
    }
}

#[derive(Clone)]
pub struct PoseDebugData {
    pub sword_idle: PoseValues,
    pub sword_windup: PoseValues,
    pub sword_slash: PoseValues,
    pub sword_heavy_charging: PoseValues,
    pub sword_heavy_windup: PoseValues,
    pub sword_heavy_slash: PoseValues,
    pub fists_idle: PoseValues,
    pub fists_windup: PoseValues,
    pub fists_punch: PoseValues,
    pub fists_heavy_charging: PoseValues,
    pub fists_heavy_windup: PoseValues,
    pub fists_heavy_active: PoseValues,
}

impl PoseDebugData {
    /// Загружаем текущие значения из ArmPose
    pub fn from_defaults() -> Self {
        Self {
            // Sword
            sword_idle: PoseValues {
                hand_offset: Vec3::new(0.25, -0.30, -0.55),
                elbow_hint: Vec3::new(0.4, -0.4, 0.1),
                rotation_degrees: Vec3::new(0.0, 0.0, 0.0),
            },
            sword_windup: PoseValues {
                hand_offset: Vec3::new(0.55, -0.15, -0.35),
                elbow_hint: Vec3::new(0.60, -0.30, 0.30),
                rotation_degrees: Vec3::new(0.0, 15.0, -30.0),
            },
            sword_slash: PoseValues {
                hand_offset: Vec3::new(0.10, -0.25, -0.85),
                elbow_hint: Vec3::new(0.10, -0.40, -0.40),
                rotation_degrees: Vec3::new(75.0, -61.5, -11.1),
            },
            sword_heavy_charging: PoseValues {
                hand_offset: Vec3::new(0.40, 0.05, -0.30),
                elbow_hint: Vec3::new(0.35, 0.1, 0.2),
                rotation_degrees: Vec3::new(-20.0, 0.0, 0.0),
            },
            sword_heavy_windup: PoseValues {
                hand_offset: Vec3::new(0.35, 0.25, -0.15),
                elbow_hint: Vec3::new(0.30, 0.3, 0.35),
                rotation_degrees: Vec3::new(-45.0, 0.0, 0.0),
            },
            sword_heavy_slash: PoseValues {
                hand_offset: Vec3::new(0.05, -0.45, -1.00),
                elbow_hint: Vec3::new(0.15, -0.5, -0.5),
                rotation_degrees: Vec3::new(30.0, 0.0, 0.0),
            },

            // Fists
            fists_idle: PoseValues {
                hand_offset: Vec3::new(0.30, -0.35, -0.60),
                elbow_hint: Vec3::new(0.4, -0.5, 0.1),
                rotation_degrees: Vec3::ZERO,
            },
            fists_windup: PoseValues {
                hand_offset: Vec3::new(0.40, -0.25, -0.35),
                elbow_hint: Vec3::new(0.5, -0.4, 0.4),
                rotation_degrees: Vec3::ZERO,
            },
            fists_punch: PoseValues {
                hand_offset: Vec3::new(0.08, -0.20, -1.10),
                elbow_hint: Vec3::new(0.20, -0.5, -0.5),
                rotation_degrees: Vec3::ZERO,
            },
            fists_heavy_charging: PoseValues {
                hand_offset: Vec3::new(0.20, -0.15, -0.25),
                elbow_hint: Vec3::new(0.4, -0.2, 0.3),
                rotation_degrees: Vec3::ZERO,
            },
            fists_heavy_windup: PoseValues {
                hand_offset: Vec3::new(0.25, -0.05, -0.15),
                elbow_hint: Vec3::new(0.45, -0.1, 0.35),
                rotation_degrees: Vec3::ZERO,
            },
            fists_heavy_active: PoseValues {
                hand_offset: Vec3::new(0.05, -0.35, -1.40),
                elbow_hint: Vec3::new(0.15, -0.5, -0.6),
                rotation_degrees: Vec3::ZERO,
            },
        }
    }

    pub fn get(&self, pose: DebugPose) -> &PoseValues {
        match pose {
            DebugPose::SwordIdle => &self.sword_idle,
            DebugPose::SwordWindup => &self.sword_windup,
            DebugPose::SwordSlash => &self.sword_slash,
            DebugPose::SwordHeavyCharging => &self.sword_heavy_charging,
            DebugPose::SwordHeavyWindup => &self.sword_heavy_windup,
            DebugPose::SwordHeavySlash => &self.sword_heavy_slash,
            DebugPose::FistsIdle => &self.fists_idle,
            DebugPose::FistsWindup => &self.fists_windup,
            DebugPose::FistsPunch => &self.fists_punch,
            DebugPose::FistsHeavyCharging => &self.fists_heavy_charging,
            DebugPose::FistsHeavyWindup => &self.fists_heavy_windup,
            DebugPose::FistsHeavyActive => &self.fists_heavy_active,
        }
    }

    pub fn get_mut(&mut self, pose: DebugPose) -> &mut PoseValues {
        match pose {
            DebugPose::SwordIdle => &mut self.sword_idle,
            DebugPose::SwordWindup => &mut self.sword_windup,
            DebugPose::SwordSlash => &mut self.sword_slash,
            DebugPose::SwordHeavyCharging => &mut self.sword_heavy_charging,
            DebugPose::SwordHeavyWindup => &mut self.sword_heavy_windup,
            DebugPose::SwordHeavySlash => &mut self.sword_heavy_slash,
            DebugPose::FistsIdle => &mut self.fists_idle,
            DebugPose::FistsWindup => &mut self.fists_windup,
            DebugPose::FistsPunch => &mut self.fists_punch,
            DebugPose::FistsHeavyCharging => &mut self.fists_heavy_charging,
            DebugPose::FistsHeavyWindup => &mut self.fists_heavy_windup,
            DebugPose::FistsHeavyActive => &mut self.fists_heavy_active,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// INPUT SYSTEM
// ═══════════════════════════════════════════════════════════════════

pub fn pose_debug_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pose_debug: ResMut<PoseDebugState>,
) {
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    // F10 — toggle
    if keyboard.just_pressed(KeyCode::F10) {
        pose_debug.enabled = !pose_debug.enabled;
        if pose_debug.enabled {
            info!("════════════════════════════════════════════════════");
            info!("🎨 POSE DEBUG: ON");
            info!("   F11 = next pose, Shift+F11 = prev pose");
            info!("   F12 = switch edit target (offset/elbow/rotation)");
            info!("   ← → = switch axis (X/Y/Z)");
            info!("   ↑ ↓ = adjust value, Shift = large step");
            info!("   F9  = print all poses (copy to code)");
            info!("════════════════════════════════════════════════════");
            print_current_state(&pose_debug);
        } else {
            info!("🎨 POSE DEBUG: OFF");
        }
    }

    if !pose_debug.enabled {
        return;
    }

    // F11 — switch pose
    if keyboard.just_pressed(KeyCode::F11) {
        pose_debug.current_pose = if shift {
            pose_debug.current_pose.prev()
        } else {
            pose_debug.current_pose.next()
        };
        info!("🎨 Pose: {}", pose_debug.current_pose.name());
    }

    // F12 — switch edit target
    if keyboard.just_pressed(KeyCode::F12) {
        pose_debug.edit_target = pose_debug.edit_target.next();
        info!("🎨 Editing: {}", pose_debug.edit_target.name());
    }

    // ← → — switch axis
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        pose_debug.axis = (pose_debug.axis + 2) % 3;
        info!(
            "🎨 Axis: {}",
            axis_name(pose_debug.axis, pose_debug.edit_target)
        );
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        pose_debug.axis = (pose_debug.axis + 1) % 3;
        info!(
            "🎨 Axis: {}",
            axis_name(pose_debug.axis, pose_debug.edit_target)
        );
    }

    // ↑ ↓ — adjust value
    let delta = if keyboard.just_pressed(KeyCode::ArrowUp) {
        1.0
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        -1.0
    } else {
        0.0
    };

    if delta != 0.0 {
        // Копируем все нужные значения ДО mutable borrow
        let current_pose = pose_debug.current_pose;
        let edit_target = pose_debug.edit_target;
        let axis = pose_debug.axis;
        let step = match edit_target {
            EditTarget::HandRotation => pose_debug.step_rotation,
            _ => {
                if shift {
                    pose_debug.step_large
                } else {
                    pose_debug.step_small
                }
            }
        };

        // Теперь можно делать mutable borrow
        let pose_values = pose_debug.poses.get_mut(current_pose);

        let vec = match edit_target {
            EditTarget::HandOffset => &mut pose_values.hand_offset,
            EditTarget::ElbowHint => &mut pose_values.elbow_hint,
            EditTarget::HandRotation => &mut pose_values.rotation_degrees,
        };

        match axis {
            0 => vec.x += delta * step,
            1 => vec.y += delta * step,
            2 => vec.z += delta * step,
            _ => {}
        }

        let axis_char = match axis {
            0 => 'X',
            1 => 'Y',
            2 => 'Z',
            _ => '?',
        };
        let value = match axis {
            0 => vec.x,
            1 => vec.y,
            _ => vec.z,
        };
        info!(
            "🎨 {}.{} = {:.2}",
            edit_target.name().split_whitespace().next().unwrap(),
            axis_char,
            value
        );
    }

    // F9 — print all poses
    if keyboard.just_pressed(KeyCode::F9) {
        print_all_poses(&pose_debug.poses);
    }
}

fn axis_name(axis: usize, target: EditTarget) -> &'static str {
    match target {
        EditTarget::HandRotation => match axis {
            0 => "X (pitch)",
            1 => "Y (yaw)",
            _ => "Z (roll)",
        },
        _ => match axis {
            0 => "X (left/right)",
            1 => "Y (up/down)",
            _ => "Z (forward/back)",
        },
    }
}

fn print_current_state(pose_debug: &PoseDebugState) {
    let pose = pose_debug.poses.get(pose_debug.current_pose);
    info!("   Pose: {}", pose_debug.current_pose.name());
    info!("   Editing: {}", pose_debug.edit_target.name());
    info!(
        "   Axis: {}",
        axis_name(pose_debug.axis, pose_debug.edit_target)
    );
    info!(
        "   hand_offset:   ({:.2}, {:.2}, {:.2})",
        pose.hand_offset.x, pose.hand_offset.y, pose.hand_offset.z
    );
    info!(
        "   elbow_hint:    ({:.2}, {:.2}, {:.2})",
        pose.elbow_hint.x, pose.elbow_hint.y, pose.elbow_hint.z
    );
    info!(
        "   rotation_deg:  ({:.1}, {:.1}, {:.1})",
        pose.rotation_degrees.x, pose.rotation_degrees.y, pose.rotation_degrees.z
    );
}

fn print_all_poses(poses: &PoseDebugData) {
    info!("");
    info!("════════════════════════════════════════════════════════════════");
    info!("🎨 ALL POSES (copy to components.rs):");
    info!("════════════════════════════════════════════════════════════════");

    for &pose_type in DebugPose::ALL {
        let pose = poses.get(pose_type);
        info!("");
        info!("pub fn {}() -> Self {{", pose_type.fn_name());
        info!("    Self {{");
        info!(
            "        hand_offset: Vec3::new({:.2}, {:.2}, {:.2}),",
            pose.hand_offset.x, pose.hand_offset.y, pose.hand_offset.z
        );
        info!(
            "        elbow_hint: Vec3::new({:.2}, {:.2}, {:.2}),",
            pose.elbow_hint.x, pose.elbow_hint.y, pose.elbow_hint.z
        );
        info!(
            "        hand_rotation: Self::rot({:.1}, {:.1}, {:.1}),",
            pose.rotation_degrees.x, pose.rotation_degrees.y, pose.rotation_degrees.z
        );
        info!("    }}");
        info!("}}");
    }

    info!("");
    info!("════════════════════════════════════════════════════════════════");
}

// ═══════════════════════════════════════════════════════════════════
// APPLY DEBUG POSE TO IK TARGET
// ═══════════════════════════════════════════════════════════════════

/// Когда pose debug включён — принудительно применяем выбранную позу
pub fn apply_debug_pose_to_ik(debug: Res<PoseDebugState>, mut ik_targets: Query<&mut IkTarget>) {
    if !debug.enabled {
        return;
    }

    let pose_values = debug.poses.get(debug.current_pose);
    let arm_pose = pose_values.to_arm_pose();

    for mut ik_target in &mut ik_targets {
        // Применяем только к правой руке (дебаг позы всегда для правой)
        if ik_target.side != ArmSide::Right {
            continue;
        }

        // Принудительно ставим позу без интерполяции
        ik_target.position = arm_pose.hand_offset;
        ik_target.elbow_hint = arm_pose.elbow_hint;
        ik_target.hand_rotation = arm_pose.hand_rotation;
    }
}
