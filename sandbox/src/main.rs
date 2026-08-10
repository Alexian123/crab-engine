mod movement;

use engine::GfxContext;
use engine::loader::Loader;
use engine::renderer::Renderer;
use engine::scene::camera::ThirdPersonCamera;
use engine::scene::terrain;
use engine::scene::*;
use engine::{Application, InputManager, run};
use glam::{Mat4, Quat, Vec3};
use movement::MovementController;
use std::path::Path;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window};

struct Sandbox {
    renderer: Option<Renderer>,
    loader: Option<Loader>,
    scene: Scene,
    movement_ctrl: MovementController,
}

impl Application for Sandbox {
    fn init(&mut self, window: &Window, gfx: &Rc<GfxContext>) {
        tracing::info!("Sandbox initialization started");

        let _ = window.request_inner_size(PhysicalSize::new(1280, 720));
        window
            .set_cursor_grab(CursorGrabMode::Locked)
            .expect("Failed to grab cursor");
        window.set_cursor_visible(false);

        gfx.set_clear_color(0.1, 0.1, 0.15, 1.0);
        gfx.set_depth_test(true);

        self.renderer = Some(Renderer::new(Rc::clone(gfx)));
        self.loader = Some(Loader::new(Rc::clone(gfx)));

        tracing::info!("Scene initialization started");

        let loader = self.loader.as_mut().unwrap();

        tracing::info!("Loading terrain...");

        let terrain_material = loader
            .load_material(Path::new("./assets/materials/terrain.mat"), None)
            .unwrap();

        terrain::generate_terrain_grid(
            self.scene.world_mut(),
            loader,
            0xdeadbeef,
            2,
            -10.0,
            800,
            128,
            10.0,
            Some(terrain_material),
        );
        tracing::info!("Done.");

        let world = self.scene.world_mut();

        tracing::info!("Loading camera...");

        let camera_entity = world.create_entity();
        world.add_component(
            camera_entity,
            CameraComponent {
                position: Vec3::ZERO,
                view: Mat4::IDENTITY,
                projection: Mat4::IDENTITY,
            },
        );

        tracing::info!("Done.");

        tracing::info!("Loading primitive models...");

        let cube_mesh = loader.load_cube_mesh().unwrap();

        let crate_material = loader
            .load_material(Path::new("./assets/materials/crate.mat"), None)
            .unwrap();

        let crate_entity = world.create_entity();
        world.add_component(
            crate_entity,
            LocalTransformComponent {
                position: Vec3::new(-2.0, 0.0, -4.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
        );
        world.add_component(
            crate_entity,
            MeshComponent {
                mesh: Rc::clone(&cube_mesh),
            },
        );
        world.add_component(
            crate_entity,
            MaterialComponent {
                material: Rc::clone(&crate_material),
            },
        );

        tracing::info!("Done.");

        tracing::info!("Loading lights...");

        let lighting_entity = world.create_entity();
        world.add_component(
            lighting_entity,
            LightingComponent {
                lights_mask: 0,
                directional_lights: Vec::new(),
                point_lights: Vec::new(),
                spot_lights: Vec::new(),
            },
        );
        let lighting = world
            .get_component_mut::<LightingComponent>(lighting_entity)
            .unwrap();

        lighting.directional_lights.push(DirectionalLight {
            color: LightColor {
                ambient: Vec3::new(0.05, 0.05, 0.05),
                diffuse: Vec3::new(0.6, 0.6, 0.6),
                specular: Vec3::new(0.8, 0.8, 0.8),
            },
            direction: Vec3::new(0.2, -1.0, 0.3),
        });

        lighting.point_lights.push(PointLight {
            color: LightColor {
                ambient: Vec3::new(0.05, 0.05, 0.05),
                diffuse: Vec3::new(0.8, 0.8, 0.8),
                specular: Vec3::new(1.0, 1.0, 1.0),
            },
            position: Vec3::new(1.2, 1.0, 2.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });
        lighting.point_lights.push(PointLight {
            color: LightColor {
                ambient: Vec3::new(0.05, 0.05, 0.05),
                diffuse: Vec3::new(0.8, 0.8, 0.8),
                specular: Vec3::new(1.0, 1.0, 1.0),
            },
            position: Vec3::new(1.0, 2.0, 2.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });

        lighting.spot_lights.push(SpotLight {
            pl: PointLight {
                color: LightColor {
                    ambient: Vec3::new(0.0, 0.0, 0.0),
                    diffuse: Vec3::new(1.0, 1.0, 1.0),
                    specular: Vec3::new(1.0, 1.0, 1.0),
                },
                position: Vec3::new(0.0, 0.0, 0.0),
                constant: 1.0,
                linear: 0.09,
                quadratic: 0.032,
            },
            direction: Vec3::new(0.0, 0.0, 0.0),
            cutoff: (12.5 as f32).to_radians().cos(),
            outer_cutoff: (15.0 as f32).to_radians().cos(),
        });

        lighting.lights_mask = (lighting.directional_lights.len() as u32)
            | (((lighting.point_lights.len() as u32) & 0xFF) << 8)
            | (((lighting.spot_lights.len() as u32) & 0xFF) << 16);

        tracing::info!("Done.");

        tracing::info!("Loading asset models...");

        let backpack = loader
            .load_model(
                Path::new(
                    "./assets/models/survival_guitar_backpack/survival_guitar_backpack.glb.model",
                ),
                &mut self.scene,
            )
            .expect("Failed to load backpack model");
        let backpack_transform = self
            .scene
            .world_mut()
            .get_component_mut::<LocalTransformComponent>(backpack)
            .unwrap();
        backpack_transform.position = Vec3::new(-2.0, 5.0, -4.0);
        backpack_transform.scale = Vec3::new(0.01, 0.01, 0.01);

        let pug = loader
            .load_model(
                Path::new("./assets/models/pug/a_pug.glb.model"),
                &mut self.scene,
            )
            .expect("Failed to load pug model");
        let pug_transform = self
            .scene
            .world_mut()
            .get_component_mut::<LocalTransformComponent>(pug)
            .unwrap();
        pug_transform.position = Vec3::new(60.0, 2.0, -18.0);
        pug_transform.scale = Vec3::new(10.0, 10.0, 10.0);

        let rat = loader
            .load_model(
                Path::new("./assets/models/rat/street_rat_1k.gltf.model"),
                &mut self.scene,
            )
            .expect("Failed to load rat model");
        let rat_transform = self
            .scene
            .world_mut()
            .get_component_mut::<LocalTransformComponent>(rat)
            .unwrap();
        rat_transform.position = Vec3::new(0.0, 0.0, 0.0);
        rat_transform.scale = Vec3::new(50.0, 50.0, 50.0);

        self.scene
            .world_mut()
            .set_component::<NameComponent>(rat, NameComponent("player".to_string()));

        self.scene.update(self.movement_ctrl.get_active_camera());

        tracing::info!("Done.");
    }

    fn update(&mut self, input: &InputManager, dt: f32) -> bool {
        if input.is_key_released(KeyCode::Escape) {
            return true;
        }

        let player = self.scene.find_entity_by_name("player").unwrap();

        self.movement_ctrl
            .update(dt, input, self.scene.world_mut(), player);

        let (_, lighting) = self
            .scene
            .world_mut()
            .query_mut::<LightingComponent>()
            .next()
            .unwrap();

        lighting.lights_mask &= 0xFFFF;
        if input.is_mouse_down(MouseButton::Left) {
            lighting.lights_mask |= 1 << 16;
        }

        // update spot light position and direction to simulate FPS flashlight
        lighting.spot_lights[0].pl.position = self.movement_ctrl.get_active_camera().position();
        lighting.spot_lights[0].direction = self.movement_ctrl.get_active_camera().forward();

        self.scene.update(self.movement_ctrl.get_active_camera());

        false
    }

    fn render(&mut self, _window: &Window, _gfx: &Rc<GfxContext>) {
        self.renderer.as_ref().unwrap().render(&self.scene);
    }

    fn on_resize(&mut self, width: u32, height: u32, gfx: &Rc<GfxContext>) {
        tracing::info!("resized to {width}x{height}");
        gfx.set_viewport(0, 0, width as i32, height as i32);
        self.movement_ctrl
            .set_aspect_ratio(width as f32 / height as f32);
    }
}

fn main() {
    engine::logging::init();
    let app = Sandbox {
        renderer: None,
        loader: None,
        scene: Scene::new(),
        movement_ctrl: MovementController::new(
            FlyCamera::new(45.0, 1280.0 / 720.0, 0.1, 1000.0),
            ThirdPersonCamera::new(45.0, 1280.0 / 720.0, 0.1, 1000.0, 10.0),
        ),
    };
    run("Sandbox", app);
}
