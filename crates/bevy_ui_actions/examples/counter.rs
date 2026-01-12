//! Минимальный пример - счётчик.
//!
//! TODO: Полная реализация будет добавлена в Phase 8.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Counter Example (WIP)".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Spawn placeholder text
    commands.spawn((
        Text::new("Counter example - under construction"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            ..default()
        },
    ));
}
