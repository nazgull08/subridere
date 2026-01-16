use bevy::prelude::*;

use crate::app::AppState;
use crate::game_init::{lighting::spawn_lighting, player::spawn_player};
use crate::items::registry::registry_loaded;
use crate::player::component::Player;

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
            // ✅ Освещение всегда (нужно в меню тоже)
            .add_systems(Startup, (setup_ambient_light, spawn_lighting))
            // ✅ Запуск игры только при входе в InGame
            .add_systems(OnEnter(AppState::InGame), start_game_init)
            // ✅ Остальное без изменений
            .add_systems(OnEnter(InitStage::Setup), load_game_assets)
            .add_systems(
                Update,
                wait_for_assets.run_if(in_state(InitStage::AssetsLoading)),
            )
            .add_systems(OnEnter(InitStage::Setup), spawn_maze_rooms)
            .add_systems(OnEnter(InitStage::MazeReady), spawn_room_lights)
            .add_systems(OnEnter(InitStage::LightsReady), spawn_player)
            .add_systems(OnEnter(InitStage::EnemiesReady), spawn_test_enemies)
            .add_systems(
                OnEnter(InitStage::ItemsReady),
                spawn_loot.run_if(registry_loaded),
            )
            // ✅ Cleanup при выходе из игры
            .add_systems(OnExit(AppState::InGame), cleanup_game);
    }
}

/// Запускает инициализацию игры при входе в InGame state
fn start_game_init(mut next_init: ResMut<NextState<InitStage>>) {
    info!("🎮 Starting game initialization...");
    next_init.set(InitStage::Setup);
}

/// Очищает все игровые сущности при выходе из InGame
fn cleanup_game(
    mut commands: Commands,
    mut next_init: ResMut<NextState<InitStage>>,
    players: Query<Entity, With<Player>>,
) {
    info!("🧹 Starting game cleanup...");

    // Despawn player (с камерой - recursive)
    for entity in &players {
        commands.entity(entity).despawn();
        info!("  ✓ Despawned player");
    }

    // Сброс InitStage для следующей игры
    next_init.set(InitStage::Setup);

    info!("🧹 Game cleanup complete");
}
