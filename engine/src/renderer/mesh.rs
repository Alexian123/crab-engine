use crate::GfxContext;
use crate::gfx::buffers::{
    VertexArrayObject, VertexBufferDataUsage, VertexBufferObject, VertexBufferTarget,
};
use crate::gfx::{DrawMode, vertex::*};
use std::rc::Rc;

pub struct Mesh {
    gfx: Rc<GfxContext>,
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: Option<VertexBufferObject>,
    vertex_count: usize,
    index_count: usize,
}

impl Mesh {
    pub fn new(
        gfx: Rc<GfxContext>,
        vertices: &[f32],
        indices: &[u32],
        layout: VertexLayout,
    ) -> Result<Self, String> {
        // Create VAO, VBO, and EBO
        let vao = gfx.create_vao()?;
        let vbo = gfx.create_buffer()?;
        let ebo = if !indices.is_empty() {
            Some(gfx.create_buffer()?)
        } else {
            None
        };

        // Bind VBO and EBO to VAO
        gfx.bind_vao(Some(&vao));
        gfx.bind_buffer(VertexBufferTarget::Array, Some(&vbo));
        gfx.set_buffer_data_u8(
            VertexBufferTarget::Array,
            VertexBufferDataUsage::StaticDraw,
            bytemuck::cast_slice(vertices),
        );
        if let Some(ebo) = &ebo {
            gfx.bind_buffer(VertexBufferTarget::Element, Some(ebo));
            gfx.set_buffer_data_u8(
                VertexBufferTarget::Element,
                VertexBufferDataUsage::StaticDraw,
                bytemuck::cast_slice(indices),
            );
        }

        // Set vertex attribute pointers
        let stride = layout.stride();
        for attrib in &layout.attribs {
            gfx.enable_vertex_attrib_array(attrib.location);
            gfx.set_vertex_attrib_pointer(attrib, stride);
        }

        Ok(Self {
            gfx,
            vao,
            vbo,
            ebo,
            vertex_count: vertices.len(),
            index_count: indices.len(),
        })
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn index_count(&self) -> usize {
        self.index_count
    }

    pub fn bind(&self) {
        self.gfx.bind_vao(Some(&self.vao));
    }

    pub fn unbind(&self) {
        self.gfx.bind_vao(None);
    }

    pub fn draw(&self) {
        if self.index_count > 0 {
            self.gfx
                .draw_elements(DrawMode::Triangles, self.index_count as i32, 0);
        } else {
            self.gfx
                .draw_arrays(DrawMode::Triangles, 0, self.vertex_count as i32);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        self.gfx.delete_buffer(&self.vbo);
        if let Some(ebo) = &self.ebo {
            self.gfx.delete_buffer(ebo);
        }
        self.gfx.delete_vao(&self.vao);
    }
}
