use bevy::prelude::*;
use crate::block_bodies::body::{BlockBody, BlockPart};
use crate::block_bodies::humanoid::make_blocky_humanoid;

pub fn make_jester_body(
    red: Handle<StandardMaterial>,
    gray: Handle<StandardMaterial>,
) -> BlockBody {
    let body = make_blocky_humanoid(red, gray);

    // При желании можно модифицировать части, например:
    // body.parts.iter_mut().for_each(|p| if p.name == "Head" { ... });

    body
}
