use bevy::prelude::*;
use crate::enemy::component::*;

pub fn walk_behavior_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &TargetPos), With<EnemyState>>,
) {
    for (mut transform, target) in &mut query {
        let dir = (target.0 - transform.translation).normalize_or_zero();
        transform.translation += dir * time.delta_secs() * 2.5;
    }
}
