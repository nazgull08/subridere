use crate::inventory::systems::TargetedItem;
use bevy::prelude::*;

/// Marker component for pickup hint text
#[derive(Component)]
pub struct PickupHintText;

/// Spawn pickup hint UI element (called once at startup)
pub fn spawn_pickup_hint(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands.spawn((
        Text::new(""), // Start empty
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(80.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        PickupHintText,
        Name::new("Pickup Hint"),
    ));

    info!("✅ Pickup hint UI spawned");
}

/// Update hint text based on what player is looking at
pub fn update_pickup_hint(
    targeted: Res<TargetedItem>,
    mut hint_query: Query<&mut Text, With<PickupHintText>>,
) {
    let Ok(mut text) = hint_query.single_mut() else {
        return;
    };

    // If looking at an item, show hint
    if let Some(item_name) = &targeted.name {
        text.0 = format!("Press [E] to pick up {}", item_name);
    } else {
        // Not looking at anything, clear text
        text.0.clear();
    }
}
