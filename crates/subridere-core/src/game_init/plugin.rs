use bevy::prelude::*;

use crate::game_init::{lighting::spawn_lighting, player::spawn_player};

use super::{
    assets::{load_game_assets, wait_for_assets},
    enemies::spawn_test_enemies,
    lighting::setup_ambient_light,
    loot::spawn_loot,
    maze_rooms::{spawn_maze_rooms, spawn_room_lights},
    state::InitStage,
};

pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InitStage>()
            .add_systems(Startup, (setup_ambient_light, spawn_lighting))
            .add_systems(OnEnter(InitStage::Setup), load_game_assets)
            .add_systems(
                Update,
                wait_for_assets.run_if(in_state(InitStage::AssetsLoading)),
            )
            .add_systems(OnEnter(InitStage::Setup), spawn_maze_rooms)
            .add_systems(OnEnter(InitStage::MazeReady), spawn_room_lights)
            .add_systems(OnEnter(InitStage::LightsReady), spawn_player)
            .add_systems(OnEnter(InitStage::EnemiesReady), spawn_test_enemies)
            .add_systems(OnEnter(InitStage::ItemsReady), spawn_loot);
    }
}
