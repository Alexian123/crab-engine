use crate::GfxContext;
use crate::gfx::buffers::{
    TextureDataType, TextureFilterMode, TextureFormat, TextureObject, TextureTarget,
    TextureWrapMode,
};
use std::rc::Rc;

pub struct Texture {
    gfx: Rc<GfxContext>,
    texture: TextureObject,
    width: u32,
    height: u32,
    channels: u32,
}

impl Texture {
    pub fn new(
        gfx: Rc<GfxContext>,
        width: u32,
        height: u32,
        channels: u32,
        data: &[u8],
    ) -> Result<Self, String> {
        let texture = gfx.create_texture_object()?;

        let (internal_format, format) = match channels {
            1 => (TextureFormat::RED, TextureFormat::RED),
            2 => (TextureFormat::RG, TextureFormat::RG),
            3 => (TextureFormat::RGB, TextureFormat::RGB),
            4 => (TextureFormat::RGBA, TextureFormat::RGBA),
            _ => return Err("Invalid number of channels".to_string()),
        };
        gfx.bind_texture(TextureTarget::Texture2D, Some(&texture));

        gfx.tex_image_2d(
            TextureTarget::Texture2D,
            0,
            internal_format,
            width as i32,
            height as i32,
            0,
            format,
            TextureDataType::UnsignedByte,
            Some(data),
        );

        gfx.generate_mipmap(TextureTarget::Texture2D);

        gfx.set_texture_wrap(
            TextureTarget::Texture2D,
            Some(TextureWrapMode::Repeat),
            Some(TextureWrapMode::Repeat),
        );

        gfx.set_texture_min_mag_filter(
            TextureTarget::Texture2D,
            Some(TextureFilterMode::LinearMipmapLinear),
            Some(TextureFilterMode::Linear),
        );

        Ok(Self {
            gfx,
            texture,
            width,
            height,
            channels,
        })
    }

    pub fn bind(&self, unit: u32) {
        self.gfx.set_active_texture(unit.min(15));
        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(&self.texture));
    }

    pub fn unbind(&self, unit: u32) {
        self.gfx.set_active_texture(unit.min(15));
        self.gfx.bind_texture(TextureTarget::Texture2D, None);
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
        self.gfx.delete_texture_object(&self.texture);
    }
}
