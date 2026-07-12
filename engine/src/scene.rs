pub mod camera;
pub mod object;

pub use crate::scene::camera::Camera;
pub use crate::scene::camera::FlyCamera;
pub use crate::scene::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
}
