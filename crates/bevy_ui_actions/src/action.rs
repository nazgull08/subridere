use bevy::prelude::*;

/// Trait для определения действия при взаимодействии с UI элементом.
///
/// # Пример
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ui_actions::UiAction;
///
/// #[derive(Resource, Default)]
/// struct Counter(i32);
///
/// struct IncrementAction { amount: i32 }
///
/// impl UiAction for IncrementAction {
///     fn execute(&self, world: &mut World) {
///         world.resource_mut::<Counter>().0 += self.amount;
///     }
/// }
/// ```
///
/// # Execution Model
///
/// Действие выполняется через `Commands::add()`, что означает:
/// - Выполнение происходит в конце текущего frame
/// - Гарантирован эксклюзивный доступ к World
/// - Безопасно для Bevy scheduler
pub trait UiAction: Send + Sync + 'static {
    /// Выполнить действие с полным доступом к World.
    fn execute(&self, world: &mut World);
}
