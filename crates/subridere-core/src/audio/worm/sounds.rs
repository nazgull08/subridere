use super::events::{WormBiteEvent, WormHurtEvent};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

/// Plays bite sound when worm attacks player
pub fn play_worm_bite_sfx(
    mut evr: EventReader<WormBiteEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for _ in evr.read() {
        let sound = asset_server.load("sounds/worm/bite.wav");
        audio.play(sound).with_volume(0.7);
        info!("🔊 Playing worm bite sound");
    }
}

/// Plays hurt sound when worm takes damage
pub fn play_worm_hurt_sfx(
    mut evr: EventReader<WormHurtEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for _ in evr.read() {
        let sound = asset_server.load("sounds/worm/hurt.wav");
        audio.play(sound).with_volume(0.6);
        info!("🔊 Playing worm hurt sound");
    }
}
