use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::render::RapierDebugRenderPlugin;

use crate::audio::plugin::SubAudioPlugin;
use crate::camera::plugin::CameraPlugin;
use crate::core::fps_stats::FpsStatsPlugin;
use crate::enemies::EnemiesPlugin;
use crate::fighting::projectile::plugin::ProjectilePlugin;
use crate::game_init::plugin::GameInitPlugin;
use crate::input::plugin::InputPlugin;
use crate::items::plugin::ItemsPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::stats::plugin::StatsPlugin;
use crate::ui::hud::plugin::HudUiPlugin;
use crate::unit::plugin::UnitPlugin;
use crate::world::plugin::WorldPlugin;

pub fn run() {
    App::new()
        // ── Engine plugins ───────────────────────
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Subridere".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameInitPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AudioPlugin)
        .add_plugins(SubAudioPlugin)
        // ── Core systems ─────────────────────────
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(FpsStatsPlugin)
        .add_plugins(HudUiPlugin)
        // ── Game logic ───────────────────────────
        .add_plugins(StatsPlugin)
        .add_plugins(UnitPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(ItemsPlugin)
        .run();
}
