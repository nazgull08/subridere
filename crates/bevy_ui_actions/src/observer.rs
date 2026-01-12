use crate::{ActionButton, ButtonStyle, OnHover, OnPress};
use bevy::prelude::*;

/// Observer для кликов.
pub(crate) fn on_action_button_click(
    trigger: Trigger<Pointer<Click>>,
    query: Query<&ActionButton>,
    mut commands: Commands,
) {
    let target = trigger.target();
    if let Ok(button) = query.get(target) {
        button.execute(&mut commands);
    }
}

/// Система для OnHover — срабатывает при смене Interaction на Hovered.
pub(crate) fn handle_hover_actions(
    query: Query<(&Interaction, &OnHover), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, on_hover) in &query {
        if *interaction == Interaction::Hovered {
            on_hover.execute(&mut commands);
        }
    }
}

/// Система для OnPress — срабатывает при смене Interaction на Pressed.
pub(crate) fn handle_press_actions(
    query: Query<(&Interaction, &OnPress), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, on_press) in &query {
        if *interaction == Interaction::Pressed {
            on_press.execute(&mut commands);
        }
    }
}

/// Система для визуального feedback.
pub(crate) fn update_button_visuals(
    style: Res<ButtonStyle>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &ActionButton), Changed<Interaction>>,
) {
    for (interaction, mut bg, button) in &mut query {
        *bg = BackgroundColor(match (button.enabled, interaction) {
            (false, _) => style.disabled,
            (true, Interaction::Pressed) => style.pressed,
            (true, Interaction::Hovered) => style.hovered,
            (true, Interaction::None) => style.normal,
        });
    }
}
