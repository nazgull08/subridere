use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::player::component::Player;
use crate::stats::{AttributeType, Attributes};

/// Action: Increase attribute by 1
pub struct IncreaseAttributeAction(pub AttributeType);

impl UiAction for IncreaseAttributeAction {
    fn execute(&self, world: &mut World) {
        let mut query = world.query_filtered::<&mut Attributes, With<Player>>();

        let Ok(mut attributes) = query.single_mut(world) else {
            return;
        };

        if attributes.increase(self.0) {
            info!(
                "⬆️ Increased {:?} (points left: {})",
                self.0, attributes.unspent_points
            );
        } else {
            info!("❌ Cannot increase {:?}", self.0);
        }
    }
}
