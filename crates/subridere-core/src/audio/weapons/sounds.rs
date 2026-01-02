use super::events::{MagicBoltFireEvent, PhysicsCubeFireEvent};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

/// Plays sound when magic bolt is fired
pub fn play_magic_bolt_fire_sfx(
    mut evr: EventReader<MagicBoltFireEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for _ in evr.read() {
        let sound = asset_server.load("sounds/weapons/magic_bolt.wav");
        audio.play(sound).with_volume(0.5);
    }
}

/// Plays sound when physical cube is launched
pub fn play_physical_cube_fire_sfx(
    mut evr: EventReader<PhysicsCubeFireEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for _ in evr.read() {
        let sound = asset_server.load("sounds/weapons/cube_launch.wav");
        audio.play(sound).with_volume(0.6);
    }
}
