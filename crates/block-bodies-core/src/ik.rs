// crates/block-bodies-core/src/ik.rs
//
// Two-Bone Inverse Kinematics Solver
// Используется для рук, ног, и любых двухзвенных цепочек.

use glam::{Quat, Vec3};

/// Результат IK решения для двухзвенной цепочки
#[derive(Debug, Clone, Copy)]
pub struct TwoBoneIkResult {
    /// Мировая позиция среднего сустава (локоть/колено)
    pub middle_pos: Vec3,
    /// Ротация верхнего сегмента (плечо/бедро)
    pub upper_rotation: Quat,
    /// Ротация нижнего сегмента (локоть/колено)  
    pub lower_rotation: Quat,
    /// Достигнута ли цель полностью
    pub target_reached: bool,
}

/// Решает 2-bone IK для цепочки root → middle → end
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

    let (adjusted_dist, target_reached) = if target_dist > chain_length - 0.001 {
        (chain_length - 0.001, false)
    } else if target_dist < min_length + 0.001 {
        (min_length + 0.001, false)
    } else {
        (target_dist, true)
    };

    let target_dir = if target_dist > 0.001 {
        to_target / target_dist
    } else {
        Vec3::NEG_Z
    };

    // Закон косинусов
    let a = upper_length;
    let b = lower_length;
    let c = adjusted_dist;

    let cos_angle_at_root = ((a * a + c * c - b * b) / (2.0 * a * c)).clamp(-1.0, 1.0);
    let angle_at_root = cos_angle_at_root.acos();

    // Плоскость изгиба через pole target
    let to_pole = pole_target - root_pos;
    let pole_on_target_plane = to_pole - target_dir * to_pole.dot(target_dir);

    let bend_normal = if pole_on_target_plane.length_squared() > 0.0001 {
        pole_on_target_plane.normalize()
    } else {
        let arbitrary = if target_dir.dot(Vec3::Y).abs() < 0.99 {
            Vec3::Y
        } else {
            Vec3::X
        };
        target_dir.cross(arbitrary).normalize()
    };

    let bend_axis = target_dir.cross(bend_normal).normalize();

    // Позиция локтя
    let upper_rotation_to_middle = Quat::from_axis_angle(bend_axis, angle_at_root);
    let upper_dir = upper_rotation_to_middle * target_dir;
    let middle_pos = root_pos + upper_dir * upper_length;

    // === ИСПРАВЛЕНИЕ: используем look_rotation с up-вектором ===
    let upper_rotation = look_rotation(upper_dir, bend_normal, upper_local_forward);

    let lower_dir = (target_pos - middle_pos).normalize();
    let lower_world_rotation = look_rotation(lower_dir, bend_normal, lower_local_forward);

    let lower_rotation = upper_rotation.inverse() * lower_world_rotation;

    TwoBoneIkResult {
        middle_pos,
        upper_rotation,
        lower_rotation,
        target_reached,
    }
}

/// Создаёт ротацию чтобы local_forward смотрел в direction, с контролем up
fn look_rotation(direction: Vec3, up_hint: Vec3, local_forward: Vec3) -> Quat {
    let forward = direction.normalize();

    // Вычисляем right перпендикулярно forward и up
    let right = up_hint.cross(forward);
    if right.length_squared() < 0.0001 {
        // forward параллелен up, используем fallback
        return Quat::from_rotation_arc(local_forward.normalize(), forward);
    }
    let right = right.normalize();

    // Пересчитываем up перпендикулярно forward и right
    let up = forward.cross(right).normalize();

    // Строим матрицу ротации
    let rotation_matrix = glam::Mat3::from_cols(right, up, forward);

    // Конвертируем в кватернион
    let world_rotation = Quat::from_mat3(&rotation_matrix);

    // Учитываем локальный forward
    let local_rotation = Quat::from_rotation_arc(Vec3::Z, local_forward.normalize());

    world_rotation * local_rotation.inverse()
}

/// Упрощённая версия для руки
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
        Vec3::NEG_Z,
        Vec3::NEG_Z,
    )
}

/// Упрощённая версия для ноги
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
        Vec3::NEG_Y,
        Vec3::NEG_Y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_arm() {
        let result = solve_arm_ik(
            Vec3::ZERO,
            0.3,
            0.3,
            Vec3::new(0.0, 0.0, -0.59),
            Vec3::new(0.0, -1.0, 0.0),
        );
        assert!(result.middle_pos.z < 0.0);
    }

    #[test]
    fn test_unreachable_target() {
        let result = solve_arm_ik(
            Vec3::ZERO,
            0.3,
            0.3,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, -1.0, 0.0),
        );
        assert!(!result.target_reached);
    }
}
