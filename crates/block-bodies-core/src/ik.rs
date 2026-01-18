// ik.rs - Two-Bone Inverse Kinematics Solver
//
// Используется для рук, ног, и любых двухзвенных цепочек.
//
// Пример руки:
//   Shoulder ──[upper_arm]──> Elbow ──[forearm]──> Hand
//       ↑                                            ↑
//    root (fixed)                               target (goal)

use glam::{Quat, Vec3};

/// Результат IK решения для двухзвенной цепочки
#[derive(Debug, Clone, Copy)]
pub struct TwoBoneIkResult {
    /// Мировая позиция локтя/колена (среднего сустава)
    pub middle_pos: Vec3,
    /// Локальная ротация верхнего сегмента (плечо/бедро)
    pub upper_rotation: Quat,
    /// Локальная ротация нижнего сегмента (локоть/колено)
    pub lower_rotation: Quat,
    /// Достигнута ли цель полностью
    pub target_reached: bool,
}

/// Решает 2-bone IK для цепочки root → middle → end
///
/// # Arguments
/// * `root_pos` - Позиция корня (плечо/бедро) - фиксирована
/// * `upper_length` - Длина верхнего сегмента (плечо → локоть)
/// * `lower_length` - Длина нижнего сегмента (локоть → кисть)
/// * `target_pos` - Целевая позиция для конца цепочки (куда тянется рука)
/// * `pole_target` - Точка, к которой "смотрит" средний сустав (локоть/колено)
///                   Определяет плоскость изгиба
/// * `upper_local_forward` - Локальное направление "вперёд" для верхнего сегмента
/// * `lower_local_forward` - Локальное направление "вперёд" для нижнего сегмента
///
/// # Returns
/// `TwoBoneIkResult` с позицией среднего сустава и ротациями
///
/// # Example
/// ```
/// use block_bodies_core::ik::solve_two_bone_ik;
/// use glam::Vec3;
///
/// let result = solve_two_bone_ik(
///     Vec3::ZERO,           // shoulder at origin
///     0.3,                  // upper arm length
///     0.3,                  // forearm length  
///     Vec3::new(0.4, 0.0, -0.3), // target
///     Vec3::new(0.0, -1.0, 0.0), // elbow points down
///     Vec3::X,              // upper arm forward
///     Vec3::X,              // forearm forward
/// );
/// ```
pub fn solve_two_bone_ik(
    root_pos: Vec3,
    upper_length: f32,
    lower_length: f32,
    target_pos: Vec3,
    pole_target: Vec3,
    upper_local_forward: Vec3,
    lower_local_forward: Vec3,
) -> TwoBoneIkResult {
    let to_target = target_pos - root_pos;
    let target_dist = to_target.length();

    let chain_length = upper_length + lower_length;
    let min_length = (upper_length - lower_length).abs();

    // Определяем достижимость цели
    let (adjusted_dist, target_reached) = if target_dist > chain_length - 0.001 {
        // Цель слишком далеко - вытягиваем максимально
        (chain_length - 0.001, false)
    } else if target_dist < min_length + 0.001 {
        // Цель слишком близко - минимальный изгиб
        (min_length + 0.001, false)
    } else {
        (target_dist, true)
    };

    // Направление к цели
    let target_dir = if target_dist > 0.001 {
        to_target / target_dist
    } else {
        Vec3::NEG_Z // fallback
    };

    // === Закон косинусов для нахождения угла локтя ===
    // a = upper_length, b = lower_length, c = adjusted_dist
    // cos(angle_at_root) = (a² + c² - b²) / (2ac)

    let a = upper_length;
    let b = lower_length;
    let c = adjusted_dist;

    let cos_angle_at_root = ((a * a + c * c - b * b) / (2.0 * a * c)).clamp(-1.0, 1.0);
    let angle_at_root = cos_angle_at_root.acos();

    let cos_angle_at_middle = ((a * a + b * b - c * c) / (2.0 * a * b)).clamp(-1.0, 1.0);
    let angle_at_middle = cos_angle_at_middle.acos();

    // === Определяем плоскость изгиба через pole target ===
    let to_pole = pole_target - root_pos;

    // Вектор перпендикулярный направлению на цель, в плоскости pole
    let pole_on_target_plane = to_pole - target_dir * to_pole.dot(target_dir);

    let bend_normal = if pole_on_target_plane.length_squared() > 0.0001 {
        pole_on_target_plane.normalize()
    } else {
        // Fallback: если pole на линии к цели, используем мировой up
        let arbitrary = if target_dir.dot(Vec3::Y).abs() < 0.99 {
            Vec3::Y
        } else {
            Vec3::X
        };
        target_dir.cross(arbitrary).normalize()
    };

    // Ось вращения для поворота от target_dir к позиции локтя
    let bend_axis = target_dir.cross(bend_normal).normalize();

    // === Вычисляем позицию среднего сустава (локтя) ===
    // Поворачиваем направление на цель на angle_at_root вокруг bend_axis
    let upper_rotation_to_middle = Quat::from_axis_angle(bend_axis, angle_at_root);
    let upper_dir = upper_rotation_to_middle * target_dir;
    let middle_pos = root_pos + upper_dir * upper_length;

    // === Вычисляем ротации сегментов ===

    // Ротация верхнего сегмента: от локального forward к upper_dir
    let upper_rotation =
        Quat::from_rotation_arc(upper_local_forward.normalize(), upper_dir.normalize());

    // Направление нижнего сегмента: от middle к target
    let lower_dir = (target_pos - middle_pos).normalize();

    // Локальная ротация нижнего сегмента относительно верхнего
    // Сначала вычисляем мировую ротацию нижнего сегмента
    let lower_world_rotation = Quat::from_rotation_arc(lower_local_forward.normalize(), lower_dir);

    // Локальная = inverse(upper) * world
    let lower_rotation = upper_rotation.inverse() * lower_world_rotation;

    TwoBoneIkResult {
        middle_pos,
        upper_rotation,
        lower_rotation,
        target_reached,
    }
}

/// Упрощённая версия для типичной руки (направление вниз-вперёд)
pub fn solve_arm_ik(
    shoulder_pos: Vec3,
    upper_arm_length: f32,
    forearm_length: f32,
    hand_target: Vec3,
    elbow_hint: Vec3,
) -> TwoBoneIkResult {
    solve_two_bone_ik(
        shoulder_pos,
        upper_arm_length,
        forearm_length,
        hand_target,
        elbow_hint,
        Vec3::NEG_Z, // рука направлена вперёд по -Z
        Vec3::NEG_Z,
    )
}

/// Упрощённая версия для ноги (колено вперёд)
pub fn solve_leg_ik(
    hip_pos: Vec3,
    thigh_length: f32,
    shin_length: f32,
    foot_target: Vec3,
    knee_hint: Vec3,
) -> TwoBoneIkResult {
    solve_two_bone_ik(
        hip_pos,
        thigh_length,
        shin_length,
        foot_target,
        knee_hint,
        Vec3::NEG_Y, // нога направлена вниз
        Vec3::NEG_Y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_arm() {
        // Цель прямо впереди, на максимальном расстоянии
        let result = solve_arm_ik(
            Vec3::ZERO,
            0.3,
            0.3,
            Vec3::new(0.0, 0.0, -0.59), // почти макс длина
            Vec3::new(0.0, -1.0, 0.0),
        );

        // Локоть должен быть примерно посередине
        assert!(result.middle_pos.z < 0.0);
        assert!((result.middle_pos.z - (-0.3)).abs() < 0.1);
    }

    #[test]
    fn test_bent_arm() {
        // Цель близко - рука должна согнуться
        let result = solve_arm_ik(
            Vec3::ZERO,
            0.3,
            0.3,
            Vec3::new(0.0, 0.0, -0.3), // половина длины
            Vec3::new(0.0, -1.0, 0.0),
        );

        // Локоть должен отклониться вниз (к pole)
        assert!(result.middle_pos.y < 0.0);
    }

    #[test]
    fn test_unreachable_target() {
        // Цель слишком далеко
        let result = solve_arm_ik(
            Vec3::ZERO,
            0.3,
            0.3,
            Vec3::new(0.0, 0.0, -1.0), // 1m при длине 0.6m
            Vec3::new(0.0, -1.0, 0.0),
        );

        assert!(!result.target_reached);
    }

    #[test]
    fn test_target_reached() {
        let result = solve_arm_ik(
            Vec3::ZERO,
            0.3,
            0.3,
            Vec3::new(0.0, 0.0, -0.4),
            Vec3::new(0.0, -1.0, 0.0),
        );

        assert!(result.target_reached);
    }
}
