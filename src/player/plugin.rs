use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    camera::flycam::FlyCamera, 
    unit::component::Unit,
    player::component::{Player, PlayerBody, PlayerVisual, PLAYER_START_POS},
    player::visual::create_player_body_bundle,
    player::systems::{
        sync_player_body_transform,
        update_player_body_size
    },
    player::visual::{
        toggle_player_body_visibility,
        update_player_body_color
    },
    input::components::{
        MovementInput, 
        MovementStats, 
        MovementState, 
        PlayerControlled
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (
               kill_plane_system,
               sync_player_body_transform,
               update_player_body_size,
               toggle_player_body_visibility,
               update_player_body_color,
           ));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_visual = PlayerVisual::default();
    
    // Создаем основную сущность игрока (коллайдер + контроллер + управление)
    let player_id = commands
        .spawn((
            Player,
            Unit,
            player_visual.clone(),
            // Компоненты управления
            PlayerControlled,
            MovementInput::default(),
            MovementStats::default(),
            MovementState::default(),
            // Физика
            Collider::capsule_y(0.9, 0.3),
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
            KinematicCharacterControllerOutput::default(),
            Transform::from_translation(PLAYER_START_POS),
            Name::new("Player"),
        ))
        .id();

    // Создаем визуальное тело игрока (отдельная сущность)
    if player_visual.show_body {
        let (mesh, material) = create_player_body_bundle(&mut meshes, &mut materials, &player_visual);
        
        commands.spawn((
            PlayerBody,
            mesh,
            material,
            Transform::from_translation(PLAYER_START_POS),
            Visibility::Visible,
            Name::new("PlayerBody"),
        ));
    }

    // Камера — дочерний узел игрока, смещённый на уровень «головы»
    commands.entity(player_id).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            FlyCamera::default(),
            Transform::from_xyz(0.0, 1.6, 0.0),
            Name::new("PlayerCamera"),
        ));
    });
}

fn kill_plane_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut MovementState), With<Player>>,
) {
    for (entity, transform, mut movement_state) in &mut query {
        if transform.translation.y < -50.0 {
            println!("💀 Игрок погиб. {:?} Respawning...", transform); 
            commands.entity(entity).insert(Transform::from_translation(PLAYER_START_POS));
            
            // Сбрасываем скорость при возрождении
            movement_state.velocity = Vec3::ZERO;
            movement_state.is_grounded = false;
        }
    }
}
