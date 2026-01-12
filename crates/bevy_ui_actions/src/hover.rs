use crate::action::UiAction;
use bevy::prelude::*;
use std::sync::Arc;

/// Действие при наведении курсора
#[derive(Component)]
pub struct OnHover {
    action: Arc<dyn UiAction>,
}

impl OnHover {
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

/// Действие при уходе курсора
#[derive(Component)]
pub struct OnHoverExit {
    action: Arc<dyn UiAction>,
}

impl OnHoverExit {
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

/// Действие при нажатии (до отпускания)
#[derive(Component)]
pub struct OnPress {
    action: Arc<dyn UiAction>,
}

impl OnPress {
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
