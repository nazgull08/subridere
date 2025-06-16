use bevy::prelude::*;

/// Компонент для хранения состояния ввода движения
#[derive(Component, Default)]
pub struct MovementInput {
    pub direction: Vec2,    // WASD направление (x: влево-вправо, y: вперед-назад)
    pub jump: bool,         // Пробел
    pub crouch: bool,       // Ctrl/Shift
    pub dash: bool,         // Alt или отдельная кнопка
    pub run: bool,          // Shift (бег)
    pub mouse_delta: Vec2,  // Движения мыши для поворота
}

/// Параметры движения для каждого юнита
#[derive(Component)]
pub struct MovementStats {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_force: f32,
    pub dash_force: f32,
    pub dash_cooldown: f32,
    pub crouch_speed_multiplier: f32,
    pub crouch_height_multiplier: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub air_control: f32,     // Управление в воздухе (0.0 - 1.0)
}

impl Default for MovementStats {
    fn default() -> Self {
        Self {
            walk_speed: 5.0,
            run_speed: 8.0,
            jump_force: 12.0,
            dash_force: 15.0,
            dash_cooldown: 1.0,
            crouch_speed_multiplier: 0.5,
            crouch_height_multiplier: 0.6,
            acceleration: 10.0,
            deceleration: 8.0,
            air_control: 0.3,
        }
    }
}

/// Состояние движения
#[derive(Component, Default)]
pub struct MovementState {
    pub velocity: Vec3,
    pub is_grounded: bool,
    pub is_crouching: bool,
    pub dash_timer: f32,
    pub can_dash: bool,
}

/// Маркер для сущностей, которые могут быть управляемы игроком
#[derive(Component)]
pub struct PlayerControlled;
