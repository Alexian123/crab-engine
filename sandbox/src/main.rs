use engine::glow::{self, HasContext};
use engine::loader::Loader;
use engine::renderer::Renderer;
use engine::scene::{Camera, Object, Scene};
use engine::{Application, InputManager, run};
use glam::{Quat, Vec3};
use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::keyboard::KeyCode;
use winit::window::Window;

struct Sandbox {
    renderer: Option<Renderer>,
    loader: Option<RefCell<Loader>>,
    scene: Scene,
    camera: Camera,
}

impl Application for Sandbox {
    fn init(&mut self, window: &Window, gl: &Rc<glow::Context>) {
        tracing::info!("sandbox initialized");
        let _ = window.request_inner_size(PhysicalSize::new(1280, 720));
        unsafe {
            gl.clear_color(0.1, 0.1, 0.15, 1.0);
        }
        self.renderer = Some(Renderer::new(Rc::clone(gl)));
        self.loader = Some(RefCell::new(Loader::new(Rc::clone(gl))));

        let cube_mesh = self
            .loader
            .as_ref()
            .unwrap()
            .borrow_mut()
            .load_cube_mesh()
            .unwrap();

        let crate_material = self
            .loader
            .as_ref()
            .unwrap()
            .borrow_mut()
            .load_material("./assets/materials/crate.mat")
            .unwrap();

        let crate_object = Object::new(
            Some(Rc::clone(&cube_mesh)),
            Some(Rc::clone(&crate_material)),
            Vec3::new(-2.0, 0.0, -5.0),
            Quat::IDENTITY,
            Vec3::new(1.0, 1.0, 1.0),
        );
        self.scene.objects.push(crate_object);

        unsafe {
            gl.enable(glow::DEPTH_TEST);
        }
    }

    fn update(&mut self, input: &InputManager, dt: f32) {
        let camera_speed = dt * 2.5;
        if input.is_key_down(KeyCode::KeyS) {
            self.camera.position.z += 1.0 * camera_speed;
        }
        if input.is_key_down(KeyCode::KeyW) {
            self.camera.position.z -= 1.0 * camera_speed;
        }
        if input.is_key_down(KeyCode::KeyA) {
            self.camera.position.x -= 1.0 * camera_speed;
        }
        if input.is_key_down(KeyCode::KeyD) {
            self.camera.position.x += 1.0 * camera_speed;
        }
        if input.is_key_down(KeyCode::Space) {
            self.camera.position.y += 1.0 * camera_speed;
        }
        if input.is_key_down(KeyCode::ControlLeft) {
            self.camera.position.y -= 1.0 * camera_speed;
        }
    }

    fn render(&mut self, _window: &Window, _gl: &Rc<glow::Context>) {
        self.renderer
            .as_ref()
            .unwrap()
            .render(&self.scene, &self.camera);
    }

    fn on_resize(&mut self, width: u32, height: u32, gl: &Rc<glow::Context>) {
        tracing::info!("resized to {width}x{height}");
        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}

fn main() {
    engine::logging::init();
    let app = Sandbox {
        renderer: None,
        loader: None,
        scene: Scene {
            objects: Vec::new(),
        },
        camera: Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            fov_y_degrees: 45.0,
            aspect: 1280.0 / 720.0,
            near: 0.1,
            far: 100.0,
        },
    };
    run("Sandbox", app);
}
