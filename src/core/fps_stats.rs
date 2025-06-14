use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Resource, Default)]
pub struct FpsData {
    pub current: f64,
}

pub struct FpsStatsPlugin;

impl Plugin for FpsStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FpsData::default())
            .add_systems(Update, update_fps);
    }
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut fps: ResMut<FpsData>) {
    if let Some(value) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()) {
        fps.current = value;
    }
}
