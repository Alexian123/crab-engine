pub mod buffers;
pub mod vertex;

use buffers::*;
use glow::HasContext;
use vertex::*;

pub enum DrawMode {
    Triangles,
    Lines,
    Points,
}

pub struct GfxContext {
    gl: glow::Context,
}

impl GfxContext {
    pub const COLOR_BUFFER_BIT: u32 = glow::COLOR_BUFFER_BIT;
    pub const DEPTH_BUFFER_BIT: u32 = glow::DEPTH_BUFFER_BIT;

    pub fn new(gl: glow::Context) -> Self {
        Self { gl }
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
        }
    }

    pub fn set_depth_test(&self, enabled: bool) {
        unsafe {
            if enabled {
                self.gl.enable(glow::DEPTH_TEST);
            } else {
                self.gl.disable(glow::DEPTH_TEST);
            }
        }
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(x, y, width, height);
        }
    }

    pub fn clear(&self, mask: u32) {
        unsafe {
            self.gl.clear(mask);
        }
    }

    pub fn create_vao(&self) -> Result<VertexArrayObject, String> {
        let vao = unsafe { self.gl.create_vertex_array()? };
        Ok(VertexArrayObject { internal_vao: vao })
    }

    pub fn delete_vao(&self, vao: &VertexArrayObject) {
        unsafe {
            self.gl.delete_vertex_array(vao.internal_vao);
        }
    }

    pub fn bind_vao(&self, vao: Option<&VertexArrayObject>) {
        unsafe {
            self.gl.bind_vertex_array(vao.map(|v| v.internal_vao));
        }
    }

    pub fn create_buffer(&self) -> Result<BufferObject, String> {
        let buffer = unsafe { self.gl.create_buffer()? };
        Ok(BufferObject {
            internal_buffer: buffer,
        })
    }

    pub fn delete_buffer(&self, buffer: &BufferObject) {
        unsafe {
            self.gl.delete_buffer(buffer.internal_buffer);
        }
    }

    pub fn bind_buffer(&self, target: BufferTarget, buffer: Option<&BufferObject>) {
        unsafe {
            self.gl.bind_buffer(
                self.get_target_u32(target),
                buffer.map(|b| b.internal_buffer),
            );
        }
    }

    fn get_target_u32(&self, target: BufferTarget) -> u32 {
        match target {
            BufferTarget::Array => glow::ARRAY_BUFFER,
            BufferTarget::Element => glow::ELEMENT_ARRAY_BUFFER,
        }
    }

    pub fn set_buffer_data_u8(&self, target: BufferTarget, usage: BufferDataUsage, data: &[u8]) {
        unsafe {
            self.gl.buffer_data_u8_slice(
                self.get_target_u32(target),
                data,
                self.get_usage_u32(usage),
            );
        }
    }

    fn get_usage_u32(&self, usage: BufferDataUsage) -> u32 {
        match usage {
            BufferDataUsage::StaticDraw => glow::STATIC_DRAW,
            BufferDataUsage::DynamicDraw => glow::DYNAMIC_DRAW,
        }
    }

    pub fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe {
            self.gl.enable_vertex_attrib_array(index);
        }
    }

    pub fn set_vertex_attrib_pointer(&self, attrib: &VertexAttribute, stride: usize) {
        unsafe {
            self.gl.vertex_attrib_pointer_f32(
                attrib.location,
                attrib.count as i32,
                self.get_vertex_format_u32(attrib.format),
                attrib.normalized,
                stride as i32,
                attrib.offset as i32,
            );
        }
    }

    fn get_vertex_format_u32(&self, format: VertexFormat) -> u32 {
        match format {
            VertexFormat::Float32 => glow::FLOAT,
            VertexFormat::Uint32 => glow::UNSIGNED_INT,
            VertexFormat::Int32 => glow::INT,
            VertexFormat::Uint8 => glow::UNSIGNED_BYTE,
        }
    }

    pub fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        unsafe {
            self.gl
                .draw_arrays(self.get_draw_mode_u32(mode), first, count);
        }
    }

    pub fn draw_elements(&self, mode: DrawMode, count: i32, offset: i32) {
        unsafe {
            self.gl.draw_elements(
                self.get_draw_mode_u32(mode),
                count,
                glow::UNSIGNED_INT,
                offset,
            );
        }
    }

    fn get_draw_mode_u32(&self, mode: DrawMode) -> u32 {
        match mode {
            DrawMode::Triangles => glow::TRIANGLES,
            DrawMode::Lines => glow::LINES,
            DrawMode::Points => glow::POINTS,
        }
    }

    pub fn create_texture_object(&self) -> Result<TextureObject, String> {
        let texture = unsafe { self.gl.create_texture()? };
        Ok(TextureObject {
            internal_texture: texture,
        })
    }

    pub fn delete_texture_object(&self, texture: &TextureObject) {
        unsafe {
            self.gl.delete_texture(texture.internal_texture);
        }
    }

    pub fn set_active_texture(&self, unit_offset: u32) {
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + unit_offset);
        }
    }

    pub fn bind_texture(&self, target: TextureTarget, texture: Option<&TextureObject>) {
        unsafe {
            self.gl.bind_texture(
                self.get_texture_target_u32(target),
                texture.map(|t| t.internal_texture),
            );
        }
    }

    fn get_texture_target_u32(&self, target: TextureTarget) -> u32 {
        match target {
            TextureTarget::Texture2D => glow::TEXTURE_2D,
            TextureTarget::TextureCubeMap => glow::TEXTURE_CUBE_MAP,
        }
    }

    pub fn set_tex_image_2d(
        &self,
        target: TextureTarget,
        level: i32,
        internal_format: TextureFormat,
        width: i32,
        height: i32,
        border: i32,
        format: TextureFormat,
        pixels: Option<&[u8]>,
    ) {
        unsafe {
            self.gl.tex_image_2d(
                self.get_texture_target_u32(target),
                level,
                self.get_texture_format_u32(internal_format) as i32,
                width,
                height,
                border,
                self.get_texture_format_u32(format),
                glow::UNSIGNED_BYTE,
                pixels,
            );
        }
    }

    fn get_texture_format_u32(&self, format: TextureFormat) -> u32 {
        match format {
            TextureFormat::RED => glow::RED,
            TextureFormat::RG => glow::RG,
            TextureFormat::RGB => glow::RGB,
            TextureFormat::RGBA => glow::RGBA,
        }
    }

    pub fn generate_mipmap(&self, target: TextureTarget) {
        unsafe {
            self.gl.generate_mipmap(self.get_texture_target_u32(target));
        }
    }

    pub fn set_texture_wrap(
        &self,
        target: TextureTarget,
        wrap_s: Option<TextureWrapMode>,
        wrap_t: Option<TextureWrapMode>,
    ) {
        let target = self.get_texture_target_u32(target);
        unsafe {
            if let Some(wrap_s) = wrap_s {
                self.gl.tex_parameter_i32(
                    target,
                    glow::TEXTURE_WRAP_S,
                    self.get_texture_wrap_i32(wrap_s),
                );
            }
            if let Some(wrap_t) = wrap_t {
                self.gl.tex_parameter_i32(
                    target,
                    glow::TEXTURE_WRAP_T,
                    self.get_texture_wrap_i32(wrap_t),
                );
            }
        }
    }

    fn get_texture_wrap_i32(&self, wrap: TextureWrapMode) -> i32 {
        match wrap {
            TextureWrapMode::Repeat => glow::REPEAT as i32,
            TextureWrapMode::ClampToEdge => glow::CLAMP_TO_EDGE as i32,
        }
    }

    pub fn set_texture_min_mag_filter(
        &self,
        target: TextureTarget,
        min: Option<TextureFilterMode>,
        mag: Option<TextureFilterMode>,
    ) {
        let target = self.get_texture_target_u32(target);
        unsafe {
            if let Some(min) = min {
                self.gl.tex_parameter_i32(
                    target,
                    glow::TEXTURE_MIN_FILTER,
                    self.get_texture_filter_i32(min),
                );
            }
            if let Some(mag) = mag {
                self.gl.tex_parameter_i32(
                    target,
                    glow::TEXTURE_MAG_FILTER,
                    self.get_texture_filter_i32(mag),
                );
            }
        }
    }

    fn get_texture_filter_i32(&self, filter: TextureFilterMode) -> i32 {
        match filter {
            TextureFilterMode::Linear => glow::LINEAR as i32,
            TextureFilterMode::LinearMipmapLinear => glow::LINEAR_MIPMAP_LINEAR as i32,
        }
    }

    pub fn create_shader(&self, shader_type: ShaderType) -> Result<ShaderObject, String> {
        let shader = unsafe {
            self.gl
                .create_shader(self.get_shader_type_u32(shader_type))?
        };
        Ok(ShaderObject {
            internal_shader: shader,
        })
    }

    pub fn delete_shader(&self, shader: &ShaderObject) {
        unsafe {
            self.gl.delete_shader(shader.internal_shader);
        }
    }

    fn get_shader_type_u32(&self, shader_type: ShaderType) -> u32 {
        match shader_type {
            ShaderType::Vertex => glow::VERTEX_SHADER,
            ShaderType::Fragment => glow::FRAGMENT_SHADER,
        }
    }

    pub fn set_shader_source(&self, shader: &ShaderObject, source: &str) {
        unsafe {
            self.gl.shader_source(shader.internal_shader, source);
        }
    }

    pub fn compile_shader(&self, shader: &ShaderObject) {
        unsafe {
            self.gl.compile_shader(shader.internal_shader);
        }
    }

    pub fn get_shader_compile_status(&self, shader: &ShaderObject) -> bool {
        unsafe { self.gl.get_shader_compile_status(shader.internal_shader) }
    }

    pub fn get_shader_info_log(&self, shader: &ShaderObject) -> String {
        unsafe { self.gl.get_shader_info_log(shader.internal_shader) }
    }

    pub fn create_program(&self) -> Result<ProgramObject, String> {
        let program = unsafe { self.gl.create_program()? };
        Ok(ProgramObject {
            internal_program: program,
        })
    }

    pub fn delete_program(&self, program: &ProgramObject) {
        unsafe {
            self.gl.delete_program(program.internal_program);
        }
    }

    pub fn attach_shader(&self, program: &ProgramObject, shader: &ShaderObject) {
        unsafe {
            self.gl
                .attach_shader(program.internal_program, shader.internal_shader);
        }
    }

    pub fn link_program(&self, program: &ProgramObject) {
        unsafe {
            self.gl.link_program(program.internal_program);
        }
    }

    pub fn get_program_link_status(&self, program: &ProgramObject) -> bool {
        unsafe { self.gl.get_program_link_status(program.internal_program) }
    }

    pub fn get_program_info_log(&self, program: &ProgramObject) -> String {
        unsafe { self.gl.get_program_info_log(program.internal_program) }
    }

    pub fn use_program(&self, program: Option<&ProgramObject>) {
        unsafe {
            self.gl.use_program(program.map(|p| p.internal_program));
        }
    }

    pub fn get_uniform_location(
        &self,
        program: &ProgramObject,
        name: &str,
    ) -> Option<UniformLocationObject> {
        unsafe {
            self.gl
                .get_uniform_location(program.internal_program, name)
                .map(|location| UniformLocationObject {
                    internal_location: location,
                })
        }
    }

    pub fn uniform_1_f32(&self, location: Option<&UniformLocationObject>, value: f32) {
        unsafe {
            self.gl
                .uniform_1_f32(location.map(|l| &l.internal_location), value);
        }
    }

    pub fn uniform_1_i32(&self, location: Option<&UniformLocationObject>, value: i32) {
        unsafe {
            self.gl
                .uniform_1_i32(location.map(|l| &l.internal_location), value);
        }
    }

    pub fn uniform_1_u32(&self, location: Option<&UniformLocationObject>, value: u32) {
        unsafe {
            self.gl
                .uniform_1_u32(location.map(|l| &l.internal_location), value);
        }
    }

    pub fn uniform_2_f32(&self, location: Option<&UniformLocationObject>, x: f32, y: f32) {
        unsafe {
            self.gl
                .uniform_2_f32(location.map(|l| &l.internal_location), x, y);
        }
    }

    pub fn uniform_3_f32(&self, location: Option<&UniformLocationObject>, x: f32, y: f32, z: f32) {
        unsafe {
            self.gl
                .uniform_3_f32(location.map(|l| &l.internal_location), x, y, z);
        }
    }

    pub fn uniform_4_f32(
        &self,
        location: Option<&UniformLocationObject>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        unsafe {
            self.gl
                .uniform_4_f32(location.map(|l| &l.internal_location), x, y, z, w);
        }
    }

    pub fn uniform_matrix_4_f32_slice(
        &self,
        location: Option<&UniformLocationObject>,
        transpose: bool,
        v: &[f32],
    ) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                location.map(|l| &l.internal_location),
                transpose,
                v,
            );
        }
    }
}
