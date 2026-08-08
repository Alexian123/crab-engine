pub mod camera;
pub mod components;
pub mod ecs;
pub mod lights;

pub use camera::{Camera, FlyCamera};
pub use components::*;
pub use ecs::{Component, Entity, World};
use glam::Mat4;
pub use lights::{DirectionalLight, LightColor, PointLight, SpotLight};

pub struct Scene {
    world: World,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::default(),
        }
    }

    pub fn update(&mut self) {
        self.update_world_transforms();
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn set_parent(&mut self, child: Entity, new_parent: Entity) {
        if let Some(&ParentComponent(old_parent)) =
            self.world.get_component::<ParentComponent>(child)
        {
            if let Some(children) = self
                .world
                .get_component_mut::<ChildrenComponent>(old_parent)
            {
                children.0.retain(|&e| e != child);
            }
        }
        self.world.set_component(child, ParentComponent(new_parent));
        match self
            .world
            .get_component_mut::<ChildrenComponent>(new_parent)
        {
            Some(children) => children.0.push(child),
            None => self
                .world
                .add_component(new_parent, ChildrenComponent(vec![child])),
        }
    }

    pub fn clear_parent(&mut self, entity: Entity) {
        if let Some(&ParentComponent(old_parent)) =
            self.world.get_component::<ParentComponent>(entity)
        {
            if let Some(children) = self
                .world
                .get_component_mut::<ChildrenComponent>(old_parent)
            {
                children.0.retain(|&e| e != entity);
            }
        }
        self.world.remove_component::<ParentComponent>(entity);
    }

    pub fn find_entity_by_name(&self, name: &str) -> Option<Entity> {
        self.world
            .query::<NameComponent>()
            .find(|(_, n)| n.0 == name)
            .map(|(entity, _)| entity)
    }

    pub fn create_terrain_tile(&mut self, grid_x: i32, grid_z: i32, size: u32) -> Entity {
        let entity = self.world.create_entity();
        self.world
            .add_component(entity, TerrainTileComponent { grid_x, grid_z });
        self.world.add_component(
            entity,
            LocalTransformComponent {
                position: glam::Vec3::new(
                    grid_x as f32 * size as f32,
                    0.0,
                    grid_z as f32 * size as f32,
                ),
                rotation: glam::Quat::IDENTITY,
                scale: glam::Vec3::ONE,
            },
        );
        entity
    }

    pub fn get_terrain_tile(&self, grid_x: i32, grid_z: i32) -> Option<Entity> {
        for (entity, tile) in self.world.query::<TerrainTileComponent>() {
            if tile.grid_x == grid_x && tile.grid_z == grid_z {
                return Some(entity);
            }
        }
        None
    }

    fn update_world_transforms(&mut self) {
        let roots: Vec<Entity> = self
            .world
            .query::<LocalTransformComponent>()
            .filter(|(entity, _)| !self.world.has_component::<ParentComponent>(*entity))
            .map(|(entity, _)| entity)
            .collect();

        let mut stack: Vec<(Entity, Mat4)> =
            roots.into_iter().map(|e| (e, Mat4::IDENTITY)).collect();
        let mut updates: Vec<(Entity, Mat4)> = Vec::new();

        while let Some((entity, parent_world)) = stack.pop() {
            let local_matrix = self
                .world
                .get_component::<LocalTransformComponent>(entity)
                .map(LocalTransformComponent::model_matrix)
                .unwrap_or(Mat4::IDENTITY);
            let world_matrix = parent_world * local_matrix;
            updates.push((entity, world_matrix));

            if let Some(children) = self.world.get_component::<ChildrenComponent>(entity) {
                for &child in &children.0 {
                    stack.push((child, world_matrix));
                }
            }
        }

        for (entity, matrix) in updates {
            match self
                .world
                .get_component_mut::<WorldTransformComponent>(entity)
            {
                Some(wt) => wt.model_matrix = matrix,
                None => self.world.add_component(
                    entity,
                    WorldTransformComponent {
                        model_matrix: matrix,
                    },
                ),
            }
        }
    }
}
