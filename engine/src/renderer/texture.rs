mod sampler_2d;
mod sampler_cube;

pub use sampler_2d::Sampler2D;
pub use sampler_cube::SamplerCube;

pub trait TextureSampler {
    fn bind(&self, unit: u32);
    fn unbind(&self, unit: u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn channels(&self) -> u32;
}
