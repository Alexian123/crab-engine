pub struct VertexArrayObject {
    pub(super) internal_vao: glow::VertexArray,
}

pub struct FrameBufferObject {
    pub(super) internal_fbo: glow::Framebuffer,
}

pub enum FrameBufferTextureAttachment {
    Color,
    Depth,
}

pub enum FrameBufferRenderBufferAttachment {
    Depth,
}

pub enum VertexBufferTarget {
    Array,
    Element,
}

pub enum VertexBufferDataUsage {
    StaticDraw,
    DynamicDraw,
}

pub struct VertexBufferObject {
    pub(super) internal_buffer: glow::Buffer,
}

pub struct RenderBufferObject {
    pub(super) internal_rbo: glow::Renderbuffer,
}

pub enum RenderBufferFormat {
    DepthComponent24,
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
    RGBA8,
    DepthComponent,
    DepthComponent24,
}

pub enum TextureDataType {
    UnsignedByte,
    UnsignedInt,
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
