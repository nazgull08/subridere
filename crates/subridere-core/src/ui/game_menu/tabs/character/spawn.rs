use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::stats::AttributeType;
use crate::ui::game_menu::layout::*;

use super::actions::IncreaseAttributeAction;
use super::components::*;

pub fn spawn_character_content(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            Name::new("Character Content"),
        ))
        .with_children(|content| {
            // Header: Level & XP
            spawn_level_section(content, font);

            // Main content: Attributes | Stats
            content
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        column_gap: Val::Px(30.0),
                        ..default()
                    },
                    Name::new("Character Main"),
                ))
                .with_children(|main| {
                    // Left: Attributes
                    spawn_attributes_section(main, font);

                    // Divider
                    main.spawn((
                        Node {
                            width: Val::Px(1.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.35)),
                    ));

                    // Right: Derived Stats
                    spawn_stats_section(main, font);
                });
        });
}

// ============================================================
// Level Section
// ============================================================

fn spawn_level_section(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.5)),
            BorderColor(Color::srgb(0.25, 0.25, 0.3)),
            Name::new("Level Section"),
        ))
        .with_children(|section| {
            // Row: Level + Attribute Points
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|row| {
                    // Level
                    row.spawn((
                        Text::new("Level 1"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        LevelText,
                    ));

                    // Attribute Points
                    row.spawn((
                        Text::new("Attribute Points: 0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.4, 0.8, 0.4)),
                        AttributePointsText,
                    ));
                });

            // XP Bar
            section
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    Name::new("XP Row"),
                ))
                .with_children(|row| {
                    // Progress bar background
                    row.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(16.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
                        BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                        Name::new("XP Bar BG"),
                    ))
                    .with_children(|bar_bg| {
                        // Fill
                        bar_bg.spawn((
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.6, 0.9)),
                            XpProgressBar,
                        ));
                    });

                    // XP Text
                    row.spawn((
                        Text::new("0/100"),
                        TextFont {
                            font: font.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(TEXT_DIM),
                        Node {
                            min_width: Val::Px(80.0),
                            ..default()
                        },
                        XpText,
                    ));
                });
        });
}

// ============================================================
// Attributes Section
// ============================================================

fn spawn_attributes_section(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                min_width: Val::Px(280.0),
                ..default()
            },
            Name::new("Attributes Section"),
        ))
        .with_children(|section| {
            // Title
            section.spawn((
                Text::new("ATTRIBUTES"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_DIM),
            ));

            // Attribute rows
            for attr in AttributeType::ALL {
                spawn_attribute_row(section, font, attr);
            }
        });
}

fn spawn_attribute_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    attr: AttributeType,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Name::new(format!("Attr: {:?}", attr)),
        ))
        .with_children(|row| {
            // Name
            row.spawn((
                Text::new(format!("{:10}", attr.name())),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    min_width: Val::Px(90.0),
                    ..default()
                },
            ));

            // Progress bar
            row.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(12.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
                BorderColor(Color::srgb(0.3, 0.3, 0.35)),
            ))
            .with_children(|bar_bg| {
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(10.0), // Will be synced
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(attr_color(attr)),
                    AttributeBar(attr),
                ));
            });

            // Value
            row.spawn((
                Text::new("3"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    min_width: Val::Px(25.0),
                    ..default()
                },
                AttributeValueText(attr),
            ));

            // [+] Button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.22)),
                BorderColor(Color::srgb(0.4, 0.4, 0.45)),
                OnClick::new(IncreaseAttributeAction(attr)),
                InteractiveVisual,
                IncreaseAttributeButton(attr),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("+"),
                    TextFont {
                        font: font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.8, 0.5)),
                ));
            });
        });
}

fn attr_color(attr: AttributeType) -> Color {
    match attr {
        AttributeType::Might => Color::srgb(0.8, 0.3, 0.3),
        AttributeType::Fortitude => Color::srgb(0.8, 0.6, 0.3),
        AttributeType::Agility => Color::srgb(0.3, 0.8, 0.3),
        AttributeType::Arcana => Color::srgb(0.4, 0.4, 0.9),
        AttributeType::Resolve => Color::srgb(0.7, 0.5, 0.8),
    }
}

// ============================================================
// Stats Section
// ============================================================

fn spawn_stats_section(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                min_width: Val::Px(200.0),
                ..default()
            },
            Name::new("Stats Section"),
        ))
        .with_children(|section| {
            section.spawn((
                Text::new("STATS"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_DIM),
            ));

            // Resources
            spawn_stat_row(section, font, "Max HP", StatType::MaxHealth);
            spawn_stat_row(section, font, "Max Mana", StatType::MaxMana);
            spawn_stat_row(section, font, "Max Stamina", StatType::MaxStamina);

            // Spacer
            section.spawn(Node {
                height: Val::Px(5.0),
                ..default()
            });

            // Combat
            spawn_stat_row(section, font, "Melee Dmg", StatType::MeleeDamage);
            spawn_stat_row(section, font, "Magic Dmg", StatType::MagicDamage);
            spawn_stat_row(section, font, "Atk Speed", StatType::AttackSpeed);
            spawn_stat_row(section, font, "Move Speed", StatType::MoveSpeed);

            // Spacer
            section.spawn(Node {
                height: Val::Px(5.0),
                ..default()
            });

            // Defense
            spawn_stat_row(section, font, "Phys Def", StatType::PhysDefense);
            spawn_stat_row(section, font, "Magic Res", StatType::MagicResist);
        });
}

fn spawn_stat_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    stat_type: StatType,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(TEXT_DIM),
            ));

            row.spawn((
                Text::new("0"),
                TextFont {
                    font: font.clone(),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                StatText(stat_type),
            ));
        });
}
