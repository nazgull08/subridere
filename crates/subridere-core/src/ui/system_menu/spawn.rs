use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use super::components::SystemMenuRoot;
use crate::app::AppState;

pub fn spawn_system_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            GlobalZIndex(200),
            SystemMenuRoot,
            Name::new("System Menu Root"),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(30.0)),
                    row_gap: Val::Px(15.0),
                    border: UiRect::all(Val::Px(2.0)),
                    min_width: Val::Px(250.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
                BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                Name::new("System Menu Panel"),
            ))
            .with_children(|panel| {
                // Title
                panel.spawn((
                    Text::new("PAUSED"),
                    TextFont {
                        font: font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

                // Spacer
                panel.spawn(Node {
                    height: Val::Px(10.0),
                    ..default()
                });

                // Resume button
                spawn_menu_button(panel, &font, "Resume", ResumeAction);

                // Settings button (заглушка)
                spawn_menu_button(panel, &font, "Settings", SettingsAction);

                // Main Menu button ← НОВОЕ
                spawn_menu_button(panel, &font, "Main Menu", MainMenuAction);

                // Quit button
                spawn_menu_button(panel, &font, "Quit Game", QuitGameAction);
            });
        });

    info!("⚙️ System Menu spawned");
}

pub fn despawn_system_menu(mut commands: Commands, query: Query<Entity, With<SystemMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("⚙️ System Menu despawned");
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
                width: Val::Px(200.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.22)),
            OnClick::new(action),
            InteractiveVisual,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));
        });
}

// === Actions ===

struct ResumeAction;

impl UiAction for ResumeAction {
    fn execute(&self, world: &mut World) {
        world
            .resource_mut::<NextState<super::state::SystemMenuState>>()
            .set(super::state::SystemMenuState::Closed);
    }
}

struct SettingsAction;

impl UiAction for SettingsAction {
    fn execute(&self, _world: &mut World) {
        info!("⚙️ Settings clicked (not implemented)");
    }
}

// ← НОВЫЙ ACTION
struct MainMenuAction;

impl UiAction for MainMenuAction {
    fn execute(&self, world: &mut World) {
        info!("🏠 Returning to Main Menu...");

        // Закрыть System Menu
        world
            .resource_mut::<NextState<super::state::SystemMenuState>>()
            .set(super::state::SystemMenuState::Closed);

        // Перейти в MainMenu (это триггернёт cleanup_game)
        world
            .resource_mut::<NextState<AppState>>()
            .set(AppState::MainMenu);
    }
}

struct QuitGameAction;

impl UiAction for QuitGameAction {
    fn execute(&self, world: &mut World) {
        info!("👋 Quitting game");
        world.send_event(AppExit::Success);
    }
}
