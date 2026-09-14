pub mod camera;
mod framebuffer;
mod gpu_data;
mod material;
mod mesh;
pub mod postprocessing;
mod shader;
mod skybox;
mod texture;
pub mod ubo;
mod uniform;

use crate::GfxContext;
use crate::scene::*;
use crate::ui::*;
use camera::Camera;
pub use framebuffer::{Framebuffer, FramebufferBuilder};
use gpu_data::*;
pub use material::Material;
pub use mesh::Mesh;
pub use postprocessing::PostProcessingStage;
use postprocessing::*;
pub use shader::ShaderProgram;
pub use skybox::Skybox;
use std::rc::Rc;
pub use texture::{Sampler2D, SamplerCube, TextureSampler};
use ubo::Ubo;

pub struct Renderer {
    gfx: Rc<GfxContext>,
    skybox: Option<Rc<Skybox>>,
    framebuffer: Framebuffer,
    pp_pipeline: PostProcessingPipeline,
    camera_data: Ubo,
    transform_data: Ubo,
    lighting_data: Ubo,
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

        let camera_data = Ubo::new(Rc::clone(&gfx), std::mem::size_of::<GpuCameraData>(), 0)?;
        let transform_data = Ubo::new(Rc::clone(&gfx), std::mem::size_of::<GpuTransformData>(), 1)?;
        let lighting_data = Ubo::new(Rc::clone(&gfx), std::mem::size_of::<GpuLightingData>(), 2)?;

        Ok(Self {
            gfx,
            skybox,
            framebuffer,
            pp_pipeline,
            camera_data,
            transform_data,
            lighting_data,
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

    pub fn render(&self, scene: &Scene, camera: &dyn Camera, ui: Option<&UI>) {
        // update CameraData UBO
        self.camera_data
            .store(0, bytemuck::bytes_of(&GpuCameraData::from_camera(camera)));

        self.framebuffer.bind();
        self.render_world(scene.world());
        self.render_skybox();
        self.framebuffer.unbind();
        self.pp_pipeline
            .run(self.framebuffer.color_texture().unwrap());
        self.render_ui(ui);
    }

    fn render_world(&self, world: &World) {
        self.gfx
            .clear(GfxContext::COLOR_BUFFER_BIT | GfxContext::DEPTH_BUFFER_BIT);

        // update LightingData UBO
        let lighting = world.query::<LightingComponent>().next().map(|(_, c)| c);
        if let Some(lighting) = lighting {
            self.lighting_data.store(
                0,
                bytemuck::bytes_of(&GpuLightingData::from_lighting_component(lighting)),
            );
        }

        for (entity, world_transform, mesh_comp) in
            world.query2::<WorldTransformComponent, MeshComponent>()
        {
            if let Some(material_component) = world.get_component::<MaterialComponent>(entity) {
                material_component.material.bind();

                // update TransformData UBO
                self.transform_data.store(
                    0,
                    bytemuck::bytes_of(&GpuTransformData::from_world_transform(&world_transform)),
                );

                mesh_comp.mesh.bind();
                mesh_comp.mesh.draw();
                mesh_comp.mesh.unbind();
            }
        }
    }

    fn render_skybox(&self) {
        if let Some(skybox) = &self.skybox {
            self.gfx.set_depth_func(crate::gfx::DepthFunc::LessEqual);
            skybox.draw();
            self.gfx.set_depth_func(crate::gfx::DepthFunc::Less);
        }
    }

    fn render_ui(&self, ui: Option<&UI>) {
        if let Some(ui) = ui {
            self.gfx.set_depth_test(false);
            self.gfx.set_blend(true);
            self.gfx.set_blend_func(
                crate::gfx::BlendFunc::SrcAlpha,
                crate::gfx::BlendFunc::OneMinusSrcAlpha,
            );

            let mesh = ui.mesh();
            let shader = ui.shader();

            mesh.bind();
            shader.bind();

            // render each HUD element
            for element in &ui.hud {
                element.texture.bind(0);
                shader.set_uniform("uTransform", &element.transform());
                mesh.draw();
            }

            shader.unbind();
            mesh.unbind();

            self.gfx.set_blend(false);
            self.gfx.set_depth_test(true);
        }
    }
}
