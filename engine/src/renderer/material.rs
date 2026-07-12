pub use crate::renderer::ShaderProgram;
pub use crate::renderer::Texture;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Material {
    shader: Rc<ShaderProgram>,
    textures: Vec<(String, Rc<Texture>)>,
    float_params: HashMap<String, f32>,
    float2_params: HashMap<String, glam::Vec2>,
    float3_params: HashMap<String, glam::Vec3>,
    float4_params: HashMap<String, glam::Vec4>,
}

impl Material {
    pub fn new(shader: Rc<ShaderProgram>) -> Self {
        Self {
            shader,
            textures: Vec::new(),
            float_params: HashMap::new(),
            float2_params: HashMap::new(),
            float3_params: HashMap::new(),
            float4_params: HashMap::new(),
        }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.shader
    }

    pub fn set_texture(&mut self, name: &str, texture: Rc<Texture>) {
        self.textures.push((name.to_string(), texture));
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        self.float_params.insert(name.to_string(), value);
    }

    pub fn set_float2(&mut self, name: &str, value: glam::Vec2) {
        self.float2_params.insert(name.to_string(), value);
    }

    pub fn set_float3(&mut self, name: &str, value: glam::Vec3) {
        self.float3_params.insert(name.to_string(), value);
    }

    pub fn set_float4(&mut self, name: &str, value: glam::Vec4) {
        self.float4_params.insert(name.to_string(), value);
    }

    pub fn bind(&self) {
        self.shader.bind();

        for (name, param) in &self.float_params {
            self.shader.set_uniform(name, param);
        }

        for (name, value) in &self.float2_params {
            self.shader.set_uniform(name, value);
        }

        for (name, value) in &self.float3_params {
            self.shader.set_uniform(name, value);
        }

        for (name, value) in &self.float4_params {
            self.shader.set_uniform(name, value);
        }

        for (index, (name, texture)) in self.textures.iter().enumerate() {
            self.shader.set_texture(name, texture, index as u32);
        }
    }
}
