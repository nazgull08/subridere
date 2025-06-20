use bevy::prelude::*;

use crate::{enemy::spawn::spawn_jester_in_room, game_init::{lighting::spawn_lighting, player::spawn_player}};

use super::{
    lighting::setup_ambient_light, maze_rooms::{spawn_maze_rooms, spawn_room_lights}, play_animation::{load_fox_animation, play_fox_animation}, spawn_model::spawn_test_model, state::InitStage
};


pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app
            // Регистрируем наш State
            .init_state::<InitStage>()
            // --- Startup этап: базовые вещи + генерация лабиринта ---
            .add_systems(Startup, (
                setup_ambient_light,
                spawn_lighting,
                spawn_test_model,
                load_fox_animation
            ))
            .add_systems(OnEnter(InitStage::Setup), spawn_maze_rooms)
            .add_systems(OnEnter(InitStage::MazeReady), spawn_room_lights)
            .add_systems(OnEnter(InitStage::LightsReady), spawn_player)
            .add_systems(OnEnter(InitStage::PlayerReady), spawn_jester_in_room)
            .add_systems(Update, play_fox_animation);
            // --- Бесконечный Update для анимации ---
    }
}
