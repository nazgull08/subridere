use bevy::prelude::*;

use crate::app::AppState;
use crate::player::component::Player;
use crate::stats::{health::component::Health, mana::component::Mana, stamina::component::Stamina};

#[derive(Component)]
struct HpText;
#[derive(Component)]
struct MpText;
#[derive(Component)]
struct SpText;

/// Marker for the stats UI root (for cleanup)
#[derive(Component)]
struct StatsUiRoot;

pub struct UiStatsPlugin;

impl Plugin for UiStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_stats_ui)
            .add_systems(OnExit(AppState::InGame), despawn_stats_ui)
            .add_systems(Update, update_stats_ui.run_if(in_state(AppState::InGame)));
    }
}

fn spawn_stats_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            StatsUiRoot,
            Name::new("Stats UI Root"),
        ))
        .with_children(|parent| {
            // HP
            parent
                .spawn((
                    Text::new("HP: "),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                ))
                .with_child((
                    TextSpan::from("0"),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.2, 0.2)),
                    HpText,
                ));

            // MP
            parent
                .spawn((
                    Text::new("MP: "),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                ))
                .with_child((
                    TextSpan::from("0"),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 1.0)),
                    MpText,
                ));

            // SP
            parent
                .spawn((
                    Text::new("SP: "),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                ))
                .with_child((
                    TextSpan::from("0"),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 1.0, 0.2)),
                    SpText,
                ));
        });

    info!("✅ Stats UI spawned");
}

fn despawn_stats_ui(mut commands: Commands, query: Query<Entity, With<StatsUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("🧹 Stats UI despawned");
}

fn update_stats_ui(
    player_query: Query<(&Health, &Mana, &Stamina), With<Player>>,
    mut hp_query: Query<&mut TextSpan, (With<HpText>, Without<MpText>, Without<SpText>)>,
    mut mp_query: Query<&mut TextSpan, (With<MpText>, Without<HpText>, Without<SpText>)>,
    mut sp_query: Query<&mut TextSpan, (With<SpText>, Without<HpText>, Without<MpText>)>,
) {
    let Ok((health, mana, stamina)) = player_query.single() else {
        return;
    };

    if let Ok(mut hp) = hp_query.single_mut() {
        **hp = format!("{:.0}/{:.0}", health.current, health.max);
    }
    if let Ok(mut mp) = mp_query.single_mut() {
        **mp = format!("{:.0}/{:.0}", mana.current, mana.max);
    }
    if let Ok(mut sp) = sp_query.single_mut() {
        **sp = format!("{:.0}/{:.0}", stamina.current, stamina.max);
    }
}
