use crate::gfx::GfxContext;
use crate::gfx::buffers::{BufferDataUsage, BufferObject, BufferTarget};
use std::rc::Rc;

pub struct Ubo {
    gfx: Rc<GfxContext>,
    buffer: BufferObject,
    size: usize,
}

impl Ubo {
    pub fn new(gfx: Rc<GfxContext>, size: usize, index: u32) -> Result<Self, String> {
        let buffer = gfx.create_buffer()?;
        gfx.bind_buffer(BufferTarget::Uniform, Some(&buffer));
        gfx.set_buffer_data_size(
            BufferTarget::Uniform,
            size as i32,
            BufferDataUsage::StaticDraw,
        );
        gfx.bind_buffer(BufferTarget::Uniform, None);
        gfx.bind_buffer_range(BufferTarget::Uniform, index, Some(&buffer), 0, size as i32);
        Ok(Self { gfx, buffer, size })
    }

    pub fn store(&self, offset: usize, data: &[u8]) {
        self.gfx
            .bind_buffer(BufferTarget::Uniform, Some(&self.buffer));
        self.gfx
            .set_buffer_sub_data_u8(BufferTarget::Uniform, offset as i32, data);
        self.gfx.bind_buffer(BufferTarget::Uniform, None);
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for Ubo {
    fn drop(&mut self) {
        self.gfx.delete_buffer(&self.buffer);
    }
}
