use bevy::prelude::*;
use crate::enemy::component::*;
use crate::block_bodies::animation::component::AnimationCycle;
use crate::block_bodies::pose::BlockPose;

pub fn update_enemy_animation_on_state_change(
    mut commands: Commands,
    mut query: Query<(Entity, &EnemyState, Option<&mut AnimationCycle>, Option<&AnimationKind>), Changed<EnemyState>>,
) {
    for (entity, state, _maybe_cycle, current_kind) in &mut query {
        let desired = match state {
            EnemyState::Idle => AnimationKind::Idle,
            EnemyState::Walk => AnimationKind::Walk,
            EnemyState::Attack(attack_state) => match attack_state {
                EnemyAttackState::Bite => AnimationKind::BiteAttack,
                EnemyAttackState::Slash => AnimationKind::SlashAttack,
                _ => continue, // не меняем анимацию для Approach/Cooldown
            },
            EnemyState::Dead => continue,
        };

        if current_kind == Some(&desired) {
            continue;
        }

        let poses = match desired {
            AnimationKind::Idle => load_poses("idle"),
            AnimationKind::Walk => load_poses("walk"),
            AnimationKind::BiteAttack => load_poses("bite"),
            AnimationKind::SlashAttack => load_poses("slash"),
        };

        commands.entity(entity)
            .insert(AnimationCycle::new(poses, 0.8))
            .insert(desired);
    }
}

fn load_poses(tag: &str) -> Vec<BlockPose> {
    use std::fs;

    let dir = format!("assets/poses/jimbo/{}", tag);
    let mut files = fs::read_dir(&dir)
        .expect("directory exists")
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|p| p.extension().map(|ext| ext == "ron").unwrap_or(false))
        .collect::<Vec<_>>();

    files.sort(); // чтобы порядок был: prepare → attack → recover

    files.into_iter()
        .map(|p| BlockPose::from_ron_file(p.to_str().unwrap().to_string()).unwrap())
        .collect()
}
