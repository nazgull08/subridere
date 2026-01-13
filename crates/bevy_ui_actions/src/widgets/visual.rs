use crate::core::ButtonStyle;
use bevy::prelude::*;

/// Marker: автоматический визуальный feedback при hover/press
#[derive(Component)]
pub struct InteractiveVisual;

/// Marker: элемент отключён (не реагирует на клики)
#[derive(Component)]
pub struct Disabled;

/// Система для визуального feedback
pub(crate) fn update_interactive_visuals(
    style: Res<ButtonStyle>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, Has<Disabled>),
        (Changed<Interaction>, With<InteractiveVisual>),
    >,
) {
    for (interaction, mut bg, is_disabled) in &mut query {
        *bg = BackgroundColor(if is_disabled {
            style.disabled
        } else {
            match interaction {
                Interaction::Pressed => style.pressed,
                Interaction::Hovered => style.hovered,
                Interaction::None => style.normal,
            }
        });
    }
}
