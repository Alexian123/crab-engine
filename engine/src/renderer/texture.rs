use glow::HasContext;
use std::rc::Rc;

pub struct Texture {
    gl: Rc<glow::Context>,
    texture: glow::Texture,
    width: u32,
    height: u32,
    channels: u32,
}

impl Texture {
    pub fn new(
        gl: Rc<glow::Context>,
        width: u32,
        height: u32,
        channels: u32,
        data: &[u8],
    ) -> Result<Self, String> {
        let texture = unsafe { gl.create_texture()? };

        let (internal_format, format) = match channels {
            1 => (glow::RED, glow::RED),
            2 => (glow::RG, glow::RG),
            3 => (glow::RGB, glow::RGB),
            4 => (glow::RGBA, glow::RGBA),
            _ => return Err("Invalid number of channels".to_string()),
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                internal_format as i32,
                width as i32,
                height as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                Some(data),
            );

            gl.generate_mipmap(glow::TEXTURE_2D);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
        }

        Ok(Self {
            gl,
            texture,
            width,
            height,
            channels,
        })
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + unit);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture);
        }
    }
}
