use bevy::prelude::*;

use crate::ui::game_menu::layout::*;

pub fn spawn_character_tab(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Name::new("Character Tab Content"),
        ))
        .with_children(|content| {
            content.spawn((
                Text::new("⚔️ Character"),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            content.spawn((
                Text::new("Attributes, level, and stats will be here"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_DIM),
            ));
        });
}
