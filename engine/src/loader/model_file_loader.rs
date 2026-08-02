use engine_asset::ModelAsset;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelLoadError {
    #[error("error reading model file: {0}")]
    ReadModelFile(#[source] std::io::Error),

    #[error("json parsing failed: {0}")]
    JsonParsing(#[source] serde_json::Error),
}

pub struct ModelFileLoader {
    cache: HashMap<PathBuf, Rc<ModelAsset>>,
}

impl ModelFileLoader {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Rc<ModelAsset>, ModelLoadError> {
        let path = path.as_ref().to_path_buf();
        if let Some(model) = self.cache.get(&path) {
            return Ok(model.clone());
        }

        let json = std::fs::read_to_string(&path).map_err(ModelLoadError::ReadModelFile)?;
        let model_file: ModelAsset =
            serde_json::from_str(&json).map_err(ModelLoadError::JsonParsing)?;

        let model_file = Rc::new(model_file);
        self.cache.insert(
            PathBuf::from(model_file.name.clone()),
            Rc::clone(&model_file),
        );
        Ok(model_file)
    }
}
