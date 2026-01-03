use super::{
    fps::UiFpsPlugin,
    hitflash::{HitFlashEvent, spawn_hit_overlay, update_hit_overlay},
    pickup_hint::{spawn_pickup_hint, update_pickup_hint},
    stats::UiStatsPlugin,
};
use bevy::prelude::*;

pub struct HudUiPlugin;

impl Plugin for HudUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiFpsPlugin)
            .add_plugins(UiStatsPlugin)
            .add_event::<HitFlashEvent>()
            .add_systems(Startup, spawn_pickup_hint)
            .add_systems(Update, update_pickup_hint)
            .add_systems(Update, (spawn_hit_overlay, update_hit_overlay));
    }
}
