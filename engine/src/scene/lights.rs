use glam::Vec3;

pub struct LightColor {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

pub struct DirectionalLight {
    pub color: LightColor,
    pub direction: Vec3,
}

pub struct PointLight {
    pub color: LightColor,
    pub position: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

pub struct SpotLight {
    pub pl: PointLight,
    pub direction: Vec3,
    pub cutoff: f32,
    pub outer_cutoff: f32,
}
