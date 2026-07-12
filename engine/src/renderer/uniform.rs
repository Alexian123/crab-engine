use glow::HasContext;

pub trait Uniform {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation);
}

impl Uniform for f32 {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation) {
        unsafe {
            gl.uniform_1_f32(Some(&location), *self);
        }
    }
}

impl Uniform for i32 {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation) {
        unsafe {
            gl.uniform_1_i32(Some(&location), *self);
        }
    }
}

impl Uniform for glam::Vec2 {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation) {
        unsafe {
            gl.uniform_2_f32(Some(&location), self.x, self.y);
        }
    }
}

impl Uniform for glam::Vec3 {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation) {
        unsafe {
            gl.uniform_3_f32(Some(&location), self.x, self.y, self.z);
        }
    }
}

impl Uniform for glam::Vec4 {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation) {
        unsafe {
            gl.uniform_4_f32(Some(&location), self.x, self.y, self.z, self.w);
        }
    }
}

impl Uniform for glam::Mat4 {
    fn upload(&self, gl: &glow::Context, location: &glow::UniformLocation) {
        unsafe {
            gl.uniform_matrix_4_f32_slice(Some(&location), false, self.as_ref());
        }
    }
}
