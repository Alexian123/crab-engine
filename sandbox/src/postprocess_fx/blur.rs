use engine::renderer::postprocessing::*;

use std::rc::Rc;

pub struct Blur {
    gfx: Rc<GfxContext>,
    hblur_shader: Rc<ShaderProgram>,
    vblur_shader: Rc<ShaderProgram>,
    hblur_renderer: StageRenderer,
    vblur_renderer: StageRenderer,
}

impl Blur {
    pub fn new(
        gfx: Rc<GfxContext>,
        width: u32,
        height: u32,
        hblur_shader: Rc<ShaderProgram>,
        vblur_shader: Rc<ShaderProgram>,
        down_scale_factor: f32,
    ) -> Self {
        let hblur_renderer = StageRenderer::new(Rc::clone(&gfx), width, height)
            .expect("Failed to create stage renderer");
        let vblur_renderer = StageRenderer::new(Rc::clone(&gfx), width, height)
            .expect("Failed to create stage renderer");

        hblur_shader.bind();
        hblur_shader.set_uniform("uColorTexture", &(0 as i32));
        hblur_shader.set_uniform("uTargetWidth", &((width as f32) / down_scale_factor));
        hblur_shader.unbind();

        vblur_shader.bind();
        vblur_shader.set_uniform("uColorTexture", &(0 as i32));
        vblur_shader.set_uniform("uTargetHeight", &((height as f32) / down_scale_factor));
        vblur_shader.unbind();

        Self {
            gfx,
            hblur_shader,
            vblur_shader,
            hblur_renderer,
            vblur_renderer,
        }
    }
}

impl PostProcessingStage for Blur {
    fn run(&self, texture: &TextureObject, screen_quad: &Mesh) -> &TextureObject {
        // run horizontal blur sub stage
        self.hblur_shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(texture));
        self.hblur_renderer.render(screen_quad);
        self.hblur_shader.unbind();

        // run vertical blur sub stage
        self.vblur_shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx.bind_texture(
            TextureTarget::Texture2D,
            Some(self.hblur_renderer.get_output_texture()),
        );
        self.vblur_renderer.render(screen_quad);
        self.vblur_shader.unbind();

        // final output
        self.vblur_renderer.get_output_texture()
    }
}
