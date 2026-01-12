use std::sync::Arc;
use bevy::prelude::*;
use crate::action::UiAction;

/// Component для кнопки с привязанным действием.
///
/// # Пример
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ui_actions::{ActionButton, UiAction};
///
/// struct MyAction;
/// impl UiAction for MyAction {
///     fn execute(&self, world: &mut World) {
///         info!("Button clicked!");
///     }
/// }
///
/// fn setup(mut commands: Commands) {
///     commands.spawn((
///         Button,
///         ActionButton::new(MyAction),
///     ));
/// }
/// ```
#[derive(Component)]
pub struct ActionButton {
    action: Arc<dyn UiAction>,
    pub enabled: bool,
}

impl ActionButton {
    /// Создать новую кнопку с действием.
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
            enabled: true,
        }
    }

    /// Выполнить действие (вызывается из observer).
    pub(crate) fn execute(&self, commands: &mut Commands) {
        if self.enabled {
            let action = self.action.clone();
            commands.queue(move |world: &mut World| {
                action.execute(world);
            });
        }
    }
}

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

/// Действие при нажатии (до отпускания).
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
