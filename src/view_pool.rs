use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct ResourceViewPoolDescriptor {
    pub raw: id,
}

impl ResourceViewPoolDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLResourceViewPoolDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new(
                "MTLResourceViewPoolDescriptor is not available",
            ));
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        let raw = msg_id(allocated, sel(b"init\0"));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create MTLResourceViewPoolDescriptor",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn resource_view_count(&self) -> usize {
        msg_usize(self.raw, sel(b"resourceViewCount\0"))
    }

    pub fn set_resource_view_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setResourceViewCount:\0"), count);
    }

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for ResourceViewPoolDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

/// Protocol wrapper for `MTLResourceViewPool`.
#[derive(Debug, Clone, Copy)]
pub struct ResourceViewPool {
    pub raw: id,
}

impl ResourceViewPool {
    pub fn from_texture_view_pool(pool: &TextureViewPool) -> Self {
        Self { raw: pool.raw }
    }

    pub fn base_resource_id(&self) -> ResourceID {
        msg_resource_id(self.raw, sel(b"baseResourceID\0"))
    }

    pub fn resource_view_count(&self) -> usize {
        msg_usize(self.raw, sel(b"resourceViewCount\0"))
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }
}

#[derive(Debug)]
pub struct TextureViewPool {
    pub raw: id,
}

impl TextureViewPool {
    pub fn base_resource_id(&self) -> ResourceID {
        msg_resource_id(self.raw, sel(b"baseResourceID\0"))
    }

    pub fn resource_view_count(&self) -> usize {
        msg_usize(self.raw, sel(b"resourceViewCount\0"))
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn copy_resource_views_from_pool(
        &self,
        source_pool: &ResourceViewPool,
        source_range: Range,
        destination_index: usize,
    ) -> ResourceID {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range, usize) -> ResourceID =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyResourceViewsFromPool:sourceRange:destinationIndex:\0"),
                source_pool.raw,
                source_range,
                destination_index,
            )
        }
    }

    pub fn set_texture_view(
        &self,
        texture: &Texture,
        index: usize,
    ) -> Result<ResourceID, MetalError> {
        let selector = sel(b"setTextureView:atIndex:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("setTextureView:atIndex: is not supported"));
        }
        Ok(unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) -> ResourceID =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, texture.raw, index)
        })
    }

    pub fn set_texture_view_with_descriptor(
        &self,
        texture: &Texture,
        descriptor: &TextureViewDescriptor,
        index: usize,
    ) -> Result<ResourceID, MetalError> {
        let selector = sel(b"setTextureView:descriptor:atIndex:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setTextureView:descriptor:atIndex: is not supported",
            ));
        }
        Ok(unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, id, usize) -> ResourceID =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, texture.raw, descriptor.raw, index)
        })
    }

    pub fn set_texture_view_from_buffer(
        &self,
        buffer: &Buffer,
        descriptor: &TextureDescriptor,
        offset: usize,
        bytes_per_row: usize,
        index: usize,
    ) -> Result<ResourceID, MetalError> {
        let selector = sel(b"setTextureViewFromBuffer:descriptor:offset:bytesPerRow:atIndex:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setTextureViewFromBuffer:descriptor:offset:bytesPerRow:atIndex: is not supported",
            ));
        }
        Ok(unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, id, usize, usize, usize) -> ResourceID =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                buffer.raw,
                descriptor.raw,
                offset,
                bytes_per_row,
                index,
            )
        })
    }
}

impl Drop for TextureViewPool {
    fn drop(&mut self) {
        release(self.raw);
    }
}
