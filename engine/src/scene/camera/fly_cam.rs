use glam::{Mat4, Vec3};

use super::Camera;

pub struct FlyCamera {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    yaw: f32,
    pitch: f32,

    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,

    view: Mat4,
    projection: Mat4,

    view_dirty: bool,
    projection_dirty: bool,
    vectors_dirty: bool,
}

impl FlyCamera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut cam = Self {
            fov,
            aspect,
            near,
            far,
            yaw: -90.0, // looking towards negative z-axis
            pitch: 0.0,
            position: Vec3::new(0.0, 0.0, 3.0),
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
            view_dirty: true,
            projection_dirty: true,
            vectors_dirty: true,
        };
        cam.update_vectors_if_dirty();
        cam.update_view_if_dirty();
        cam.update_projection_if_dirty();
        cam
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn front(&self) -> Vec3 {
        self.front
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn right(&self) -> Vec3 {
        self.right
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.view_dirty = true;
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov.clamp(1.0, 45.0);
        self.projection_dirty = true;
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.projection_dirty = true;
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.projection_dirty = true;
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.projection_dirty = true;
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.vectors_dirty = true;
        self.view_dirty = true;
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(-89.0, 89.0);
        self.vectors_dirty = true;
        self.view_dirty = true;
    }

    pub fn move_x(&mut self, amount: f32) {
        self.update_vectors_if_dirty();
        self.position += self.right * amount;
        self.view_dirty = true;
    }

    pub fn move_y(&mut self, amount: f32) {
        self.position += self.up * amount;
        self.view_dirty = true;
    }

    pub fn move_z(&mut self, amount: f32) {
        self.update_vectors_if_dirty();
        self.position += self.front * amount;
        self.view_dirty = true;
    }

    pub fn move_yaw(&mut self, amount: f32) {
        self.yaw += amount;
        self.vectors_dirty = true;
        self.view_dirty = true;
    }

    pub fn move_pitch(&mut self, amount: f32) {
        self.pitch += amount;
        self.pitch = self.pitch.clamp(-89.0, 89.0);
        self.vectors_dirty = true;
        self.view_dirty = true;
    }

    pub fn zoom(&mut self, amount: f32) {
        self.set_fov(self.fov - amount);
    }

    fn calculate_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    fn calculate_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov.to_radians(), self.aspect, self.near, self.far)
    }

    fn calculate_right_vector(&self) -> Vec3 {
        self.front.cross(self.up).normalize()
    }

    fn calculate_front_vector(&self) -> Vec3 {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        let (yaw, pitch) = (self.yaw.to_radians(), self.pitch.to_radians());
        direction.x = yaw.cos() * pitch.cos();
        direction.y = pitch.sin();
        direction.z = yaw.sin() * pitch.cos();
        direction.normalize()
    }

    fn update_vectors_if_dirty(&mut self) {
        if self.vectors_dirty {
            self.front = self.calculate_front_vector();
            self.right = self.calculate_right_vector();
            self.vectors_dirty = false;
            self.view_dirty = true; // Mark view matrix dirty since it depends on the vectors
        }
    }

    fn update_view_if_dirty(&mut self) {
        if self.view_dirty {
            self.view = self.calculate_view_matrix();
            self.view_dirty = false;
        }
    }

    fn update_projection_if_dirty(&mut self) {
        if self.projection_dirty {
            self.projection = self.calculate_projection_matrix();
            self.projection_dirty = false;
        }
    }
}

impl Camera for FlyCamera {
    fn view(&mut self) -> Mat4 {
        self.update_vectors_if_dirty();
        self.update_view_if_dirty();
        self.view
    }

    fn projection(&mut self) -> Mat4 {
        self.update_projection_if_dirty();
        self.projection
    }

    fn position(&mut self) -> Vec3 {
        self.position
    }
}
