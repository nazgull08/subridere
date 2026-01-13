use crate::core::UiAction;
use crate::widgets::Disabled;
use bevy::prelude::*;
use std::sync::Arc;

/// Действие при клике (левая кнопка мыши)
#[derive(Component)]
pub struct OnClick {
    action: Arc<dyn UiAction>,
}

impl OnClick {
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

/// Система для OnClick — срабатывает при Pressed
pub(crate) fn handle_clicks(
    query: Query<(&Interaction, &OnClick), (Changed<Interaction>, Without<Disabled>)>,
    mut commands: Commands,
) {
    for (interaction, on_click) in &query {
        if *interaction == Interaction::Pressed {
            on_click.execute(&mut commands);
        }
    }
}
