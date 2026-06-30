use super::ffi::*;
use super::resource::Texture;
use super::types::*;

#[derive(Debug)]
pub struct RenderPassDescriptor {
    pub(crate) raw: id,
}

impl RenderPassDescriptor {
    pub fn new() -> Self {
        unsafe {
            let raw = retain(msg_id(
                class(b"MTLRenderPassDescriptor\0"),
                sel(b"renderPassDescriptor\0"),
            ));
            Self { raw }
        }
    }

    pub fn set_color_attachment(
        &self,
        index: usize,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_color: ClearColor,
    ) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
            msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
            msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
            msg_void_clear_color(attachment, sel(b"setClearColor:\0"), clear_color);
        }
    }

    pub fn set_color_attachment_resolve_texture(&self, index: usize, texture: &Texture) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_id(attachment, sel(b"setResolveTexture:\0"), texture.raw);
        }
    }

    pub fn set_depth_attachment(
        &self,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_depth: f64,
    ) {
        unsafe {
            let attachment = msg_id(self.raw, sel(b"depthAttachment\0"));
            msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
            msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
            msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
            msg_void_f64(attachment, sel(b"setClearDepth:\0"), clear_depth);
        }
    }

    pub fn set_depth_resolve_texture(&self, texture: &Texture) {
        unsafe {
            let attachment = msg_id(self.raw, sel(b"depthAttachment\0"));
            msg_void_id(attachment, sel(b"setResolveTexture:\0"), texture.raw);
        }
    }

    pub fn set_stencil_attachment(
        &self,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_stencil: u32,
    ) {
        unsafe {
            let attachment = msg_id(self.raw, sel(b"stencilAttachment\0"));
            msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
            msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
            msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
            msg_void_usize(
                attachment,
                sel(b"setClearStencil:\0"),
                clear_stencil as usize,
            );
        }
    }
}

impl Default for RenderPassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RenderPassDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}
