use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeshAsset {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
    pub tangents: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialAsset {}

#[derive(Serialize, Deserialize)]
pub struct SceneAsset {}

#[derive(Serialize, Deserialize)]
pub struct AnimationAsset {}
