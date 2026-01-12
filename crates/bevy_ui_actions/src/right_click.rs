use crate::action::UiAction;
use bevy::prelude::*;
use std::sync::Arc;

/// Действие при правом клике
#[derive(Component)]
pub struct OnRightClick {
    action: Arc<dyn UiAction>,
}

impl OnRightClick {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }

    pub(crate) fn execute(&self, commands: &mut Commands) {
        let action = self.action.clone();
        commands.queue(move |world: &mut World| {
            action.execute(world);
        });
    }
}
