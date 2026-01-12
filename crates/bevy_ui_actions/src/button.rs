use bevy::prelude::*;
use crate::action::UiAction;

/// Component для кнопки с привязанным действием.
///
/// TODO: Полная документация будет добавлена в следующих фазах.
#[derive(Component)]
pub struct ActionButton {
    pub(crate) action: Box<dyn UiAction>,
    pub enabled: bool,
}

impl ActionButton {
    /// Создать новую кнопку с действием.
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Box::new(action),
            enabled: true,
        }
    }
}
