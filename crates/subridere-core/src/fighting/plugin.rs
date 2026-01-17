use bevy::prelude::*;

use crate::app::AppState;

use super::events::MeleeHitEvent;
use super::melee::{apply_melee_damage, process_combat_state};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MeleeHitEvent>().add_systems(
            Update,
            (process_combat_state, apply_melee_damage)
                .chain()
                .run_if(in_state(AppState::InGame)),
        );

        info!("✅ Combat plugin initialized");
    }
}
