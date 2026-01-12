use bevy::prelude::*;
use crate::action::UiAction;
use crate::button::{ActionButton, OnHover, OnPress};

/// Extension trait для удобного создания кнопок с действиями.
pub trait SpawnActionButton {
    /// Создать кнопку с дефолтными настройками.
    fn spawn_action_button(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
    ) -> Entity;

    /// Создать кнопку с кастомными настройками.
    fn spawn_action_button_with(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
        config: ButtonConfig,
    ) -> Entity;
}

/// Конфигурация внешнего вида кнопки.
#[derive(Clone, Debug)]
pub struct ButtonConfig {
    pub width: Val,
    pub height: Val,
    pub enabled: bool,
    pub background_color: Color,
    pub font_size: f32,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            enabled: true,
            background_color: Color::srgb(0.2, 0.2, 0.2),
            font_size: 20.0,
        }
    }
}

impl SpawnActionButton for ChildSpawnerCommands<'_> {
    fn spawn_action_button(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
    ) -> Entity {
        self.spawn_action_button_with(action, label, ButtonConfig::default())
    }

    fn spawn_action_button_with(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
        config: ButtonConfig,
    ) -> Entity {
        let mut button = ActionButton::new(action);
        button.enabled = config.enabled;

        self.spawn((
            Button,
            Node {
                width: config.width,
                height: config.height,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(config.background_color),
            button,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label.into()),
                TextFont {
                    font_size: config.font_size,
                    ..default()
                },
            ));
        })
        .id()
    }
}

pub trait ActionButtonExt {
    fn with_on_hover(self, action: impl UiAction) -> Self;
    fn with_on_press(self, action: impl UiAction) -> Self;
}

impl ActionButtonExt for EntityCommands<'_> {
    fn with_on_hover(mut self, action: impl UiAction) -> Self {
        self.insert(OnHover::new(action));
        self
    }

    fn with_on_press(mut self, action: impl UiAction) -> Self {
        self.insert(OnPress::new(action));
        self
    }
}
