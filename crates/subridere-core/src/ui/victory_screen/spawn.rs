use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::app::AppState;

use super::components::VictoryScreenRoot;

pub fn spawn_victory_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.05, 0.1, 0.95)),
            GlobalZIndex(300),
            VictoryScreenRoot,
            Name::new("Victory Screen Root"),
        ))
        .with_children(|root| {
            // "VICTORY" text
            root.spawn((
                Text::new("VICTORY"),
                TextFont {
                    font: font.clone(),
                    font_size: 56.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)), // Gold
            ));

            // Subtitle
            root.spawn((
                Text::new("The Ring of Power is yours!"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.8, 0.9)),
            ));

            // Spacer
            root.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Buttons container
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|buttons| {
                // Play Again button
                spawn_victory_button(buttons, &font, "Play Again", PlayAgainAction);

                // Main Menu button
                spawn_victory_button(buttons, &font, "Main Menu", MainMenuAction);
            });
        });

    info!("🏆 Victory screen spawned");
}

pub fn despawn_victory_screen(
    mut commands: Commands,
    query: Query<Entity, With<VictoryScreenRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("🏆 Victory screen despawned");
}

fn spawn_victory_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: impl UiAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.25, 0.15)),
            OnClick::new(action),
            InteractiveVisual,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.95, 0.8)),
            ));
        });
}

// === Actions ===

struct PlayAgainAction;

impl UiAction for PlayAgainAction {
    fn execute(&self, world: &mut World) {
        info!("🔄 Playing again...");
        world
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
    }
}

struct MainMenuAction;

impl UiAction for MainMenuAction {
    fn execute(&self, world: &mut World) {
        info!("🏠 Returning to Main Menu...");
        world
            .resource_mut::<NextState<AppState>>()
            .set(AppState::MainMenu);
    }
}
