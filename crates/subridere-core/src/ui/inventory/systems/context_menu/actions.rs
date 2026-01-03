use bevy::prelude::*;
use super::state::*;

/// Handle menu button clicks and execute corresponding actions
pub fn handle_menu_button_clicks(
    equip_button: Query<&Interaction, (Changed<Interaction>, With<EquipButton>)>,
    drop_button: Query<&Interaction, (Changed<Interaction>, With<DropButton>)>,
    cancel_button: Query<&Interaction, (Changed<Interaction>, With<CancelButton>)>,
    mut menu_state: ResMut<ContextMenuState>,
    mut commands: Commands,
    camera_query: Query<&GlobalTransform, With<crate::camera::flycam::FlyCamera>>,
    mut player_query: Query<
        (&GlobalTransform, &mut crate::inventory::Inventory, &mut crate::inventory::Equipment),
        With<crate::player::component::Player>,
    >,
    game_assets: Res<crate::game_init::assets::GameAssets>,
    visuals: Res<Assets<crate::items::visual::definition::VisualDefinition>>,
    item_defs: Res<Assets<crate::items::definition::ItemDefinition>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Check Cancel button (simple case)
    for interaction in &cancel_button {
        if *interaction == Interaction::Pressed {
            info!("❌ Cancelled");
            menu_state.close();
            return;
        }
    }

    // Check Equip button
    for interaction in &equip_button {
        if *interaction == Interaction::Pressed {
            handle_equip_action(
                &mut menu_state,
                &mut player_query,
                &game_assets,
                &item_defs,
            );
            return;
        }
    }

    // Check Drop button
    for interaction in &drop_button {
        if *interaction == Interaction::Pressed {
            handle_drop_action(
                &mut menu_state,
                &mut commands,
                &camera_query,
                &mut player_query,
                &game_assets,
                &visuals,
                &item_defs,
                &mut meshes,
                &mut materials,
            );
            return;
        }
    }
}

/// Handle Equip button action
fn handle_equip_action(
    menu_state: &mut ContextMenuState,
    player_query: &mut Query<
        (&GlobalTransform, &mut crate::inventory::Inventory, &mut crate::inventory::Equipment),
        With<crate::player::component::Player>,
    >,
    game_assets: &crate::game_init::assets::GameAssets,
    item_defs: &Assets<crate::items::definition::ItemDefinition>,
) {
    let Some(inv_slot) = menu_state.inventory_slot else {
        warn!("⚠️ Equip button clicked but no inventory slot selected");
        menu_state.close();
        return;
    };

    let Ok((_, mut inventory, mut equipment)) = player_query.single_mut() else {
        warn!("⚠️ Player not found");
        menu_state.close();
        return;
    };

    // Get item from inventory
    let Some(item) = inventory.slots[inv_slot].as_ref() else {
        warn!("⚠️ No item in slot {} to equip", inv_slot);
        menu_state.close();
        return;
    };

    // Determine which equipment slot to use based on item type
    let Some(equip_slot_type) = determine_equipment_slot(&item.item_id, game_assets, item_defs) else {
        info!("❌ Cannot determine equipment slot for this item");
        menu_state.close();
        return;
    };

    // Use core equip logic
    crate::ui::inventory::systems::actions::equip_item_core(
        &mut inventory,
        &mut equipment,
        inv_slot,
        equip_slot_type,
        game_assets,
        item_defs,
    );

    menu_state.close();
}

/// Handle Drop button action
fn handle_drop_action(
    menu_state: &mut ContextMenuState,
    commands: &mut Commands,
    camera_query: &Query<&GlobalTransform, With<crate::camera::flycam::FlyCamera>>,
    player_query: &mut Query<
        (&GlobalTransform, &mut crate::inventory::Inventory, &mut crate::inventory::Equipment),
        With<crate::player::component::Player>,
    >,
    game_assets: &crate::game_init::assets::GameAssets,
    visuals: &Assets<crate::items::visual::definition::VisualDefinition>,
    item_defs: &Assets<crate::items::definition::ItemDefinition>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        warn!(" Camera not found");
        menu_state.close();
        return;
    };

    let Ok((_, mut inventory, mut equipment)) = player_query.single_mut() else {
        warn!("⚠️ Player not found");
        menu_state.close();
        return;
    };

    // Calculate drop position and velocity
    let forward = camera_transform.forward();
    let drop_position = camera_transform.translation() + *forward * 1.5;
    let drop_velocity = *forward * 3.0;

    // Drop from inventory or equipment
    if let Some(inv_slot) = menu_state.inventory_slot {
        crate::inventory::systems::drop::drop_from_inventory(
            commands,
            &mut inventory,
            inv_slot,
            drop_position,
            drop_velocity,
            game_assets,
            visuals,
            item_defs,
            meshes,
            materials,
        );
    } else if let Some(equip_slot) = menu_state.equipment_slot {
        crate::inventory::systems::drop::drop_from_equipment(
            commands,
            &mut equipment,
            equip_slot,
            drop_position,
            drop_velocity,
            game_assets,
            visuals,
            item_defs,
            meshes,
            materials,
        );
    }

    menu_state.close();
}

/// Determine which equipment slot an item should go into
fn determine_equipment_slot(
    item_id: &str,
    game_assets: &crate::game_init::assets::GameAssets,
    item_defs: &Assets<crate::items::definition::ItemDefinition>,
) -> Option<crate::ui::inventory::systems::EquipmentSlotType> {
    use crate::items::definition::ItemProperties;
    use crate::ui::inventory::systems::EquipmentSlotType;

    // Get item definition
    let def_handle = match item_id {
        "wooden_staff" => &game_assets.wooden_staff_def,
        "iron_helmet" => &game_assets.iron_helmet_def,
        _ => return None,
    };

    let item_def = item_defs.get(def_handle)?;

    // Determine slot based on item properties
    match &item_def.properties {
        ItemProperties::Weapon(_) => Some(EquipmentSlotType::MainHand),
        ItemProperties::Armor(armor_props) => {
            use crate::items::definition::ArmorSlot;
            match armor_props.slot {
                ArmorSlot::Helmet => Some(EquipmentSlotType::Helmet),
                ArmorSlot::LeftPauldron => Some(EquipmentSlotType::LeftPauldron),
                ArmorSlot::RightPauldron => Some(EquipmentSlotType::RightPauldron),
                ArmorSlot::Chest => Some(EquipmentSlotType::Chest),
                ArmorSlot::LeftGlove => Some(EquipmentSlotType::LeftGlove),
                ArmorSlot::RightGlove => Some(EquipmentSlotType::RightGlove),
                ArmorSlot::Greaves => Some(EquipmentSlotType::Greaves),
                ArmorSlot::LeftBoot => Some(EquipmentSlotType::LeftBoot),
                ArmorSlot::RightBoot => Some(EquipmentSlotType::RightBoot),
            }
        }
        ItemProperties::Consumable(_) => None,
    }
}
