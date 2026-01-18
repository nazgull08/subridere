// fighting/plugin.rs

use bevy::prelude::*;

use crate::app::AppState;

use super::components::CurrentAttackTimings;
use super::events::MeleeHitEvent;
use super::melee::{process_combat_state, process_melee_collisions};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentAttackTimings>()
            .add_event::<MeleeHitEvent>()
            .add_systems(
                Update,
                (process_combat_state, process_melee_collisions)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );

        info!("✅ Combat plugin initialized (Souls-like)");
    }
}
