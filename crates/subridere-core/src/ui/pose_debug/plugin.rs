// crates/subridere-core/src/ui/pose_debug/plugin.rs

use bevy::prelude::*;

use crate::app::AppState;
use crate::player::arm::pose_debug::PoseDebugState;

use super::components::PoseDebugRoot;
use super::spawn::{despawn_pose_debug_ui, spawn_pose_debug_ui, sync_pose_debug_ui};

pub struct PoseDebugUiPlugin;

impl Plugin for PoseDebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_pose_debug_ui, sync_pose_debug_ui)
                .chain()
                .run_if(in_state(AppState::InGame)),
        );

        info!("✅ Pose Debug UI plugin initialized");
    }
}

/// Спавнит/деспавнит UI при изменении PoseDebugState.enabled
fn toggle_pose_debug_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<PoseDebugState>,
    ui_query: Query<Entity, With<PoseDebugRoot>>,
) {
    let ui_exists = !ui_query.is_empty();

    if state.enabled && !ui_exists {
        // Спавним UI
        spawn_pose_debug_ui(commands, asset_server);
    } else if !state.enabled && ui_exists {
        // Деспавним UI
        despawn_pose_debug_ui(commands, ui_query);
    }
}
