use crate::renderer::Texture;
use crate::renderer::uniform::Uniform;
use glow::HasContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ShaderProgram {
    gl: Rc<glow::Context>,
    uniform_cache: RefCell<HashMap<String, glow::UniformLocation>>,
    program: glow::Program,
}

impl ShaderProgram {
    pub fn new(gl: Rc<glow::Context>, vertex: &str, fragment: &str) -> Result<Self, String> {
        let vertex_shader = unsafe {
            let shader = gl.create_shader(glow::VERTEX_SHADER)?;
            gl.shader_source(shader, vertex);
            gl.compile_shader(shader);
            shader
        };

        let success = unsafe { gl.get_shader_compile_status(vertex_shader) };
        if !success {
            let err_msg =
                String::from("VERTEX:") + unsafe { gl.get_shader_info_log(vertex_shader) }.as_str();
            unsafe { gl.delete_shader(vertex_shader) };
            return Err(err_msg);
        }

        let fragment_shader = unsafe {
            let shader = gl.create_shader(glow::FRAGMENT_SHADER)?;
            gl.shader_source(shader, fragment);
            gl.compile_shader(shader);
            shader
        };

        let success = unsafe { gl.get_shader_compile_status(fragment_shader) };
        if !success {
            let err_msg = String::from("FRAGMENT:")
                + unsafe { gl.get_shader_info_log(fragment_shader) }.as_str();
            unsafe { gl.delete_shader(fragment_shader) };
            return Err(err_msg);
        }

        let program = unsafe {
            let program = gl.create_program()?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            program
        };

        let success = unsafe { gl.get_program_link_status(program) };
        if !success {
            let err_msg = unsafe { gl.get_program_info_log(program) };
            unsafe {
                gl.delete_shader(vertex_shader);
                gl.delete_shader(fragment_shader);
            }
            return Err(err_msg);
        }

        unsafe {
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
        }

        Ok(Self {
            gl,
            uniform_cache: RefCell::new(HashMap::new()),
            program,
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn set_uniform<T: Uniform>(&self, name: &str, value: &T) {
        if let Some(location) = self.get_uniform_location(name) {
            value.upload(&self.gl, &location);
        }
    }

    pub fn set_texture(&self, name: &str, texture: &Texture, unit: u32) {
        if let Some(location) = self.get_uniform_location(name) {
            texture.bind(unit);
            (unit as i32).upload(self.gl.as_ref(), &location);
        }
    }

    fn get_uniform_location(&self, name: &str) -> Option<glow::UniformLocation> {
        if let Some(location) = self.uniform_cache.borrow().get(name) {
            return Some(*location);
        }

        let location = unsafe { self.gl.get_uniform_location(self.program, name) };

        if let Some(location) = location {
            self.uniform_cache
                .borrow_mut()
                .insert(name.to_owned(), location);
        }

        location
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}
