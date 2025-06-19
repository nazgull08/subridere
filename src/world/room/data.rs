use bevy::prelude::*;

/// Размер кубика-вокселя (метры в мире Rapier / Bevy).
pub const CUBE_SIZE: f32 = 0.3;

/// Габариты комнаты в кубиках (по осям X, Y, Z).
pub const ROOM_WIDTH:  usize = 100;
pub const ROOM_HEIGHT: usize = 20;
pub const ROOM_DEPTH:  usize = 200;

/// Смещение комнаты по Z: чтобы platforms были «перед» входом.
pub const ROOM_Z_OFFSET: f32 = -20.0;

/// Вероятность оставить блок-воксель на поверхности стен (для “обветшалости”).
pub const SURFACE_FILL_RATIO: f32 = 0.45;

/// Количество случайных платформ в пути к комнате.
pub const PLATFORM_COUNT: usize = 30;

/// Начальная точка платформенного пути (перед комнатой).
pub const PLATFORM_START: Vec3 = Vec3::new(0.0, 1.0, 10.0);

/// Мини- и макс-смещения каждой следующей платформы.
pub const PLATFORM_DX_RANGE: (f32, f32) = (-1.0, 1.0);
pub const PLATFORM_DY_RANGE: (f32, f32) = (-0.1, 0.6);
pub const PLATFORM_DZ_RANGE: (f32, f32) = (1.0, 2.0);

/// Размеры одной прыжковой платформы.
pub const PLATFORM_HALF_EXTENTS: Vec3 = Vec3::new(0.5, 0.1, 0.5);

/// Позиция финального триггера-победы в центре комнаты.
pub fn room_trigger_position() -> Vec3 {
    Vec3::new(0.0, 1.0, ROOM_Z_OFFSET)
}

/// Seed для RNG (если захочешь зафиксировать генерацию).
pub const RNG_SEED: Option<u64> = None;
