use engine::glow::{self, HasContext};
use engine::{Application, InputManager, run};
use winit::keyboard::KeyCode;
use winit::window::Window;

struct Sandbox;

impl Application for Sandbox {
    fn init(&mut self, _window: &Window, gl: &glow::Context) {
        tracing::info!("sandbox initialized");
        unsafe {
            gl.clear_color(0.1, 0.1, 0.15, 1.0);
        }
    }

    fn update(&mut self, input: &InputManager, _dt: f32) {
        if input.is_key_pressed(KeyCode::Space) {
            tracing::info!("space pressed");
        }
    }

    fn render(&mut self, _window: &Window, gl: &glow::Context) {
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn on_resize(&mut self, width: u32, height: u32, gl: &glow::Context) {
        tracing::info!("resized to {width}x{height}");
        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}

fn main() {
    engine::logging::init();
    run("Sandbox", Sandbox);
}
