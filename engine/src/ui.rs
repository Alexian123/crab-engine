mod hud;

use crate::loader::*;
use std::rc::Rc;

pub use hud::HUDElement;

use crate::renderer::{Mesh, ShaderProgram};

pub struct UI {
    shader: Rc<ShaderProgram>,
    mesh: Rc<Mesh>,
    pub hud: Vec<HUDElement>,
}

impl UI {
    pub fn new(loader: &mut Loader) -> Self {
        let shader = loader
            .load_shader_embedded(&DEFAULT_UI_SHADER)
            .expect("Failed to load UI shader");
        let mesh = loader.load_ui_quad().expect("Failed to load UI quad mesh");
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
