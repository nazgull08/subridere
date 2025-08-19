use super::{
    fps::UiFpsPlugin,
    hitflash::{HitFlashEvent, spawn_hit_overlay, update_hit_overlay},
    stats::UiStatsPlugin,
};
use bevy::prelude::*;

pub struct HudUiPlugin;

impl Plugin for HudUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiFpsPlugin)
            .add_plugins(UiStatsPlugin)
            .add_event::<HitFlashEvent>()
            .add_systems(Update, (spawn_hit_overlay, update_hit_overlay));
    }
}
