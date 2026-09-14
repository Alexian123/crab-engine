use crate::renderer::Camera;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuCameraData {
    projection: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    view_pos: [f32; 3],
    _pad0: f32,
}

impl GpuCameraData {
    pub fn from_camera(camera: &dyn Camera) -> Self {
        Self {
            projection: camera.projection().to_cols_array_2d(),
            view: camera.view().to_cols_array_2d(),
            view_pos: camera.position().to_array(),
            _pad0: 0.0,
        }
    }
}
