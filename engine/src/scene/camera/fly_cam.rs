use std::cell::{Cell, RefCell};

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

    front: RefCell<Vec3>,
    up: RefCell<Vec3>,
    right: RefCell<Vec3>,

    view: RefCell<Mat4>,
    projection: RefCell<Mat4>,

    view_dirty: Cell<bool>,
    projection_dirty: Cell<bool>,
    vectors_dirty: Cell<bool>,
}

impl FlyCamera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let cam = Self {
            fov,
            aspect,
            near,
            far,
            yaw: -90.0, // looking towards negative z-axis
            pitch: 0.0,
            position: Vec3::new(0.0, 0.0, 3.0),
            front: RefCell::new(Vec3::new(0.0, 0.0, -1.0)),
            up: RefCell::new(Vec3::new(0.0, 1.0, 0.0)),
            right: RefCell::new(Vec3::new(1.0, 0.0, 0.0)),
            view: RefCell::new(Mat4::IDENTITY),
            projection: RefCell::new(Mat4::IDENTITY),
            view_dirty: Cell::new(true),
            projection_dirty: Cell::new(true),
            vectors_dirty: Cell::new(true),
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
        *self.front.borrow()
    }

    pub fn up(&self) -> Vec3 {
        *self.up.borrow()
    }

    pub fn right(&self) -> Vec3 {
        *self.right.borrow()
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.view_dirty.set(true);
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov.clamp(1.0, 45.0);
        self.projection_dirty.set(true);
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.projection_dirty.set(true);
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.projection_dirty.set(true);
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.projection_dirty.set(true);
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.vectors_dirty.set(true);
        self.view_dirty.set(true);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(-89.0, 89.0);
        self.vectors_dirty.set(true);
        self.view_dirty.set(true);
    }

    pub fn move_x(&mut self, amount: f32) {
        self.update_vectors_if_dirty();
        self.position += *self.right.borrow() * amount;
        self.view_dirty.set(true);
    }

    pub fn move_y(&mut self, amount: f32) {
        self.position += *self.up.borrow() * amount;
        self.view_dirty.set(true);
    }

    pub fn move_z(&mut self, amount: f32) {
        self.update_vectors_if_dirty();
        self.position += *self.front.borrow() * amount;
        self.view_dirty.set(true);
    }

    pub fn move_yaw(&mut self, amount: f32) {
        self.yaw += amount;
        self.vectors_dirty.set(true);
        self.view_dirty.set(true);
    }

    pub fn move_pitch(&mut self, amount: f32) {
        self.pitch += amount;
        self.pitch = self.pitch.clamp(-89.0, 89.0);
        self.vectors_dirty.set(true);
        self.view_dirty.set(true);
    }

    pub fn zoom(&mut self, amount: f32) {
        self.set_fov(self.fov - amount);
    }

    fn calculate_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.position,
            self.position + *self.front.borrow(),
            *self.up.borrow(),
        )
    }

    fn calculate_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov.to_radians(), self.aspect, self.near, self.far)
    }

    fn calculate_right_vector(&self) -> Vec3 {
        self.front.borrow().cross(*self.up.borrow()).normalize()
    }

    fn calculate_front_vector(&self) -> Vec3 {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        let (yaw, pitch) = (self.yaw.to_radians(), self.pitch.to_radians());
        direction.x = yaw.cos() * pitch.cos();
        direction.y = pitch.sin();
        direction.z = yaw.sin() * pitch.cos();
        direction.normalize()
    }

    fn update_vectors_if_dirty(&self) {
        if self.vectors_dirty.get() {
            self.front.replace(self.calculate_front_vector());
            self.right.replace(self.calculate_right_vector());
            self.vectors_dirty.set(false);
            self.view_dirty.set(true); // Mark view matrix dirty since it depends on the vectors
        }
    }

    fn update_view_if_dirty(&self) {
        if self.view_dirty.get() {
            self.view.replace(self.calculate_view_matrix());
            self.view_dirty.set(false);
        }
    }

    fn update_projection_if_dirty(&self) {
        if self.projection_dirty.get() {
            self.projection.replace(self.calculate_projection_matrix());
            self.projection_dirty.set(false);
        }
    }
}

impl Camera for FlyCamera {
    fn view(&self) -> Mat4 {
        self.update_vectors_if_dirty();
        self.update_view_if_dirty();
        *self.view.borrow()
    }

    fn projection(&self) -> Mat4 {
        self.update_projection_if_dirty();
        *self.projection.borrow()
    }

    fn position(&self) -> Vec3 {
        self.position
    }
}
