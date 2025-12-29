use crate::enemies::worm::spawn::spawn_worm;
use bevy::prelude::*;

use super::state::InitStage;

/// Spawns test enemies in the world
pub fn spawn_test_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
) {
    // Spawn a single test worm
    let spawn_pos = Vec3::new(20.0, 10.0, 0.0);

    spawn_worm(&mut commands, &mut meshes, &mut materials, spawn_pos);

    next_state.set(InitStage::Done);
}
