use bevy::prelude::*;

use crate::items::{definition::ItemDefinition, visual::definition::VisualDefinition};

use super::state::InitStage;

/// Resource to store loaded asset handles
#[derive(Resource)]
pub struct GameAssets {
    pub wooden_staff_visual: Handle<VisualDefinition>,
    pub wooden_staff_def: Handle<ItemDefinition>,
    pub wooden_staff_icon: Handle<Image>,

    // Iron Helmet
    pub iron_helmet_visual: Handle<VisualDefinition>,
    pub iron_helmet_def: Handle<ItemDefinition>,
    pub iron_helmet_icon: Handle<Image>,
}

/// Load all game assets
pub fn load_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<InitStage>>,
) {
    info!("📦 Loading game assets...");

    // Load visual definitions
    let wooden_staff_visual = asset_server.load("visuals/items/wooden_staff.visual.ron");
    let wooden_staff_def = asset_server.load("items/weapons/wooden_staff.item.ron");
    let wooden_staff_icon = asset_server.load("textures/items/wooden_staff_icon.png");
    // Load iron helmet
    let iron_helmet_visual = asset_server.load("visuals/items/iron_helmet.visual.ron");
    let iron_helmet_def = asset_server.load("items/armor/iron_helmet.item.ron");
    let iron_helmet_icon = asset_server.load("textures/items/iron_helmet_icon.png");

    // Store handles in resource
    commands.insert_resource(GameAssets {
        wooden_staff_visual,
        wooden_staff_def,
        wooden_staff_icon,
        iron_helmet_visual,
        iron_helmet_def,
        iron_helmet_icon,
    });

    info!("✅ Asset handles created, waiting for loading...");

    // Move to next stage immediately
    // We'll check if they're loaded in the next system
    next_state.set(InitStage::AssetsLoading);
}

/// Wait for assets to finish loading
pub fn wait_for_assets(
    mut next_state: ResMut<NextState<InitStage>>,
    game_assets: Res<GameAssets>,
    visuals: Res<Assets<VisualDefinition>>,
) {
    // Check if all assets are loaded
    let staff_loaded = visuals.get(&game_assets.wooden_staff_visual).is_some();
    let helmet_loaded = visuals.get(&game_assets.iron_helmet_visual).is_some();

    if staff_loaded && helmet_loaded {
        info!("✅ All assets loaded!");
        next_state.set(InitStage::MazeReady);
    } else {
        // Still loading, this system will run again next frame
        // No log spam - just wait quietly
    }
}
