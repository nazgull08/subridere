use bevy::prelude::*;

/// Marker: автоматический визуальный feedback при hover/press
#[derive(Component)]
pub struct InteractiveVisual;

/// Marker: элемент отключён (не реагирует на клики)
#[derive(Component)]
pub struct Disabled;
