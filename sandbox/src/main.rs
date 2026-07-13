use engine::glow::{self, HasContext};
use engine::loader::Loader;
use engine::renderer::Renderer;
use engine::scene::{FlyCamera, Object, Scene};
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
        }
        self.renderer = Some(Renderer::new(Rc::clone(gl)));
        self.loader = Some(Loader::new(Rc::clone(gl)));

        let loader = self.loader.as_mut().unwrap();

        let cube_mesh = loader.load_cube_mesh().unwrap();

        let crate_material = loader
            .load_material("./assets/materials/crate.mat")
            .unwrap();

        let crate_object = Object::new(
            Some(Rc::clone(&cube_mesh)),
            Some(Rc::clone(&crate_material)),
            Vec3::new(0.0, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::new(1.0, 1.0, 1.0),
        );
        self.scene.objects.push(crate_object);

        unsafe {
            gl.enable(glow::DEPTH_TEST);
        }
    }

    fn update(&mut self, input: &InputManager, dt: f32) -> bool {
        if input.is_key_released(KeyCode::Escape) {
            return true;
        }

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

        false
    }

    fn render(&mut self, _window: &Window, _gl: &Rc<glow::Context>) {
        self.renderer
            .as_ref()
            .unwrap()
            .render(&self.scene, &mut self.camera);
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
        scene: Scene {
            objects: Vec::new(),
        },
        camera: FlyCamera::new(45.0, 1280.0 / 720.0, 0.1, 100.0),
    };
    run("Sandbox", app);
}
