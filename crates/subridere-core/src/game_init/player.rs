// crates/subridere-core/src/game_init/player.rs

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    camera::flycam::FlyCamera,
    core::components::GameEntity,
    fighting::PlayerCombatState,
    input::component::PlayerControlled,
    inventory::{Equipment, Inventory},
    player::{
        arm::spawn_player_arms,
        component::{PLAYER_START_POS, Player, PlayerVisual},
    },
    stats::plugin::StatsBundle,
    unit::component::{Grounded, Unit, Velocity},
};

use super::state::InitStage;

/// Высота игрока
const PLAYER_HEIGHT: f32 = 1.8;
/// Высота глаз относительно центра коллайдера
const EYE_HEIGHT: f32 = 0.7;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
) {
    // Материал для тела
    let body_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.8),
        ..default()
    });

    // Меш тела (видимый)
    let body_mesh = meshes.add(Cuboid::new(0.6, PLAYER_HEIGHT, 0.3));

    let player_id = commands
        .spawn_empty()
        .insert(Player)
        .insert(Unit)
        .insert(PlayerControlled)
        .insert(Grounded(true))
        .insert(Velocity::default())
        .insert(PlayerVisual::default())
        // Видимое тело
        .insert(Mesh3d(body_mesh))
        .insert(MeshMaterial3d(body_material))
        // Физика
        .insert(Collider::capsule_y(PLAYER_HEIGHT / 2.0 - 0.3, 0.3))
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        })
        .insert(KinematicCharacterControllerOutput::default())
        .insert(Transform::from_translation(PLAYER_START_POS))
        .insert(Visibility::Visible)
        .insert(Name::new("Player"))
        .insert(StatsBundle::default())
        .insert(Inventory::default())
        .insert(Equipment::default())
        .insert(PlayerCombatState::default())
        .insert(GameEntity)
        .id();

    // Камера на уровне глаз
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            FlyCamera::default(),
            Transform::from_translation(Vec3::new(0.0, EYE_HEIGHT, 0.0)),
            Name::new("PlayerCamera"),
        ))
        .id();

    commands.entity(player_id).add_child(camera_entity);

    // === НОВАЯ СИСТЕМА РУК С IK ===
    spawn_player_arms(
        &mut commands,
        camera_entity,
        meshes.as_mut(),
        materials.as_mut(),
    );

    info!("✅ Player spawned with IK arms");

    next_state.set(InitStage::EnemiesReady);
}
