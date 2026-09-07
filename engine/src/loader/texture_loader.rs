use crate::GfxContext;
pub use crate::renderer::{Sampler2D, SamplerCube};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextureLoadError {
    #[error("failed to load texture: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to load image")]
    ImageLoad(#[source] image::ImageError),

    #[error("failed to create texture: {0}")]
    TextureCreate(String),

    #[error("failed to create cube map: {0}")]
    CubeMapCreate(String),

    #[error("inconsistent face sizes")]
    InconsistentFaceSizes,
}

pub struct TextureLoader {
    gfx: Rc<GfxContext>,
    sampler_2d_cache: HashMap<PathBuf, Rc<Sampler2D>>,
    cubemap_cache: HashMap<PathBuf, Rc<SamplerCube>>,
}

impl TextureLoader {
    pub fn new(gfx: Rc<GfxContext>) -> Self {
        Self {
            gfx,
            sampler_2d_cache: HashMap::new(),
            cubemap_cache: HashMap::new(),
        }
    }

    pub fn load_cubemap<P: AsRef<Path>>(
        &mut self,
        path: P,
        face_images: [&str; 6],
    ) -> Result<Rc<SamplerCube>, TextureLoadError> {
        let path = std::fs::canonicalize(path.as_ref())?;

        if let Some(cubemap) = self.cubemap_cache.get(&path) {
            return Ok(Rc::clone(cubemap));
        }

        let face_paths: [PathBuf; 6] = face_images.map(|face| path.join(face));

        let mut width = 0;
        let mut height = 0;
        let mut face_data = Vec::with_capacity(6);

        for (i, path) in face_paths.iter().enumerate() {
            let image = image::open(path)
                .map_err(TextureLoadError::ImageLoad)?
                .into_rgba8();

            let image_width = image.width();
            let image_height = image.height();

            if i == 0 {
                width = image_width;
                height = image_height;
            } else if image_width != width || image_height != height {
                return Err(TextureLoadError::InconsistentFaceSizes);
            }

            face_data.push(image.into_raw());
        }

        let face_data: [Vec<u8>; 6] = face_data.try_into().expect("exactly 6 faces");

        let faces: [&[u8]; 6] = std::array::from_fn(|i| face_data[i].as_slice());

        let cubemap = Rc::new(
            SamplerCube::new(Rc::clone(&self.gfx), width, height, 4, faces)
                .map_err(TextureLoadError::CubeMapCreate)?,
        );

        self.cubemap_cache.insert(path, Rc::clone(&cubemap));
        Ok(cubemap)
    }

    pub fn load_sampler_2d<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<Rc<Sampler2D>, TextureLoadError> {
        let path = std::fs::canonicalize(path.as_ref())?;

        if let Some(texture) = self.sampler_2d_cache.get(&path) {
            return Ok(Rc::clone(texture));
        }

        let image = image::open(&path).map_err(TextureLoadError::ImageLoad)?;
        let image = image.flipv().into_rgba8();

        let width = image.width();
        let height = image.height();

        let data = image.into_raw();

        let texture = Rc::new(
            Sampler2D::new(Rc::clone(&self.gfx), width, height, 4, &data)
                .map_err(TextureLoadError::TextureCreate)?,
        );

        self.sampler_2d_cache.insert(path, Rc::clone(&texture));
        Ok(texture)
    }
}
