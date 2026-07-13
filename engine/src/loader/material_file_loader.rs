use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MaterialFile {
    pub shader: ShaderDesc,

    #[serde(default)]
    pub textures: Vec<String>,

    pub params: MaterialParams,
}

#[derive(Deserialize)]
pub struct ShaderDesc {
    pub vertex: String,
    pub fragment: String,
}

#[derive(Deserialize, Default)]
pub struct MaterialParams {
    pub shininess: f32,
    pub diffuse_index: u32,
    pub specular_index: u32,
    pub emission_index: u32,
}

#[derive(Debug, Error)]
pub enum MaterialLoadError {
    #[error("error reading material file: {0}")]
    ReadMaterialFile(#[source] std::io::Error),

    #[error("json parsing failed: {0}")]
    JsonParsing(#[source] serde_json::Error),
}

pub struct MaterialFileLoader {
    cache: HashMap<PathBuf, Rc<MaterialFile>>,
}

impl MaterialFileLoader {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Rc<MaterialFile>, MaterialLoadError> {
        let path = path.as_ref().to_path_buf();
        if let Some(material) = self.cache.get(&path) {
            return Ok(material.clone());
        }

        let json = std::fs::read_to_string(&path).map_err(MaterialLoadError::ReadMaterialFile)?;
        let material_file: MaterialFile =
            serde_json::from_str(&json).map_err(MaterialLoadError::JsonParsing)?;

        let material_file = Rc::new(material_file);
        self.cache.insert(path, Rc::clone(&material_file));
        Ok(material_file)
    }
}
