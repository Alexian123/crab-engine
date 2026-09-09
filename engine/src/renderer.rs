pub mod camera;
mod framebuffer;
mod material;
mod mesh;
pub mod postprocessing;
mod shader;
mod skybox;
mod texture;
mod uniform;

use crate::GfxContext;
use crate::scene::*;
use camera::Camera;
pub use framebuffer::{Framebuffer, FramebufferBuilder};
pub use material::Material;
pub use mesh::Mesh;
pub use postprocessing::PostProcessingStage;
use postprocessing::*;
pub use shader::ShaderProgram;
pub use skybox::Skybox;
use std::rc::Rc;
pub use texture::{Sampler2D, SamplerCube, TextureSampler};

pub struct Renderer {
    gfx: Rc<GfxContext>,
    skybox: Option<Rc<Skybox>>,
    framebuffer: Framebuffer,
    pp_pipeline: PostProcessingPipeline,
}

impl Renderer {
    pub fn new(
        gfx: Rc<GfxContext>,
        skybox: Option<Rc<Skybox>>,
        screen_width: u32,
        screen_height: u32,
        screen_quad: Rc<Mesh>,
        screen_shader: Rc<ShaderProgram>,
    ) -> Result<Self, String> {
        let framebuffer = FramebufferBuilder::new(Rc::clone(&gfx), screen_width, screen_height)?
            .with_color_texture()?
            .with_depth_render_buffer()?
            .build()?;
        let pp_pipeline = PostProcessingPipeline::new(
            Rc::clone(&gfx),
            screen_width,
            screen_height,
            screen_quad,
            screen_shader,
        )?;
        Ok(Self {
            gfx,
            skybox,
            framebuffer,
            pp_pipeline,
        })
    }

    pub fn add_post_processing_stage(&mut self, stage: Box<dyn PostProcessingStage>) {
        self.pp_pipeline.stages.push(stage);
    }

    pub fn clear_post_processing_stages(&mut self) {
        self.pp_pipeline.stages.clear();
    }

    pub fn set_skybox(&mut self, skybox: Option<Rc<Skybox>>) {
        self.skybox = skybox;
    }

    pub fn render(&self, scene: &Scene, camera: &dyn Camera) {
        self.framebuffer.bind();
        self.render_world(scene.world(), camera);
        self.render_skybox(camera);
        self.framebuffer.unbind();
        self.pp_pipeline
            .run(self.framebuffer.color_texture().unwrap());
    }

    fn render_world(&self, world: &World, camera: &dyn Camera) {
        self.gfx
            .clear(GfxContext::COLOR_BUFFER_BIT | GfxContext::DEPTH_BUFFER_BIT);

        let lighting = world.query::<LightingComponent>().next().map(|(_, c)| c);

        for (entity, world_transform, mesh_comp) in
            world.query2::<WorldTransformComponent, MeshComponent>()
        {
            if let Some(material_component) = world.get_component::<MaterialComponent>(entity) {
                material_component.material.bind();

                let shader = material_component.material.shader();

                shader.set_uniform("uModel", &world_transform.model_matrix);
                shader.set_uniform("uNormal", &world_transform.normal_matrix());

                shader.set_uniform("uView", &camera.view());
                shader.set_uniform("uProjection", &camera.projection());
                shader.set_uniform("uViewPos", &camera.position());

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

    fn render_skybox(&self, camera: &dyn Camera) {
        if let Some(skybox) = &self.skybox {
            self.gfx.set_depth_func(crate::gfx::DepthFunc::LessEqual);
            skybox.bind();
            let shader = skybox.shader();
            let view = glam::Mat4::from_mat3(glam::Mat3::from_mat4(camera.view())); // remove translation from the view matrix
            shader.set_uniform("uView", &view);
            shader.set_uniform("uProjection", &camera.projection());
            skybox.draw();
            self.gfx.set_depth_func(crate::gfx::DepthFunc::Less);
        }
    }
}
