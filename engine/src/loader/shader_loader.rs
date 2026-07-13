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

#[derive(Hash, Eq, PartialEq)]
struct ShaderKey {
    vertex: PathBuf,
    fragment: PathBuf,
}

pub struct ShaderLoader {
    gl: Rc<glow::Context>,
    cache: HashMap<ShaderKey, Rc<ShaderProgram>>,
}

impl ShaderLoader {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self {
            gl,
            cache: HashMap::new(),
        }
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
            ShaderProgram::new(Rc::clone(&self.gl), &vertex_source, &fragment_source)
                .map_err(|e| ShaderLoadError::ShaderProgramCreate(e))?,
        );

        self.cache.insert(key, Rc::clone(&shader));

        Ok(shader)
    }
}
