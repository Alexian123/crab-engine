pub mod exporter;
pub mod importer;

use asset_importer::Texture;
pub use engine_asset::*;

#[derive(Debug, Default)]
pub struct TextureDescription {
    pub name: String,
    pub texture: Option<Texture>,
}

#[derive(Debug, Default)]
pub struct ModelDescription {
    pub name: String,
    pub meshes: Vec<MeshAsset>,
    pub materials: Vec<MaterialAsset>,
    pub textures: Vec<TextureDescription>,
    pub root: NodeAsset,
}

pub use exporter::Exporter;
pub use importer::Importer;
