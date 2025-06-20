use bevy::prelude::*;
use bevy::animation::graph::{AnimationGraph, AnimationNodeIndex};

#[derive(Resource)]
pub struct JesterAnimAssets {
    pub graph: Handle<AnimationGraph>,
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
}

pub fn load_jester_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut anim_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let idle = asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/jester.glb"));
    let walk = asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/jester.glb"));
    let attack = asset_server.load(GltfAssetLabel::Animation(2).from_asset("models/jester.glb"));

    let mut graph = AnimationGraph::default();

    let root = graph.graph.add_node(AnimationGraphNode::default());

    let idle_node = graph.add_clip(idle, 1.0, root);
    let walk_node = graph.add_clip(walk, 1.0, root);
    let attack_node = graph.add_clip(attack, 1.0, root);

    commands.insert_resource(JesterAnimAssets {
        graph: anim_graphs.add(graph),
        idle: idle_node,
        walk: walk_node,
        attack: attack_node,
    });
}
