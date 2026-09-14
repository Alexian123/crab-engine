use glam::Vec3;

#[derive(Clone, Copy)]
pub struct LightColor {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

#[derive(Clone, Copy)]
pub struct DirectionalLight {
    pub color: LightColor,
    pub direction: Vec3,
}

#[derive(Clone, Copy)]
pub struct PointLight {
    pub color: LightColor,
    pub position: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

#[derive(Clone, Copy)]
pub struct SpotLight {
    pub pl: PointLight,
    pub direction: Vec3,
    pub cutoff: f32,
    pub outer_cutoff: f32,
}
