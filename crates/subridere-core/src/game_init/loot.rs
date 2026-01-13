// game_init/loot.rs — Spawn loot items across maze rooms

use bevy::prelude::*;
use rand::Rng;

use crate::inventory::systems::drop::spawn_world_item;
use crate::items::{ItemId, ItemRegistry};
use crate::world::room::types::RoomMap;

use super::state::InitStage;

/// Spawns random loot items across maze rooms
pub fn spawn_loot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
    room_map: Res<RoomMap>,
    registry: Res<ItemRegistry>,
) {
    let mut rng = rand::thread_rng();

    // Room configuration
    let room_size = Vec3::new(12.0, 6.0, 12.0);
    let spawn_height = 1.0;

    // Collect all room positions
    let room_positions: Vec<IVec3> = room_map.rooms.keys().copied().collect();

    if room_positions.is_empty() {
        warn!("No rooms found in RoomMap!");
        next_state.set(InitStage::Done);
        return;
    }

    info!("🪄 Spawning loot across {} rooms", room_positions.len());

    // Spawn 2 wooden staffs
    for i in 0..2 {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];
        let pos = calculate_spawn_position(&room_pos, &room_size, spawn_height, &mut rng);

        spawn_world_item(
            &mut commands,
            &registry,
            ItemId::WoodenStaff,  // ← Типобезопасно!
            1,
            pos,
            None,
            &mut meshes,
            &mut materials,
        );

        info!("  🪄 Staff {} at room {:?}", i + 1, room_pos);
    }

    // Spawn 2 iron helmets
    for i in 0..2 {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];
        let pos = calculate_spawn_position(&room_pos, &room_size, spawn_height, &mut rng);

        spawn_world_item(
            &mut commands,
            &registry,
            ItemId::IronHelmet,  // ← Типобезопасно!
            1,
            pos,
            None,
            &mut meshes,
            &mut materials,
        );

        info!("  🛡️ Helmet {} at room {:?}", i + 1, room_pos);
    }

    // Spawn 3 health potions
    for i in 0..3 {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];
        let pos = calculate_spawn_position(&room_pos, &room_size, spawn_height, &mut rng);

        spawn_world_item(
            &mut commands,
            &registry,
            ItemId::HealthPotion,  // ← Типобезопасно!
            1,
            pos,
            None,
            &mut meshes,
            &mut materials,
        );

        info!("  🧪 Potion {} at room {:?}", i + 1, room_pos);
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
