use bevy::prelude::*;

/// Основной маркер компонент игрока
#[derive(Component)]
pub struct Player;

/// Настройки визуализации игрока
#[derive(Component)]
pub struct PlayerVisual {
    pub body_color: Color,
    pub body_size: Vec3,
}

impl Default for PlayerVisual {
    fn default() -> Self {
        Self {
            body_color: Color::srgb(0.2, 0.6, 0.8), // голубоватый цвет
            body_size: Vec3::new(0.6, 1.8, 0.3),    // ширина, высота, глубина
        }
    }
}

/// Стартовая позиция игрока
pub static PLAYER_START_POS: Vec3 = Vec3::new(0.0, 2.0, 10.0);
