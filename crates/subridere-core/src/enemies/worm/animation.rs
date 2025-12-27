use super::components::WormHead;
use crate::utils::animated_body::AnimatedBody;
use bevy::prelude::*;

pub fn animate_worm_trail(
    worms: Query<&AnimatedBody>,
    heads: Query<&Transform, With<WormHead>>,
    mut transforms: Query<&mut Transform, Without<WormHead>>,
) {
    for animated in &worms {
        // Get head
        let head_entity = match animated.get_entity("Head") {
            Some(e) => e,
            None => continue,
        };
        
        let Ok(head_transform) = heads.get(head_entity) else {
            continue;
        };
        
        let head_pos = head_transform.translation;
        
        // Get descendants
        let descendants = animated.body.get_all_descendants("Head");
        if descendants.is_empty() {
            continue;
        }
        
        let mut previous_pos = head_pos;
        
        // Follow the leader
        for (i, descendant) in descendants.iter().enumerate() {
            let Some(seg_entity) = animated.get_entity(&descendant.name) else {
                continue;
            };
            
            let Ok(mut seg_transform) = transforms.get_mut(seg_entity) else {
                continue;
            };
            
            let current_pos = seg_transform.translation;
            let direction = (previous_pos - current_pos).normalize_or_zero();
            let spacing = 0.8;
            let target_pos = previous_pos - direction * spacing;
            
            seg_transform.translation = current_pos.lerp(target_pos, 0.3);
            
            previous_pos = seg_transform.translation;
        }
    }
}
