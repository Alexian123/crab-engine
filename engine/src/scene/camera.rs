pub mod fly;
pub mod third_person;

pub use fly::FlyCamera;
use glam::{Mat4, Vec3};
pub use third_person::ThirdPersonCamera;

pub trait Camera {
    fn view(&self) -> Mat4;
    fn projection(&self) -> Mat4;
    fn position(&self) -> Vec3;
    fn up(&self) -> Vec3;
    fn right(&self) -> Vec3;
    fn forward(&self) -> Vec3;
}
