pub mod camera;
pub mod components;
pub mod ecs;
pub mod lights;

pub use camera::{Camera, FlyCamera};
pub use components::*;
pub use ecs::{Component, Entity, World};
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

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
