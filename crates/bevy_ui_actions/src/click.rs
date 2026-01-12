use crate::action::UiAction;
use bevy::prelude::*;
use std::sync::Arc;

/// Действие при клике (левая кнопка мыши)
#[derive(Component)]
pub struct OnClick {
    action: Arc<dyn UiAction>,
}

impl OnClick {
    /// Создать новый OnClick с действием
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }

    /// Выполнить действие
    pub(crate) fn execute(&self, commands: &mut Commands) {
        let action = self.action.clone();
        commands.queue(move |world: &mut World| {
            action.execute(world);
        });
    }
}
