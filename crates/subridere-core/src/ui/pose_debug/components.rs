// crates/subridere-core/src/ui/pose_debug/components.rs

use bevy::prelude::*;

/// Корневой элемент UI панели pose debug
#[derive(Component)]
pub struct PoseDebugRoot;

/// Текст с названием текущей позы
#[derive(Component)]
pub struct PoseNameText;

/// Текст с текущим edit target
#[derive(Component)]
pub struct EditTargetText;

/// Текст с текущей осью
#[derive(Component)]
pub struct AxisText;

/// Текст с текущим значением
#[derive(Component)]
pub struct CurrentValueText;

/// Текст со всеми значениями позы
#[derive(Component)]
pub struct PoseValuesText;
