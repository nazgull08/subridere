use bevy::prelude::*;

// DISABLED - rebuilding items
// use crate::items::{definition::ItemDefinition, visual::definition::VisualDefinition};

use super::state::InitStage;

/// Resource to store loaded asset handles (placeholder for now)
#[derive(Resource, Default)]
pub struct GameAssets {
    // Items disabled - will be replaced by ItemRegistry
}

/// Load all game assets
pub fn load_game_assets(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<InitStage>>,
) {
    info!("📦 Loading game assets...");

    commands.insert_resource(GameAssets::default());

    info!("✅ Asset handles created");
    next_state.set(InitStage::AssetsLoading);
}

/// Wait for assets to finish loading
pub fn wait_for_assets(
    mut next_state: ResMut<NextState<InitStage>>,
    // No assets to wait for currently
) {
    // Skip straight to next stage
    info!("✅ All assets loaded!");
    next_state.set(InitStage::MazeReady);
}
