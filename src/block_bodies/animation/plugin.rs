use bevy::prelude::*;
use super::apply::apply_pose_once_system;
use super::lerp::lerp_pose_system;
use super::system::animation_cycle_system;

pub struct BlockAnimationPlugin;

impl Plugin for BlockAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_pose_once_system,
            lerp_pose_system,
            animation_cycle_system,
        ));
    }
}
