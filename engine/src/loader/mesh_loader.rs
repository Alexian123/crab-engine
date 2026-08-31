use crate::GfxContext;
use crate::gfx::vertex::*;
use crate::renderer::Mesh;
use crate::utils::HeightGenerator;
use bincode::{config, serde};
use engine_asset::MeshAsset;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeshLoadError {
    #[error("invalid mesh: {0}")]
    InvalidMesh(String),

    #[error("failed to load mesh: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to decode mesh: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
}

pub struct MeshLoader {
    gfx: Rc<GfxContext>,
    cache: HashMap<PathBuf, Rc<Mesh>>,
}

impl MeshLoader {
    pub fn new(gfx: Rc<GfxContext>) -> Self {
        Self {
            gfx,
            cache: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Rc<Mesh>, MeshLoadError> {
        let path = std::fs::canonicalize(path.as_ref())?;

        if let Some(mesh) = self.cache.get(&path) {
            return Ok(Rc::clone(mesh));
        }

        let bytes = std::fs::read(path.clone())?;

        let (mesh_data, _) =
            serde::borrow_decode_from_slice::<MeshAsset, _>(bytes.as_slice(), config::standard())?;

        let mut vertices = Vec::new();

        for i in 0..mesh_data.vertex_count {
            vertices.push(mesh_data.positions[i][0]);
            vertices.push(mesh_data.positions[i][1]);
            vertices.push(mesh_data.positions[i][2]);

            vertices.push(mesh_data.texcoords[i][0]);
            vertices.push(mesh_data.texcoords[i][1]);

            vertices.push(mesh_data.normals[i][0]);
            vertices.push(mesh_data.normals[i][1]);
            vertices.push(mesh_data.normals[i][2]);
        }

        let layout = VertexLayout {
            attribs: vec![
                VertexAttribute {
                    location: 0,
                    count: 3,
                    format: VertexFormat::Float32,
                    normalized: false,
                    offset: 0,
                },
                VertexAttribute {
                    location: 2,
                    count: 2,
                    format: VertexFormat::Float32,
                    normalized: false,
                    offset: 3 * std::mem::size_of::<f32>(),
                },
                VertexAttribute {
                    location: 3,
                    count: 3,
                    format: VertexFormat::Float32,
                    normalized: false,
                    offset: 5 * std::mem::size_of::<f32>(),
                },
            ],
        };

        let mesh = Rc::new(
            Mesh::new(Rc::clone(&self.gfx), &vertices, &mesh_data.indices, &layout)
                .map_err(MeshLoadError::InvalidMesh)?,
        );

        self.cache.insert(path, Rc::clone(&mesh));
        Ok(mesh)
    }

    pub fn load_terrain_mesh(
        &mut self,
        size: usize,
        vertices_per_side: usize,
        uv_scale: f32,
        height_generator: Option<&HeightGenerator>,
    ) -> Result<(Rc<Mesh>, Vec<f32>), MeshLoadError> {
        let path = PathBuf::from(format!(
            "terrain_{}_{}_{}_{}.obj",
            size,
            vertices_per_side,
            uv_scale,
            if let Some(generator) = height_generator {
                generator.get_hash()
            } else {
                0
            }
        ));

        if let Some(mesh) = self.cache.get(&path) {
            return Ok((Rc::clone(mesh), Vec::new()));
        }

        let count = vertices_per_side * vertices_per_side;
        let mut vertices: Vec<f32> = Vec::with_capacity(count * 8);
        let mut height_map: Vec<f32> = Vec::with_capacity(count);

        for i in 0..vertices_per_side {
            for j in 0..vertices_per_side {
                let x = (j as f32 / (vertices_per_side - 1) as f32) * size as f32;
                let z = (i as f32 / (vertices_per_side - 1) as f32) * size as f32;

                let height = if let Some(generator) = height_generator {
                    generator.generate(j as i32, i as i32)
                } else {
                    0.0
                };

                height_map.push(height);

                // positions
                vertices.push(x);
                vertices.push(height);
                vertices.push(z);

                // texture coords
                vertices.push(x / uv_scale);
                vertices.push(z / uv_scale);

                // normals
                let normal = if let Some(generator) = height_generator {
                    let (x, z) = (j as i32, i as i32);
                    let height_left = generator.generate(x - 1, z);
                    let height_right = generator.generate(x + 1, z);
                    let height_up = generator.generate(x, z + 1);
                    let height_down = generator.generate(x, z - 1);
                    glam::Vec3::new(height_left - height_right, 2.0, height_down - height_up)
                        .normalize()
                } else {
                    glam::vec3(0.0, 1.0, 0.0)
                };
                vertices.push(normal.x);
                vertices.push(normal.y);
                vertices.push(normal.z);
            }
        }

        let mut indices: Vec<u32> = vec![0; 6 * (vertices_per_side - 1) * (vertices_per_side - 1)];
        let mut idx = 0;

        for i in 0..(vertices_per_side - 1) {
            for j in 0..(vertices_per_side - 1) {
                let top_left = i * vertices_per_side + j;
                let top_right = top_left + 1;
                let bottom_left = (i + 1) * vertices_per_side + j;
                let bottom_right = bottom_left + 1;

                indices[idx] = top_left as u32;
                indices[idx + 1] = bottom_left as u32;
                indices[idx + 2] = top_right as u32;
                indices[idx + 3] = top_right as u32;
                indices[idx + 4] = bottom_left as u32;
                indices[idx + 5] = bottom_right as u32;
                idx += 6;
            }
        }

        let layout = VertexLayout {
            attribs: vec![
                VertexAttribute {
                    location: 0,
                    count: 3,
                    format: VertexFormat::Float32,
                    normalized: false,
                    offset: 0,
                },
                VertexAttribute {
                    location: 2,
                    count: 2,
                    format: VertexFormat::Float32,
                    normalized: false,
                    offset: 3 * std::mem::size_of::<f32>(),
                },
                VertexAttribute {
                    location: 3,
                    count: 3,
                    format: VertexFormat::Float32,
                    normalized: false,
                    offset: 5 * std::mem::size_of::<f32>(),
                },
            ],
        };

        let mesh = Rc::new(
            Mesh::new(Rc::clone(&self.gfx), &vertices, &indices, &layout)
                .map_err(MeshLoadError::InvalidMesh)?,
        );

        self.cache.insert(path, Rc::clone(&mesh));
        Ok((mesh, height_map))
    }
}
