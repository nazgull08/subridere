use bevy::prelude::*;

use super::system::despawn_expired_projectiles;
use crate::fighting::weapon_display::{
    animation::{animate_weapon_display, hide_weapon_on_shoot},
    component::WeaponFiredEvent,
    integration::{trigger_weapon_cooldown, update_weapon_display_on_switch},
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // ✅ Register event
            .add_event::<WeaponFiredEvent>()
            // ✅ Add systems
            .add_systems(
                Update,
                (
                    despawn_expired_projectiles,
                    animate_weapon_display,
                    hide_weapon_on_shoot,
                    trigger_weapon_cooldown,
                    update_weapon_display_on_switch,
                ),
            );
    }
}
