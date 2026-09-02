use engine::renderer::postprocessing::*;

use std::rc::Rc;

pub struct ContrastChanger {
    gfx: Rc<GfxContext>,
    shader: Rc<ShaderProgram>,
    renderer: StageRenderer,
}

impl ContrastChanger {
    pub fn new(
        gfx: Rc<GfxContext>,
        width: u32,
        height: u32,
        shader: Rc<ShaderProgram>,
        contrast_value: f32,
    ) -> Self {
        let renderer = StageRenderer::new(Rc::clone(&gfx), width, height)
            .expect("Failed to create stage renderer");
        shader.bind();
        shader.set_uniform("uColorTexture", &(0 as i32));
        shader.set_uniform("uContrast", &contrast_value);
        shader.unbind();
        Self {
            gfx,
            shader,
            renderer,
        }
    }
}

impl PostProcessingStage for ContrastChanger {
    fn run(&self, texture: &TextureObject, screen_quad: &Mesh) -> &TextureObject {
        self.shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(texture));
        self.renderer.render(screen_quad);
        self.shader.unbind();
        self.renderer.get_output_texture()
    }
}
