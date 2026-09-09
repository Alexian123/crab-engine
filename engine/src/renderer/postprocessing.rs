use std::rc::Rc;

pub use crate::GfxContext;
pub use crate::gfx::buffers::{TextureObject, TextureTarget};
pub use crate::renderer::{Framebuffer, FramebufferBuilder, Mesh, ShaderProgram};

pub trait PostProcessingStage {
    fn bind(&self, input_texture: &TextureObject);
}

pub struct PostProcessingPipeline {
    gfx: Rc<GfxContext>,
    framebuffer_a: Framebuffer,
    framebuffer_b: Framebuffer,
    screen_quad: Rc<Mesh>,
    screen_shader: Rc<ShaderProgram>,
    pub(super) stages: Vec<Box<dyn PostProcessingStage>>,
}

impl PostProcessingPipeline {
    pub fn new(
        gfx: Rc<GfxContext>,
        screen_width: u32,
        screen_height: u32,
        screen_quad: Rc<Mesh>,
        screen_shader: Rc<ShaderProgram>,
    ) -> Result<Self, String> {
        let framebuffer_a = FramebufferBuilder::new(Rc::clone(&gfx), screen_width, screen_height)?
            .with_color_texture()?
            .build()?;
        let framebuffer_b = FramebufferBuilder::new(Rc::clone(&gfx), screen_width, screen_height)?
            .with_color_texture()?
            .build()?;
        Ok(Self {
            gfx,
            framebuffer_a,
            framebuffer_b,
            screen_quad,
            screen_shader,
            stages: Vec::new(),
        })
    }

    pub fn run(&self, color_texture: &TextureObject) {
        self.begin();

        // render each stage
        let mut current_texture = color_texture;
        for (index, stage) in self.stages.iter().enumerate() {
            stage.bind(current_texture);
            let current_framebuffer = if index % 2 == 0 {
                &self.framebuffer_a
            } else {
                &self.framebuffer_b
            };
            self.render_stage(current_framebuffer);
            current_texture = current_framebuffer
                .color_texture()
                .expect("No color texture available in FBO");
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

    fn render_stage(&self, framebuffer: &Framebuffer) {
        framebuffer.bind();
        self.gfx.clear(GfxContext::COLOR_BUFFER_BIT);
        self.screen_quad.draw();
        framebuffer.unbind();
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
