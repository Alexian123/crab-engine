pub use crate::renderer::Texture;
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
    gl: Rc<glow::Context>,
    cache: HashMap<PathBuf, Rc<Texture>>,
}

impl TextureLoader {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self {
            gl,
            cache: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Rc<Texture>, TextureLoadError> {
        let path = std::fs::canonicalize(path.as_ref())?;
        if let Some(texture) = self.cache.get(&path) {
            return Ok(texture.clone());
        }

        let image = image::open(&path).map_err(TextureLoadError::ImageLoad)?;
        let image = image.flipv().into_rgba8();

        let width = image.width();
        let height = image.height();

        let data = image.into_raw();

        let texture = Rc::new(
            Texture::new(Rc::clone(&self.gl), width, height, 4, &data)
                .map_err(TextureLoadError::TextureCreate)?,
        );

        self.cache.insert(path, texture.clone());
        Ok(texture)
    }
}
