use crate::renderer::{Material, Mesh};
use crate::scene::*;
use glam::{Mat3, Mat4, Quat, Vec3};
use std::rc::Rc;

pub struct NameComponent(pub String);
impl Component for NameComponent {}

pub struct LocalTransformComponent {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl LocalTransformComponent {
    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

impl Component for LocalTransformComponent {}

pub struct WorldTransformComponent {
    pub model_matrix: Mat4,
}

impl WorldTransformComponent {
    pub fn normal_matrix(&self) -> Mat4 {
        Mat4::from_mat3(Mat3::from_mat4(self.model_matrix.inverse().transpose()))
    }
}

impl Component for WorldTransformComponent {}

pub struct ParentComponent(pub Entity);
impl Component for ParentComponent {}

pub struct ChildrenComponent(pub Vec<Entity>);
impl Component for ChildrenComponent {}

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
