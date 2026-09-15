use super::*;
use crate::loader::*;
use std::rc::Rc;

pub struct HBlur {
    gfx: Rc<GfxContext>,
    shader: Rc<ShaderProgram>,
}

impl HBlur {
    pub fn new(
        gfx: Rc<GfxContext>,
        width: u32,
        loader: &mut Loader,
        down_scale_factor: f32,
    ) -> Self {
        let shader = loader
            .load_shader_embedded(&DEFAULT_HBLUR_SHADER)
            .expect("Failed to load HBlur shader");
        shader.bind();
        shader.set_uniform("uColorTexture", &(0 as i32));
        shader.set_uniform("uTargetWidth", &((width as f32) / down_scale_factor));
        shader.unbind();

        Self { gfx, shader }
    }
}

impl PostProcessingStage for HBlur {
    fn bind(&self, input_texture: &TextureObject) {
        self.shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(input_texture));
    }
}

pub struct VBlur {
    gfx: Rc<GfxContext>,
    shader: Rc<ShaderProgram>,
}

impl VBlur {
    pub fn new(
        gfx: Rc<GfxContext>,
        height: u32,
        loader: &mut Loader,
        down_scale_factor: f32,
    ) -> Self {
        let shader = loader
            .load_shader_embedded(&DEFAULT_VBLUR_SHADER)
            .expect("Failed to load VBlur shader");
        shader.bind();
        shader.set_uniform("uColorTexture", &(0 as i32));
        shader.set_uniform("uTargetHeight", &((height as f32) / down_scale_factor));
        shader.unbind();

        Self { gfx, shader }
    }
}

impl PostProcessingStage for VBlur {
    fn bind(&self, input_texture: &TextureObject) {
        self.shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(input_texture));
    }
}
