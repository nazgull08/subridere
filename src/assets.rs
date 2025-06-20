// src/assets.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct MyAssets {
    pub model: Handle<Scene>,
    pub anim_graph: Handle<AnimationGraph>,
    pub anim_node: AnimationNodeIndex,
}

impl FromWorld for MyAssets {
    fn from_world(world: &mut World) -> Self {
        let idle_clip: Handle<AnimationClip> = world
            .resource::<AssetServer>()
            .load("models/skeleton/Animation_Idle_withSkin.glb#Animation0");

        let (graph, node) = AnimationGraph::from_clip(idle_clip.clone());

        let graph_handle = world.resource_mut::<Assets<AnimationGraph>>().add(graph);

        Self {
            model: world
                .resource::<AssetServer>()
                .load("models/skeleton/Animation_Idle_withSkin.glb#Scene0"),
            anim_graph: graph_handle,
            anim_node: node,
        }
    }
}
