use super::{ModelDescription, TextureDescription};
use bincode::{config, serde};
use engine_asset::*;
use std::cell::Cell;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("failed to export file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to encode file: {0}")]
    Encode(#[from] bincode::error::EncodeError),

    #[error("failed to serialize file: {0}")]
    Json(#[from] serde_json::error::Error),
}

pub struct Exporter {
    unnamed_texture_index: Cell<u32>,
}

impl Exporter {
    pub fn new() -> Self {
        Self {
            unnamed_texture_index: Cell::new(0),
        }
    }

    pub fn export<P: AsRef<Path>>(
        &self,
        model: ModelDescription,
        path: P,
    ) -> Result<ModelAsset, ExportError> {
        let path = std::fs::canonicalize(path.as_ref())?;

        self.unnamed_texture_index.set(0);

        // export mesh
        for mesh in model.meshes.iter() {
            self.export_mesh(mesh, &path)?;
        }

        // export materials
        for material in model.materials.iter() {
            self.export_material(material, &path)?;
        }

        // export textures
        for texture in model.textures.iter() {
            self.export_texture(texture, &path)?;
        }

        // export node hierarchy
        let model = ModelAsset {
            name: model.name,
            root: model.root,
        };
        let path = path.join(format!("{}.model", model.name));
        let json = serde_json::to_string_pretty(&model)?;
        fs::write(&path, json)?;

        Ok(model)
    }

    fn export_mesh(&self, mesh: &MeshAsset, path: &Path) -> Result<(), ExportError> {
        let path = path.join(format!("{}.mesh", mesh.name));
        let bytes = serde::encode_to_vec(mesh, config::standard())?;
        fs::write(path, bytes)?;
        Ok(())
    }

    fn export_material(&self, material: &MaterialAsset, path: &Path) -> Result<(), ExportError> {
        let path = path.join(format!("{}.mat", material.name));
        let json = serde_json::to_string_pretty(material)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn export_texture(
        &self,
        texture_desc: &TextureDescription,
        path: &Path,
    ) -> Result<(), ExportError> {
        let as_texture =
            texture_desc
                .texture
                .as_ref()
                .ok_or(ExportError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "texture not found",
                )))?;

        let path = path.join(format!("./{}", texture_desc.name,));
        as_texture
            .save_to_file(Path::new(&path))
            .expect("Failed to process texture");
        Ok(())
    }
}
