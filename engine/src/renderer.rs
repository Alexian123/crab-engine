pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod uniform;
pub mod vertex;

pub use crate::renderer::material::Material;
pub use crate::renderer::mesh::Mesh;
pub use crate::renderer::shader::ShaderProgram;
pub use crate::renderer::texture::Texture;
pub use crate::scene::Camera;
pub use crate::scene::Scene;
use std::rc::Rc;

pub struct Renderer {
    gl: Rc<glow::Context>,
}

impl Renderer {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self { gl }
    }

    pub fn render(&self, scene: &Scene, camera: &mut dyn Camera) {
        for obj in &scene.objects {
            use glow::HasContext;

            unsafe {
                self.gl
                    .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            }

            if let Some(material) = &obj.material {
                material.bind();

                let shader = &material.shader();
                shader.set_uniform("uModel", &obj.model_matrix());
                shader.set_uniform("uView", &camera.view());
                shader.set_uniform("uProjection", &camera.projection());
                shader.set_uniform("uCameraPos", &camera.position());
            }

            if let Some(mesh) = &obj.mesh {
                mesh.bind();
                mesh.draw();
                mesh.unbind();
            }
        }
    }
}
