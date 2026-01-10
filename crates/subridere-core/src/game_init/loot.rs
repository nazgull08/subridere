use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;

use crate::items::WorldItem;
use crate::items::component::Pickupable;
use crate::items::definition::ItemDefinition;
use crate::items::spawn::{WorldItemSpawnConfig, spawn_world_item};
use crate::items::visual::definition::VisualDefinition;
use crate::items::visual::shape::VisualPart;
use crate::items::visual::spawn_item_visual_with_colliders;
use crate::world::room::types::RoomMap;

use super::assets::GameAssets;
use super::state::InitStage;

/// Spawns random loot items across maze rooms
pub fn spawn_loot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
    room_map: Res<RoomMap>,
    game_assets: Res<GameAssets>,
    visuals: Res<Assets<VisualDefinition>>,
    item_defs: Res<Assets<ItemDefinition>>,
) {
    let mut rng = rand::thread_rng();

    // Room configuration
    let room_size = Vec3::new(12.0, 6.0, 12.0);
    let spawn_height = 1.0; // 1m above floor (floating items)

    // Collect all room positions
    let room_positions: Vec<IVec3> = room_map.rooms.keys().copied().collect();

    if room_positions.is_empty() {
        warn!("No rooms found in RoomMap!");
        next_state.set(InitStage::Done);
        return;
    }

    // Get visuals (guaranteed to be loaded at this stage)
    let staff_visual = visuals
        .get(&game_assets.wooden_staff_visual)
        .expect("Wooden staff visual should be loaded by now!");
    let helmet_visual = visuals
        .get(&game_assets.iron_helmet_visual)
        .expect("Iron helmet visual should be loaded by now!");

    info!("🪄 Spawning loot across {} rooms", room_positions.len());

    // Spawn 2 wooden staffs
    for i in 0..2 {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];
        let final_pos = calculate_spawn_position(&room_pos, &room_size, spawn_height, &mut rng);

        spawn_world_item(
            &mut commands,
            WorldItemSpawnConfig {
                item_id: "wooden_staff".to_string(),
                quantity: 1,
                position: final_pos,
                initial_velocity: None, // Static loot, no velocity
            },
            &game_assets,
            &visuals,
            &item_defs,
            &mut meshes,
            &mut materials,
        );

        info!(
            "  🪄 Staff {} spawned at room {:?} (world: {:?})",
            i + 1,
            room_pos,
            final_pos
        );
    }

    // Spawn 2 iron helmets
    for i in 0..2 {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];
        let final_pos = calculate_spawn_position(&room_pos, &room_size, spawn_height, &mut rng);

        spawn_world_item(
            &mut commands,
            WorldItemSpawnConfig {
                item_id: "iron_helmet".to_string(),
                quantity: 1,
                position: final_pos,
                initial_velocity: None, // Static loot, no velocity
            },
            &game_assets,
            &visuals,
            &item_defs,
            &mut meshes,
            &mut materials,
        );

        info!(
            "  Helmet {} spawned at room {:?} (world: {:?})",
            i + 1,
            room_pos,
            final_pos
        );
    }

    next_state.set(InitStage::Done);
}

/// Calculate random spawn position within a room
fn calculate_spawn_position(
    room_pos: &IVec3,
    room_size: &Vec3,
    spawn_height: f32,
    rng: &mut impl Rng,
) -> Vec3 {
    let world_pos = Vec3::new(
        room_pos.x as f32 * room_size.x,
        spawn_height,
        room_pos.z as f32 * room_size.z,
    );

    let offset = Vec3::new(rng.gen_range(-2.0..2.0), 0.0, rng.gen_range(-2.0..2.0));

    world_pos + offset
}
