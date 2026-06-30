use super::device::Device;
use super::ffi::*;
use super::resource::Texture;
use super::types::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct MetalLayer {
    pub(crate) raw: id,
}

impl MetalLayer {
    pub unsafe fn attach_to_view(
        ns_view: *mut c_void,
        device: &Device,
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<Self, MetalError> {
        unsafe {
            let raw = retain(msg_id(class(b"CAMetalLayer\0"), sel(b"layer\0")));
            if raw.is_null() {
                return Err(MetalError::new("failed to create CAMetalLayer"));
            }

            let layer = Self { raw };
            layer.set_device(device);
            layer.set_pixel_format(pixel_format);
            layer.set_framebuffer_only(true);
            layer.set_presents_with_transaction(false);
            layer.set_contents_scale(scale);
            layer.set_drawable_size(width, height);

            msg_void_bool(ns_view, sel(b"setWantsLayer:\0"), YES);
            msg_void_id(ns_view, sel(b"setLayer:\0"), layer.raw);

            Ok(layer)
        }
    }

    pub fn set_device(&self, device: &Device) {
        unsafe {
            msg_void_id(self.raw, sel(b"setDevice:\0"), device.raw);
        }
    }

    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
        }
    }

    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setFramebufferOnly:\0"),
                if framebuffer_only { YES } else { NO },
            );
        }
    }

    pub fn set_presents_with_transaction(&self, presents_with_transaction: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setPresentsWithTransaction:\0"),
                if presents_with_transaction { YES } else { NO },
            );
        }
    }

    pub fn set_contents_scale(&self, scale: f64) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, f64) = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"setContentsScale:\0"), scale);
        }
    }

    pub fn set_drawable_size(&self, width: usize, height: usize) {
        unsafe {
            msg_void_size(
                self.raw,
                sel(b"setDrawableSize:\0"),
                CGSize {
                    width: width as f64,
                    height: height as f64,
                },
            );
        }
    }

    pub fn next_drawable(&self) -> Option<Drawable> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"nextDrawable\0")));
            (!raw.is_null()).then_some(Drawable { raw })
        }
    }
}

impl Drop for MetalLayer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Drawable {
    pub(crate) raw: id,
}

impl Drawable {
    pub fn texture(&self) -> Texture {
        unsafe {
            Texture {
                raw: retain(msg_id(self.raw, sel(b"texture\0"))),
            }
        }
    }

    pub fn present(&self) {
        unsafe { msg_void(self.raw, sel(b"present\0")) };
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}
