pub use super::vertex::*;
use glow::HasContext;
use std::rc::Rc;

pub struct Mesh {
    gl: Rc<glow::Context>,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: Option<glow::Buffer>,
    vertex_count: usize,
    index_count: usize,
}

impl Mesh {
    pub fn new(
        gl: Rc<glow::Context>,
        vertices: &[f32],
        indices: &[u32],
        layout: &VertexLayout,
    ) -> Result<Self, String> {
        // Create VAO, VBO, and EBO
        let vao = unsafe { gl.create_vertex_array()? };
        let vbo = unsafe { gl.create_buffer()? };
        let ebo = if !indices.is_empty() {
            Some(unsafe { gl.create_buffer()? })
        } else {
            None
        };

        // Bind VBO and EBO to VAO
        unsafe {
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            if let Some(ebo) = ebo {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    bytemuck::cast_slice(indices),
                    glow::STATIC_DRAW,
                );
            }
        }

        // Set vertex attribute pointers
        let stride = layout.stride();
        for attrib in &layout.attribs {
            unsafe {
                gl.enable_vertex_attrib_array(attrib.location);
                gl.vertex_attrib_pointer_f32(
                    attrib.location,
                    attrib.count as i32,
                    attrib.format.gl_type(),
                    attrib.normalized,
                    stride as i32,
                    attrib.offset as i32,
                );
            };
        }

        Ok(Self {
            gl,
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
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }

    pub fn draw(&self) {
        unsafe {
            if self.index_count > 0 {
                self.gl.draw_elements(
                    glow::TRIANGLES,
                    self.index_count as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            } else {
                self.gl
                    .draw_arrays(glow::TRIANGLES, 0, self.vertex_count as i32);
            }
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo);
            if let Some(ebo) = self.ebo {
                self.gl.delete_buffer(ebo);
            }
            self.gl.delete_vertex_array(self.vao);
        }
    }
}
