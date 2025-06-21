use bevy::prelude::*;

/// Тип "сокета", если на эту часть можно повесить предмет (оружие, броню и т.д.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocketType {
    WeaponMain,
    WeaponOffhand,
    Helmet,
    Shield,
}

/// Описание одной части блочного тела
#[derive(Clone)]
pub struct BlockPart {
    pub name: String,
    pub offset: Vec3,
    pub size: Vec3,
    pub material: Handle<StandardMaterial>,
    pub socket: Option<SocketType>,
}

impl BlockPart {
    pub fn new(
        name: &str,
        offset: Vec3,
        size: Vec3,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            name: name.to_string(),
            offset,
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

/// Тело, состоящее из набора блочных частей
#[derive(Clone)]
pub struct BlockBody {
    pub parts: Vec<BlockPart>,
}

impl BlockBody {
    pub fn new(parts: Vec<BlockPart>) -> Self {
        Self { parts }
    }

    /// Заспавнить все части тела как детей к указанному родителю
    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        parent: Entity,
    ) {
        commands.entity(parent).with_children(|child| {
            for part in &self.parts {
                let mesh = meshes.add(Mesh::from(Cuboid::new(
                    part.size.x,
                    part.size.y,
                    part.size.z,
                )));
                child.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(part.material.clone()),
                    Transform::from_translation(part.offset),
                    Name::new(part.name.clone()),
                    // Можно добавить Tag-компоненты на основе socket, если нужно
                ));
            }
        });
    }
}
