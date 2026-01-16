use bevy::prelude::*;
use rand::Rng;

use crate::items::{ItemId, ItemRegistry, spawn_world_item};
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

    let room_size = Vec3::new(12.0, 6.0, 12.0);
    let spawn_height = 1.0;

    let room_positions: Vec<IVec3> = room_map.rooms.keys().copied().collect();

    if room_positions.is_empty() {
        warn!("No rooms found in RoomMap!");
        next_state.set(InitStage::Done);
        return;
    }

    info!("🎲 Spawning loot across {} rooms", room_positions.len());

    // === WEAPONS ===
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::WoodenStaff,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🪄 Staff",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::IronSword,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "⚔️ Sword",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::WoodenShield,
        1,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🛡️ Shield",
    );

    // === ARMOR ===
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::IronHelmet,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🪖 Helmet",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::ChainmailVest,
        1,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🦺 Chainmail",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::LeatherBootLeft,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "👢 Boot L",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::LeatherBootRight,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "👢 Boot R",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::IronGauntletLeft,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🧤 Gauntlet L",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::IronGauntletRight,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🧤 Gauntlet R",
    );

    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::VitalityHelm,
        2,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "💚 Vitality Helmet",
    );

    // === ACCESSORIES ===
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::GoldRing,
        3,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "💍 Ring",
    );

    // === CONSUMABLES ===
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::HealthPotion,
        4,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "❤️ Health Pot",
    );
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::ManaPotion,
        3,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "💙 Mana Pot",
    );

    // === MISC ===
    spawn_items_with_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        &registry,
        ItemId::Torch,
        5,
        &room_positions,
        room_size,
        spawn_height,
        &mut rng,
        "🔥 Torch",
    );

    info!("✅ Loot spawning complete!");
    next_state.set(InitStage::Done);
}

/// Helper function to spawn multiple items and mark them as GameEntity
fn spawn_items_with_marker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    registry: &Res<ItemRegistry>,
    item_id: ItemId,
    count: usize,
    room_positions: &[IVec3],
    room_size: Vec3,
    spawn_height: f32,
    rng: &mut impl Rng,
    label: &str,
) {
    for i in 0..count {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];
        let pos = calculate_spawn_position(&room_pos, &room_size, spawn_height, rng);

        let _entity =
            spawn_world_item(commands, registry, item_id, 1, pos, None, meshes, materials);
        info!("  {} {} at room {:?}", label, i + 1, room_pos);
    }
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
