use bevy::prelude::*;

#[derive(Component)]
pub struct Crosshair;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair);
    }
}

fn spawn_crosshair(mut commands: Commands) {
    // Центральная точка
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            margin: UiRect {
                left: Val::Px(-2.0), // Смещение на половину ширины
                top: Val::Px(-2.0),  // Смещение на половину высоты
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Crosshair,
        Name::new("Crosshair"),
    ));
}
