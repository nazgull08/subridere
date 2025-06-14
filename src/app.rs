use bevy::prelude::*;
use bevy_flycam::{MovementSettings, PlayerPlugin};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use crate::{core::fps_stats::FpsStatsPlugin, ui::fps::UiOverlayPlugin}; // ← готовая FPS-камера

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Subridere – Flycam".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00008, // default: 0.00012
            speed: 6.0, // default: 12.0
        })
        .add_plugins(FpsStatsPlugin)
        .add_plugins(UiOverlayPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Мир: куб
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        ..default()
    });

    // Свет
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
