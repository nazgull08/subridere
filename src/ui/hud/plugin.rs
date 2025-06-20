use bevy::prelude::*;
use super::{fps::UiFpsPlugin, stats::UiStatsPlugin};

pub struct HudUiPlugin;

impl Plugin for HudUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiFpsPlugin)
           .add_plugins(UiStatsPlugin);
    }
}
