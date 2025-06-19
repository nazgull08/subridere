use bevy::prelude::*;
use bevy_animation::AnimationClip;

/// Таймер, чтобы дать ассетам время загрузиться
#[derive(Resource)]
pub struct AnimationCheckTimer(Timer);

pub fn setup_animation_check(mut commands: Commands) {
    commands.insert_resource(AnimationCheckTimer(Timer::from_seconds(1.5, TimerMode::Once)));
}

pub fn check_available_animations(
    time: Res<Time>,
    mut timer: ResMut<AnimationCheckTimer>,
    asset_server: Res<AssetServer>,
    animation_clips: Res<Assets<AnimationClip>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let base_path = "models/char1.glb";

        for i in 0..10 {
            let handle: Handle<AnimationClip> =
                asset_server.load(&format!("{base_path}#Animation{i}"));

            if let Some(clip) = animation_clips.get(&handle) {
                println!("✅ Found animation {i}: duration = {:.2}s", clip.duration());
            } else {
                println!("⛔ No animation found at index {i}");
            }
        }
    }
}
