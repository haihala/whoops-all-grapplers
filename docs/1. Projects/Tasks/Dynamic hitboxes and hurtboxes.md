# 2022-11-21 update:
- You can find parts of the model and attach hitboxes to those parts
- Maybe you could do this for hurboxes as well
	- Don't need any fancy dynamic things
	- Expand `Area` to a multi-collider system
	- Capsules and Rectangles. Circle is a capsule with both foci in the same spot

- [bevy-scene-hook](https://github.com/nicopap/bevy-scene-hook "https://github.com/nicopap/bevy-scene-hook")
- use a naming scheme 
	- With another mesh attached to the same skeleton
- gltfextras

Here's a handy function:
```rs
pub fn get_verts_indices(mesh: &Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        None => panic!("Mesh does not contain vertex positions"),
        Some(vertex_values) => match &vertex_values {
            VertexAttributeValues::Float32x3(positions) => positions
                .iter()
                .map(|[x, y, z]| Vec3::new(*x, *y, *z))
                .collect(),
            _ => panic!("Unexpected types in {:?}", Mesh::ATTRIBUTE_POSITION),
        },
    };
    
    let indices = match mesh.indices().unwrap() {
        Indices::U16(_) => {
            panic!("expected u32 indices");
        }
        Indices::U32(indices) => indices
            .chunks(3)
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect(),
    };
    (vertices, indices)
}
```

And with rapier 3d:
```rs
let (vertices, indices) = get_verts_indices(meshes.get(mesh).unwrap());
cmds.entity(child.id())
    .insert(bevy_rapier3d::prelude::Collider::trimesh(vertices, indices));
```

