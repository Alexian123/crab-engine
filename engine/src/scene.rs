pub mod camera;
pub mod components;
pub mod ecs;
pub mod hierarchy;
pub mod lights;
pub mod terrain;

pub use camera::{Camera, FlyCamera};
pub use components::*;
pub use ecs::{Component, Entity, World};
pub use hierarchy::{clear_parent, set_parent};
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
        hierarchy::update_world_transforms(&mut self.world);
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn find_entity_by_name(&self, name: &str) -> Option<Entity> {
        self.world
            .query::<NameComponent>()
            .find(|(_, n)| n.0 == name)
            .map(|(entity, _)| entity)
    }
}
