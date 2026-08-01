use crate::renderer::{Material, Mesh};
use crate::scene::*;
use glam::{Mat3, Mat4, Quat, Vec3};
use std::rc::Rc;

pub struct TransformComponent {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl TransformComponent {
    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn normal_matrix(&self) -> Mat4 {
        let model = self.model_matrix();
        Mat4::from_mat3(Mat3::from_mat4(model.inverse().transpose()))
    }
}

impl Component for TransformComponent {}

pub struct MeshComponent {
    pub mesh: Rc<Mesh>,
}

impl Component for MeshComponent {}

pub struct MaterialComponent {
    pub material: Rc<Material>,
}

impl Component for MaterialComponent {}

pub struct CameraComponent {
    pub position: Vec3,
    pub projection: Mat4,
    pub view: Mat4,
}

impl Component for CameraComponent {}

pub struct LightingComponent {
    pub lights_mask: u32,
    pub directional_lights: Vec<DirectionalLight>,
    pub point_lights: Vec<PointLight>,
    pub spot_lights: Vec<SpotLight>,
}

impl Component for LightingComponent {}
