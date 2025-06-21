use bevy::prelude::*;
use crate::block_bodies::pose::BlockPose;

use super::pose::PoseToApply;

/// Применяет позу к дочерним сущностям с `Name`, `Transform`
pub fn apply_pose_to_body(
    parent: Entity,
    pose: &BlockPose,
    children: &Children,
    transforms: &mut Query<(&Name, &mut Transform)>,
) {
    for child in children.iter() {
        if let Ok((name, mut transform)) = transforms.get_mut(child) {
            if let Some(part_pose) = pose.get_part(name.as_str()) {
                transform.translation = part_pose.offset;
                transform.rotation = part_pose.rotation;
            }
        }
    }
}


pub fn apply_pose_once_system(
    mut commands: Commands,
    query: Query<(Entity, &Children, &PoseToApply)>,
    mut transforms: Query<(&Name, &mut Transform)>,
) {
    for (entity, children, pose) in &query {
        println!("apply_pose_system");
        super::animation::apply_pose_to_body(entity, &pose.0, children, &mut transforms);
        commands.entity(entity).remove::<PoseToApply>();
    }
}
