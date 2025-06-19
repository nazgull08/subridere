use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("music/background_audio.wav"))
        .looped();
}
