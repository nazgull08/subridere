use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    camera::flycam::FlyCamera, fighting::{projectile::weapons::CurrentWeapon, weapon_display::spawn::create_weapon_display}, input::component::PlayerControlled, inventory::{Equipment, Inventory}, player::{
        component::{Player, PlayerVisual, PLAYER_START_POS},
        visual::create_player_body_bundle,
    }, stats::{health::component::Health, mana::component::Mana, stamina::component::Stamina}, unit::component::{Grounded, Unit, Velocity}
};

use super::state::InitStage;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<InitStage>>,
) {
    let visual = PlayerVisual::default();
    let (mesh, material) = create_player_body_bundle(&mut meshes, &mut materials, &visual);

    let player_id = commands
        .spawn_empty()
        .insert(Player)
        .insert(Unit)
        .insert(PlayerControlled)
        .insert(Grounded(true))
        .insert(Velocity::default())
        .insert(visual)
        .insert(mesh)
        .insert(material)
        .insert(Collider::capsule_y(0.9, 0.3))
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        })
        .insert(KinematicCharacterControllerOutput::default())
        .insert(Transform::from_translation(PLAYER_START_POS))
        .insert(Visibility::Visible)
        .insert(Name::new("Player"))
        .insert(Health::default())
        .insert(Mana::default())
        .insert(Stamina::default())
        .insert(CurrentWeapon::default())
        .insert(Inventory::default())
        .insert(Equipment::default())
        .id();

    commands.entity(player_id).with_children(|parent| {
        parent
            .spawn((
                Camera3d::default(),
                FlyCamera::default(),
                Name::new("PlayerCamera"),
            ))
            .with_children(|camera_parent| {
                create_weapon_display(
                    camera_parent,
                    CurrentWeapon::default().weapon_type,
                    meshes.as_mut(),
                    materials.as_mut(),
                );
            });
    });

    next_state.set(InitStage::EnemiesReady);
}
