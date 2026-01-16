use bevy::prelude::*;

use crate::app::AppState;
use crate::core::components::GameEntity;
use crate::game_init::{lighting::spawn_lighting, player::spawn_player};
use crate::items::registry::registry_loaded;
use crate::ui::game_menu::state::GameMenuState; // ← ДОБАВИТЬ
use crate::world::room::types::RoomMap;

use super::{
    assets::{load_game_assets, wait_for_assets},
    enemies::spawn_test_enemies,
    lighting::setup_ambient_light,
    loot::spawn_loot,
    maze_rooms::{spawn_maze_rooms, spawn_room_lights},
    menu_camera::{MenuCamera, despawn_menu_camera},
    state::InitStage,
    win_condition::check_victory_condition,
};

pub struct GameInitPlugin;

impl Plugin for GameInitPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InitStage>()
            .add_systems(Startup, setup_ambient_light)
            // UI States: спавн камеры для рендера UI
            .add_systems(OnEnter(AppState::MainMenu), spawn_menu_camera_if_missing)
            .add_systems(OnEnter(AppState::Dead), spawn_menu_camera_if_missing)
            .add_systems(OnEnter(AppState::Victory), spawn_menu_camera_if_missing)
            // InGame: убрать menu camera, начать init
            .add_systems(
                OnEnter(AppState::InGame),
                (despawn_menu_camera, start_game_init).chain(),
            )
            // Init stages
            .add_systems(
                OnEnter(InitStage::Setup),
                (load_game_assets, spawn_maze_rooms, spawn_lighting),
            )
            .add_systems(
                Update,
                wait_for_assets.run_if(in_state(InitStage::AssetsLoading)),
            )
            .add_systems(OnEnter(InitStage::MazeReady), spawn_room_lights)
            .add_systems(OnEnter(InitStage::LightsReady), spawn_player)
            .add_systems(OnEnter(InitStage::EnemiesReady), spawn_test_enemies)
            .add_systems(
                OnEnter(InitStage::ItemsReady),
                spawn_loot.run_if(registry_loaded),
            )
            // Win condition check
            .add_systems(
                Update,
                check_victory_condition.run_if(in_state(AppState::InGame)),
            )
            // Cleanup
            .add_systems(OnExit(AppState::InGame), cleanup_game)
            .add_systems(OnExit(AppState::Dead), cleanup_game)
            .add_systems(OnExit(AppState::Victory), cleanup_game);
    }
}

fn start_game_init(mut next_init: ResMut<NextState<InitStage>>) {
    info!("🎮 Starting game initialization...");
    next_init.set(InitStage::Setup);
}

fn spawn_menu_camera_if_missing(mut commands: Commands, query: Query<Entity, With<MenuCamera>>) {
    if query.is_empty() {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            MenuCamera,
            Name::new("MenuCamera"),
        ));
        info!("📷 Menu camera spawned");
    }
}

fn cleanup_game(
    mut commands: Commands,
    mut next_init: ResMut<NextState<InitStage>>,
    mut next_game_menu: ResMut<NextState<GameMenuState>>, // ← ДОБАВИТЬ
    mut room_map: ResMut<RoomMap>,
    game_entities: Query<Entity, With<GameEntity>>,
) {
    info!("🧹 Starting game cleanup...");

    // Закрыть GameMenu если открыто
    next_game_menu.set(GameMenuState::Closed); // ← ДОБАВИТЬ

    let count = game_entities.iter().count();
    for entity in &game_entities {
        commands.entity(entity).despawn();
    }
    info!("  ✓ Despawned {} game entities", count);

    room_map.rooms.clear();
    info!("  ✓ Cleared RoomMap");

    next_init.set(InitStage::Idle);

    info!("🧹 Game cleanup complete");
}
