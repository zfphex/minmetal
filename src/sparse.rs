use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SparseTextureMappingMode {
    Map = 0,
    Unmap = 1,
}

impl Device {
    pub fn supports_sparse_textures(&self) -> bool {
        unsafe {
            let selector = sel(b"sparseTileSizeWithTextureType:pixelFormat:sampleCount:\0");
            responds_to_selector(self.raw, selector)
        }
    }

    pub fn sparse_tile_size(
        &self,
        texture_type: TextureType,
        pixel_format: PixelFormat,
        sample_count: usize,
    ) -> Size {
        unsafe {
            let selector = sel(b"sparseTileSizeWithTextureType:pixelFormat:sampleCount:\0");
            if !responds_to_selector(self.raw, selector) {
                return Size::new(0, 0, 0);
            }
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, texture_type as usize, pixel_format.as_raw(), sample_count)
        }
    }
}

impl ResourceStateCommandEncoder {
    pub fn update_texture_mapping(
        &self,
        texture: &Texture,
        mode: SparseTextureMappingMode,
        region: Region,
        mip_level: usize,
        slice: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"updateTextureMapping:mode:region:mipLevel:slice:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "updateTextureMapping:mode:region:mipLevel:slice: not supported on this ResourceStateCommandEncoder"
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, Region, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                texture.raw,
                mode as usize,
                region,
                mip_level,
                slice,
            );
            Ok(())
        }
    }
}

