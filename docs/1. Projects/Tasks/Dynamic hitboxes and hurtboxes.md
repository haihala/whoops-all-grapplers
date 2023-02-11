# 2023-02-11 implementation starts
Needs:
- Visualization
- Not setting hit and hurtboxes separately for each frame of each move

Wants:
- Fold to 2D so that you can't mystery dodge by going to the background/foreground

## Implementation: Joint anchors
- On load
	- [x] Initialize Joints component
		- Will get a hold of every joint entity on the model
		- Can be used later to anchor spawning stuff at the Joints
	- [ ] Initialize JointColliders component
		- Will take in descriptions on what joints form boxes
			- Vec of joints => one axis aligned collider
				- Based on min and max x and y
	- [ ] Create an empty node to hold the colliders
	- [ ] Spawn collider sprites in that with
		- Area
		- Marker component
		- Sprite
- Each tick
	- [ ] Check joint positions and update areas
	- [ ] Update sprites based on areas
	- [ ] Check collision based on areas, get a link to the main

### Upgrade (later)
- Include a thickness
- Instead of an axis aligned box, have a diagonal rectangle from one focus to the other with the given thickness.

# Previous notes on the subject
## Rejected implementations
### Blender hitboxes
- Define hitboxes and hurtboxes in blender

Problems:
- Blender is configured to only export visible models, can you export invisible things?

# 2022-11-21 update:
- You can find parts of the model and attach hitboxes to those parts
- Maybe you could do this for hurboxes as well
	- Don't need any fancy dynamic things
	- Expand `Area` to a multi-collider system
	- Capsules and Rectangles. Circle is a capsule with both foci in the same spot

- [bevy-scene-hook](https://github.com/nicopap/bevy-scene-hook "https://github.com/nicopap/bevy-scene-hook") - Way too green to be used
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

