use crate::renderer::WorldTransformComponent;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuTransformData {
    model: [[f32; 4]; 4],
    normal: [[f32; 4]; 4],
}

impl GpuTransformData {
    pub fn from_world_transform(comp: &WorldTransformComponent) -> Self {
        Self {
            model: comp.model_matrix.to_cols_array_2d(),
            normal: comp.normal_matrix().to_cols_array_2d(),
        }
    }
}
