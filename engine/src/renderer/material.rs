pub use crate::renderer::ShaderProgram;
pub use crate::renderer::Texture;
use std::rc::Rc;

pub struct Material {
    shader: Rc<ShaderProgram>,
    pub textures: Vec<Rc<Texture>>,
    pub shininess: f32,
    pub diffuse_index: Option<u32>,
    pub specular_index: Option<u32>,
    pub emission_index: Option<u32>,
}

impl Material {
    pub const MAX_TEXTURES: usize = 16;

    pub fn new(shader: Rc<ShaderProgram>) -> Self {
        Self {
            shader,
            textures: Vec::with_capacity(16),
            shininess: 32.0,
            diffuse_index: None,
            specular_index: None,
            emission_index: None,
        }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.shader
    }

    pub fn bind(&self) {
        self.shader.bind();

        for i in 0..self.textures.len().min(Self::MAX_TEXTURES) {
            let texture = &self.textures[i];
            texture.bind(i as u32);
            self.shader
                .set_uniform(format!("uTextures[{}]", i).as_str(), &(i as i32)); // MUST BE i32 for Sampler2D, u32 will not work
        }

        let index_mask: u32 = (self.emission_index.unwrap_or(0) << 8)
            | (self.specular_index.unwrap_or(0) << 4)
            | self.diffuse_index.unwrap_or(0);

        let use_mask: u32 = ((self.emission_index.is_some() as u32) << 2)
            | ((self.specular_index.is_some() as u32) << 1)
            | (self.diffuse_index.is_some() as u32);

        self.shader.set_uniform("uMaterial.indexMask", &index_mask);
        self.shader.set_uniform("uMaterial.useMask", &use_mask);
        self.shader
            .set_uniform("uMaterial.shininess", &self.shininess);
    }
}
