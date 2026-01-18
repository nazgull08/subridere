use crate::enemies::worm::spawn::spawn_worm;
use crate::world::room::types::RoomMap;
use bevy::prelude::*;
use rand::Rng;

use super::state::InitStage;
use crate::core::components::GameEntity;

/// Spawns worms randomly across maze rooms
pub fn spawn_test_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
    room_map: Res<RoomMap>,
) {
    let mut rng = rand::thread_rng();

    let room_size = Vec3::new(12.0, 6.0, 12.0);
    let spawn_height = 2.0;

    let room_positions: Vec<IVec3> = room_map.rooms.keys().copied().collect();

    if room_positions.is_empty() {
        warn!("No rooms found in RoomMap!");
        next_state.set(InitStage::ItemsReady);
        return;
    }

    let worm_count = 0; //rng.gen_range(3..=5);

    info!(
        "🐛 Spawning {} worms across {} rooms",
        worm_count,
        room_positions.len()
    );

    for i in 0..worm_count {
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];

        let world_pos = Vec3::new(
            room_pos.x as f32 * room_size.x,
            spawn_height,
            room_pos.z as f32 * room_size.z,
        );

        let offset = Vec3::new(rng.gen_range(-3.0..3.0), 0.0, rng.gen_range(-3.0..3.0));

        let final_pos = world_pos + offset;

        let worm_entity = spawn_worm(&mut commands, &mut meshes, &mut materials, final_pos);

        // ← ДОБАВИТЬ: пометить worm как GameEntity
        commands.entity(worm_entity).insert(GameEntity);

        info!(
            "  🐛 Worm {} spawned at room {:?} (world: {:?})",
            i + 1,
            room_pos,
            final_pos
        );
    }

    next_state.set(InitStage::ItemsReady);
}
