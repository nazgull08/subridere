use crate::{BlockBodyError, BlockPart, PartId};
use slotmap::SlotMap;
use std::collections::HashMap;

/// A hierarchical collection of block parts forming a complete body
#[derive(Debug, Clone)]
pub struct BlockBody {
    /// All parts indexed by their SlotMap ID
    pub parts: SlotMap<PartId, BlockPart>,

    /// Mapping from part names to SlotMap IDs for lookup
    pub name_to_id: HashMap<String, PartId>,

    /// Cache of children for each part (private - maintained automatically)
    children: HashMap<PartId, Vec<PartId>>,

    /// Root part IDs (parts with no parent)
    pub roots: Vec<PartId>,
}

impl BlockBody {
    /// Create a new empty body
    pub fn new() -> Self {
        Self {
            parts: SlotMap::new(),
            name_to_id: HashMap::new(),
            children: HashMap::new(),
            roots: Vec::new(),
        }
    }

    /// Add a part to the body
    pub fn add_part(&mut self, part: BlockPart) -> Result<PartId, BlockBodyError> {
        // Validate part name
        if part.name.is_empty() {
            return Err(BlockBodyError::InvalidPartName(part.name.clone()));
        }

        // Check if part with this name already exists
        if self.name_to_id.contains_key(&part.name) {
            return Err(BlockBodyError::PartAlreadyExists(part.name.clone()));
        }

        // Check if parent exists (if specified)
        let parent_id = if let Some(parent_name) = &part.parent {
            match self.name_to_id.get(parent_name) {
                Some(id) => Some(*id),
                None => {
                    return Err(BlockBodyError::ParentNotFound {
                        parent: parent_name.clone(),
                        child: part.name.clone(),
                    })
                }
            }
        } else {
            None
        };

        let is_root = part.is_root();
        let name = part.name.clone();

        // Insert into SlotMap
        let id = self.parts.insert(part);

        // Update name mapping
        self.name_to_id.insert(name, id);

        // Update children cache
        if let Some(parent_id) = parent_id {
            self.children.entry(parent_id).or_default().push(id);
        }

        // Update roots if necessary
        if is_root {
            self.roots.push(id);
        }

        Ok(id)
    }

    /// Remove a part and all its descendants from the body
    pub fn remove_part(&mut self, name: &str) -> Result<Vec<BlockPart>, BlockBodyError> {
        let part_id = self
            .name_to_id
            .get(name)
            .copied()
            .ok_or_else(|| BlockBodyError::PartNotFound(name.to_string()))?;

        // Collect all descendants (including the part itself)
        let mut to_remove = Vec::new();
        self.collect_descendants(part_id, &mut to_remove);

        // Remove from all data structures
        let mut removed_parts = Vec::new();
        for id in to_remove {
            if let Some(part) = self.parts.remove(id) {
                self.name_to_id.remove(&part.name);
                self.children.remove(&id);

                // Remove from parent's children list
                if let Some(parent_name) = &part.parent {
                    if let Some(parent_id) = self.name_to_id.get(parent_name) {
                        if let Some(siblings) = self.children.get_mut(parent_id) {
                            siblings.retain(|&child_id| child_id != id);
                        }
                    }
                }

                // Remove from roots if it was a root
                self.roots.retain(|&root_id| root_id != id);

                removed_parts.push(part);
            }
        }

        Ok(removed_parts)
    }

    /// Get a part by name
    pub fn get_part_by_name(&self, name: &str) -> Option<&BlockPart> {
        let id = self.name_to_id.get(name)?;
        self.parts.get(*id)
    }

    /// Get a mutable part by name
    pub fn get_part_by_name_mut(&mut self, name: &str) -> Option<&mut BlockPart> {
        let id = self.name_to_id.get(name)?;
        self.parts.get_mut(*id)
    }

    /// Get a part by ID
    pub fn get_part(&self, id: PartId) -> Option<&BlockPart> {
        self.parts.get(id)
    }

    /// Get a mutable part by ID
    pub fn get_part_mut(&mut self, id: PartId) -> Option<&mut BlockPart> {
        self.parts.get_mut(id)
    }

    /// Get the ID of a part by name
    pub fn get_part_id(&self, name: &str) -> Option<PartId> {
        self.name_to_id.get(name).copied()
    }

    /// Get all direct children of a part (O(1) lookup)
    pub fn get_children(&self, parent_name: &str) -> Vec<&BlockPart> {
        let parent_id = match self.name_to_id.get(parent_name) {
            Some(id) => *id,
            None => return Vec::new(),
        };

        self.children
            .get(&parent_id)
            .map(|child_ids| {
                child_ids
                    .iter()
                    .filter_map(|&id| self.parts.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all direct children IDs of a part (O(1) lookup)
    pub fn get_children_ids(&self, parent_name: &str) -> Vec<PartId> {
        let parent_id = match self.name_to_id.get(parent_name) {
            Some(id) => *id,
            None => return Vec::new(),
        };

        self.children.get(&parent_id).cloned().unwrap_or_default()
    }

    /// Get chain from part to root (for IK) - returns [part, parent, grandparent, ..., root]
    pub fn get_chain_to_root(&self, part_name: &str) -> Vec<&BlockPart> {
        let mut chain = Vec::new();
        let mut current_name = part_name;

        while let Some(part) = self.get_part_by_name(current_name) {
            chain.push(part);
            match &part.parent {
                Some(parent_name) => current_name = parent_name,
                None => break, // Reached root
            }
        }

        chain
    }

    /// Get chain from part to root as IDs
    pub fn get_chain_to_root_ids(&self, part_name: &str) -> Vec<PartId> {
        let mut chain = Vec::new();
        let mut current_name = part_name;

        while let Some(part_id) = self.get_part_id(current_name) {
            chain.push(part_id);
            if let Some(part) = self.parts.get(part_id) {
                match &part.parent {
                    Some(parent_name) => current_name = parent_name,
                    None => break, // Reached root
                }
            }
        }

        chain
    }

    /// Get all descendants of a part (recursive)
    pub fn get_all_descendants(&self, parent_name: &str) -> Vec<&BlockPart> {
        let parent_id = match self.name_to_id.get(parent_name) {
            Some(id) => *id,
            None => return Vec::new(),
        };

        let mut descendants = Vec::new();
        self.collect_descendants_recursive(parent_id, &mut descendants);
        descendants
    }

    /// Sever at a part - remove part and all descendants, return them as new body
    pub fn sever_at(&mut self, part_name: &str) -> Result<BlockBody, BlockBodyError> {
        let mut removed_parts = self.remove_part(part_name)?;

        // Fix parent references: the severed part becomes a root
        if let Some(severed_root) = removed_parts.iter_mut().find(|p| p.name == part_name) {
            severed_root.parent = None; // Make it a root in the new body
        }

        // Create new body from removed parts
        let mut new_body = BlockBody::new();
        for part in removed_parts {
            new_body.add_part(part)?;
        }

        Ok(new_body)
    }

    /// Attach another body to this one at specified attachment point
    pub fn attach_body(
        &mut self,
        mut other: BlockBody,
        attach_to: &str,
        mount_point: Option<&str>,
    ) -> Result<(), BlockBodyError> {
        // Verify attachment point exists
        if !self.name_to_id.contains_key(attach_to) {
            return Err(BlockBodyError::PartNotFound(attach_to.to_string()));
        }

        // Find the attachment root in other body
        let attachment_root = if let Some(mount) = mount_point {
            mount.to_string()
        } else {
            // Use first root if no mount point specified
            other
                .get_roots()
                .first()
                .ok_or_else(|| BlockBodyError::PartNotFound("root".to_string()))?
                .name
                .clone()
        };

        // Update parent of attachment root
        if let Some(root_part) = other.get_part_by_name_mut(&attachment_root) {
            root_part.parent = Some(attach_to.to_string());
        }

        // Add all parts from other body
        for (_, part) in other.parts {
            self.add_part(part)?;
        }

        Ok(())
    }

    /// Get all root parts
    pub fn get_roots(&self) -> Vec<&BlockPart> {
        self.roots
            .iter()
            .filter_map(|id| self.parts.get(*id))
            .collect()
    }

    /// Validate the body structure (check for cycles, orphans, etc.)
    pub fn validate(&self) -> Result<(), BlockBodyError> {
        // Check that all root IDs exist
        for root_id in &self.roots {
            if !self.parts.contains_key(*root_id) {
                return Err(BlockBodyError::PartNotFound("root".to_string()));
            }
        }

        // Check that all parent references are valid
        for part in self.parts.values() {
            if let Some(parent_name) = &part.parent {
                if !self.name_to_id.contains_key(parent_name) {
                    return Err(BlockBodyError::ParentNotFound {
                        parent: parent_name.clone(),
                        child: part.name.clone(),
                    });
                }
            }
        }

        // TODO: Check for cycles in the hierarchy

        Ok(())
    }

    // Private helper methods

    /// Recursively collect all descendants of a part
    fn collect_descendants_recursive<'a>(
        &'a self,
        parent_id: PartId,
        descendants: &mut Vec<&'a BlockPart>,
    ) {
        if let Some(children_ids) = self.children.get(&parent_id) {
            for &child_id in children_ids {
                if let Some(child_part) = self.parts.get(child_id) {
                    descendants.push(child_part);
                    self.collect_descendants_recursive(child_id, descendants);
                }
            }
        }
    }

    /// Collect part and all its descendants (for removal)
    fn collect_descendants(&self, part_id: PartId, to_remove: &mut Vec<PartId>) {
        to_remove.push(part_id);

        if let Some(children_ids) = self.children.get(&part_id) {
            for &child_id in children_ids {
                self.collect_descendants(child_id, to_remove);
            }
        }
    }
}

impl Default for BlockBody {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlockPart, Vec3};

    #[test]
    fn test_empty_body() {
        let body = BlockBody::new();
        assert_eq!(body.parts.len(), 0);
        assert_eq!(body.roots.len(), 0);
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_add_root_part() {
        let mut body = BlockBody::new();
        let part = BlockPart::new("torso", Vec3::ZERO, Vec3::ONE);

        let id = body.add_part(part).unwrap();

        assert_eq!(body.parts.len(), 1);
        assert_eq!(body.roots.len(), 1);
        assert_eq!(body.roots[0], id);
        assert!(body.get_part_by_name("torso").is_some());
    }

    #[test]
    fn test_add_child_part() {
        let mut body = BlockBody::new();

        // Add parent first
        let torso = BlockPart::new("torso", Vec3::ZERO, Vec3::ONE);
        body.add_part(torso).unwrap();

        // Add child
        let head =
            BlockPart::with_parent("head", Vec3::new(0.0, 1.0, 0.0), Vec3::splat(0.5), "torso");
        let head_id = body.add_part(head).unwrap();

        assert_eq!(body.parts.len(), 2);
        assert_eq!(body.roots.len(), 1); // Only torso is root

        let head_part = body.get_part(head_id).unwrap();
        assert_eq!(head_part.parent, Some("torso".to_string()));

        let children = body.get_children("torso");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "head");
    }

    #[test]
    fn test_duplicate_name_error() {
        let mut body = BlockBody::new();

        let part1 = BlockPart::new("torso", Vec3::ZERO, Vec3::ONE);
        body.add_part(part1).unwrap();

        let part2 = BlockPart::new("torso", Vec3::new(1.0, 0.0, 0.0), Vec3::ONE);
        let result = body.add_part(part2);

        assert!(matches!(result, Err(BlockBodyError::PartAlreadyExists(_))));
    }

    #[test]
    fn test_missing_parent_error() {
        let mut body = BlockBody::new();

        let head = BlockPart::with_parent("head", Vec3::ZERO, Vec3::ONE, "nonexistent");
        let result = body.add_part(head);

        assert!(matches!(result, Err(BlockBodyError::ParentNotFound { .. })));
    }

    #[test]
    fn test_empty_name_error() {
        let mut body = BlockBody::new();

        let part = BlockPart::new("", Vec3::ZERO, Vec3::ONE);
        let result = body.add_part(part);

        assert!(matches!(result, Err(BlockBodyError::InvalidPartName(_))));
    }

    #[test]
    fn test_children_cache() {
        let mut body = BlockBody::new();

        // Build hierarchy: torso -> (head, arm)
        let torso = BlockPart::new("torso", Vec3::ZERO, Vec3::ONE);
        body.add_part(torso).unwrap();

        let head =
            BlockPart::with_parent("head", Vec3::new(0.0, 1.0, 0.0), Vec3::splat(0.5), "torso");
        body.add_part(head).unwrap();

        let arm =
            BlockPart::with_parent("arm", Vec3::new(1.0, 0.0, 0.0), Vec3::splat(0.8), "torso");
        body.add_part(arm).unwrap();

        // Test O(1) children lookup
        let children = body.get_children("torso");
        assert_eq!(children.len(), 2);

        let child_names: Vec<&str> = children.iter().map(|p| p.name.as_str()).collect();
        assert!(child_names.contains(&"head"));
        assert!(child_names.contains(&"arm"));
    }

    #[test]
    fn test_chain_to_root() {
        let mut body = BlockBody::new();

        // Build chain: torso -> shoulder -> arm -> hand
        body.add_part(BlockPart::new("torso", Vec3::ZERO, Vec3::ONE))
            .unwrap();
        body.add_part(BlockPart::with_parent(
            "shoulder",
            Vec3::X,
            Vec3::ONE,
            "torso",
        ))
        .unwrap();
        body.add_part(BlockPart::with_parent(
            "arm",
            Vec3::X,
            Vec3::ONE,
            "shoulder",
        ))
        .unwrap();
        body.add_part(BlockPart::with_parent("hand", Vec3::X, Vec3::ONE, "arm"))
            .unwrap();

        let chain = body.get_chain_to_root("hand");
        let names: Vec<&str> = chain.iter().map(|p| p.name.as_str()).collect();

        assert_eq!(names, vec!["hand", "arm", "shoulder", "torso"]);
    }

    #[test]
    fn test_sever_operation() {
        let mut body = BlockBody::new();

        // Build hierarchy
        body.add_part(BlockPart::new("torso", Vec3::ZERO, Vec3::ONE))
            .unwrap();
        body.add_part(BlockPart::with_parent("arm", Vec3::X, Vec3::ONE, "torso"))
            .unwrap();
        body.add_part(BlockPart::with_parent("hand", Vec3::X, Vec3::ONE, "arm"))
            .unwrap();
        body.add_part(BlockPart::with_parent("head", Vec3::Y, Vec3::ONE, "torso"))
            .unwrap();

        // Sever arm (should take hand with it)
        let severed_body = body.sever_at("arm").unwrap();

        // Original body should only have torso and head
        assert_eq!(body.parts.len(), 2);
        assert!(body.get_part_by_name("torso").is_some());
        assert!(body.get_part_by_name("head").is_some());
        assert!(body.get_part_by_name("arm").is_none());
        assert!(body.get_part_by_name("hand").is_none());

        // Severed body should have arm and hand
        assert_eq!(severed_body.parts.len(), 2);
        assert!(severed_body.get_part_by_name("arm").is_some());
        assert!(severed_body.get_part_by_name("hand").is_some());

        // Arm should now be a root in severed body
        let roots = severed_body.get_roots();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].name, "arm");
    }

    #[test]
    fn test_attach_bodies() {
        let mut main_body = BlockBody::new();
        main_body
            .add_part(BlockPart::new("torso", Vec3::ZERO, Vec3::ONE))
            .unwrap();

        let mut weapon_body = BlockBody::new();
        weapon_body
            .add_part(BlockPart::new(
                "sword",
                Vec3::ZERO,
                Vec3::new(0.1, 1.0, 0.1),
            ))
            .unwrap();

        // Attach weapon to torso
        main_body
            .attach_body(weapon_body, "torso", Some("sword"))
            .unwrap();

        // Should now have both parts
        assert_eq!(main_body.parts.len(), 2);
        assert!(main_body.get_part_by_name("torso").is_some());
        assert!(main_body.get_part_by_name("sword").is_some());

        // Sword should be child of torso
        let sword = main_body.get_part_by_name("sword").unwrap();
        assert_eq!(sword.parent, Some("torso".to_string()));

        // Torso should have sword as child
        let children = main_body.get_children("torso");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "sword");
    }
}
