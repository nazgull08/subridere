use bevy::prelude::*;

pub struct BackdropPlugin;

impl Plugin for BackdropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_backdrop_structures);
    }
}

fn spawn_backdrop_structures(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let skyscraper_material = materials.add(Color::srgb(0.1, 0.1, 0.15));
    let bridge_material = materials.add(Color::srgb(0.2, 0.2, 0.25));
    let neon_material = materials.add(Color::srgb(0.3, 0.7, 0.9));

    // Несколько башен, разные размеры и координаты
    let towers = vec![
        Vec3::new(-40.0, 50.0, -30.0),
        Vec3::new(60.0, 40.0, -50.0),
        Vec3::new(-80.0, 60.0, 10.0),
        Vec3::new(90.0, 70.0, 30.0),
    ];

}
