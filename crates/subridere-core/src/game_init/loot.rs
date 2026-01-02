use bevy::prelude::*;
use rand::Rng;

use crate::items::component::Pickupable;
use crate::items::visual::definition::VisualDefinition;
use crate::items::visual::spawn::spawn_item_visual;
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

    // Get the staff visual (guaranteed to be loaded at this stage)
    let staff_visual = visuals
        .get(&game_assets.wooden_staff_visual)
        .expect("Wooden staff visual should be loaded by now!");

    // Spawn 2 wooden staffs in random rooms
    let staff_count = 2;

    info!(
        "🪄 Spawning {} wooden staffs across {} rooms",
        staff_count,
        room_positions.len()
    );

    for i in 0..staff_count {
        // Pick random room
        let room_idx = rng.gen_range(0..room_positions.len());
        let room_pos = room_positions[room_idx];

        // Calculate world position (center of room)
        let world_pos = Vec3::new(
            room_pos.x as f32 * room_size.x,
            spawn_height,
            room_pos.z as f32 * room_size.z,
        );

        // Random offset within room
        let offset = Vec3::new(
            rng.gen_range(-2.0..2.0), // ±2m in X
            0.0,
            rng.gen_range(-2.0..2.0), // ±2m in Z
        );

        let final_pos = world_pos + offset;

        // Spawn the staff
        commands
            .spawn((
                Transform::from_translation(final_pos),
                GlobalTransform::default(),
                Visibility::default(),
                Pickupable,
                Name::new("Wooden Staff"),
            ))
            .with_children(|parent| {
                spawn_item_visual(parent, &staff_visual.parts, &mut meshes, &mut materials);
            });

        info!(
            "  🪄 Staff {} spawned at room {:?} (world: {:?})",
            i + 1,
            room_pos,
            final_pos
        );
    }

    next_state.set(InitStage::Done);
}
