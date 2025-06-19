use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn play_impact_sounds_system(
    mut events: EventReader<ContactForceEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for event in events.read() {
        let force = event.total_force.length();
        println!("impact - forec {:?}", force);

        if force > 10.0 {
            println!("!!!impact - forec {:?}", force);
            audio.play(asset_server.load("sounds/impact3.wav"));
        }
    }
}
