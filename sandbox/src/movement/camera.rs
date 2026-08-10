use crate::{
    Entity, FlyCamera, InputManager, KeyCode, MouseButton, ThirdPersonCamera, World,
    WorldTransformComponent,
};

pub struct TPCameraController {
    pub camera: ThirdPersonCamera,
    pub sensitivity: f32,
    pub active: bool,
}

impl TPCameraController {
    pub fn new(camera: ThirdPersonCamera, sensitivity: f32) -> Self {
        Self {
            camera,
            sensitivity,
            active: false,
        }
    }

    pub fn update(&mut self, input: &InputManager, player: Entity, world: &World) {
        if !self.active {
            return;
        }
        if let Some(player_wt) = world.get_component::<WorldTransformComponent>(player) {
            let (_, _, world_pos) = player_wt.model_matrix.to_scale_rotation_translation();
            self.camera.set_target(Some(world_pos));
        } else {
            self.camera.set_target(None);
        }

        let (_, scroll_offset_y) = input.mouse_wheel();
        if scroll_offset_y != 0.0 {
            self.camera.zoom(scroll_offset_y);
        }

        let delta = input.mouse_delta();
        self.camera.move_yaw(delta.0 as f32 * self.sensitivity);
        self.camera.move_pitch(-delta.1 as f32 * self.sensitivity);
    }
}

pub struct FlyCameraController {
    pub camera: FlyCamera,
    pub sensitivity: f32,
    pub regular_speed: f32,
    pub boost_speed: f32,
    pub active: bool,
}

impl FlyCameraController {
    pub fn new(camera: FlyCamera, sensitivity: f32, regular_speed: f32, boost_speed: f32) -> Self {
        Self {
            camera,
            sensitivity,
            regular_speed,
            boost_speed,
            active: false,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputManager) {
        if !self.active {
            return;
        }
        let (_, scroll_offset_y) = input.mouse_wheel();
        if scroll_offset_y != 0.0 {
            self.camera.zoom(scroll_offset_y);
        }

        if input.is_mouse_down(MouseButton::Right) {
            let delta = input.mouse_delta();
            self.camera.move_yaw(delta.0 as f32 * self.sensitivity);
            self.camera.move_pitch(-delta.1 as f32 * self.sensitivity);
        }

        let camera_speed = if input.is_key_down(KeyCode::ShiftLeft) {
            dt * self.boost_speed
        } else {
            dt * self.regular_speed
        };
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
    }
}
