use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    camera::flycam::FlyCamera,
    input::component::PlayerControlled,
    player::component::{PLAYER_START_POS, Player, PlayerVisual},
    player::visual::create_player_body_bundle,
    unit::component::{Grounded, Unit, Velocity},
};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let visual = PlayerVisual::default();
    let (mesh, material) = create_player_body_bundle(&mut meshes, &mut materials, &visual);

    let player_id = commands
        .spawn((
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
        ))
        .id();

    commands.entity(player_id).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            FlyCamera::default(),
            Transform::from_xyz(0.0, 1.6, 0.0),
            Name::new("PlayerCamera"),
        ));
    });
}
