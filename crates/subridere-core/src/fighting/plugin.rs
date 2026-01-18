use bevy::prelude::*;

use crate::app::AppState;

use super::events::MeleeHitEvent;
use super::melee::{process_combat_state, process_melee_collisions, track_item_physics};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MeleeHitEvent>().add_systems(
            Update,
            (
                process_combat_state,
                process_melee_collisions,
                track_item_physics,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );

        info!("✅ Combat plugin initialized");
    }
}
