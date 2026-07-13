use crate::renderer::Material;
use crate::renderer::Mesh;
use glam::*;
use std::rc::Rc;

pub struct Object {
    pub mesh: Option<Rc<Mesh>>,
    pub material: Option<Rc<Material>>,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Object {
    pub fn new(
        mesh: Option<Rc<Mesh>>,
        material: Option<Rc<Material>>,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Self {
        Self {
            mesh,
            material,
            position,
            rotation,
            scale,
        }
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn normal_matrix(&self) -> Mat4 {
        let model = self.model_matrix();
        Mat4::from_mat3(Mat3::from_mat4(model.inverse().transpose()))
    }
}
