// crates/subridere-core/src/player/arm/debug.rs
//
// Debug система для настройки позиции рук
//
// Управление:
//   F5 - переключить ось (X → Y → Z → X)
//   ↑/↓ - изменить shoulder_offset по выбранной оси
//   Shift + ↑/↓ - большой шаг
//   F6 - вывести текущие значения в консоль

use bevy::prelude::*;

use super::components::{ArmConfig, ArmSide, IkTarget, Shoulder};

/// Ресурс для debug настройки
#[derive(Resource)]
pub struct ArmDebugState {
    /// Какую ось сейчас редактируем (0=X, 1=Y, 2=Z)
    pub current_axis: usize,
    /// Шаг изменения
    pub step: f32,
    /// Включен ли debug режим
    pub enabled: bool,
}

impl Default for ArmDebugState {
    fn default() -> Self {
        Self {
            current_axis: 2, // Начинаем с Z (вперёд-назад)
            step: 0.05,
            enabled: false,
        }
    }
}

/// Система debug управления
pub fn arm_debug_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<ArmDebugState>,
    mut config: ResMut<ArmConfig>,
    mut ik_targets: Query<&mut IkTarget>,
    mut shoulder_query: Query<(&mut Transform, &Shoulder)>,
) {
    if !debug_state.enabled {
        return;
    }

    // F5 - переключить ось
    if keyboard.just_pressed(KeyCode::F5) {
        debug_state.current_axis = (debug_state.current_axis + 1) % 3;
        let axis_name = match debug_state.current_axis {
            0 => "X (left/right)",
            1 => "Y (up/down)",
            2 => "Z (forward/back)",
            _ => "?",
        };
        info!(
            "🎮 ARM DEBUG: Now editing axis {} - {}",
            debug_state.current_axis, axis_name
        );
    }

    // F6 - вывести текущие значения
    if keyboard.just_pressed(KeyCode::F6) {
        info!("════════════════════════════════════════");
        info!("🦴 ARM CONFIG VALUES (copy to components.rs):");
        info!(
            "   shoulder_offset_right: Vec3::new({:.2}, {:.2}, {:.2}),",
            config.shoulder_offset_right.x,
            config.shoulder_offset_right.y,
            config.shoulder_offset_right.z,
        );
        info!(
            "   shoulder_offset_left: Vec3::new({:.2}, {:.2}, {:.2}),",
            config.shoulder_offset_left.x,
            config.shoulder_offset_left.y,
            config.shoulder_offset_left.z,
        );

        for ik_target in &ik_targets {
            if ik_target.side == ArmSide::Right {
                info!("   // IkTarget::right()");
                info!(
                    "   position: Vec3::new({:.2}, {:.2}, {:.2}),",
                    ik_target.position.x, ik_target.position.y, ik_target.position.z,
                );
                info!(
                    "   elbow_hint: Vec3::new({:.2}, {:.2}, {:.2}),",
                    ik_target.elbow_hint.x, ik_target.elbow_hint.y, ik_target.elbow_hint.z,
                );
            }
        }
        info!("════════════════════════════════════════");
    }

    let mut changed = false;

    // ↑ - увеличить значение
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        let multiplier =
            if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                3.0
            } else {
                1.0
            };
        let delta = debug_state.step * multiplier;
        adjust_values(
            &mut config,
            &mut ik_targets,
            debug_state.current_axis,
            delta,
        );
        changed = true;
    }

    // ↓ - уменьшить значение
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        let multiplier =
            if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                3.0
            } else {
                1.0
            };
        let delta = -debug_state.step * multiplier;
        adjust_values(
            &mut config,
            &mut ik_targets,
            debug_state.current_axis,
            delta,
        );
        changed = true;
    }

    if changed {
        print_current_value(&config, &debug_state);

        // === ОБНОВЛЯЕМ TRANSFORM ПЛЕЧ НАПРЯМУЮ ===
        for (mut transform, shoulder) in shoulder_query.iter_mut() {
            match shoulder.side {
                ArmSide::Right => transform.translation = config.shoulder_offset_right,
                ArmSide::Left => transform.translation = config.shoulder_offset_left,
            }
        }
    }
}

fn adjust_values(
    config: &mut ArmConfig,
    ik_targets: &mut Query<&mut IkTarget>,
    axis: usize,
    delta: f32,
) {
    // Меняем shoulder_offset для обеих рук (зеркально по X)
    match axis {
        0 => {
            config.shoulder_offset_right.x += delta;
            config.shoulder_offset_left.x -= delta; // зеркально
        }
        1 => {
            config.shoulder_offset_right.y += delta;
            config.shoulder_offset_left.y += delta;
        }
        2 => {
            config.shoulder_offset_right.z += delta;
            config.shoulder_offset_left.z += delta;
        }
        _ => {}
    }

    // Также меняем IkTarget позицию (чтобы рука следовала за плечом)
    for mut ik_target in ik_targets.iter_mut() {
        match axis {
            0 => {
                if ik_target.side == ArmSide::Right {
                    ik_target.position.x += delta;
                } else {
                    ik_target.position.x -= delta;
                }
            }
            1 => {
                ik_target.position.y += delta;
            }
            2 => {
                ik_target.position.z += delta;
            }
            _ => {}
        }
    }
}

fn print_current_value(config: &ArmConfig, debug_state: &ArmDebugState) {
    let axis_name = match debug_state.current_axis {
        0 => "X",
        1 => "Y",
        2 => "Z",
        _ => "?",
    };
    let value = match debug_state.current_axis {
        0 => config.shoulder_offset_right.x,
        1 => config.shoulder_offset_right.y,
        2 => config.shoulder_offset_right.z,
        _ => 0.0,
    };
    info!("🎮 shoulder_offset_right.{} = {:.2}", axis_name, value);
}
