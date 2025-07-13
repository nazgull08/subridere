use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SocketType {
    WeaponMain,
    WeaponOffhand,
    Helmet,
    Shield,
}

/// Описание одной части тела в модели (иерархическая)
#[derive(Clone)]
pub struct BlockModelPart {
    pub name: String,
    pub parent: Option<String>, // ← ключ к иерархии
    pub local_offset: Vec3,
    pub size: Vec3,
    pub material: Handle<StandardMaterial>,
    pub socket: Option<SocketType>,
}

impl BlockModelPart {
    pub fn new(
        name: &str,
        parent: Option<&str>,
        local_offset: Vec3,
        size: Vec3,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            name: name.to_string(),
            parent: parent.map(|s| s.to_string()),
            local_offset,
            size,
            material,
            socket: None,
        }
    }

    pub fn with_socket(mut self, socket: SocketType) -> Self {
        self.socket = Some(socket);
        self
    }
}

/// Модель тела: набор иерархически связанных частей
#[derive(Clone)]
pub struct BlockModel {
    pub parts: Vec<BlockModelPart>,
}

impl BlockModel {
    pub fn new(parts: Vec<BlockModelPart>) -> Self {
        Self { parts }
    }
}

pub fn spawn_model_hierarchical(
    model: &BlockModel,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    root_entity: Entity,
) {
    let visual_root = commands
        .spawn((
            Name::new("VisualRoot"),
            // Повернём на -90° по Y, чтобы `-Z` модели стал смотреть «вперёд» в мире Bevy
            Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
            GlobalTransform::default(),
        ))
        .id();

    commands.entity(root_entity).add_child(visual_root);

    let mut entity_map: HashMap<String, Entity> = HashMap::new();

    // Создаём части
    for part in &model.parts {
        let mesh = meshes.add(Mesh::from(Cuboid::new(
            part.size.x,
            part.size.y,
            part.size.z,
        )));

        let entity = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(part.material.clone()),
                Transform::from_translation(part.local_offset),
                Name::new(part.name.clone()),
            ))
            .id();

        entity_map.insert(part.name.clone(), entity);
    }

    // Собираем иерархию
    for part in &model.parts {
        let entity = entity_map[&part.name];
        if let Some(parent_name) = &part.parent {
            if let Some(parent_entity) = entity_map.get(parent_name) {
                commands.entity(*parent_entity).add_child(entity);
            } else {
                warn!(
                    "Parent '{}' not found for part '{}'",
                    parent_name, part.name
                );
                commands.entity(visual_root).add_child(entity); // fallback
            }
        } else {
            commands.entity(visual_root).add_child(entity); // теперь на visual_root, не на root_entity
        }
    }
}
