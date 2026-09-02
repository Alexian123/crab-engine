use crate::GfxContext;
use crate::gfx::buffers::*;
use std::rc::Rc;

enum DepthAttachment {
    None,
    Texture(TextureObject),
    RenderBuffer(RenderBufferObject),
}

pub struct Framebuffer {
    gfx: Rc<GfxContext>,
    width: u32,
    height: u32,
    fbo: FrameBufferObject,
    color_texture: Option<TextureObject>,
    depth_attachment: DepthAttachment,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        match &self.depth_attachment {
            DepthAttachment::Texture(texture) => {
                self.gfx.delete_texture_object(texture);
            }
            DepthAttachment::RenderBuffer(rbo) => {
                self.gfx.delete_rbo(rbo);
            }
            DepthAttachment::None => {}
        }

        if let Some(color_texture) = &self.color_texture {
            self.gfx.delete_texture_object(color_texture);
        }
        self.gfx.delete_fbo(&self.fbo);
    }
}

impl Framebuffer {
    pub fn bind(&self) {
        self.gfx.bind_fbo(Some(&self.fbo));
        self.gfx
            .set_viewport(0, 0, self.width as i32, self.height as i32);
    }

    pub fn unbind(&self) {
        self.gfx.bind_fbo(None);
    }

    pub fn color_texture(&self) -> Option<&TextureObject> {
        self.color_texture.as_ref()
    }

    pub fn depth_texture(&self) -> Option<&TextureObject> {
        match &self.depth_attachment {
            DepthAttachment::Texture(texture) => Some(texture),
            _ => None,
        }
    }

    pub fn depth_render_buffer(&self) -> Option<&RenderBufferObject> {
        match &self.depth_attachment {
            DepthAttachment::RenderBuffer(rbo) => Some(rbo),
            _ => None,
        }
    }
}

pub struct FramebufferBuilder {
    gfx: Rc<GfxContext>,
    width: u32,
    height: u32,
    fbo: FrameBufferObject,
    color_texture: Option<TextureObject>,
    depth_attachment: DepthAttachment,
}

impl FramebufferBuilder {
    pub fn new(gfx: Rc<GfxContext>, width: u32, height: u32) -> Result<Self, String> {
        let fbo = gfx.create_fbo()?;
        Ok(Self {
            gfx,
            width,
            height,
            fbo,
            color_texture: None,
            depth_attachment: DepthAttachment::None,
        })
    }

    fn delete_depth_attachment(&self, attachment: DepthAttachment) {
        match attachment {
            DepthAttachment::Texture(texture) => {
                self.gfx.delete_texture_object(&texture);
            }
            DepthAttachment::RenderBuffer(rbo) => {
                self.gfx.delete_rbo(&rbo);
            }
            DepthAttachment::None => {}
        }
    }

    pub fn with_color_texture(mut self) -> Result<Self, String> {
        self.gfx.bind_fbo(Some(&self.fbo));

        if let Some(color_texture) = self.color_texture.take() {
            self.gfx.delete_texture_object(&color_texture);
        }

        let color_texture = match self.gfx.create_texture_object() {
            Ok(texture) => texture,
            Err(_) => {
                self.gfx.bind_fbo(None);
                return Err("Failed to create color texture".to_string());
            }
        };

        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(&color_texture));
        self.gfx.tex_image_2d(
            TextureTarget::Texture2D,
            0,
            TextureFormat::RGBA8,
            self.width as i32,
            self.height as i32,
            0,
            TextureFormat::RGBA,
            TextureDataType::UnsignedByte,
            None,
        );
        self.gfx.set_texture_min_mag_filter(
            TextureTarget::Texture2D,
            Some(TextureFilterMode::Linear),
            Some(TextureFilterMode::Linear),
        );
        self.gfx.set_texture_wrap(
            TextureTarget::Texture2D,
            Some(TextureWrapMode::ClampToEdge),
            Some(TextureWrapMode::ClampToEdge),
        );
        self.gfx.framebuffer_texture_2d(
            FrameBufferTextureAttachment::Color,
            TextureTarget::Texture2D,
            Some(&color_texture),
            0,
        );

        self.gfx.bind_fbo(None);
        self.color_texture = Some(color_texture);

        Ok(self)
    }

    pub fn with_depth_texture(mut self) -> Result<Self, String> {
        self.gfx.bind_fbo(Some(&self.fbo));

        let old_attachment = std::mem::replace(&mut self.depth_attachment, DepthAttachment::None);
        self.delete_depth_attachment(old_attachment);

        let depth_texture = match self.gfx.create_texture_object() {
            Ok(texture) => texture,
            Err(_) => {
                self.gfx.bind_fbo(None);
                return Err("Failed to create depth texture".to_string());
            }
        };

        self.gfx
            .bind_texture(TextureTarget::Texture2D, Some(&depth_texture));
        self.gfx.tex_image_2d(
            TextureTarget::Texture2D,
            0,
            TextureFormat::DepthComponent24,
            self.width as i32,
            self.height as i32,
            0,
            TextureFormat::DepthComponent,
            TextureDataType::UnsignedInt,
            None,
        );
        self.gfx.set_texture_min_mag_filter(
            TextureTarget::Texture2D,
            Some(TextureFilterMode::Linear),
            Some(TextureFilterMode::Linear),
        );
        self.gfx.framebuffer_texture_2d(
            FrameBufferTextureAttachment::Depth,
            TextureTarget::Texture2D,
            Some(&depth_texture),
            0,
        );

        self.depth_attachment = DepthAttachment::Texture(depth_texture);

        self.gfx.bind_fbo(None);

        Ok(self)
    }

    pub fn with_depth_render_buffer(mut self) -> Result<Self, String> {
        self.gfx.bind_fbo(Some(&self.fbo));

        let old_attachment = std::mem::replace(&mut self.depth_attachment, DepthAttachment::None);
        self.delete_depth_attachment(old_attachment);

        let depth_buffer = match self.gfx.create_rbo() {
            Ok(rbo) => rbo,
            Err(_) => {
                self.gfx.bind_fbo(None);
                return Err("Failed to create depth render buffer".to_string());
            }
        };

        self.gfx.bind_rbo(Some(&depth_buffer));

        self.gfx.render_buffer_storage(
            RenderBufferFormat::DepthComponent24,
            self.width,
            self.height,
        );
        self.gfx.framebuffer_renderbuffer(
            FrameBufferRenderBufferAttachment::Depth,
            Some(&depth_buffer),
        );

        self.depth_attachment = DepthAttachment::RenderBuffer(depth_buffer);

        self.gfx.bind_fbo(None);

        Ok(self)
    }

    pub fn build(self) -> Result<Framebuffer, String> {
        self.gfx.bind_fbo(Some(&self.fbo));

        let complete = self.gfx.is_fbo_complete();

        self.gfx.bind_fbo(None);

        if !complete {
            return Err("Framebuffer is not complete".to_string());
        }

        Ok(Framebuffer {
            gfx: self.gfx,
            width: self.width,
            height: self.height,
            fbo: self.fbo,
            color_texture: self.color_texture,
            depth_attachment: self.depth_attachment,
        })
    }
}
