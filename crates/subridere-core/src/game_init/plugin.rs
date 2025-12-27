use bevy::prelude::*;

use crate::game_init::{lighting::spawn_lighting, player::spawn_player};

use super::{
    enemies::spawn_test_enemies,
    lighting::setup_ambient_light,
    maze_rooms::{spawn_maze_rooms, spawn_room_lights},
    state::InitStage,
};

pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InitStage>()
            .add_systems(Startup, (setup_ambient_light, spawn_lighting))
            .add_systems(OnEnter(InitStage::Setup), spawn_maze_rooms)
            .add_systems(OnEnter(InitStage::MazeReady), spawn_room_lights)
            .add_systems(OnEnter(InitStage::LightsReady), spawn_player)
            .add_systems(OnEnter(InitStage::EnemiesReady), spawn_test_enemies);
    }
}
