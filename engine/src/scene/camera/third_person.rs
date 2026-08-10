use super::{Camera, FlyCamera};
use glam::{Mat4, Vec3};

pub struct ThirdPersonCamera {
    fly_camera: FlyCamera,
    target: Option<Vec3>,
    distance: f32,
}

impl ThirdPersonCamera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32, distance: f32) -> Self {
        Self {
            fly_camera: FlyCamera::new(fov, aspect, near, far),
            target: None,
            distance,
        }
    }

    pub fn fly_camera(&self) -> &FlyCamera {
        &self.fly_camera
    }

    pub fn fly_camera_mut(&mut self) -> &mut FlyCamera {
        &mut self.fly_camera
    }

    pub fn target(&self) -> Option<Vec3> {
        self.target
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_target(&mut self, target: Option<Vec3>) {
        self.target = target;
        self.sync_position();
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
        self.sync_position();
    }

    pub fn move_yaw(&mut self, amount: f32) {
        self.fly_camera.move_yaw(amount);
        self.sync_position();
    }

    pub fn move_pitch(&mut self, amount: f32) {
        self.fly_camera.move_pitch(amount);
        self.sync_position();
    }

    pub fn zoom(&mut self, amount: f32) {
        let new_distance = (self.distance - amount).clamp(2.0, 30.0);
        self.distance = new_distance;
        self.sync_position();
    }

    fn sync_position(&mut self) {
        if let Some(target) = self.target {
            let front = self.fly_camera.forward();
            self.fly_camera.set_position(target - front * self.distance);
        }
    }
}

impl Camera for ThirdPersonCamera {
    fn view(&self) -> Mat4 {
        self.fly_camera.view()
    }

    fn projection(&self) -> Mat4 {
        self.fly_camera.projection()
    }

    fn position(&self) -> Vec3 {
        self.fly_camera.position()
    }

    fn up(&self) -> Vec3 {
        self.fly_camera.up()
    }

    fn right(&self) -> Vec3 {
        self.fly_camera.right()
    }

    fn forward(&self) -> Vec3 {
        self.fly_camera.forward()
    }
}
