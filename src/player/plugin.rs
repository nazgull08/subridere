use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    camera::flycam::FlyCamera, 
    player::component::{Player, PlayerVisual, PLAYER_START_POS},
    player::visual::create_player_body_bundle,
    unit::component::{Unit, Velocity, Grounded},
    input::component::PlayerControlled,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (
               kill_plane_system,
           ));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let visual = PlayerVisual::default();
    let (mesh, material) = create_player_body_bundle(&mut meshes, &mut materials, &visual);

    let player_id = commands.spawn((
        Player,
        Unit,
        PlayerControlled,
        Grounded(true),
        Velocity::default(),
        visual,
        mesh,
        material,
        Collider::capsule_y(0.9, 0.3),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
        Transform::from_translation(PLAYER_START_POS),
        Visibility::Visible,
        Name::new("Player"),
    )).id();

    // Camera attached to head
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
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y < -50.0 {
            println!("💀 Player died. Respawning...");
            transform.translation = PLAYER_START_POS;
            velocity.0 = Vec3::ZERO;
        }
    }
}
