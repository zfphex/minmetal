use super::ffi::*;
use super::types::*;
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr;

#[derive(Debug)]
pub struct Buffer {
    pub(crate) raw: id,
}

impl Buffer {
    pub fn len(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"length\0")) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contents(&self) -> *mut c_void {
        unsafe { msg_id(self.raw, sel(b"contents\0")) }
    }

    pub fn did_modify_range(&self, range: Range) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Range) = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"didModifyRange:\0"), range);
        }
    }

    pub fn write<T: Copy>(&self, value: &T) {
        let size = std::mem::size_of::<T>();
        assert!(size <= self.len());
        unsafe {
            ptr::copy_nonoverlapping(
                value as *const T as *const u8,
                self.contents() as *mut u8,
                size,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct TextureDescriptor {
    pub(crate) raw: id,
}

impl TextureDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLTextureDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn texture_2d(
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> Self {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, BOOL) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                class(b"MTLTextureDescriptor\0"),
                sel(b"texture2DDescriptorWithPixelFormat:width:height:mipmapped:\0"),
                pixel_format.as_raw(),
                width,
                height,
                if mipmapped { YES } else { NO },
            ));
            Self { raw }
        }
    }

    pub fn texture_2d_array(
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        array_length: usize,
        mipmapped: bool,
    ) -> Self {
        let descriptor = Self::texture_2d(pixel_format, width, height, mipmapped);
        descriptor.set_texture_type(TextureType::D2Array);
        descriptor.set_array_length(array_length);
        descriptor
    }

    pub fn texture_cube(
        pixel_format: PixelFormat,
        size: usize,
        array_length: usize,
        mipmapped: bool,
    ) -> Self {
        let descriptor = Self::texture_2d(pixel_format, size, size, mipmapped);
        if array_length > 1 {
            descriptor.set_texture_type(TextureType::CubeArray);
            descriptor.set_array_length(array_length * 6);
        } else {
            descriptor.set_texture_type(TextureType::Cube);
            descriptor.set_array_length(6);
        }
        descriptor
    }

    pub fn set_texture_type(&self, texture_type: TextureType) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setTextureType:\0"), texture_type as usize);
        }
    }

    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
        }
    }

    pub fn set_width(&self, width: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setWidth:\0"), width);
        }
    }

    pub fn set_height(&self, height: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setHeight:\0"), height);
        }
    }

    pub fn set_depth(&self, depth: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setDepth:\0"), depth);
        }
    }

    pub fn set_mipmap_level_count(&self, mipmap_level_count: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMipmapLevelCount:\0"), mipmap_level_count);
        }
    }

    pub fn set_array_length(&self, array_length: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setArrayLength:\0"), array_length);
        }
    }

    pub fn set_sample_count(&self, sample_count: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setSampleCount:\0"), sample_count);
        }
    }

    pub fn set_usage(&self, usage: TextureUsage) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setUsage:\0"), usage.as_raw());
        }
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setStorageMode:\0"), storage_mode as usize);
        }
    }
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TextureDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Texture {
    pub(crate) raw: id,
}

impl Texture {
    pub fn replace_region(
        &self,
        region: Region,
        mipmap_level: usize,
        bytes: &[u8],
        bytes_per_row: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Region, usize, *const c_void, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"replaceRegion:mipmapLevel:withBytes:bytesPerRow:\0"),
                region,
                mipmap_level,
                bytes.as_ptr() as *const c_void,
                bytes_per_row,
            );
        }
    }

    pub fn get_bytes(
        &self,
        region: Region,
        mipmap_level: usize,
        out: &mut [u8],
        bytes_per_row: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *mut c_void, usize, Region, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"getBytes:bytesPerRow:fromRegion:mipmapLevel:\0"),
                out.as_mut_ptr() as *mut c_void,
                bytes_per_row,
                region,
                mipmap_level,
            );
        }
    }

    pub fn new_texture_view(&self, pixel_format: PixelFormat) -> Result<Texture, MetalError> {
        unsafe {
            let raw = retain(msg_id_usize(
                self.raw,
                sel(b"newTextureViewWithPixelFormat:\0"),
                pixel_format.as_raw(),
            ));
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal texture view"))
            } else {
                Ok(Texture { raw })
            }
        }
    }

    pub fn width(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"width\0")) }
    }

    pub fn height(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"height\0")) }
    }

    pub fn pixel_format(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"pixelFormat\0")) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}
