use bevy::prelude::*;
use bevy::animation::{graph::AnimationGraphHandle, AnimationPlayer};

use crate::enemy::{
    assets::JesterAnimAssets,
    component::{Enemy, EnemyState},
};

#[allow(clippy::type_complexity)]
pub fn sync_anim_state(
    anims: Res<JesterAnimAssets>,
    mut commands: Commands,
    enemies: Query<(Entity, &EnemyState), With<Enemy>>,
    children_q: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    graph_handles: Query<&AnimationGraphHandle>,
) {
    for (entity, state) in &enemies {
        // Locate children (scene root + animation player)
        if let Ok(children) = children_q.get(entity) {
            for child in children.iter() {
                // We only care about the child that has an AnimationPlayer
                if let Ok(mut player) = players.get_mut(child) {
                    // Attach the graph once if it's missing
                    if graph_handles.get(child).is_err() {
                        commands
                            .entity(child)
                            .insert(AnimationGraphHandle(anims.graph.clone()));
                    }

                    // Map logical state → animation node index
                    let node = match state {
                        EnemyState::Idle => anims.idle,
                        EnemyState::Walk => anims.walk,
                        EnemyState::Attack => anims.attack,
                    };

                    // Always call `play` (cheap); Bevy dedupes identical requests internally.
                    player.play(node).repeat();
                }
            }
        }
    }
}
