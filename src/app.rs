use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::render::RapierDebugRenderPlugin;

use crate::camera::plugin::CameraPlugin;
use crate::core::fps_stats::FpsStatsPlugin;
use crate::input::plugin::InputPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::ui::fps::UiOverlayPlugin;
use crate::unit::plugin::UnitPlugin;
use crate::world::room::plugin::RoomPlugin;

pub fn run() {
    App::new()
        // ── Системные и графические плагины ─────────────────────────────
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Subridere".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::new(100))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true))
//        .add_plugins(RapierDebugRenderPlugin::default())
        // ── Камера ───────────────────────────────────────────────────────
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        // -- player
        .add_plugins(PlayerPlugin)
        .add_plugins(UnitPlugin)
        // ── Логика ───────────────────────────────────────────────────────
        .add_plugins(FpsStatsPlugin)
        .add_plugins(RoomPlugin)
        // ── Стартовые объекты (временные) ───────────────────────────────
        .add_systems(Startup, spawn_light)
        .add_plugins(UiOverlayPlugin)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        }
    );
}
