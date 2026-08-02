use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MeshAsset {
    pub name: String,
    pub vertex_count: usize,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
    pub tangents: Vec<[f32; 3]>,
    pub bitangents: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MaterialAsset {
    pub name: String,
    pub shader: ShaderDesc,
    pub textures: Vec<String>,
    pub params: MaterialParams,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ShaderDesc {
    pub vertex: String,
    pub fragment: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MaterialParams {
    pub shininess: f32,
    pub diffuse_index: Option<u32>,
    pub specular_index: Option<u32>,
    pub emission_index: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MeshInstance {
    pub mesh: PathBuf,
    pub material: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NodeAsset {
    pub name: String,
    pub mesh_instances: Vec<MeshInstance>,
    pub children: Vec<NodeAsset>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModelAsset {
    pub name: String,
    pub root: NodeAsset,
}
