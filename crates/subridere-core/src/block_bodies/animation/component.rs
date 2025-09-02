// src/block_bodies/animation/component.rs

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::block_bodies::pose::BlockPose;

#[derive(Component)]
pub struct AnimationCycle {
    pub poses: VecDeque<BlockPose>,
    pub current_index: usize,
    pub pose_duration: f32,
}

impl AnimationCycle {
    pub fn new(poses: Vec<BlockPose>, pose_duration: f32) -> Self {
        Self {
            poses: poses.into(),
            current_index: 0,
            pose_duration,
        }
    }
}
