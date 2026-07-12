use thiserror::Error;

pub use crate::renderer::Mesh;
use crate::renderer::vertex::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Error)]
pub enum MeshLoadError {
    //#[error("mesh file not found: {0}")]
    //MeshFileNotFound(String),
    #[error("invalid mesh: {0}")]
    InvalidMesh(String),

    #[error("path canonicalization failed: {0}")]
    PathCanonicalization(#[source] std::io::Error),
}

pub struct MeshLoader {
    gl: Rc<glow::Context>,
    cache: HashMap<PathBuf, Rc<Mesh>>,
}

impl MeshLoader {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self {
            gl,
            cache: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Rc<Mesh>, MeshLoadError> {
        let path =
            std::fs::canonicalize(path.as_ref()).map_err(MeshLoadError::PathCanonicalization)?;

        if let Some(mesh) = self.cache.get(&path) {
            return Ok(Rc::clone(mesh));
        }

        unimplemented!("Load mesh with a library");
    }

    pub fn load_cube(&mut self) -> Result<Rc<Mesh>, MeshLoadError> {
        let path = PathBuf::from("cube.obj");

        if let Some(mesh) = self.cache.get(&path) {
            return Ok(Rc::clone(mesh));
        }

        const POSITIONS: &[f32] = &[
            -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5,
            -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5,
            0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
            -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
            -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5,
            -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5,
            -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
        ];
        const TEXTURE_COORDS: &[f32] = &[
            0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        const NORMALS: &[f32] = &[
            0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0,
            0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
            0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        ];

        let mut vertices = Vec::with_capacity(36 * 8);

        for i in 0..36 {
            vertices.push(POSITIONS[i * 3 + 0]);
            vertices.push(POSITIONS[i * 3 + 1]);
            vertices.push(POSITIONS[i * 3 + 2]);

            vertices.push(TEXTURE_COORDS[i * 2 + 0]);
            vertices.push(TEXTURE_COORDS[i * 2 + 1]);

            vertices.push(NORMALS[i * 3 + 0]);
            vertices.push(NORMALS[i * 3 + 1]);
            vertices.push(NORMALS[i * 3 + 2]);
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
            Mesh::new(Rc::clone(&self.gl), &vertices, &[], &layout)
                .map_err(MeshLoadError::InvalidMesh)?,
        );

        self.cache.insert(path, Rc::clone(&mesh));
        Ok(mesh)
    }
}
