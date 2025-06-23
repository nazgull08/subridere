use bevy::prelude::*;
use crate::enemy::component::*;
use crate::block_bodies::animation::component::AnimationCycle;
use crate::block_bodies::pose::BlockPose;

pub fn update_enemy_animation_on_state_change(
    mut commands: Commands,
    mut query: Query<(Entity, &EnemyState, Option<&mut AnimationCycle>, Option<&AnimationKind>), Changed<EnemyState>>,
) {
    for (entity, state, maybe_cycle, current_kind) in &mut query {
        let desired = match state {
            EnemyState::Idle => AnimationKind::Idle,
            EnemyState::Walk => AnimationKind::Walk,
        };

        if current_kind == Some(&desired) {
            continue; // уже проигрывается нужная
        }

        // Загружаем новые позы
        let poses = match desired {
            AnimationKind::Idle => load_poses("idle"),
            AnimationKind::Walk => load_poses("walk"),
        };

        // Заменяем анимацию
        commands.entity(entity)
            .insert(AnimationCycle::new(poses, 0.8))
            .insert(desired);
    }
}

fn load_poses(tag: &str) -> Vec<BlockPose> {
    let paths = match tag {
        "idle" => vec![
            "poses/jimbo/idle/neutral.ron",
            "poses/jimbo/idle/breath_in.ron",
            "poses/jimbo/idle/breath_out.ron",
        ],
        "walk" => vec![
            "poses/jimbo/walk/start.ron",
            "poses/jimbo/walk/step1.ron",
            "poses/jimbo/walk/step2.ron",
        ],
        _ => vec![],
    };

    paths.into_iter()
         .map(|p| BlockPose::from_ron_file(format!("assets/{}", p)).unwrap())
         .collect()
}
