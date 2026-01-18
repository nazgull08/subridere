// fighting/plugin.rs

use bevy::prelude::*;

use crate::app::AppState;
use crate::fighting::ChargeConfig;
use crate::player::arm::{
    WeaponDebugState, apply_weapon_debug_transform, sync_equipped_weapon_visual, weapon_debug_input,
};

use super::components::CurrentAttackTimings;
use super::events::MeleeHitEvent;
use super::melee::{process_combat_state, process_melee_collisions};
use super::weapon::sync_weapon_timings;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentAttackTimings>()
            .init_resource::<ChargeConfig>()
            .init_resource::<WeaponDebugState>()
            .add_event::<MeleeHitEvent>()
            .add_systems(
                Update,
                (
                    weapon_debug_input,
                    sync_weapon_timings,
                    sync_equipped_weapon_visual,
                    apply_weapon_debug_transform,
                    process_combat_state,
                    process_melee_collisions,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );

        info!("✅ Combat plugin initialized (Souls-like)");
        info!("🗡️ WEAPON DEBUG: F7=toggle, F8=axis, ↑/↓=adjust, F9=print");
    }
}
