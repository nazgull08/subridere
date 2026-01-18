// crates/subridere-core/src/player/arm/plugin.rs

use bevy::prelude::*;

use crate::app::AppState;

use super::components::ArmConfig;
use super::debug::{ArmDebugState, arm_debug_system};
use super::ik_system::{apply_arm_ik, update_ik_target_from_combat};

pub struct PlayerArmPlugin;

impl Plugin for PlayerArmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArmConfig>()
            .init_resource::<ArmDebugState>()
            .add_systems(
                Update,
                (arm_debug_system, update_ik_target_from_combat, apply_arm_ik)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );

        info!("✅ Player arm IK plugin initialized");
        info!("🎮 ARM DEBUG: F5=switch axis, ↑/↓=adjust, F6=print values");
    }
}
