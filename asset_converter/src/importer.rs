use super::{ModelDescription, TextureDescription};
use asset_importer::mesh::Mesh;
use asset_importer::node::Node;
use asset_importer::postprocess::PostProcessSteps;
use asset_importer::scene::Scene;
use asset_importer::{Material, Texture, TextureType};
use engine_asset::*;
use std::cell::Cell;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("failed to import model: {0}")]
    Io(#[from] std::io::Error),
}

pub struct Importer {
    as_importer: asset_importer::Importer,
    unnamed_mesh_index: Cell<usize>,
    unnamed_material_index: Cell<usize>,
    unnamed_texture_index: Cell<usize>,
}

impl Importer {
    pub fn new() -> Self {
        Self {
            as_importer: asset_importer::Importer::new(),
            unnamed_mesh_index: Cell::new(0),
            unnamed_material_index: Cell::new(0),
            unnamed_texture_index: Cell::new(0),
        }
    }

    pub fn import<P: AsRef<Path>>(&self, path: P) -> Result<ModelDescription, ImportError> {
        let path = std::fs::canonicalize(path.as_ref())?;

        self.unnamed_mesh_index.set(0);
        self.unnamed_material_index.set(0);
        self.unnamed_texture_index.set(0);

        let mut model = ModelDescription::default();
        model.name = path
            .components()
            .last()
            .and_then(|c| c.as_os_str().to_str())
            .unwrap_or("untitled")
            .to_string();

        let as_scene = self
            .as_importer
            .read_file(path)
            .with_post_process(PostProcessSteps::TRIANGULATE | PostProcessSteps::GEN_NORMALS)
            .import()
            .expect("Failed to load scene");

        for as_mesh in as_scene.meshes() {
            model.meshes.push(self.process_mesh(&as_mesh));
        }

        for as_material in as_scene.materials() {
            model.materials.push(self.process_material(&as_material));
        }

        for as_texture in as_scene.textures() {
            model.textures.push(self.process_texture(&as_texture));
        }

        let root = self.process_node(&as_scene.root_node().unwrap(), &as_scene, &model);
        model.root = root;

        Ok(model)
    }

    fn process_node(
        &self,
        as_node: &Node,
        as_scene: &Scene,
        model: &ModelDescription,
    ) -> NodeAsset {
        let mut node = NodeAsset::default();
        node.name = as_node.name();

        for i in as_node.mesh_indices() {
            if let Some(as_mesh) = as_scene.mesh(i) {
                let mesh = model.meshes.get(i).unwrap();
                let material = model.materials.get(as_mesh.material_index()).unwrap();
                node.mesh_instances.push(MeshInstance {
                    mesh: mesh.name.clone().into(),
                    material: material.name.clone().into(),
                });
            }
        }

        let (scale, rotation, translation) =
            as_node.transformation().to_scale_rotation_translation();
        node.scale = [scale.x, scale.y, scale.z];
        node.rotation = [rotation.x, rotation.y, rotation.z, rotation.w];
        node.translation = [translation.x, translation.y, translation.z];

        for child in as_node.children() {
            node.children
                .push(self.process_node(&child, as_scene, model));
        }

        node
    }

    fn process_mesh(&self, as_mesh: &Mesh) -> MeshAsset {
        let mut mesh = MeshAsset::default();

        mesh.name = String::from(as_mesh.name());
        if mesh.name.is_empty() {
            mesh.name = format!("mesh_{}", self.unnamed_mesh_index.get());
            self.unnamed_mesh_index
                .set(self.unnamed_mesh_index.get() + 1);
        }

        let vertices = as_mesh.vertices();

        for i in 0..as_mesh.num_vertices() {
            mesh.positions
                .push([vertices[i].x, vertices[i].y, vertices[i].z]);

            if as_mesh.has_normals() {
                let normals = as_mesh.normals().unwrap();
                mesh.normals
                    .push([normals[i].x, normals[i].y, normals[i].z]);
            } else {
                mesh.normals.push([0.0, 0.0, 0.0]);
            }

            if as_mesh.has_texture_coords(0) {
                let texcoords = as_mesh.texture_coords(0).unwrap();
                mesh.texcoords.push([texcoords[i].x, texcoords[i].y]);
            } else {
                mesh.texcoords.push([0.0, 0.0]);
            }

            if as_mesh.has_tangents() {
                let tangents = as_mesh.tangents().unwrap();
                mesh.tangents
                    .push([tangents[i].x, tangents[i].y, tangents[i].z]);
            } else {
                mesh.tangents.push([0.0, 0.0, 0.0]);
            }

            if as_mesh.has_bitangents() {
                let bitangents = as_mesh.bitangents().unwrap();
                mesh.bitangents
                    .push([bitangents[i].x, bitangents[i].y, bitangents[i].z]);
            } else {
                mesh.bitangents.push([0.0, 0.0, 0.0]);
            }
        }

        for face in as_mesh.faces() {
            for index in face.indices() {
                mesh.indices.push(*index);
            }
        }

        mesh
    }

    fn process_material(&self, as_material: &Material) -> MaterialAsset {
        let mut material = MaterialAsset {
            name: String::from(as_material.name()),
            shader: ShaderDesc {
                vertex: String::from("./assets/shaders/static_shader.vert"),
                fragment: String::from("./assets/shaders/static_shader.frag"),
            },
            textures: Vec::new(),
            params: MaterialParams {
                shininess: as_material.shininess().unwrap_or(32.0),
                diffuse_index: None,
                specular_index: None,
                emission_index: None,
            },
        };
        if material.name.is_empty() {
            material.name = format!("material_{}", self.unnamed_material_index.get());
            self.unnamed_material_index
                .set(self.unnamed_material_index.get() + 1);
        }

        if as_material.texture_count(TextureType::Diffuse) > 0 {
            let diffuse_texture = as_material.texture(TextureType::Diffuse, 0).unwrap();
            material.textures.push(String::from(diffuse_texture.path));
            material.params.diffuse_index = Some(0);
        }

        if as_material.texture_count(TextureType::Specular) > 0 {
            let specular_texture = as_material.texture(TextureType::Specular, 0).unwrap();
            material.params.specular_index = Some(material.textures.len() as u32);
            material.textures.push(String::from(specular_texture.path));
        }

        if as_material.texture_count(TextureType::Emissive) > 0 {
            let emission_texture = as_material.texture(TextureType::Emissive, 0).unwrap();
            material.params.emission_index = Some(material.textures.len() as u32);
            material.textures.push(String::from(emission_texture.path));
        }

        material
    }

    pub fn process_texture(&self, as_texture: &Texture) -> TextureDescription {
        let mut texture = TextureDescription::default();

        if let Some(filename) = as_texture.filename()
            && !filename.is_empty()
        {
            texture.name = String::from(filename);
        } else {
            texture.name = format!("texture_{}", self.unnamed_texture_index.get());
            self.unnamed_texture_index
                .set(self.unnamed_texture_index.get() + 1);
        }

        texture.texture = Some(as_texture.clone());

        texture
    }
}
