pub use crate::renderer::ShaderProgram;
pub use crate::renderer::Texture;
use std::rc::Rc;

pub struct Material {
    shader: Rc<ShaderProgram>,
    pub textures: Vec<Rc<Texture>>,
    pub shininess: f32,
    pub diffuse_index: u32,
    pub specular_index: u32,
    pub emission_index: u32,
}

impl Material {
    pub const MAX_TEXTURES: usize = 16;

    pub fn new(shader: Rc<ShaderProgram>) -> Self {
        Self {
            shader,
            textures: Vec::with_capacity(16),
            shininess: 32.0,
            diffuse_index: 0,
            specular_index: 1,
            emission_index: 2,
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

        let index_mask: u32 =
            (self.emission_index << 8) | (self.specular_index << 4) | self.diffuse_index;

        self.shader.set_uniform("uMaterial.indexMask", &index_mask);
        self.shader
            .set_uniform("uMaterial.shininess", &self.shininess);
    }
}
