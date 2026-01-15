use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use super::components::GameMenuRoot;
use super::layout::*;
use super::state::GameMenuActiveTab;
use super::tabs::{
    spawn_character_content, spawn_inventory_content, spawn_journal_tab, spawn_map_tab,
};

pub fn spawn_game_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_tab: Res<GameMenuActiveTab>,
) {
    let font = asset_server.load("fonts/dogica.ttf");
    let active = active_tab.0;

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            GlobalZIndex(100),
            GameMenuRoot,
            Name::new("Game Menu Root"),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(MENU_WIDTH),
                    height: Val::Px(MENU_HEIGHT),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(PANEL_BG),
                BorderColor(PANEL_BORDER),
                TabGroup::new(active),
                Name::new("Game Menu Panel"),
            ))
            .with_children(|panel| {
                // Tab buttons
                panel
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        Name::new("Tab Buttons"),
                    ))
                    .with_children(|row| {
                        spawn_tab_button(row, &font, 0, "Inventory", active == 0);
                        spawn_tab_button(row, &font, 1, "Character", active == 1);
                        spawn_tab_button(row, &font, 2, "Journal", active == 2);
                        spawn_tab_button(row, &font, 3, "Map", active == 3);
                    });

                // Tab content area
                panel
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..default()
                        },
                        Name::new("Tab Content Area"),
                    ))
                    .with_children(|content_area| {
                        // Tab 0: Inventory
                        content_area
                            .spawn((
                                Node {
                                    display: if active == 0 {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                TabContent::new(0),
                                Name::new("Tab Content: Inventory"),
                            ))
                            .with_children(|tab| {
                                spawn_inventory_content(tab, &font);
                            });

                        // Tab 1: Character
                        content_area
                            .spawn((
                                Node {
                                    display: if active == 1 {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                TabContent::new(1),
                                Name::new("Tab Content: Character"),
                            ))
                            .with_children(|tab| {
                                spawn_character_content(tab, &font);
                            });

                        // Tab 2: Journal
                        content_area
                            .spawn((
                                Node {
                                    display: if active == 2 {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                TabContent::new(2),
                                Name::new("Tab Content: Journal"),
                            ))
                            .with_children(|tab| {
                                spawn_journal_tab(tab, &font);
                            });

                        // Tab 3: Map
                        content_area
                            .spawn((
                                Node {
                                    display: if active == 3 {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                TabContent::new(3),
                                Name::new("Tab Content: Map"),
                            ))
                            .with_children(|tab| {
                                spawn_map_tab(tab, &font);
                            });
                    });
            });
        });

    info!("🎮 Game Menu spawned (tab: {})", active);
}

/// Сохранить активный таб перед закрытием
pub fn save_active_tab(mut active_tab: ResMut<GameMenuActiveTab>, query: Query<&TabGroup>) {
    for tab_group in &query {
        active_tab.0 = tab_group.active;
    }
}

pub fn despawn_game_menu(mut commands: Commands, query: Query<Entity, With<GameMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("🎮 Game Menu despawned");
}

fn spawn_tab_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    label: &str,
    is_active: bool,
) {
    let mut entity = parent.spawn((
        Button,
        Node {
            flex_grow: 1.0,
            padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(if is_active {
            Color::srgb(0.18, 0.18, 0.22)
        } else {
            Color::srgb(0.12, 0.12, 0.15)
        }),
        BorderColor(if is_active {
            Color::srgb(0.5, 0.5, 0.6)
        } else {
            Color::srgb(0.2, 0.2, 0.25)
        }),
        Tab::new(index),
        VisualStyle::tab(),
        InteractiveVisual,
        Interaction::None,
        Name::new(format!("Tab: {}", label)),
    ));

    if is_active {
        entity.insert(Active);
    }

    entity.with_children(|btn| {
        btn.spawn((
            Text::new(label),
            TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
    });
}
