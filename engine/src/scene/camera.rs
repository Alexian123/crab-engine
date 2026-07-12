pub mod fly_cam;

pub use fly_cam::FlyCamera;
use glam::{Mat4, Vec3};

pub trait Camera {
    fn view(&mut self) -> Mat4;
    fn projection(&mut self) -> Mat4;
    fn position(&mut self) -> Vec3;
}
