use bevy::prelude::*;
use crate::stats::{
    health::component::Health,
    mana::component::Mana,
    stamina::component::Stamina,
};
use crate::player::component::Player;

#[derive(Component)]
struct HpText;
#[derive(Component)]
struct MpText;
#[derive(Component)]
struct SpText;

pub struct UiStatsPlugin;

impl Plugin for UiStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stats_ui)
           .add_systems(Update, update_stats_ui);
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
            Name::new("Stats UI Root"),
        ))
        .with_children(|parent| {
            // HP
            parent.spawn((Text::new("HP: "), TextFont { font: font.clone(), ..default() }))
                  .with_child((
                      TextSpan::from("0"),
                      TextFont { font: font.clone(), ..default() },
                      TextColor(Color::srgb(1.0, 0.2, 0.2)),
                      HpText,
                  ));

            // MP
            parent.spawn((Text::new("MP: "), TextFont { font: font.clone(), ..default() }))
                  .with_child((
                      TextSpan::from("0"),
                      TextFont { font: font.clone(), ..default() },
                      TextColor(Color::srgb(0.2, 0.2, 1.0)),
                      MpText,
                  ));

            // SP
            parent.spawn((Text::new("SP: "), TextFont { font: font.clone(), ..default() }))
                  .with_child((
                      TextSpan::from("0"),
                      TextFont { font: font.clone(), ..default() },
                      TextColor(Color::srgb(0.2, 1.0, 0.2)),
                      SpText,
                  ));
        });
}

fn update_stats_ui(
    player: Query<(&Health, Option<&Mana>, Option<&Stamina>), With<Player>>,
    mut span_sets: ParamSet<(
        Query<&mut TextSpan, With<HpText>>,
        Query<&mut TextSpan, With<MpText>>,
        Query<&mut TextSpan, With<SpText>>,
    )>,
) {
    if let Ok((health, mana, stamina)) = player.single() {
        if let Ok(mut span) = span_sets.p0().single_mut() {
            **span = format!("{:.0}", health.current);
        }
        if let (Some(m), Ok(mut span)) = (mana, span_sets.p1().single_mut()) {
            **span = format!("{:.0}", m.current);
        }
        if let (Some(s), Ok(mut span)) = (stamina, span_sets.p2().single_mut()) {
            **span = format!("{:.0}", s.current);
        }
    }
}
