use crate::scene::{DirectionalLight, LightColor, LightingComponent, PointLight, SpotLight};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuLightColor {
    ambient: [f32; 3],
    _pad0: f32,

    diffuse: [f32; 3],
    _pad1: f32,

    specular: [f32; 3],
    _pad2: f32,
}

impl GpuLightColor {
    pub fn from_light_color(color: LightColor) -> Self {
        Self {
            ambient: color.ambient.to_array(),
            _pad0: 0.0,
            diffuse: color.diffuse.to_array(),
            _pad1: 0.0,
            specular: color.specular.to_array(),
            _pad2: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuDirLight {
    color: GpuLightColor,
    direction: [f32; 3],
    _pad0: f32,
}

impl GpuDirLight {
    pub fn from_dir_light(dir_light: DirectionalLight) -> Self {
        Self {
            color: GpuLightColor::from_light_color(dir_light.color),
            direction: dir_light.direction.to_array(),
            _pad0: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuPointLight {
    color: GpuLightColor,

    position: [f32; 3],
    constant: f32,

    linear: f32,
    quadratic: f32,
    _pad0: f32,
    _pad1: f32,
}

impl GpuPointLight {
    pub fn from_point_light(point_light: PointLight) -> Self {
        Self {
            color: GpuLightColor::from_light_color(point_light.color),
            position: point_light.position.to_array(),
            constant: point_light.constant,
            linear: point_light.linear,
            quadratic: point_light.quadratic,
            _pad0: 0.0,
            _pad1: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuSpotLight {
    pl: GpuPointLight,

    direction: [f32; 3],
    cutoff: f32,

    outer_cutoff: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
}

impl GpuSpotLight {
    pub fn from_spot_light(spot_light: SpotLight) -> Self {
        Self {
            pl: GpuPointLight::from_point_light(spot_light.pl),
            direction: spot_light.direction.to_array(),
            cutoff: spot_light.cutoff,
            outer_cutoff: spot_light.outer_cutoff,
            _pad0: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        }
    }
}

pub const MAX_NUM_DIR_LIGHTS: usize = 4;
pub const MAX_NUM_POINT_LIGHTS: usize = 8;
pub const MAX_NUM_SPOT_LIGHTS: usize = 2;

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuLightingData {
    dir_lights: [GpuDirLight; MAX_NUM_DIR_LIGHTS],
    point_lights: [GpuPointLight; MAX_NUM_POINT_LIGHTS],
    spot_lights: [GpuSpotLight; MAX_NUM_SPOT_LIGHTS],
    num_lights_mask: u32,
    _pad0: [u32; 3],
}

impl GpuLightingData {
    pub fn from_lighting_component(lighting: &LightingComponent) -> Self {
        let mut data = Self::default();

        for (i, light) in lighting
            .directional_lights
            .iter()
            .take(MAX_NUM_DIR_LIGHTS)
            .enumerate()
        {
            data.dir_lights[i] = GpuDirLight::from_dir_light(*light);
        }

        for (i, light) in lighting
            .point_lights
            .iter()
            .take(MAX_NUM_POINT_LIGHTS)
            .enumerate()
        {
            data.point_lights[i] = GpuPointLight::from_point_light(*light);
        }

        for (i, light) in lighting
            .spot_lights
            .iter()
            .take(MAX_NUM_SPOT_LIGHTS)
            .enumerate()
        {
            data.spot_lights[i] = GpuSpotLight::from_spot_light(*light);
        }

        data.num_lights_mask = lighting.lights_mask;

        data
    }
}
