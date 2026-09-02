use crate::GfxContext;
pub use crate::renderer::MeshTextureSampler2D;
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
}

pub struct TextureLoader {
    gfx: Rc<GfxContext>,
    cache: HashMap<PathBuf, Rc<MeshTextureSampler2D>>,
}

impl TextureLoader {
    pub fn new(gfx: Rc<GfxContext>) -> Self {
        Self {
            gfx,
            cache: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<Rc<MeshTextureSampler2D>, TextureLoadError> {
        let path = std::fs::canonicalize(path.as_ref())?;

        if let Some(texture) = self.cache.get(&path) {
            return Ok(Rc::clone(texture));
        }

        let image = image::open(&path).map_err(TextureLoadError::ImageLoad)?;
        let image = image.flipv().into_rgba8();

        let width = image.width();
        let height = image.height();

        let data = image.into_raw();

        let texture = Rc::new(
            MeshTextureSampler2D::new(Rc::clone(&self.gfx), width, height, 4, &data)
                .map_err(TextureLoadError::TextureCreate)?,
        );

        self.cache.insert(path, Rc::clone(&texture));
        Ok(texture)
    }
}
