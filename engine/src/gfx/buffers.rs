pub struct VertexArrayObject {
    pub(super) internal_vao: glow::VertexArray,
}

pub enum BufferTarget {
    Array,
    Element,
}

pub enum BufferDataUsage {
    StaticDraw,
    DynamicDraw,
}

pub struct BufferObject {
    pub(super) internal_buffer: glow::Buffer,
}

pub enum TextureTarget {
    Texture2D,
    TextureCubeMap,
}

pub enum TextureFormat {
    RED,
    RG,
    RGB,
    RGBA,
}

pub enum TextureWrapMode {
    Repeat,
    ClampToEdge,
}

pub enum TextureFilterMode {
    Linear,
    LinearMipmapLinear,
}

pub struct TextureObject {
    pub(super) internal_texture: glow::Texture,
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct ShaderObject {
    pub(super) internal_shader: glow::Shader,
}

pub struct ProgramObject {
    pub(super) internal_program: glow::Program,
}

#[derive(Clone, Copy)]
pub struct UniformLocationObject {
    pub(super) internal_location: glow::UniformLocation,
}
