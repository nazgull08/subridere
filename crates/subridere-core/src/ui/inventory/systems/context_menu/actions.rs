use bevy::prelude::*;
use bevy_ui_actions::UiAction;

use super::state::ContextMenuState;
use crate::camera::flycam::FlyCamera;
use crate::game_init::assets::GameAssets;
use crate::inventory::{Equipment, Inventory, InventorySlot};
use crate::items::definition::{ItemDefinition, ItemProperties};
use crate::items::spawn::{spawn_world_item, WorldItemSpawnConfig};
use crate::items::visual::definition::VisualDefinition;
use crate::player::component::Player;
use crate::ui::inventory::systems::EquipmentSlotType;

/// Action: экипировать предмет из инвентаря
pub struct EquipItemAction {
    pub slot_index: usize,
}

impl UiAction for EquipItemAction {
    fn execute(&self, world: &mut World) {
        let slot_index = self.slot_index;

        // Phase 1: Получаем item_id через query
        let item_id = {
            let mut player_query = world.query_filtered::<&Inventory, With<Player>>();
            let Ok(inventory) = player_query.single(world) else {
                warn!("⚠️ Player not found");
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            let Some(item) = inventory.slots.get(slot_index).and_then(|s| s.as_ref()) else {
                warn!("⚠️ No item in slot {} to equip", slot_index);
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            item.item_id.clone()
        };

        // Phase 2: Определяем slot type (теперь можем брать ресурсы)
        let equip_slot_type = {
            let game_assets = world.resource::<GameAssets>();
            let item_defs = world.resource::<Assets<ItemDefinition>>();

            let Some(slot_type) = determine_equipment_slot(&item_id, game_assets, item_defs) else {
                info!("❌ Cannot determine equipment slot for this item");
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            slot_type
        };

        // Phase 3: Выполняем equip (мутация)
        {
            let mut player_query =
                world.query_filtered::<(&mut Inventory, &mut Equipment), With<Player>>();
            let Ok((mut inventory, mut equipment)) = player_query.single_mut(world) else {
                warn!("⚠️ Player not found");
                return;
            };

            let Some(item) = inventory.slots[slot_index].take() else {
                warn!("⚠️ No item in slot {}", slot_index);
                return;
            };

            let item_id = item.item_id;
            let equip_slot = equipment.get_slot_mut(equip_slot_type);

            // Swap if slot occupied
            if let Some(old_item_id) = equip_slot.take() {
                inventory.slots[slot_index] = Some(InventorySlot {
                    item_id: old_item_id,
                    quantity: 1,
                });
            }

            *equip_slot = Some(item_id.clone());
            info!("✅ Equipped {} to {:?}", item_id, equip_slot_type);
        }

        world.resource_mut::<ContextMenuState>().close();
    }
}

/// Action: выбросить предмет из инвентаря
pub struct DropFromInventoryAction {
    pub slot_index: usize,
}

impl UiAction for DropFromInventoryAction {
    fn execute(&self, world: &mut World) {
        let slot_index = self.slot_index;

        // Phase 1: Собираем данные
        let drop_data = {
            let mut camera_query = world.query_filtered::<&GlobalTransform, With<FlyCamera>>();
            let Ok(camera_transform) = camera_query.single(world) else {
                warn!("⚠️ Camera not found");
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            let forward = camera_transform.forward();
            let drop_position = camera_transform.translation() + *forward * 1.5;
            let drop_velocity = *forward * 3.0;

            let mut player_query = world.query_filtered::<&mut Inventory, With<Player>>();
            let Ok(mut inventory) = player_query.single_mut(world) else {
                warn!("⚠️ Player not found");
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            let Some(item) = inventory.slots[slot_index].take() else {
                warn!("⚠️ No item in slot {} to drop", slot_index);
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            (item.item_id, item.quantity, drop_position, drop_velocity)
        };

        let (item_id, quantity, drop_position, drop_velocity) = drop_data;

        // Phase 2: Spawn item using nested resource_scope
        world.resource_scope(|world, game_assets: Mut<GameAssets>| {
            world.resource_scope(|world, visuals: Mut<Assets<VisualDefinition>>| {
                world.resource_scope(|world, item_defs: Mut<Assets<ItemDefinition>>| {
                    world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
                        world.resource_scope(
                            |world, mut materials: Mut<Assets<StandardMaterial>>| {
                                let mut commands = world.commands();

                                spawn_world_item(
                                    &mut commands,
                                    WorldItemSpawnConfig {
                                        item_id: item_id.clone(),
                                        quantity,
                                        position: drop_position,
                                        initial_velocity: Some(drop_velocity),
                                    },
                                    &game_assets,
                                    &visuals,
                                    &item_defs,
                                    &mut meshes,
                                    &mut materials,
                                );

                                info!(
                                    "📤 Dropped {} (x{}) from inventory slot {}",
                                    item_id, quantity, slot_index
                                );
                            },
                        );
                    });
                });
            });
        });

        world.resource_mut::<ContextMenuState>().close();
    }
}

/// Action: выбросить предмет из экипировки
pub struct DropFromEquipmentAction {
    pub slot_type: EquipmentSlotType,
}

impl UiAction for DropFromEquipmentAction {
    fn execute(&self, world: &mut World) {
        let slot_type = self.slot_type;

        // Phase 1: Собираем данные
        let drop_data = {
            let mut camera_query = world.query_filtered::<&GlobalTransform, With<FlyCamera>>();
            let Ok(camera_transform) = camera_query.single(world) else {
                warn!("⚠️ Camera not found");
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            let forward = camera_transform.forward();
            let drop_position = camera_transform.translation() + *forward * 1.5;
            let drop_velocity = *forward * 3.0;

            let mut player_query = world.query_filtered::<&mut Equipment, With<Player>>();
            let Ok(mut equipment) = player_query.single_mut(world) else {
                warn!("⚠️ Player not found");
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            let equip_slot = equipment.get_slot_mut(slot_type);
            let Some(id) = equip_slot.take() else {
                warn!("⚠️ No item equipped in {:?} to drop", slot_type);
                world.resource_mut::<ContextMenuState>().close();
                return;
            };

            (id, drop_position, drop_velocity)
        };

        let (item_id, drop_position, drop_velocity) = drop_data;

        // Phase 2: Spawn item
        world.resource_scope(|world, game_assets: Mut<GameAssets>| {
            world.resource_scope(|world, visuals: Mut<Assets<VisualDefinition>>| {
                world.resource_scope(|world, item_defs: Mut<Assets<ItemDefinition>>| {
                    world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
                        world.resource_scope(
                            |world, mut materials: Mut<Assets<StandardMaterial>>| {
                                let mut commands = world.commands();

                                spawn_world_item(
                                    &mut commands,
                                    WorldItemSpawnConfig {
                                        item_id: item_id.clone(),
                                        quantity: 1,
                                        position: drop_position,
                                        initial_velocity: Some(drop_velocity),
                                    },
                                    &game_assets,
                                    &visuals,
                                    &item_defs,
                                    &mut meshes,
                                    &mut materials,
                                );

                                info!("📤 Dropped {} from equipment {:?}", item_id, slot_type);
                            },
                        );
                    });
                });
            });
        });

        world.resource_mut::<ContextMenuState>().close();
    }
}

/// Action: закрыть меню
pub struct CloseMenuAction;

impl UiAction for CloseMenuAction {
    fn execute(&self, world: &mut World) {
        info!("❌ Cancelled");
        world.resource_mut::<ContextMenuState>().close();
    }
}

/// Determine which equipment slot an item should go into
fn determine_equipment_slot(
    item_id: &str,
    game_assets: &GameAssets,
    item_defs: &Assets<ItemDefinition>,
) -> Option<EquipmentSlotType> {
    let def_handle = match item_id {
        "wooden_staff" => &game_assets.wooden_staff_def,
        "iron_helmet" => &game_assets.iron_helmet_def,
        _ => return None,
    };

    let item_def = item_defs.get(def_handle)?;

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
