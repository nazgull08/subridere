// fighting/melee/debug.rs

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Маркер: предмет недавно получил импульс, отслеживаем его
#[derive(Component)]
pub struct PhysicsDebugTracker {
    pub start_time: f32,
    pub start_pos: Vec3,
    pub max_speed: f32,
    pub item_name: String,
}

/// Система: отслеживает скорость предметов
pub fn track_item_physics(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PhysicsDebugTracker, &Velocity, &Transform)>,
    time: Res<Time>,
) {
    for (entity, mut tracker, velocity, transform) in &mut query {
        let speed = velocity.linvel.length();
        let elapsed = time.elapsed_secs() - tracker.start_time;

        // Обновляем максимальную скорость
        if speed > tracker.max_speed {
            tracker.max_speed = speed;
        }

        // Логируем пока предмет движется или первые 0.5 секунды
        if speed > 0.3 || elapsed < 0.3 {
            let distance = transform.translation.distance(tracker.start_pos);
            info!(
                "🔬 '{}' | t={:.2}s | speed={:.1} m/s | max={:.1} m/s | dist={:.2}m",
                tracker.item_name, elapsed, speed, tracker.max_speed, distance
            );
        }

        // Остановился — финальный отчёт
        if speed < 0.1 && elapsed > 0.2 {
            let distance = transform.translation.distance(tracker.start_pos);
            info!(
                "🔬 '{}' STOPPED | time={:.2}s | max_speed={:.1} m/s | distance={:.2}m",
                tracker.item_name, elapsed, tracker.max_speed, distance
            );
            commands.entity(entity).remove::<PhysicsDebugTracker>();
        }

        // Таймаут
        if elapsed > 5.0 {
            commands.entity(entity).remove::<PhysicsDebugTracker>();
        }
    }
}
