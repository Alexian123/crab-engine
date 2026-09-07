use crate::renderer::{Mesh, SamplerCube, ShaderProgram, TextureSampler};
use std::rc::Rc;

pub struct Skybox {
    mesh: Rc<Mesh>,
    shader: Rc<ShaderProgram>,
    sampler: Rc<SamplerCube>,
}

impl Skybox {
    pub fn new(mesh: Rc<Mesh>, shader: Rc<ShaderProgram>, sampler: Rc<SamplerCube>) -> Self {
        Self {
            mesh,
            shader,
            sampler,
        }
    }

    pub fn shader(&self) -> &ShaderProgram {
        &self.shader
    }

    pub fn bind(&self) {
        self.shader.bind();
        self.mesh.bind();
        self.sampler.bind(0);
    }

    pub fn draw(&self) {
        self.mesh.draw();
    }
}
