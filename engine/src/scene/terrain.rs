use super::{
    Entity, LocalTransformComponent, MaterialComponent, MeshComponent, TerrainTileComponent, World,
};
use crate::loader::Loader;
use crate::renderer::Material;
use crate::utils::HeightGenerator;
use std::rc::Rc;

pub fn generate_terrain_grid(
    world: &mut World,
    loader: &mut Loader,
    seed: u64,
    grid_size: usize,
    y_offset: f32,
    tile_size: usize,
    vertices_per_side: usize,
    uv_scale: f32,
    material: Option<Rc<Material>>,
) {
    assert!(tile_size > 0);
    assert!(vertices_per_side > 1);
    assert!(uv_scale != 0.0);

    for grid_x in 0..grid_size {
        for grid_z in 0..grid_size {
            let (grid_x, grid_z) = (grid_x as i32, grid_z as i32);
            let entity = create_terrain_tile(world, grid_x, grid_z, y_offset, tile_size);
            let height_generator = HeightGenerator::new(seed, grid_x, grid_z, vertices_per_side);
            let mesh = loader
                .load_terrain_mesh(
                    tile_size,
                    vertices_per_side,
                    uv_scale,
                    Some(&height_generator),
                )
                .unwrap();
            world.add_component(entity, MeshComponent { mesh });
            if let Some(material) = &material {
                world.add_component(
                    entity,
                    MaterialComponent {
                        material: Rc::clone(&material),
                    },
                );
            }
        }
    }
}

pub fn create_terrain_tile(
    world: &mut World,
    grid_x: i32,
    grid_z: i32,
    y_offset: f32,
    size: usize,
) -> Entity {
    let entity = world.create_entity();
    world.add_component(entity, TerrainTileComponent { grid_x, grid_z });
    world.add_component(
        entity,
        LocalTransformComponent {
            position: glam::Vec3::new(
                grid_x as f32 * size as f32,
                y_offset,
                grid_z as f32 * size as f32,
            ),
            rotation: glam::Quat::IDENTITY,
            scale: glam::Vec3::ONE,
        },
    );
    entity
}

pub fn get_terrain_tile(world: &mut World, grid_x: i32, grid_z: i32) -> Option<Entity> {
    for (entity, tile) in world.query::<TerrainTileComponent>() {
        if tile.grid_x == grid_x && tile.grid_z == grid_z {
            return Some(entity);
        }
    }
    None
}
