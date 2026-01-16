use bevy::prelude::*;

use crate::app::AppState;

use super::{
    crosshair::CrosshairPlugin,
    fps::UiFpsPlugin,
    hitflash::{HitFlashEvent, spawn_hit_overlay, update_hit_overlay},
    pickup_hint::{despawn_pickup_hint, spawn_pickup_hint, update_pickup_hint},
    stats::UiStatsPlugin,
};

pub struct HudUiPlugin;

impl Plugin for HudUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Sub-plugins (они сами управляют своим lifecycle)
            .add_plugins(UiFpsPlugin)
            .add_plugins(UiStatsPlugin)
            .add_plugins(CrosshairPlugin)
            // Events
            .add_event::<HitFlashEvent>()
            // Pickup hint - managed here
            .add_systems(OnEnter(AppState::InGame), spawn_pickup_hint)
            .add_systems(OnExit(AppState::InGame), despawn_pickup_hint)
            // Update systems - only in game
            .add_systems(
                Update,
                (spawn_hit_overlay, update_hit_overlay, update_pickup_hint)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
