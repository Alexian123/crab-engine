use crate::GfxContext;
use crate::gfx::buffers::TextureObject;
use crate::renderer::{Framebuffer, FramebufferBuilder, Mesh};
use std::rc::Rc;

pub struct StageRenderer {
    gfx: Rc<GfxContext>,
    framebuffer: Framebuffer,
}

impl StageRenderer {
    pub fn new(gfx: Rc<GfxContext>, width: u32, height: u32) -> Result<Self, String> {
        let fb = FramebufferBuilder::new(Rc::clone(&gfx), width, height)?
            .with_color_texture()?
            .build()?;
        Ok(Self {
            gfx,
            framebuffer: fb,
        })
    }

    pub fn render(&self, screen_quad: &Mesh) {
        self.framebuffer.bind();
        self.gfx.clear(GfxContext::COLOR_BUFFER_BIT);
        screen_quad.draw();
        self.framebuffer.unbind();
    }

    pub fn get_output_texture(&self) -> &TextureObject {
        self.framebuffer
            .color_texture()
            .expect("No color texture available in FBO")
    }
}
