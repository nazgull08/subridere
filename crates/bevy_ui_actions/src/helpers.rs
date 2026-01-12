use crate::action::UiAction;
use crate::click::OnClick;
use crate::drag::{Draggable, DropTarget, OnDrop};
use crate::right_click::OnRightClick;
use crate::tooltip::Tooltip;
use crate::visual::InteractiveVisual;
use bevy::prelude::*;

/// Конфигурация кнопки
#[derive(Clone, Debug)]
pub struct ButtonConfig {
    pub width: Val,
    pub height: Val,
    pub background_color: Color,
    pub font_size: f32,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            background_color: Color::srgb(0.2, 0.2, 0.2),
            font_size: 20.0,
        }
    }
}

/// Конфигурация слота (для инвентаря, equipment и т.д.)
#[derive(Clone)]
pub struct SlotConfig<C, R, D>
where
    C: UiAction,
    R: UiAction,
    D: UiAction,
{
    pub size: f32,
    pub background_color: Color,
    pub on_click: Option<C>,
    pub on_right_click: Option<R>,
    pub on_drop: Option<D>,
    pub tooltip: Option<String>,
    pub draggable: bool,
}

impl<C, R, D> Default for SlotConfig<C, R, D>
where
    C: UiAction,
    R: UiAction,
    D: UiAction,
{
    fn default() -> Self {
        Self {
            size: 50.0,
            background_color: Color::srgb(0.15, 0.15, 0.15),
            on_click: None,
            on_right_click: None,
            on_drop: None,
            tooltip: None,
            draggable: true,
        }
    }
}

/// Extension trait для удобного создания UI элементов
pub trait SpawnUiExt {
    /// Создать кнопку с действием
    fn spawn_button(&mut self, action: impl UiAction, label: impl Into<String>) -> Entity;

    /// Создать кнопку с настройками
    fn spawn_button_with(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
        config: ButtonConfig,
    ) -> Entity;
}

impl SpawnUiExt for ChildSpawnerCommands<'_> {
    fn spawn_button(&mut self, action: impl UiAction, label: impl Into<String>) -> Entity {
        self.spawn_button_with(action, label, ButtonConfig::default())
    }

    fn spawn_button_with(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
        config: ButtonConfig,
    ) -> Entity {
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
            OnClick::new(action),
            InteractiveVisual,
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

// Сохраняем старый trait для обратной совместимости
pub trait SpawnActionButton {
    fn spawn_action_button(&mut self, action: impl UiAction, label: impl Into<String>) -> Entity;
    fn spawn_action_button_with(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
        config: ButtonConfig,
    ) -> Entity;
}

impl SpawnActionButton for ChildSpawnerCommands<'_> {
    fn spawn_action_button(&mut self, action: impl UiAction, label: impl Into<String>) -> Entity {
        self.spawn_button(action, label)
    }

    fn spawn_action_button_with(
        &mut self,
        action: impl UiAction,
        label: impl Into<String>,
        config: ButtonConfig,
    ) -> Entity {
        self.spawn_button_with(action, label, config)
    }
}
