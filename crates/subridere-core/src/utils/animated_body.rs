use bevy::prelude::*;
use block_bodies_core::{BlockBody, PartId};
use std::collections::HashMap;

/// Component that stores hierarchical body structure and entity mapping
///
/// Allows animation systems to manipulate body parts using BlockBody API
/// while maintaining references to spawned Bevy entities.
#[derive(Component)]
pub struct AnimatedBody {
    /// Hierarchical body structure from block-bodies-core
    pub body: BlockBody,

    /// Mapping from PartId (SlotMap key) to spawned Bevy Entity
    ///
    /// Used to translate between BlockBody's internal representation
    /// and actual entities in the Bevy world.
    pub part_entities: HashMap<PartId, Entity>,
}

impl AnimatedBody {
    /// Create new AnimatedBody with given structure
    pub fn new(body: BlockBody) -> Self {
        Self {
            body,
            part_entities: HashMap::new(),
        }
    }

    /// Get Bevy Entity for a body part by name
    pub fn get_entity(&self, part_name: &str) -> Option<Entity> {
        let part_id = self.body.get_part_id(part_name)?;
        self.part_entities.get(&part_id).copied()
    }

    /// Get all child entities of a part
    pub fn get_child_entities(&self, parent_name: &str) -> Vec<Entity> {
        let child_ids = self.body.get_children_ids(parent_name);
        child_ids
            .iter()
            .filter_map(|&id| self.part_entities.get(&id).copied())
            .collect()
    }
}
