use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::events::PlayerDamageEvent;

pub fn play_player_damage_sfx(
    mut evr: EventReader<PlayerDamageEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for _ in evr.read() {
        let sound = asset_server.load("./sounds/player/hurt_male_01.wav");
        audio.play(sound);
    }
}
