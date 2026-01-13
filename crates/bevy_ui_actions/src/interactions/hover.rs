use crate::core::UiAction;
use crate::widgets::Disabled;
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

/// Предыдущее состояние Interaction для отслеживания переходов
#[derive(Component, Default)]
pub struct PreviousInteraction(pub Interaction);

// ============ Systems ============

/// Система для OnHover — срабатывает при входе в Hovered
pub(crate) fn handle_hover_actions(
    query: Query<(&Interaction, &OnHover), (Changed<Interaction>, Without<Disabled>)>,
    mut commands: Commands,
) {
    for (interaction, on_hover) in &query {
        if *interaction == Interaction::Hovered {
            on_hover.execute(&mut commands);
        }
    }
}

/// Система для OnHoverExit — срабатывает при выходе из Hovered
pub(crate) fn handle_hover_exit_actions(
    mut query: Query<(&Interaction, &mut PreviousInteraction, &OnHoverExit), Without<Disabled>>,
    mut commands: Commands,
) {
    for (interaction, mut prev, on_hover_exit) in &mut query {
        if prev.0 == Interaction::Hovered && *interaction != Interaction::Hovered {
            on_hover_exit.execute(&mut commands);
        }
        prev.0 = *interaction;
    }
}

/// Система для OnPress — срабатывает при нажатии
pub(crate) fn handle_press_actions(
    query: Query<(&Interaction, &OnPress), (Changed<Interaction>, Without<Disabled>)>,
    mut commands: Commands,
) {
    for (interaction, on_press) in &query {
        if *interaction == Interaction::Pressed {
            on_press.execute(&mut commands);
        }
    }
}
