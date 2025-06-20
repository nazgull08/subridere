use bevy::prelude::*;

use crate::game_init::{lighting::spawn_lighting, player::spawn_player};

use super::{
    lighting::setup_ambient_light, maze_rooms::{spawn_maze_rooms, spawn_room_lights}, play_animation::{load_fox_animation, play_fox_animation}, spawn_model::spawn_test_model
};

/// Плагин инициализации игрового состояния:
/// освещение, игрок, начальные комнаты и т.п.
pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_ambient_light,
                spawn_lighting,
                spawn_player,
                spawn_maze_rooms,
                spawn_room_lights,
                spawn_test_model,
                load_fox_animation,
            ),
        )
        .add_systems(Update, play_fox_animation);
    }
}
