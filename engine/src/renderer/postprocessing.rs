mod stage_renderer;

use std::rc::Rc;

pub use crate::GfxContext;
pub use crate::gfx::buffers::{TextureObject, TextureTarget};
pub use crate::renderer::{Mesh, ShaderProgram};
pub use stage_renderer::StageRenderer;

pub trait PostProcessingStage {
    fn run(&self, texture: &TextureObject, screen_quad: &Mesh) -> &TextureObject;
}

pub struct PostProcessingPipeline {
    gfx: Rc<GfxContext>,
    screen_quad: Rc<Mesh>,
    screen_shader: Rc<ShaderProgram>,
    pub(super) stages: Vec<Box<dyn PostProcessingStage>>,
}

impl PostProcessingPipeline {
    pub fn new(
        gfx: Rc<GfxContext>,
        screen_quad: Rc<Mesh>,
        screen_shader: Rc<ShaderProgram>,
    ) -> Self {
        Self {
            gfx,
            screen_quad,
            screen_shader,
            stages: Vec::new(),
        }
    }

    pub fn run(&self, color_texture: &TextureObject) {
        self.begin();

        // render each stage
        let mut current_texture = color_texture;
        for stage in &self.stages {
            current_texture = stage.run(current_texture, &self.screen_quad);
        }

        // present to screen
        self.present(current_texture);

        self.end();
    }

    fn begin(&self) {
        self.screen_quad.bind();
        self.gfx.set_depth_test(false);
    }

    fn end(&self) {
        self.gfx.set_depth_test(true);
        self.screen_quad.unbind();
    }

    fn present(&self, output_texture: &TextureObject) {
        self.screen_shader.bind();
        self.gfx.set_active_texture(0);
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(output_texture));
        self.gfx.clear(GfxContext::COLOR_BUFFER_BIT);
        self.screen_quad.draw();
        self.screen_shader.unbind();
    }
}
