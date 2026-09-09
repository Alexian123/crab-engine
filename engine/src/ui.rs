mod hud;

use std::rc::Rc;

pub use hud::HUDElement;

use crate::renderer::{Mesh, ShaderProgram};

pub struct UI {
    shader: Rc<ShaderProgram>,
    mesh: Rc<Mesh>,
    pub hud: Vec<HUDElement>,
}

impl UI {
    pub fn new(shader: Rc<ShaderProgram>, mesh: Rc<Mesh>) -> Self {
        shader.bind();
        shader.set_uniform("uUITexture", &(0 as i32));
        shader.unbind();
        Self {
            shader,
            mesh,
            hud: Vec::new(),
        }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.shader
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}
