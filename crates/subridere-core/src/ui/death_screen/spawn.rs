use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::app::AppState;

use super::components::DeathScreenRoot;

pub fn spawn_death_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            BackgroundColor(Color::srgba(0.1, 0.0, 0.0, 0.9)),
            GlobalZIndex(300),
            DeathScreenRoot,
            Name::new("Death Screen Root"),
        ))
        .with_children(|root| {
            // "YOU DIED" text
            root.spawn((
                Text::new("YOU DIED"),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.1, 0.1)),
            ));

            // Subtitle
            root.spawn((
                Text::new("The dungeon claims another soul..."),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.4, 0.4)),
            ));

            // Spacer
            root.spawn(Node {
                height: Val::Px(30.0),
                ..default()
            });

            // Buttons container
            root.spawn((Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                align_items: AlignItems::Center,
                ..default()
            },))
                .with_children(|buttons| {
                    // Respawn button
                    spawn_death_button(buttons, &font, "Try Again", RespawnAction);

                    // Main Menu button
                    spawn_death_button(buttons, &font, "Main Menu", MainMenuAction);
                });
        });

    info!("💀 Death screen spawned");
}

pub fn despawn_death_screen(mut commands: Commands, query: Query<Entity, With<DeathScreenRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("💀 Death screen despawned");
}

fn spawn_death_button(
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
            BackgroundColor(Color::srgb(0.3, 0.1, 0.1)),
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
                TextColor(Color::srgb(0.9, 0.8, 0.8)),
            ));
        });
}

// === Actions ===

struct RespawnAction;

impl UiAction for RespawnAction {
    fn execute(&self, world: &mut World) {
        info!("🔄 Respawning...");
        // Переходим в InGame - cleanup произойдёт автоматически через OnExit(Dead)
        // А потом OnEnter(InGame) запустит новый забег
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
