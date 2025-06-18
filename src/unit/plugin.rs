use bevy::prelude::*;

use super::systems::intent::{
    update_grounded_system,
    apply_move_intents,
    apply_jump_intents,
    apply_dash_intents,
    apply_velocity,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_grounded_system,
                apply_move_intents,
                apply_jump_intents,
                apply_dash_intents,
                apply_velocity,
            ));
    }
}
