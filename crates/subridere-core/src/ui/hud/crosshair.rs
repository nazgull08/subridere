use bevy::prelude::*;

use crate::app::AppState;

#[derive(Component)]
pub struct Crosshair;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_crosshair)
            .add_systems(OnExit(AppState::InGame), despawn_crosshair);
    }
}

fn spawn_crosshair(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            margin: UiRect {
                left: Val::Px(-2.0),
                top: Val::Px(-2.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Crosshair,
        Name::new("Crosshair"),
    ));
    info!("✅ Crosshair spawned");
}

fn despawn_crosshair(mut commands: Commands, query: Query<Entity, With<Crosshair>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("🧹 Crosshair despawned");
}
