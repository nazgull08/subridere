use crate::block_bodies::pose::{BlockPose, BlockPosePart, PoseToApply};
use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct PoseLerp {
    pub from: BlockPose,
    pub to: BlockPose,
    pub timer: Timer,
    pub looping: bool,
}

pub fn lerp_pose_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PoseLerp)>,
) {
    for (entity, mut lerp) in &mut query {
        lerp.timer.tick(time.delta());

        let t = lerp.timer.fraction().clamp(0.0, 1.0);

        let from_map = lerp.from.part_map();
        let to_map = lerp.to.part_map();

        let mut parts = vec![];

        for name in from_map.keys() {
            if let (Some(a), Some(b)) = (from_map.get(name), to_map.get(name)) {
                let rot = a.rotation.slerp(b.rotation, t);
                parts.push(BlockPosePart {
                    name: name.clone(),
                    rotation: rot,
                });
            }
        }

        commands
            .entity(entity)
            .insert(PoseToApply(BlockPose { parts }));

        if lerp.timer.finished() {
            if lerp.looping {
                let from = std::mem::take(&mut lerp.from);
                lerp.from = std::mem::take(&mut lerp.to);
                lerp.to = from;
                lerp.timer.reset();
            } else {
                commands.entity(entity).remove::<PoseLerp>();
            }
        }
    }
}
