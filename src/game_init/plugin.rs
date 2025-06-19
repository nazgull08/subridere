use bevy::prelude::*;

use crate::game_init::{
    lighting::spawn_lighting, player::spawn_player,
};

use super::{lighting::setup_ambient_light, maze_rooms::{spawn_maze_rooms, spawn_room_lights}};

/// Плагин инициализации игрового состояния:
/// освещение, игрок, начальные комнаты и т.п.
pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ambient_light, spawn_lighting, spawn_player, spawn_maze_rooms, spawn_room_lights));
    }
}


