use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
};

pub fn merge_cubes(cube_positions: &[Vec3], cube_size: f32) -> Mesh {
    let base = Mesh::from(Cuboid::new(cube_size, cube_size, cube_size));
    let base_positions = base.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
    let base_normals = base.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();
    let base_uvs = base.attribute(Mesh::ATTRIBUTE_UV_0).unwrap();
    let base_indices = base.indices().unwrap();

    let mut positions = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    let mut indices = vec![];

    let mut vertex_offset = 0;

    if let (
        VertexAttributeValues::Float32x3(base_pos),
        VertexAttributeValues::Float32x3(base_norm),
        VertexAttributeValues::Float32x2(base_uv),
        Indices::U32(base_idx),
    ) = (base_positions, base_normals, base_uvs, base_indices.clone())
    {
        for pos in cube_positions {
            positions.extend(
                base_pos
                    .iter()
                    .map(|[x, y, z]| [x + pos.x, y + pos.y, z + pos.z]),
            );
            normals.extend_from_slice(base_norm);
            uvs.extend_from_slice(base_uv);
            indices.extend(base_idx.iter().map(|i| i + vertex_offset));
            vertex_offset += base_pos.len() as u32;
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}
