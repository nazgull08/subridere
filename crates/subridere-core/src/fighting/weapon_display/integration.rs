use bevy::prelude::*;
use crate::camera::flycam::FlyCamera;
use crate::fighting::projectile::weapons::CurrentWeapon;
use crate::fighting::weapon_display::component::WeaponFiredEvent;
use crate::input::component::PlayerControlled;
use super::component::WeaponDisplay;
use super::spawn::create_weapon_display;

/// Triggers cooldown on weapon display after player shoots
pub fn trigger_weapon_cooldown(
    mut weapon_fired: EventReader<WeaponFiredEvent>,  // ✅ ИЗМЕНИТЬ на EventReader
    players: Query<&Children, With<PlayerControlled>>,
    cameras: Query<&Children, With<FlyCamera>>,
    mut displays: Query<&mut WeaponDisplay>,
) {
    // ✅ ДОБАВИТЬ: Только если был выстрел
    for _event in weapon_fired.read() {
        // Find player's camera
        for player_children in &players {
            for child in player_children.iter() {
                if let Ok(camera_children) = cameras.get(child) {
                    // Find weapon display
                    for weapon_child in camera_children.iter() {
                        if let Ok(mut display) = displays.get_mut(weapon_child) {
                            display.trigger_cooldown();
                            info!("⏱️ Weapon cooldown triggered");
                        }
                    }
                }
            }
        }
    }
}

/// Updates weapon display when player switches weapons
pub fn update_weapon_display_on_switch(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    players: Query<(&Children, &CurrentWeapon), (With<PlayerControlled>, Changed<CurrentWeapon>)>,
    cameras: Query<(Entity, &Children), With<FlyCamera>>,
    displays: Query<Entity, With<WeaponDisplay>>,
) {
    for (player_children, current_weapon) in &players {
        // Find player's camera
        for child in player_children.iter() {
            if let Ok((camera_entity, camera_children)) = cameras.get(child) {
                // Remove old weapon display
                for weapon_child in camera_children.iter() {
                    if displays.get(weapon_child).is_ok() {
                        commands.entity(weapon_child).despawn();
                        info!("🗑️ Despawned old weapon display");
                    }
                }
                
                // Spawn new weapon display
                commands.entity(camera_entity).with_children(|parent| {
                    create_weapon_display(
                        parent,
                        current_weapon.weapon_type,
                        meshes.as_mut(),  // ✅ Добавить .as_mut()
                        materials.as_mut(),  // ✅ Добавить .as_mut()
                    );
                    info!("✨ Spawned new weapon display: {:?}", current_weapon.weapon_type);
                });
            }
        }
    }
}

/// Prevents shooting while weapon is on cooldown
pub fn check_weapon_ready(
    players: Query<&Children, With<PlayerControlled>>,
    cameras: Query<&Children, With<FlyCamera>>,
    displays: Query<&WeaponDisplay>,
) -> bool {
    for player_children in &players {
        for child in player_children.iter() {
            if let Ok(camera_children) = cameras.get(child) {
                for weapon_child in camera_children.iter() {
                    if let Ok(display) = displays.get(weapon_child) {
                        return display.is_ready();
                    }
                }
            }
        }
    }
    true  // Default: allow shooting if no display found
}
