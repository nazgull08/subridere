// crates/subridere-core/src/player/arm/plugin.rs

use bevy::prelude::*;

use crate::app::AppState;
use crate::player::body::VisibleBodyConfig;

use super::components::ArmConfig;
use super::ik_system::{apply_arm_ik, update_ik_target_from_combat};
use super::pose_debug::{PoseDebugState, apply_debug_pose_to_ik, pose_debug_input};
use super::weapon_visual::{
    WeaponDebugState, apply_weapon_debug_transform, sync_equipped_weapon_visual, weapon_debug_input,
};

pub struct PlayerArmPlugin;

impl Plugin for PlayerArmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArmConfig>()
            .init_resource::<VisibleBodyConfig>()
            .init_resource::<WeaponDebugState>()
            .init_resource::<PoseDebugState>()
            .add_systems(
                Update,
                (
                    // Input systems (всегда активны для toggle)
                    weapon_debug_input,
                    pose_debug_input,
                    // IK systems
                    update_ik_target_from_combat,
                    apply_debug_pose_to_ik, // Перезаписывает IK target если дебаг включён
                    apply_arm_ik,
                    // Weapon visual
                    sync_equipped_weapon_visual,
                    apply_weapon_debug_transform,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );

        info!("✅ Player arm IK plugin initialized");
        info!("🎮 DEBUG CONTROLS:");
        info!("   F5/F6  = Arm position debug");
        info!("   F7-F9  = Weapon grip debug");
        info!("   F10-F12 = Pose debug (NEW)");
    }
}
