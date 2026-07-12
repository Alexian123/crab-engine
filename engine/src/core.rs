use std::ffi::CString;
use std::num::NonZeroU32;

use crate::input::InputManager;

use glow::HasContext;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub trait Application {
    fn init(&mut self, _window: &Window, _gl: &Rc<glow::Context>) {}
    fn update(&mut self, _input: &InputManager, _dt: f32) -> bool {
        // return true to exit
        false
    }
    fn render(&mut self, _window: &Window, _gl: &Rc<glow::Context>) {}
    fn on_resize(&mut self, _width: u32, _height: u32, _gl: &Rc<glow::Context>) {}
}

struct GlState {
    gl: Rc<glow::Context>,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

struct Runner<A: Application> {
    app: A,
    title: String,
    window: Option<Window>,
    gl_state: Option<GlState>,
    input: InputManager,
    last_frame_time: std::time::Instant,
}

impl<A: Application> ApplicationHandler for Runner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title(&self.title);

        // Prefer configs with more MSAA samples, alpha channel for blending.
        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .expect("no GL configs available")
            })
            .expect("failed to build window/config");

        let window = window.expect("glutin-winit did not create a window");
        let raw_window_handle = window.window_handle().ok().map(|h| h.as_raw());

        let gl_display = gl_config.display();

        // Try for a modern core profile first, fall back to something older
        // for systems with less capable drivers.
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 1))))
            .build(raw_window_handle);

        let fallback_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(raw_window_handle);

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    tracing::warn!("4.1 core context unavailable, falling back to 3.3");
                    gl_display
                        .create_context(&gl_config, &fallback_attributes)
                        .expect("failed to create GL context")
                })
        };

        let surface_attributes = window
            .build_surface_attributes(Default::default())
            .expect("failed to build surface attributes");

        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .expect("failed to create window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("failed to make GL context current");

        // Enable vsync.
        if let Err(e) = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            tracing::warn!("failed to set vsync: {e:?}");
        }

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                let s = CString::new(s).unwrap();
                gl_display.get_proc_address(&s) as *const _
            })
        };

        unsafe {
            tracing::info!("GL version: {}", gl.get_parameter_string(glow::VERSION));
            tracing::info!("GL renderer: {}", gl.get_parameter_string(glow::RENDERER));
        }

        let gl = Rc::new(gl);

        self.app.init(&window, &gl);

        self.gl_state = Some(GlState {
            gl,
            gl_surface,
            gl_context,
        });
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.input.on_window_event(&event);
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("close requested, shutting down");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(gl_state) = &self.gl_state {
                    if size.width > 0 && size.height > 0 {
                        gl_state.gl_surface.resize(
                            &gl_state.gl_context,
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );
                        self.app.on_resize(size.width, size.height, &gl_state.gl);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let dt = (now - self.last_frame_time).as_secs_f32();
                self.last_frame_time = now;
                if self.app.update(&self.input, dt) {
                    event_loop.exit();
                }

                if let (Some(window), Some(gl_state)) = (&self.window, &self.gl_state) {
                    self.app.render(window, &gl_state.gl);
                    gl_state
                        .gl_surface
                        .swap_buffers(&gl_state.gl_context)
                        .expect("failed to swap buffers");
                    self.input.begin_frame();
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.input.on_device_event(&event);
    }
}

/// Runs the engine with the given `Application` implementation.
pub fn run<A: Application + 'static>(title: &str, app: A) {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut runner = Runner {
        app,
        title: title.to_string(),
        window: None,
        gl_state: None,
        input: InputManager::new(),
        last_frame_time: std::time::Instant::now(),
    };

    event_loop
        .run_app(&mut runner)
        .expect("event loop exited with error");
}
