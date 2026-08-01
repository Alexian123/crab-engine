pub mod fly_cam;

pub use fly_cam::FlyCamera;
use glam::{Mat4, Vec3};

pub trait Camera {
    fn view(&self) -> Mat4;
    fn projection(&self) -> Mat4;
    fn position(&self) -> Vec3;
}
