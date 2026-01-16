use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use super::components::MainMenuRoot;
use crate::app::AppState;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
            GlobalZIndex(100),
            MainMenuRoot,
            Name::new("Main Menu Root"),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(40.0)),
                    row_gap: Val::Px(20.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                Name::new("Main Menu Panel"),
            ))
            .with_children(|panel| {
                // Title
                panel.spawn((
                    Text::new("SUBRIDERE"),
                    TextFont {
                        font: font.clone(),
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

                // Subtitle
                panel.spawn((
                    Text::new("Dungeon Crawler"),
                    TextFont {
                        font: font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.65)),
                ));

                // Spacer
                panel.spawn(Node {
                    height: Val::Px(20.0),
                    ..default()
                });

                // New Game button
                spawn_menu_button(panel, &font, "New Game", NewGameAction);

                // Settings button (заглушка)
                spawn_menu_button(panel, &font, "Settings", SettingsAction);

                // Quit button
                spawn_menu_button(panel, &font, "Quit", QuitAction);
            });
        });

    info!("🎮 Main Menu spawned");
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("🎮 Main Menu despawned");
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: impl UiAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.18)),
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
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));
        });
}

// === Actions ===

struct NewGameAction;

impl UiAction for NewGameAction {
    fn execute(&self, world: &mut World) {
        info!("🎮 New Game clicked");
        world
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
    }
}

struct SettingsAction;

impl UiAction for SettingsAction {
    fn execute(&self, _world: &mut World) {
        info!("⚙️ Settings clicked (not implemented yet)");
    }
}

struct QuitAction;

impl UiAction for QuitAction {
    fn execute(&self, world: &mut World) {
        info!("👋 Quitting game");
        world.send_event(AppExit::Success);
    }
}
