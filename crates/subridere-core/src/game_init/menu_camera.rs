// game_init/menu_camera.rs — Camera for MainMenu rendering

use bevy::prelude::*;

/// Marker for the menu camera (used in MainMenu state)
#[derive(Component)]
pub struct MenuCamera;

/// Spawn a static camera for rendering UI in MainMenu
pub fn spawn_menu_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        MenuCamera,
        Name::new("MenuCamera"),
    ));
    info!("📷 Menu camera spawned");
}

/// Despawn menu camera when entering game
pub fn despawn_menu_camera(mut commands: Commands, query: Query<Entity, With<MenuCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn();
        info!("📷 Menu camera despawned");
    }
}
