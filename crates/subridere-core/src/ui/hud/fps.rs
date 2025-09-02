use crate::core::fps_stats::FpsData;
use bevy::prelude::*;

#[derive(Component)]
struct FpsText;

pub struct UiFpsPlugin;

impl Plugin for UiFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fps_ui)
            .add_systems(Update, update_fps_ui);
    }
}

fn spawn_fps_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)),
            Name::new("FPS UI Root"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("FPS: "),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                ))
                .with_child((
                    TextSpan::from("0.0"),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 0.5, 0.7)),
                    FpsText,
                ));
        });
}

fn update_fps_ui(fps: Res<FpsData>, mut query: Query<&mut TextSpan, With<FpsText>>) {
    if let Ok(mut span) = query.single_mut() {
        **span = format!("{:.1}", fps.current);
    }
}
