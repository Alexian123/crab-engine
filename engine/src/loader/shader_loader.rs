pub use crate::GfxContext;
pub use crate::renderer::ShaderProgram;
use crate::utils::preprocess_shader;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderLoadError {
    #[error("failed to create shader program: {0}")]
    ShaderProgramCreate(String),

    #[error("failed to read shader: {0}")]
    Io(#[from] std::io::Error),

    #[error("preprocess: {0}")]
    Preprocess(#[from] crate::utils::shader_preprocessor::ShaderPreprocessError),
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct EmbeddedShader {
    pub vertex_path: &'static str,
    pub fragment_path: &'static str,
}

#[derive(Hash, Eq, PartialEq)]
struct ShaderKey {
    vertex: PathBuf,
    fragment: PathBuf,
}

pub struct ShaderLoader {
    gfx: Rc<GfxContext>,
    cache: HashMap<ShaderKey, Rc<ShaderProgram>>,
    embedded_cache: HashMap<EmbeddedShader, Rc<ShaderProgram>>,
}

impl ShaderLoader {
    pub fn new(gfx: Rc<GfxContext>) -> Self {
        Self {
            gfx,
            cache: HashMap::new(),
            embedded_cache: HashMap::new(),
        }
    }

    pub fn load_embedded(
        &mut self,
        embedded_shader: &EmbeddedShader,
    ) -> Result<Rc<ShaderProgram>, ShaderLoadError> {
        if let Some(shader) = self.embedded_cache.get(embedded_shader) {
            return Ok(Rc::clone(shader));
        }

        let vertex_source = preprocess_shader(&embedded_shader.vertex_path)?;
        let fragment_source = preprocess_shader(&embedded_shader.fragment_path)?;

        let shader = Rc::new(
            ShaderProgram::new(Rc::clone(&self.gfx), &vertex_source, &fragment_source)
                .map_err(|e| ShaderLoadError::ShaderProgramCreate(e))?,
        );

        self.embedded_cache
            .insert(embedded_shader.clone(), Rc::clone(&shader));

        Ok(shader)
    }

    pub fn load<P1, P2>(
        &mut self,
        vert_path: P1,
        frag_path: P2,
    ) -> Result<Rc<ShaderProgram>, ShaderLoadError>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let vert_path = std::fs::canonicalize(vert_path.as_ref())?;
        let frag_path = std::fs::canonicalize(frag_path.as_ref())?;
        let key = ShaderKey {
            vertex: vert_path,
            fragment: frag_path,
        };

        if let Some(shader) = self.cache.get(&key) {
            return Ok(Rc::clone(shader));
        }

        let vertex_source = preprocess_shader(&key.vertex)?;
        let fragment_source = preprocess_shader(&key.fragment)?;

        let shader = Rc::new(
            ShaderProgram::new(Rc::clone(&self.gfx), &vertex_source, &fragment_source)
                .map_err(|e| ShaderLoadError::ShaderProgramCreate(e))?,
        );

        self.cache.insert(key, Rc::clone(&shader));

        Ok(shader)
    }
}
