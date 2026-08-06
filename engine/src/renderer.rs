pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod uniform;

use crate::GfxContext;
use crate::scene::*;
pub use material::Material;
pub use mesh::Mesh;
pub use shader::ShaderProgram;
use std::rc::Rc;
pub use texture::Texture;

pub struct Renderer {
    gfx: Rc<GfxContext>,
}

impl Renderer {
    pub fn new(gfx: Rc<GfxContext>) -> Self {
        Self { gfx }
    }

    pub fn render(&self, scene: &Scene) {
        self.gfx
            .clear(GfxContext::COLOR_BUFFER_BIT | GfxContext::DEPTH_BUFFER_BIT);

        let world = scene.world();
        let lighting = world.query::<LightingComponent>().next().map(|(_, c)| c);
        let camera = world.query::<CameraComponent>().next().map(|(_, c)| c);

        for (entity, world_transform, mesh_comp) in
            world.query2::<WorldTransformComponent, MeshComponent>()
        {
            if let Some(material_component) = world.get_component::<MaterialComponent>(entity) {
                material_component.material.bind();

                let shader = material_component.material.shader();

                shader.set_uniform("uModel", &world_transform.model_matrix);
                shader.set_uniform("uNormal", &world_transform.normal_matrix());

                if let Some(camera_comp) = camera {
                    shader.set_uniform("uView", &camera_comp.view);
                    shader.set_uniform("uProjection", &camera_comp.projection);
                    shader.set_uniform("uViewPos", &camera_comp.position);
                }

                if let Some(lighting) = lighting {
                    // directional lights
                    for (i, light) in lighting.directional_lights.iter().enumerate() {
                        shader
                            .set_uniform(&format!("uDirLights[{}].direction", i), &light.direction);
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
                    for (i, light) in lighting.point_lights.iter().enumerate() {
                        shader
                            .set_uniform(&format!("uPointLights[{}].position", i), &light.position);
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
                        shader
                            .set_uniform(&format!("uPointLights[{}].constant", i), &light.constant);
                        shader.set_uniform(&format!("uPointLights[{}].linear", i), &light.linear);
                        shader.set_uniform(
                            &format!("uPointLights[{}].quadratic", i),
                            &light.quadratic,
                        );
                    }

                    // spot lights
                    for (i, light) in lighting.spot_lights.iter().enumerate() {
                        shader.set_uniform(
                            &format!("uSpotLights[{}].direction", i),
                            &light.direction,
                        );
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
                        shader.set_uniform(
                            &format!("uSpotLights[{}].pl.linear", i),
                            &light.pl.linear,
                        );
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

                    shader.set_uniform("uNumLightsMask", &lighting.lights_mask);
                }

                mesh_comp.mesh.bind();
                mesh_comp.mesh.draw();
                mesh_comp.mesh.unbind();
            }
        }
    }
}
