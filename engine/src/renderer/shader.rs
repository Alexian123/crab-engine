use crate::GfxContext;
use crate::gfx::buffers::{ProgramObject, ShaderType, UniformLocationObject};
use crate::renderer::uniform::Uniform;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ShaderProgram {
    gfx: Rc<GfxContext>,
    uniform_cache: RefCell<HashMap<String, UniformLocationObject>>,
    block_index_cache: RefCell<HashMap<String, u32>>,
    program: ProgramObject,
}

impl ShaderProgram {
    pub fn new(gfx: Rc<GfxContext>, vertex: &str, fragment: &str) -> Result<Self, String> {
        let vertex_shader = {
            let shader = gfx.create_shader(ShaderType::Vertex)?;
            gfx.set_shader_source(&shader, vertex);
            gfx.compile_shader(&shader);
            shader
        };

        let success = gfx.get_shader_compile_status(&vertex_shader);
        if !success {
            let err_msg =
                String::from("VERTEX:") + gfx.get_shader_info_log(&vertex_shader).as_str();
            gfx.delete_shader(&vertex_shader);
            return Err(err_msg);
        }

        let fragment_shader = {
            let shader = gfx.create_shader(ShaderType::Fragment)?;
            gfx.set_shader_source(&shader, fragment);
            gfx.compile_shader(&shader);
            shader
        };

        let success = gfx.get_shader_compile_status(&fragment_shader);
        if !success {
            let err_msg =
                String::from("FRAGMENT:") + gfx.get_shader_info_log(&fragment_shader).as_str();
            gfx.delete_shader(&fragment_shader);
            return Err(err_msg);
        }

        let program = gfx.create_program()?;
        gfx.attach_shader(&program, &vertex_shader);
        gfx.attach_shader(&program, &fragment_shader);
        gfx.link_program(&program);

        let success = gfx.get_program_link_status(&program);
        if !success {
            let err_msg = gfx.get_program_info_log(&program);
            gfx.delete_shader(&vertex_shader);
            gfx.delete_shader(&fragment_shader);
            return Err(err_msg);
        }

        gfx.delete_shader(&vertex_shader);
        gfx.delete_shader(&fragment_shader);

        let shader = Self {
            gfx,
            uniform_cache: RefCell::new(HashMap::new()),
            block_index_cache: RefCell::new(HashMap::new()),
            program,
        };

        // Try to bind all default uniform blocks if they exist
        shader.bind();
        shader.bind_uniform_block("CameraData", 0);
        shader.bind_uniform_block("TransformData", 1);
        shader.bind_uniform_block("LightingData", 2);
        shader.unbind();

        Ok(shader)
    }

    pub fn bind(&self) {
        self.gfx.use_program(Some(&self.program));
    }

    pub fn unbind(&self) {
        self.gfx.use_program(None);
    }

    pub fn set_uniform<T: Uniform>(&self, name: &str, value: &T) {
        if let Some(location) = self.get_uniform_location(name) {
            value.upload(&self.gfx, &location);
        }
    }

    fn get_uniform_location(&self, name: &str) -> Option<UniformLocationObject> {
        if let Some(location) = self.uniform_cache.borrow().get(name) {
            return Some(*location);
        }

        let location = self.gfx.get_uniform_location(&self.program, name);

        if let Some(location) = location {
            self.uniform_cache
                .borrow_mut()
                .insert(name.to_owned(), location);
        }

        location
    }

    pub fn bind_uniform_block(&self, name: &str, binding: u32) {
        if let Some(index) = self.get_uniform_block_index(name) {
            self.gfx
                .uniform_block_binding(&self.program, index, binding);
        }
    }

    fn get_uniform_block_index(&self, name: &str) -> Option<u32> {
        if let Some(index) = self.block_index_cache.borrow().get(name) {
            return Some(*index);
        }

        let index = self.gfx.get_uniform_block_index(&self.program, name);

        if let Some(index) = index {
            self.block_index_cache
                .borrow_mut()
                .insert(name.to_owned(), index);
        }

        index
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gfx.delete_program(&self.program);
    }
}
