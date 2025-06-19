use bevy::prelude::*;

use crate::game_init::{
    lighting::spawn_lighting, player::spawn_player,
};

use super::{check_animations::{check_available_animations, setup_animation_check}, lighting::setup_ambient_light, maze_rooms::{spawn_maze_rooms, spawn_room_lights}, spawn_model::spawn_test_model};

/// Плагин инициализации игрового состояния:
/// освещение, игрок, начальные комнаты и т.п.
pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ambient_light, spawn_lighting, spawn_player, spawn_maze_rooms, spawn_room_lights, spawn_test_model, setup_animation_check))
            .add_systems(Update, (
                check_available_animations)
            );
    }
}


