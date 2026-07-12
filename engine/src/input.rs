use std::collections::HashSet;
use winit::event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputManager {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    mouse_x: f64,
    mouse_y: f64,

    mouse_delta_x: f64,
    mouse_delta_y: f64,

    mouse_wheel_x: f32,
    mouse_wheel_y: f32,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),

            mouse_down: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),

            mouse_x: 0.0,
            mouse_y: 0.0,

            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0,

            mouse_wheel_x: 0.0,
            mouse_wheel_y: 0.0,
        }
    }

    pub fn begin_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();

        self.mouse_pressed.clear();
        self.mouse_released.clear();

        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;

        self.mouse_wheel_x = 0.0;
        self.mouse_wheel_y = 0.0;
    }

    pub fn on_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => {
                            if self.keys_down.insert(key) {
                                self.keys_pressed.insert(key);
                            }
                        }
                        ElementState::Released => {
                            self.keys_down.remove(&key);
                            self.keys_released.insert(key);
                        }
                    }
                }
            }

            WindowEvent::MouseInput { button, state, .. } => match state {
                ElementState::Pressed => {
                    if self.mouse_down.insert(*button) {
                        self.mouse_pressed.insert(*button);
                    }
                }
                ElementState::Released => {
                    self.mouse_down.remove(button);
                    self.mouse_released.insert(*button);
                }
            },

            _ => {}
        }
    }

    pub fn on_device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.mouse_delta_x = delta.0;
                self.mouse_delta_y = delta.1;
            }

            DeviceEvent::MouseWheel { delta } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    self.mouse_wheel_x = *x;
                    self.mouse_wheel_y = *y;
                }
                _ => {}
            },

            _ => {}
        }
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        (self.mouse_delta_x, self.mouse_delta_y)
    }

    pub fn mouse_wheel(&self) -> (f32, f32) {
        (self.mouse_wheel_x, self.mouse_wheel_y)
    }
}
