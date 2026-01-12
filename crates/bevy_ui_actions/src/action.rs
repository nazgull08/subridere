use bevy::prelude::*;

/// Trait для определения действия при взаимодействии с UI элементом.
///
/// Реализуйте этот trait для создания custom действий которые будут
/// выполняться при клике на кнопку или другие UI события.
///
/// # World Access
///
/// Метод `execute` получает полный доступ к `&mut World`. Это мощный
/// но низкоуровневый API который требует аккуратного использования.
///
/// ## Best Practices
///
/// - ✅ Используйте `world.resource_mut()` для доступа к resources
/// - ✅ Используйте `world.entity_mut()` для модификации entities
/// - ✅ Используйте `world.commands()` для spawn/despawn операций
/// - ⚠️ Не мутируйте компоненты во время итерации по ним
/// - ⚠️ Будьте осторожны с race conditions
///
/// ## Safety
///
/// World API является безопасным в Rust смысле (не требует `unsafe`),
/// но неправильное использование может привести к панике или нарушению
/// логических инвариантов ECS.
///
/// # Examples
///
/// ## Простое действие с Resource
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ui_actions::UiAction;
///
/// #[derive(Resource, Default)]
/// struct Counter(i32);
///
/// struct IncrementAction;
///
/// impl UiAction for IncrementAction {
///     fn execute(&self, world: &mut World) {
///         let mut counter = world.resource_mut::<Counter>();
///         counter.0 += 1;
///         info!("Counter: {}", counter.0);
///     }
/// }
/// ```
///
/// ## Действие с захваченным контекстом
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ui_actions::UiAction;
///
/// struct AddValueAction {
///     amount: i32,
/// }
///
/// impl UiAction for AddValueAction {
///     fn execute(&self, world: &mut World) {
///         // self.amount доступен из контекста
///         info!("Adding {}", self.amount);
///     }
/// }
/// ```
pub trait UiAction: Send + Sync + 'static {
    /// Выполнить действие.
    ///
    /// Этот метод вызывается когда пользователь взаимодействует
    /// с UI элементом (например, кликает на кнопку).
    ///
    /// # Arguments
    ///
    /// * `world` - Полный доступ к Bevy World для выполнения
    ///   любых игровых операций.
    fn execute(&self, world: &mut World);
}
