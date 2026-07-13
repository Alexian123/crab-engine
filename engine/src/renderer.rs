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
                shader.set_uniform("uNormal", &obj.normal_matrix());

                shader.set_uniform("uView", &camera.view());
                shader.set_uniform("uProjection", &camera.projection());
                shader.set_uniform("uViewPos", &camera.position());

                // directional lights
                for (i, light) in scene.directional_lights.iter().enumerate() {
                    shader.set_uniform(&format!("uDirLights[{}].direction", i), &light.direction);
                    shader.set_uniform(
                        &format!("uDirLights[{}].color.ambient", i),
                        &light.color.ambient,
                    );
                    shader.set_uniform(
                        &format!("uDirLights[{}].color.diffuse", i),
                        &light.color.diffuse,
                    );
                    shader.set_uniform(
                        &format!("uDirLights[{}].color.specular", i),
                        &light.color.specular,
                    );
                }

                // point lights
                for (i, light) in scene.point_lights.iter().enumerate() {
                    shader.set_uniform(&format!("uPointLights[{}].position", i), &light.position);
                    shader.set_uniform(
                        &format!("uPointLights[{}].color.ambient", i),
                        &light.color.ambient,
                    );
                    shader.set_uniform(
                        &format!("uPointLights[{}].color.diffuse", i),
                        &light.color.diffuse,
                    );
                    shader.set_uniform(
                        &format!("uPointLights[{}].color.specular", i),
                        &light.color.specular,
                    );
                    shader.set_uniform(&format!("uPointLights[{}].constant", i), &light.constant);
                    shader.set_uniform(&format!("uPointLights[{}].linear", i), &light.linear);
                    shader.set_uniform(&format!("uPointLights[{}].quadratic", i), &light.quadratic);
                }

                // spot lights
                for (i, light) in scene.spot_lights.iter().enumerate() {
                    shader.set_uniform(&format!("uSpotLights[{}].direction", i), &light.direction);
                    shader.set_uniform(&format!("uSpotLights[{}].cutOff", i), &light.cutoff);
                    shader.set_uniform(
                        &format!("uSpotLights[{}].outerCutOff", i),
                        &light.outer_cutoff,
                    );
                    shader.set_uniform(
                        &format!("uSpotLights[{}].pl.position", i),
                        &light.pl.position,
                    );
                    shader.set_uniform(
                        &format!("uSpotLights[{}].pl.constant", i),
                        &light.pl.constant,
                    );
                    shader.set_uniform(&format!("uSpotLights[{}].pl.linear", i), &light.pl.linear);
                    shader.set_uniform(
                        &format!("uSpotLights[{}].pl.quadratic", i),
                        &light.pl.quadratic,
                    );
                    shader.set_uniform(
                        &format!("uSpotLights[{}].pl.color.ambient", i),
                        &light.pl.color.ambient,
                    );
                    shader.set_uniform(
                        &format!("uSpotLights[{}].pl.color.diffuse", i),
                        &light.pl.color.diffuse,
                    );
                    shader.set_uniform(
                        &format!("uSpotLights[{}].pl.color.specular", i),
                        &light.pl.color.specular,
                    );
                }

                shader.set_uniform("uNumLightsMask", &scene.lights_mask);
            }

            if let Some(mesh) = &obj.mesh {
                mesh.bind();
                mesh.draw();
                mesh.unbind();
            }
        }
    }
}
