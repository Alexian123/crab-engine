#[derive(Clone, Copy, Debug)]
pub enum VertexFormat {
    Float32,
    Uint32,
    Int32,
    Uint8,
}

impl VertexFormat {
    pub fn gl_type(self) -> u32 {
        match self {
            VertexFormat::Float32 => glow::FLOAT,
            VertexFormat::Uint32 => glow::UNSIGNED_INT,
            VertexFormat::Int32 => glow::INT,
            VertexFormat::Uint8 => glow::UNSIGNED_BYTE,
        }
    }

    pub fn size(self) -> usize {
        match self {
            VertexFormat::Float32 => 4,
            VertexFormat::Uint32 => 4,
            VertexFormat::Int32 => 4,
            VertexFormat::Uint8 => 1,
        }
    }
}

pub struct VertexAttribute {
    pub location: u32,
    pub count: u32,
    pub format: VertexFormat,
    pub normalized: bool,
    pub offset: usize,
}

pub struct VertexLayout {
    pub attribs: Vec<VertexAttribute>,
}

impl VertexLayout {
    pub fn stride(&self) -> usize {
        self.attribs
            .iter()
            .map(|a| a.offset + a.count as usize * a.format.size() as usize)
            .max()
            .unwrap_or(0)
    }
}
