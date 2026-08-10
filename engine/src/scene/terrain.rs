use super::{
    Entity, LocalTransformComponent, MaterialComponent, MeshComponent, TerrainTileComponent, World,
};
use crate::loader::Loader;
use crate::renderer::Material;
use crate::utils::HeightGenerator;
use std::rc::Rc;

pub fn get_height_at_point(x: f32, z: f32, world: &World) -> f32 {
    for (_, tile) in world.query::<TerrainTileComponent>() {
        let tile_min_x = tile.grid_x as f32 * tile.tile_size as f32;
        let tile_min_z = tile.grid_z as f32 * tile.tile_size as f32;
        let tile_max_x = tile_min_x + tile.tile_size as f32;
        let tile_max_z = tile_min_z + tile.tile_size as f32;

        if x < tile_min_x || x >= tile_max_x || z < tile_min_z || z >= tile_max_z {
            continue;
        }

        if tile.height_map.is_empty() {
            return 0.0;
        }

        let local_x = x - tile_min_x;
        let local_z = z - tile_min_z;

        let res = tile.vertices_per_side - 1;
        let fx = (local_x / tile.tile_size as f32) * res as f32;
        let fz = (local_z / tile.tile_size as f32) * res as f32;

        let x0 = (fx.floor() as usize).min(res.saturating_sub(1));
        let z0 = (fz.floor() as usize).min(res.saturating_sub(1));
        let x1 = (x0 + 1).min(res);
        let z1 = (z0 + 1).min(res);

        let tx = fx - x0 as f32;
        let tz = fz - z0 as f32;

        let vps = tile.vertices_per_side;
        let h00 = tile.height_map[z0 * vps + x0];
        let h10 = tile.height_map[z0 * vps + x1];
        let h01 = tile.height_map[z1 * vps + x0];
        let h11 = tile.height_map[z1 * vps + x1];

        let h0 = h00 * (1.0 - tx) + h10 * tx;
        let h1 = h01 * (1.0 - tx) + h11 * tx;
        return h0 * (1.0 - tz) + h1 * tz;
    }
    0.0
}

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
            let entity = create_terrain_tile(
                world,
                grid_x,
                grid_z,
                y_offset,
                tile_size,
                vertices_per_side,
            );
            let height_generator = HeightGenerator::new(seed, grid_x, grid_z, vertices_per_side);
            let (mesh, height_map) = loader
                .load_terrain_mesh(
                    tile_size,
                    vertices_per_side,
                    uv_scale,
                    Some(&height_generator),
                )
                .unwrap();
            if let Some(terrain_tile) = world.get_component_mut::<TerrainTileComponent>(entity) {
                terrain_tile.height_map = height_map;
            }
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
    tile_size: usize,
    vertices_per_side: usize,
) -> Entity {
    let entity = world.create_entity();
    world.add_component(
        entity,
        TerrainTileComponent {
            grid_x,
            grid_z,
            tile_size,
            vertices_per_side,
            height_map: Vec::new(),
        },
    );
    world.add_component(
        entity,
        LocalTransformComponent {
            position: glam::Vec3::new(
                grid_x as f32 * tile_size as f32,
                y_offset,
                grid_z as f32 * tile_size as f32,
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
