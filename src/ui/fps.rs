use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContexts, egui};

use crate::core::fps_stats::FpsData;

/// Плагин UI, отвечает за отрисовку FPS/дебаг информации
pub struct UiOverlayPlugin;

impl Plugin for UiOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin)
            .add_systems(Update, show_ui);
    }
}

fn show_ui(mut contexts: EguiContexts, fps: Res<FpsData>) {
    egui::Window::new("UI")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!("FPS: {:.1}", fps.current));
        });
}
