use crate::core::ButtonStyle;
use bevy::prelude::*;

/// Marker: автоматический визуальный feedback при hover/press
#[derive(Component)]
pub struct InteractiveVisual;

/// Marker: элемент отключён (не реагирует на клики)
#[derive(Component)]
pub struct Disabled;

/// Кастомный визуальный стиль для конкретного элемента.
/// Если есть — используется вместо глобального ButtonStyle.
#[derive(Component, Clone, Debug)]
pub struct VisualStyle {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub disabled: Color,
}

impl Default for VisualStyle {
    fn default() -> Self {
        Self {
            normal: Color::srgb(0.2, 0.2, 0.2),
            hovered: Color::srgb(0.3, 0.3, 0.3),
            pressed: Color::srgb(0.1, 0.1, 0.1),
            disabled: Color::srgb(0.15, 0.15, 0.15),
        }
    }
}

impl VisualStyle {
    pub fn new(normal: Color, hovered: Color, pressed: Color, disabled: Color) -> Self {
        Self { normal, hovered, pressed, disabled }
    }

    /// Стиль для вкладок
    pub fn tab() -> Self {
        Self {
            normal: Color::srgb(0.15, 0.15, 0.18),
            hovered: Color::srgb(0.25, 0.25, 0.28),
            pressed: Color::srgb(0.2, 0.2, 0.23),
            disabled: Color::srgb(0.1, 0.1, 0.12),
        }
    }

    /// Стиль для активной вкладки
    pub fn tab_active() -> Self {
        Self {
            normal: Color::srgb(0.25, 0.25, 0.30),
            hovered: Color::srgb(0.30, 0.30, 0.35),
            pressed: Color::srgb(0.25, 0.25, 0.30),
            disabled: Color::srgb(0.15, 0.15, 0.18),
        }
    }

    /// Стиль для слотов инвентаря
    pub fn slot() -> Self {
        Self {
            normal: Color::srgb(0.15, 0.15, 0.18),
            hovered: Color::srgb(0.22, 0.22, 0.25),
            pressed: Color::srgb(0.18, 0.18, 0.20),
            disabled: Color::srgb(0.1, 0.1, 0.12),
        }
    }
}

/// Система для визуального feedback
pub(crate) fn update_interactive_visuals(
    global_style: Res<ButtonStyle>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, Has<Disabled>, Option<&VisualStyle>),
        (Changed<Interaction>, With<InteractiveVisual>),
    >,
) {
    for (interaction, mut bg, is_disabled, local_style) in &mut query {
        // Используем локальный стиль если есть, иначе глобальный
        let (normal, hovered, pressed, disabled) = match local_style {
            Some(style) => (style.normal, style.hovered, style.pressed, style.disabled),
            None => (global_style.normal, global_style.hovered, global_style.pressed, global_style.disabled),
        };

        *bg = BackgroundColor(if is_disabled {
            disabled
        } else {
            match interaction {
                Interaction::Pressed => pressed,
                Interaction::Hovered => hovered,
                Interaction::None => normal,
            }
        });
    }
}
