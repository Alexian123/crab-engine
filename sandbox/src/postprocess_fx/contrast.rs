use engine::renderer::postprocessing::*;

use std::rc::Rc;

pub struct ContrastChanger {
    gfx: Rc<GfxContext>,
    shader: Rc<ShaderProgram>,
}

impl ContrastChanger {
    pub fn new(gfx: Rc<GfxContext>, shader: Rc<ShaderProgram>, contrast_value: f32) -> Self {
        shader.bind();
        shader.set_uniform("uColorTexture", &(0 as i32));
        shader.set_uniform("uContrast", &contrast_value);
        shader.unbind();
        Self { gfx, shader }
    }
}

impl PostProcessingStage for ContrastChanger {
    fn bind(&self, input_texture: &TextureObject) {
        self.shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(input_texture));
    }
}
