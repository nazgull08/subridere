use crate::core::ButtonStyle;
use bevy::prelude::*;

/// Marker: автоматический визуальный feedback при hover/press
#[derive(Component)]
pub struct InteractiveVisual;

/// Marker: элемент отключён (не реагирует на клики)
#[derive(Component)]
pub struct Disabled;

/// Marker: элемент активен (выбранный таб, toggle on, selected item)
#[derive(Component)]
pub struct Active;

/// Кастомный визуальный стиль для конкретного элемента.
/// Если есть — используется вместо глобального ButtonStyle.
#[derive(Component, Clone, Debug)]
pub struct VisualStyle {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub disabled: Color,
    pub active: Option<Color>,
}

impl Default for VisualStyle {
    fn default() -> Self {
        Self {
            normal: Color::srgb(0.2, 0.2, 0.2),
            hovered: Color::srgb(0.3, 0.3, 0.3),
            pressed: Color::srgb(0.1, 0.1, 0.1),
            disabled: Color::srgb(0.15, 0.15, 0.15),
            active: None,
        }
    }
}

impl VisualStyle {
    pub fn new(normal: Color, hovered: Color, pressed: Color, disabled: Color) -> Self {
        Self {
            normal,
            hovered,
            pressed,
            disabled,
            active: None,
        }
    }

    pub fn with_active(mut self, active: Color) -> Self {
        self.active = Some(active);
        self
    }

    /// Стиль для вкладок
    pub fn tab() -> Self {
        Self {
            normal: Color::srgb(0.15, 0.15, 0.18),
            hovered: Color::srgb(0.25, 0.25, 0.28),
            pressed: Color::srgb(0.2, 0.2, 0.23),
            disabled: Color::srgb(0.1, 0.1, 0.12),
            active: Some(Color::srgb(0.28, 0.28, 0.32)),
        }
    }

    /// Стиль для слотов инвентаря
    pub fn slot() -> Self {
        Self {
            normal: Color::srgb(0.15, 0.15, 0.18),
            hovered: Color::srgb(0.22, 0.22, 0.25),
            pressed: Color::srgb(0.18, 0.18, 0.20),
            disabled: Color::srgb(0.1, 0.1, 0.12),
            active: Some(Color::srgb(0.25, 0.22, 0.18)),
        }
    }

    /// Вычислить цвет для комбинации состояний
    pub fn resolve(&self, interaction: Interaction, is_active: bool, is_disabled: bool) -> Color {
        if is_disabled {
            return self.disabled;
        }

        let base = if is_active {
            self.active.unwrap_or(self.normal)
        } else {
            self.normal
        };

        match interaction {
            Interaction::Pressed => self.pressed,
            Interaction::Hovered => {
                if is_active && self.active.is_some() {
                    Self::lighten(self.active.unwrap(), 0.08)
                } else {
                    self.hovered
                }
            }
            Interaction::None => base,
        }
    }

    fn lighten(color: Color, amount: f32) -> Color {
        let Srgba {
            red,
            green,
            blue,
            alpha,
        } = color.to_srgba();
        Color::srgba(
            (red + amount).min(1.0),
            (green + amount).min(1.0),
            (blue + amount).min(1.0),
            alpha,
        )
    }
}

/// Система для визуального feedback — единственный источник правды для BackgroundColor
pub(crate) fn update_interactive_visuals(
    global_style: Res<ButtonStyle>,
    mut query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            Has<Disabled>,
            Has<Active>,
            Option<&VisualStyle>,
        ),
        With<InteractiveVisual>,
    >,
    interaction_changed: Query<Entity, (Changed<Interaction>, With<InteractiveVisual>)>,
    active_added: Query<Entity, (Added<Active>, With<InteractiveVisual>)>,
    mut removed: RemovedComponents<Active>,
) {
    // Собираем все entity которые нужно обновить
    let mut to_update: Vec<Entity> = Vec::new();

    // Изменился Interaction
    for entity in &interaction_changed {
        to_update.push(entity);
    }

    // Добавлен Active
    for entity in &active_added {
        if !to_update.contains(&entity) {
            to_update.push(entity);
        }
    }

    // Удалён Active
    for entity in removed.read() {
        if !to_update.contains(&entity) {
            to_update.push(entity);
        }
    }

    // Обновляем все собранные entity
    for entity in to_update {
        if let Ok((_, interaction, mut bg, is_disabled, is_active, local_style)) =
            query.get_mut(entity)
        {
            *bg = BackgroundColor(compute_color(
                &global_style,
                local_style,
                *interaction,
                is_active,
                is_disabled,
            ));
        }
    }
}

fn compute_color(
    global_style: &ButtonStyle,
    local_style: Option<&VisualStyle>,
    interaction: Interaction,
    is_active: bool,
    is_disabled: bool,
) -> Color {
    if let Some(style) = local_style {
        style.resolve(interaction, is_active, is_disabled)
    } else {
        if is_disabled {
            global_style.disabled
        } else {
            match interaction {
                Interaction::Pressed => global_style.pressed,
                Interaction::Hovered => global_style.hovered,
                Interaction::None => global_style.normal,
            }
        }
    }
}
