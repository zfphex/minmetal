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
        let selector = sel!(b"sparseTileSizeWithTextureType:pixelFormat:sampleCount:\0");
        responds_to_selector(self.raw, selector)
    }

    pub fn sparse_tile_size(
        &self,
        texture_type: TextureType,
        pixel_format: PixelFormat,
        sample_count: usize,
    ) -> Size {
        unsafe {
            let selector = sel!(b"sparseTileSizeWithTextureType:pixelFormat:sampleCount:\0");
            if !responds_to_selector(self.raw, selector) {
                return Size::new(0, 0, 0);
            }
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) -> Size =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                texture_type as usize,
                pixel_format.as_raw(),
                sample_count,
            )
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
            let selector = sel!(b"updateTextureMapping:mode:region:mipLevel:slice:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "updateTextureMapping:mode:region:mipLevel:slice: not supported on this ResourceStateCommandEncoder",
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

    pub fn update_texture_mappings(
        &self,
        texture: &Texture,
        mode: SparseTextureMappingMode,
        regions: &[Region],
        mip_levels: &[usize],
        slices: &[usize],
    ) -> Result<(), MetalError> {
        if regions.len() != mip_levels.len() || regions.len() != slices.len() {
            return Err(MetalError::new(
                "updateTextureMappings requires regions, mip_levels, and slices to have equal length",
            ));
        }
        if regions.is_empty() {
            return Err(MetalError::new(
                "updateTextureMappings requires at least one region",
            ));
        }
        unsafe {
            let selector =
                sel!(b"updateTextureMappings:mode:regions:mipLevels:slices:numRegions:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "updateTextureMappings:mode:regions:mipLevels:slices:numRegions: not supported on this ResourceStateCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(
                id,
                SEL,
                id,
                usize,
                *const Region,
                *const usize,
                *const usize,
                usize,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                texture.raw,
                mode as usize,
                regions.as_ptr(),
                mip_levels.as_ptr(),
                slices.as_ptr(),
                regions.len(),
            );
            Ok(())
        }
    }

    pub fn update_texture_mapping_indirect(
        &self,
        texture: &Texture,
        mode: SparseTextureMappingMode,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel!(b"updateTextureMapping:mode:indirectBuffer:indirectBufferOffset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "updateTextureMapping:mode:indirectBuffer:indirectBufferOffset: not supported on this ResourceStateCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                texture.raw,
                mode as usize,
                indirect_buffer.raw,
                indirect_buffer_offset,
            );
            Ok(())
        }
    }

    pub fn move_texture_mappings(
        &self,
        source_texture: &Texture,
        source_slice: usize,
        source_level: usize,
        source_origin: Origin,
        source_size: Size,
        destination_texture: &Texture,
        destination_slice: usize,
        destination_level: usize,
        destination_origin: Origin,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel!(
                b"moveTextureMappingsFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:\0",
            );
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "moveTextureMappingsFromTexture:... not supported on this ResourceStateCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(
                id,
                SEL,
                id,
                usize,
                usize,
                Origin,
                Size,
                id,
                usize,
                usize,
                Origin,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source_texture.raw,
                source_slice,
                source_level,
                source_origin,
                source_size,
                destination_texture.raw,
                destination_slice,
                destination_level,
                destination_origin,
            );
            Ok(())
        }
    }
}
