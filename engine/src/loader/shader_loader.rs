use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

pub use crate::renderer::ShaderProgram;

#[derive(Debug, Error)]
pub enum ShaderLoadError {
    #[error("failed to read vertex shader '{path}'")]
    VertexShaderRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to read fragment shader '{path}'")]
    FragmentShaderRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to create shader program: {0}")]
    ShaderProgramCreate(String),

    #[error("path canonicalization failed: {0}")]
    PathCanonicalization(#[source] std::io::Error),
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
        let vert_path = std::fs::canonicalize(vert_path.as_ref())
            .map_err(ShaderLoadError::PathCanonicalization)?;
        let frag_path = std::fs::canonicalize(frag_path.as_ref())
            .map_err(ShaderLoadError::PathCanonicalization)?;
        let key = ShaderKey {
            vertex: vert_path,
            fragment: frag_path,
        };

        if let Some(shader) = self.cache.get(&key) {
            return Ok(Rc::clone(shader));
        }

        let vertex_code = std::fs::read_to_string(&key.vertex).map_err(|e| {
            ShaderLoadError::VertexShaderRead {
                path: key.vertex.clone(),
                source: e,
            }
        })?;
        let fragment_code = std::fs::read_to_string(&key.fragment).map_err(|e| {
            ShaderLoadError::FragmentShaderRead {
                path: key.fragment.clone(),
                source: e,
            }
        })?;

        let shader = Rc::new(
            ShaderProgram::new(
                Rc::clone(&self.gl),
                vertex_code.as_str(),
                fragment_code.as_str(),
            )
            .map_err(|e| ShaderLoadError::ShaderProgramCreate(e))?,
        );

        self.cache.insert(key, Rc::clone(&shader));

        Ok(shader)
    }
}
