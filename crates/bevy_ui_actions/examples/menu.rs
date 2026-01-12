//! Пример меню с hover эффектами.
//!
//! Демонстрирует:
//! - State management через actions
//! - Hover actions для UI feedback
//! - Визуальное изменение при hover (автоматическое)
//!
//! Запуск: `cargo run --example menu -p bevy_ui_actions`

use bevy::prelude::*;
use bevy_ui_actions::{prelude::*, OnHover};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiActionsPlugin)
        .init_state::<GameState>()
        .init_resource::<HoverInfo>()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_status_text, update_hover_text))
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Settings,
}

#[derive(Resource, Default)]
struct HoverInfo(Option<String>);

// ============ Click Actions ============

struct StartGameAction;

impl UiAction for StartGameAction {
    fn execute(&self, world: &mut World) {
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        info!("Starting game...");
    }
}

struct OpenSettingsAction;

impl UiAction for OpenSettingsAction {
    fn execute(&self, world: &mut World) {
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Settings);
        info!("Opening settings...");
    }
}

struct BackToMenuAction;

impl UiAction for BackToMenuAction {
    fn execute(&self, world: &mut World) {
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Menu);
        info!("Back to menu...");
    }
}

struct QuitAction;

impl UiAction for QuitAction {
    fn execute(&self, world: &mut World) {
        world.send_event(AppExit::Success);
    }
}

// ============ Hover Actions ============

struct ShowHintAction {
    hint: &'static str,
}

impl UiAction for ShowHintAction {
    fn execute(&self, world: &mut World) {
        world.resource_mut::<HoverInfo>().0 = Some(self.hint.to_string());
    }
}

// ============ UI Components ============

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct HoverText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(15.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Main Menu"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
            ));

            // Status
            parent.spawn((
                Text::new("State: Menu"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                StatusText,
            ));

            // Hover hint
            parent.spawn((
                Text::new("Hover over a button..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
                HoverText,
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(20.0),
                ..default()
            });

            // Buttons with hover hints
            spawn_menu_button(
                parent,
                StartGameAction,
                "Start Game",
                "Begin your adventure!",
            );
            spawn_menu_button(
                parent,
                OpenSettingsAction,
                "Settings",
                "Configure game options",
            );
            spawn_menu_button(
                parent,
                BackToMenuAction,
                "Back to Menu",
                "Return to main menu",
            );
            spawn_menu_button(parent, QuitAction, "Quit", "Exit the game");
        });
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    action: impl UiAction,
    label: &str,
    hint: &'static str,
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
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            ActionButton::new(action),
            OnHover::new(ShowHintAction { hint }),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
            ));
        });
}

fn update_status_text(state: Res<State<GameState>>, mut query: Query<&mut Text, With<StatusText>>) {
    if state.is_changed() {
        for mut text in &mut query {
            **text = format!("State: {:?}", state.get());
        }
    }
}

fn update_hover_text(hover_info: Res<HoverInfo>, mut query: Query<&mut Text, With<HoverText>>) {
    if hover_info.is_changed() {
        for mut text in &mut query {
            **text = hover_info
                .0
                .clone()
                .unwrap_or_else(|| "Hover over a button...".to_string());
        }
    }
}
