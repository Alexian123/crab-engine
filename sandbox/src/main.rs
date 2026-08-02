use engine::glow::{self, HasContext};
use engine::loader::Loader;
use engine::renderer::Renderer;
use engine::scene::*;
use engine::{Application, InputManager, run};
use glam::{Quat, Vec3};
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window};

struct Sandbox {
    renderer: Option<Renderer>,
    loader: Option<Loader>,
    scene: Scene,
    camera: FlyCamera,
}

impl Application for Sandbox {
    fn init(&mut self, window: &Window, gl: &Rc<glow::Context>) {
        tracing::info!("sandbox initialized");

        let _ = window.request_inner_size(PhysicalSize::new(1280, 720));
        window
            .set_cursor_grab(CursorGrabMode::Locked)
            .expect("Failed to grab cursor");
        window.set_cursor_visible(false);

        unsafe {
            gl.clear_color(0.1, 0.1, 0.15, 1.0);
            gl.enable(glow::DEPTH_TEST);
        }
        self.renderer = Some(Renderer::new(Rc::clone(gl)));
        self.loader = Some(Loader::new(Rc::clone(gl)));

        let loader = self.loader.as_mut().unwrap();

        let cube_mesh = loader.load_cube_mesh().unwrap();

        let crate_material = loader
            .load_material("./assets/materials/crate.mat")
            .unwrap();

        let world = self.scene.world_mut();

        let camera_entity = world.create_entity();
        world.add_component(
            camera_entity,
            CameraComponent {
                position: self.camera.position(),
                view: self.camera.view(),
                projection: self.camera.projection(),
            },
        );

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
                diffuse: Vec3::new(0.4, 0.4, 0.4),
                specular: Vec3::new(0.5, 0.5, 0.5),
            },
            direction: Vec3::new(-0.2, -1.0, -0.3),
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

        let backpack = loader
            .load_model("./test/survival_guitar_backpack.glb.model", &mut self.scene)
            .expect("Failed to load model");
        let backpack_transform = self
            .scene
            .world_mut()
            .get_component_mut::<LocalTransformComponent>(backpack)
            .unwrap();
        backpack_transform.position = Vec3::new(-2.0, 5.0, -4.0);
        backpack_transform.scale = Vec3::new(0.01, 0.01, 0.01);
    }

    fn update(&mut self, input: &InputManager, dt: f32) -> bool {
        if input.is_key_released(KeyCode::Escape) {
            return true;
        }

        let world = self.scene.world_mut();

        let (_, scroll_offset_y) = input.mouse_wheel();
        if scroll_offset_y != 0.0 {
            self.camera.zoom(scroll_offset_y);
        }

        if input.is_mouse_down(MouseButton::Right) {
            let delta = input.mouse_delta();
            let sensitivity = 0.1;
            self.camera.move_yaw(delta.0 as f32 * sensitivity);
            self.camera.move_pitch(-delta.1 as f32 * sensitivity);
        }

        let camera_speed = dt * 2.5;
        if input.is_key_down(KeyCode::KeyW) {
            self.camera.move_z(camera_speed);
        }
        if input.is_key_down(KeyCode::KeyS) {
            self.camera.move_z(-camera_speed);
        }
        if input.is_key_down(KeyCode::KeyA) {
            self.camera.move_x(-camera_speed);
        }
        if input.is_key_down(KeyCode::KeyD) {
            self.camera.move_x(camera_speed);
        }
        if input.is_key_down(KeyCode::Space) {
            self.camera.move_y(camera_speed);
        }
        if input.is_key_down(KeyCode::ControlLeft) {
            self.camera.move_y(-camera_speed);
        }

        // update camera component
        let (_, camera_comp) = world.query_mut::<CameraComponent>().next().unwrap();
        camera_comp.position = self.camera.position();
        camera_comp.view = self.camera.view();
        camera_comp.projection = self.camera.projection();

        let (_, lighting) = world.query_mut::<LightingComponent>().next().unwrap();

        lighting.lights_mask &= 0xFFFF;
        if input.is_mouse_down(MouseButton::Left) {
            lighting.lights_mask |= 1 << 16;
        }

        // update spot light position and direction to simulate FPS flashlight
        lighting.spot_lights[0].pl.position = self.camera.position();
        lighting.spot_lights[0].direction = self.camera.front();

        self.scene.update();

        false
    }

    fn render(&mut self, _window: &Window, _gl: &Rc<glow::Context>) {
        self.renderer.as_ref().unwrap().render(&self.scene);
    }

    fn on_resize(&mut self, width: u32, height: u32, gl: &Rc<glow::Context>) {
        tracing::info!("resized to {width}x{height}");
        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
            self.camera.set_aspect(width as f32 / height as f32);
        }
    }
}

fn main() {
    engine::logging::init();
    let app = Sandbox {
        renderer: None,
        loader: None,
        scene: Scene::new(),
        camera: FlyCamera::new(45.0, 1280.0 / 720.0, 0.1, 100.0),
    };
    run("Sandbox", app);
}
