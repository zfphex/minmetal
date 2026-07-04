use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct MetalLayer {
    pub raw: id,
}

impl MetalLayer {
    pub fn new() -> Result<Self, MetalError> {
        let raw = retain(msg_id(class(b"CAMetalLayer\0"), sel(b"layer\0")));
        if raw.is_null() {
            return Err(MetalError::new("failed to create CAMetalLayer"));
        }
        Ok(Self { raw })
    }

    pub unsafe fn attach_to_view(
        ns_view: *mut c_void,
        device: &Device,
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<Self, MetalError> {
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

    pub fn set_device(&self, device: &Device) {
        msg_void_id(self.raw, sel(b"setDevice:\0"), device.raw);
    }

    pub fn device(&self) -> Option<Device> {
        let ptr = msg_id(self.raw, sel(b"device\0"));
        if ptr.is_null() {
            None
        } else {
            Some(Device { raw: retain(ptr) })
        }
    }

    pub fn preferred_device(&self) -> Result<Option<Device>, MetalError> {
        let selector = sel(b"preferredDevice\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("preferredDevice is not supported"));
        }
        let ptr = msg_id(self.raw, selector);
        if ptr.is_null() {
            Ok(None)
        } else {
            Ok(Some(Device { raw: retain(ptr) }))
        }
    }

    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
    }

    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_raw(msg_usize(self.raw, sel(b"pixelFormat\0")))
    }

    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setFramebufferOnly:\0"),
            if framebuffer_only { YES } else { NO },
        );
    }

    pub fn framebuffer_only(&self) -> bool {
        msg_bool(self.raw, sel(b"framebufferOnly\0")) != 0
    }

    pub fn set_presents_with_transaction(&self, presents_with_transaction: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setPresentsWithTransaction:\0"),
            if presents_with_transaction { YES } else { NO },
        );
    }

    pub fn presents_with_transaction(&self) -> bool {
        msg_bool(self.raw, sel(b"presentsWithTransaction\0")) != 0
    }

    pub fn set_contents_scale(&self, scale: f64) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, f64) = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"setContentsScale:\0"), scale);
        }
    }

    pub fn drawable_size(&self) -> CGSize {
        msg_cgsize(self.raw, sel(b"drawableSize\0"))
    }

    pub fn set_drawable_size(&self, width: usize, height: usize) {
        msg_void_size(
            self.raw,
            sel(b"setDrawableSize:\0"),
            CGSize {
                width: width as f64,
                height: height as f64,
            },
        );
    }

    pub fn maximum_drawable_count(&self) -> Result<usize, MetalError> {
        let selector = sel(b"maximumDrawableCount\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("maximumDrawableCount is not supported"));
        }
        Ok(msg_usize(self.raw, selector))
    }

    pub fn set_maximum_drawable_count(&self, count: usize) -> Result<(), MetalError> {
        let selector = sel(b"setMaximumDrawableCount:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("setMaximumDrawableCount: is not supported"));
        }
        msg_void_usize(self.raw, selector, count);
        Ok(())
    }

    pub fn display_sync_enabled(&self) -> Result<bool, MetalError> {
        let selector = sel(b"displaySyncEnabled\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("displaySyncEnabled is not supported"));
        }
        Ok(msg_bool(self.raw, selector) != 0)
    }

    pub fn set_display_sync_enabled(&self, enabled: bool) -> Result<(), MetalError> {
        let selector = sel(b"setDisplaySyncEnabled:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("setDisplaySyncEnabled: is not supported"));
        }
        msg_void_bool(self.raw, selector, if enabled { YES } else { NO });
        Ok(())
    }

    pub fn allows_next_drawable_timeout(&self) -> Result<bool, MetalError> {
        let selector = sel(b"allowsNextDrawableTimeout\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "allowsNextDrawableTimeout is not supported",
            ));
        }
        Ok(msg_bool(self.raw, selector) != 0)
    }

    pub fn set_allows_next_drawable_timeout(&self, allows: bool) -> Result<(), MetalError> {
        let selector = sel(b"setAllowsNextDrawableTimeout:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setAllowsNextDrawableTimeout: is not supported",
            ));
        }
        msg_void_bool(self.raw, selector, if allows { YES } else { NO });
        Ok(())
    }

    pub fn colorspace(&self) -> *mut c_void {
        msg_id(self.raw, sel(b"colorspace\0")) as *mut c_void
    }

    pub fn set_colorspace(&self, colorspace: *mut c_void) {
        msg_void_id(self.raw, sel(b"setColorspace:\0"), colorspace as id);
    }

    pub fn wants_extended_dynamic_range_content(&self) -> Result<bool, MetalError> {
        let selector = sel(b"wantsExtendedDynamicRangeContent\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "wantsExtendedDynamicRangeContent is not supported",
            ));
        }
        Ok(msg_bool(self.raw, selector) != 0)
    }

    pub fn set_wants_extended_dynamic_range_content(&self, wants: bool) -> Result<(), MetalError> {
        let selector = sel(b"setWantsExtendedDynamicRangeContent:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setWantsExtendedDynamicRangeContent: is not supported",
            ));
        }
        msg_void_bool(self.raw, selector, if wants { YES } else { NO });
        Ok(())
    }

    pub fn next_drawable(&self) -> Option<Drawable> {
        let raw = retain(msg_id(self.raw, sel(b"nextDrawable\0")));
        (!raw.is_null()).then_some(Drawable { raw })
    }
}

impl Drop for MetalLayer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct Drawable {
    pub raw: id,
}

impl Drawable {
    pub fn texture(&self) -> Result<Texture, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"texture\0")));
        if raw.is_null() {
            Err(MetalError::new("failed to get texture from Metal drawable"))
        } else {
            Ok(Texture { raw })
        }
    }

    pub fn present(&self) {
        msg_void(self.raw, sel(b"present\0"));
    }

    pub fn present_at_time(&self, time: f64) {
        msg_void_f64(self.raw, sel(b"presentAtTime:\0"), time);
    }

    pub fn present_after_minimum_duration(&self, duration: f64) {
        msg_void_f64(self.raw, sel(b"presentAfterMinimumDuration:\0"), duration);
    }

    pub fn presented_time(&self) -> f64 {
        msg_f64(self.raw, sel(b"presentedTime\0"))
    }

    pub fn drawable_id(&self) -> usize {
        msg_usize(self.raw, sel(b"drawableID\0"))
    }

    pub fn layer(&self) -> Option<MetalLayer> {
        let ptr = retain(msg_id(self.raw, sel(b"layer\0")));
        if ptr.is_null() {
            None
        } else {
            Some(MetalLayer { raw: ptr })
        }
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        release(self.raw);
    }
}
