mod material_file_loader;
mod mesh_loader;
mod shader_loader;
mod texture_loader;

use crate::loader::material_file_loader::*;
use crate::loader::mesh_loader::*;
use crate::loader::shader_loader::*;
use crate::loader::texture_loader::*;
use crate::renderer::Material;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Loader {
    meshes: MeshLoader,
    shaders: ShaderLoader,
    textures: TextureLoader,
    material_files: MaterialFileLoader,
    material_cache: HashMap<PathBuf, Rc<Material>>,
}

impl Loader {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self {
            meshes: MeshLoader::new(Rc::clone(&gl)),
            shaders: ShaderLoader::new(Rc::clone(&gl)),
            textures: TextureLoader::new(Rc::clone(&gl)),
            material_files: MaterialFileLoader::new(),
            material_cache: HashMap::new(),
        }
    }

    pub fn load_material(&mut self, path: &str) -> Option<Rc<Material>> {
        let path = std::fs::canonicalize(path).ok()?;

        if let Some(cached_material) = self.material_cache.get(&path) {
            return Some(Rc::clone(cached_material));
        }

        match self.material_files.load(path.clone()) {
            Ok(material_file) => {
                let shader =
                    self.load_shader(&material_file.shader.vertex, &material_file.shader.fragment)?;

                let mut material = Material::new(shader);

                for texture_path in &material_file.textures {
                    if let Some(texture) = self.load_texture(texture_path) {
                        material.textures.push(texture);
                    } else {
                        tracing::error!("Failed to load texture: {}", texture_path);
                    }
                }

                let params = &material_file.params;

                material.shininess = params.shininess;
                material.diffuse_index = params.diffuse_index;
                material.specular_index = params.specular_index;
                material.emission_index = params.emission_index;

                let material = Rc::new(material);
                self.material_cache.insert(path, Rc::clone(&material));
                Some(material)
            }
            Err(err) => {
                tracing::error!("Failed to load material: {}", err);
                None
            }
        }
    }

    pub fn load_mesh(&mut self, path: &str) -> Option<Rc<Mesh>> {
        match self.meshes.load(path) {
            Ok(mesh) => Some(mesh),
            Err(err) => {
                tracing::error!("Failed to load mesh: {}", err);
                None
            }
        }
    }

    pub fn load_cube_mesh(&mut self) -> Option<Rc<Mesh>> {
        match self.meshes.load_cube() {
            Ok(mesh) => Some(mesh),
            Err(err) => {
                tracing::error!("Failed to load cube mesh primitve: {}", err);
                None
            }
        }
    }

    pub fn load_shader(&mut self, vertex: &str, fragment: &str) -> Option<Rc<ShaderProgram>> {
        match self.shaders.load(vertex, fragment) {
            Ok(shader) => Some(shader),
            Err(err) => {
                tracing::error!("Failed to load shader: {}", err);
                None
            }
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Option<Rc<Texture>> {
        match self.textures.load(path) {
            Ok(texture) => Some(texture),
            Err(err) => {
                tracing::error!("Failed to load texture: {}", err);
                None
            }
        }
    }
}
