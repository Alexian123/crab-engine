mod material_file_loader;
mod mesh_loader;
mod model_file_loader;
mod shader_loader;
mod texture_loader;

use crate::GfxContext;
use crate::renderer::{Material, Mesh};
use crate::scene::components::*;
use crate::scene::{Entity, Scene};
use crate::utils::HeightGenerator;
use engine_asset::*;
use glam::{Quat, Vec3};
use material_file_loader::*;
use mesh_loader::*;
use model_file_loader::*;
use shader_loader::*;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use texture_loader::*;

pub struct Loader {
    meshes: MeshLoader,
    shaders: ShaderLoader,
    textures: TextureLoader,
    material_files: MaterialFileLoader,
    model_files: ModelFileLoader,
    material_cache: HashMap<PathBuf, Rc<Material>>,
}

// TODO: DO NOT FAIL SILENTLY EVER
impl Loader {
    pub fn new(gfx: Rc<GfxContext>) -> Self {
        Self {
            meshes: MeshLoader::new(Rc::clone(&gfx)),
            shaders: ShaderLoader::new(Rc::clone(&gfx)),
            textures: TextureLoader::new(Rc::clone(&gfx)),
            material_files: MaterialFileLoader::new(),
            model_files: ModelFileLoader::new(),
            material_cache: HashMap::new(),
        }
    }

    pub fn load_model(&mut self, path: &Path, scene: &mut Scene) -> Option<Entity> {
        let path = std::fs::canonicalize(path).ok()?;

        if path.is_dir() || !path.parent()?.is_dir() {
            return None;
        }

        match self.model_files.load(path.clone()) {
            Ok(model_file) => {
                let world = scene.world_mut();
                let root_parent = world.create_entity();
                world.add_component(root_parent, NameComponent(model_file.name.clone()));
                world.add_component(
                    root_parent,
                    LocalTransformComponent {
                        position: Vec3::ZERO,
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                );
                self.load_node(
                    &path.parent().unwrap(),
                    &model_file.root,
                    Some(root_parent),
                    scene,
                );
                Some(root_parent)
            }
            Err(err) => {
                tracing::error!("Failed to load model: {}", err);
                None
            }
        }
    }

    fn load_node(
        &mut self,
        dir: &Path,
        node: &NodeAsset,
        parent: Option<Entity>,
        scene: &mut Scene,
    ) {
        let entity = scene.world_mut().create_entity();

        scene
            .world_mut()
            .add_component(entity, NameComponent(node.name.clone()));
        scene.world_mut().add_component(
            entity,
            LocalTransformComponent {
                position: Vec3::from_array(node.translation),
                rotation: Quat::from_array(node.rotation),
                scale: Vec3::from_array(node.scale),
            },
        );

        // if node has multiple mesh instances, create a child entity for each, otherwise add the mesh instance to the current entity
        if node.mesh_instances.len() == 1 {
            let mesh_instance = &node.mesh_instances[0];
            if let Some(mesh) = self.load_mesh(
                &dir.join(&mesh_instance.mesh)
                    .with_added_extension("mesh")
                    .as_path(),
            ) {
                scene
                    .world_mut()
                    .add_component(entity, MeshComponent { mesh });
            }

            if let Some(material) = self.load_material(
                dir.join(&mesh_instance.material)
                    .with_added_extension("mat")
                    .as_path(),
                Some(dir),
            ) {
                scene
                    .world_mut()
                    .add_component(entity, MaterialComponent { material });
            }
        } else {
            for (index, mesh_instance) in node.mesh_instances.iter().enumerate() {
                let child_entity = scene.world_mut().create_entity();
                scene.world_mut().add_component(
                    child_entity,
                    NameComponent(format!("{}_mesh_instance_{}", node.name, index)),
                );
                scene.world_mut().add_component(
                    child_entity,
                    LocalTransformComponent {
                        position: Vec3::ZERO,
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                );

                if let Some(mesh) = self.load_mesh(
                    dir.join(&mesh_instance.mesh)
                        .with_added_extension("mesh")
                        .as_path(),
                ) {
                    scene
                        .world_mut()
                        .add_component(child_entity, MeshComponent { mesh });
                }

                if let Some(material) = self.load_material(
                    dir.join(&mesh_instance.material)
                        .with_added_extension("mat")
                        .as_path(),
                    Some(dir),
                ) {
                    scene
                        .world_mut()
                        .add_component(child_entity, MaterialComponent { material });
                }

                // set parent of child entity to current entity
                scene.set_parent(child_entity, entity);
            }
        }

        // set parent of current entity to parent entity
        if let Some(parent) = parent {
            scene.set_parent(entity, parent);
        }

        // recursively load children
        for child in &node.children {
            self.load_node(dir, child, Some(entity), scene);
        }
    }

    pub fn load_material(
        &mut self,
        path: &Path,
        texture_dir: Option<&Path>, // None if material has absolute texture paths
    ) -> Option<Rc<Material>> {
        let path = std::fs::canonicalize(path).ok()?;

        if let Some(cached_material) = self.material_cache.get(&path) {
            return Some(Rc::clone(cached_material));
        }

        match self.material_files.load(path.clone()) {
            Ok(material_file) => {
                let shader = self.load_shader(
                    Path::new(&material_file.shader.vertex),
                    Path::new(&material_file.shader.fragment),
                )?;

                let mut material = Material::new(shader);

                for texture_path in &material_file.textures {
                    let mut texture_path = PathBuf::from(texture_path);
                    if let Some(texture_dir) = texture_dir {
                        texture_path = texture_dir.join(texture_path);
                    }
                    if let Some(texture) = self.load_texture(texture_path.as_path()) {
                        material.textures.push(texture);
                    } else {
                        tracing::error!(
                            "Failed to load texture: {}",
                            texture_path.as_os_str().to_string_lossy()
                        );
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

    pub fn load_mesh(&mut self, path: &Path) -> Option<Rc<Mesh>> {
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

    pub fn load_terrain_mesh(
        &mut self,
        size: usize,
        vertices_per_side: usize,
        uv_scale: f32,
        height_generator: Option<&HeightGenerator>,
    ) -> Option<Rc<Mesh>> {
        match self
            .meshes
            .load_terrain_mesh(size, vertices_per_side, uv_scale, height_generator)
        {
            Ok(mesh) => Some(mesh),
            Err(err) => {
                tracing::error!("Failed to load terrain mesh: {}", err);
                None
            }
        }
    }

    pub fn load_shader(&mut self, vertex: &Path, fragment: &Path) -> Option<Rc<ShaderProgram>> {
        match self.shaders.load(vertex, fragment) {
            Ok(shader) => Some(shader),
            Err(err) => {
                tracing::error!("Failed to load shader: {}", err);
                None
            }
        }
    }

    pub fn load_texture(&mut self, path: &Path) -> Option<Rc<Texture>> {
        match self.textures.load(path) {
            Ok(texture) => Some(texture),
            Err(err) => {
                tracing::error!("Failed to load texture: {}", err);
                None
            }
        }
    }
}
