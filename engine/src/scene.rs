pub mod camera;
pub mod lights;
pub mod object;

pub use crate::scene::camera::Camera;
pub use crate::scene::camera::FlyCamera;
pub use crate::scene::lights::{DirectionalLight, LightColor, PointLight, SpotLight};
pub use crate::scene::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
    pub directional_lights: Vec<DirectionalLight>,
    pub point_lights: Vec<PointLight>,
    pub spot_lights: Vec<SpotLight>,
    pub lights_mask: u32,
}
