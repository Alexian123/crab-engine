use crate::renderer::Sampler2D;
use glam::{Mat4, Quat, Vec2, Vec3};
use std::rc::Rc;

pub struct HUDElement {
    pub texture: Rc<Sampler2D>,
    pub position: Vec2,
    pub scale: Vec2,
}

impl HUDElement {
    pub fn transform(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.scale.x, self.scale.y, 1.0),
            Quat::IDENTITY,
            Vec3::new(self.position.x, self.position.y, 0.0),
        )
    }
}
