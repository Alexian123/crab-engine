use super::TextureSampler;
use crate::GfxContext;
use crate::gfx::buffers::{
    TextureDataType, TextureFilterMode, TextureFormat, TextureObject, TextureTarget,
    TextureWrapMode,
};
use std::rc::Rc;

pub struct SamplerCube {
    gfx: Rc<GfxContext>,
    texture: TextureObject,
    width: u32,
    height: u32,
    channels: u32,
}

impl SamplerCube {
    pub fn new(
        gfx: Rc<GfxContext>,
        width: u32,
        height: u32,
        channels: u32,
        faces: [&[u8]; 6], // in order: +x, -x, +y, -y, +z, -z
    ) -> Result<Self, String> {
        let texture = gfx.create_texture_object()?;

        let (internal_format, format) = match channels {
            1 => (TextureFormat::RED, TextureFormat::RED),
            2 => (TextureFormat::RG, TextureFormat::RG),
            3 => (TextureFormat::RGB, TextureFormat::RGB),
            4 => (TextureFormat::RGBA, TextureFormat::RGBA),
            _ => return Err("Invalid number of channels".to_string()),
        };
        gfx.bind_texture(TextureTarget::TextureCubeMap, Some(&texture));

        gfx.tex_image_cube_map(
            0,
            internal_format,
            width as i32,
            height as i32,
            0,
            format,
            TextureDataType::UnsignedByte,
            faces,
        );

        gfx.set_texture_wrap(
            TextureTarget::TextureCubeMap,
            Some(TextureWrapMode::ClampToEdge),
            Some(TextureWrapMode::ClampToEdge),
            Some(TextureWrapMode::ClampToEdge),
        );

        gfx.set_texture_min_mag_filter(
            TextureTarget::TextureCubeMap,
            Some(TextureFilterMode::Linear),
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
}

impl TextureSampler for SamplerCube {
    fn bind(&self, unit: u32) {
        self.gfx.set_active_texture(unit.min(15));
        self.gfx
            .bind_texture(TextureTarget::TextureCubeMap, Some(&self.texture));
    }

    fn unbind(&self, unit: u32) {
        self.gfx.set_active_texture(unit.min(15));
        self.gfx.bind_texture(TextureTarget::TextureCubeMap, None);
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn channels(&self) -> u32 {
        self.channels
    }
}

impl Drop for SamplerCube {
    fn drop(&mut self) {
        self.gfx.delete_texture_object(&self.texture);
    }
}
