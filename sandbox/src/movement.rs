mod camera;
mod player;

use crate::{Camera, Entity, FlyCamera, InputManager, KeyCode, ThirdPersonCamera, World};
use camera::{FlyCameraController, TPCameraController};
use player::PlayerController;

pub struct MovementController {
    fly_camera_ctrl: FlyCameraController,
    tp_camera_ctrl: TPCameraController,
    player_ctrl: PlayerController,
}

impl MovementController {
    pub fn new(fly_camera: FlyCamera, tp_camera: ThirdPersonCamera) -> Self {
        let mut ctrl = Self {
            fly_camera_ctrl: FlyCameraController::new(fly_camera, 0.1, 10.0, 100.0),
            tp_camera_ctrl: TPCameraController::new(tp_camera, 0.1),
            player_ctrl: PlayerController::new(10.0, 100.0, 3.0, 20.0, -50.0, 10.0),
        };
        ctrl.fly_camera_ctrl.active = true;
        ctrl
    }

    pub fn update(&mut self, dt: f32, input: &InputManager, world: &mut World, player: Entity) {
        // switch camera from fly to tp and vice versa
        if input.is_key_pressed(KeyCode::Tab) {
            if self.fly_camera_ctrl.active {
                self.fly_camera_ctrl.active = false;
                self.tp_camera_ctrl.active = true;
                self.player_ctrl.active = true;
            } else {
                self.fly_camera_ctrl.active = true;
                self.tp_camera_ctrl.active = false;
                self.player_ctrl.active = false;

                // position fly camera at tp camera position and align with tp camera
                self.fly_camera_ctrl
                    .camera
                    .set_position(self.tp_camera_ctrl.camera.position());
                self.fly_camera_ctrl
                    .camera
                    .set_pitch(self.tp_camera_ctrl.camera.fly_camera().pitch());
                self.fly_camera_ctrl
                    .camera
                    .set_yaw(self.tp_camera_ctrl.camera.fly_camera().yaw());
            }
        }

        if self.fly_camera_ctrl.active {
            self.fly_camera_ctrl.update(dt, input);
        }
        if self.tp_camera_ctrl.active {
            self.tp_camera_ctrl.update(input, player, world);
        }
        if self.player_ctrl.active {
            self.player_ctrl.update(player, dt, input, world);
        }
    }

    pub fn get_active_camera(&self) -> &dyn Camera {
        if self.fly_camera_ctrl.active {
            &self.fly_camera_ctrl.camera
        } else {
            &self.tp_camera_ctrl.camera
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.fly_camera_ctrl.camera.set_aspect_ratio(aspect_ratio);
        self.tp_camera_ctrl.camera.set_aspect_ratio(aspect_ratio);
    }
}
