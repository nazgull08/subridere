use bevy::prelude::*;

use super::systems::movement::{movement_system, jump_system};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            movement_system,
            jump_system
        ));
    }
}
