use bevy::prelude::*;
use bevy::animation::graph::AnimationGraph;
use bevy::animation::{AnimationPlayer, graph::AnimationGraphHandle};

#[derive(Resource)]
pub struct SkeletonAnimAssets {
    pub graph: Handle<AnimationGraph>,
    pub node: bevy::animation::graph::AnimationNodeIndex,
}

pub fn load_fox_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut anim_graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Загружаем первую анимацию из glb по индексу
    let clip = asset_server.load(GltfAssetLabel::Animation(2).from_asset("models/jester.glb"));

    // Оборачиваем её в граф
    let (graph, node) = AnimationGraph::from_clip(clip.clone());
    let graph_handle = anim_graphs.add(graph);

    // Сохраняем как ресурс
    commands.insert_resource(SkeletonAnimAssets {
        graph: graph_handle,
        node,
    });
}

pub fn play_fox_animation(
    anims: Res<SkeletonAnimAssets>,
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (entity, mut player) in &mut players {
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(anims.graph.clone()));
        player.play(anims.node).repeat();
    }
}
