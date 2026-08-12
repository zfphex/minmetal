use crate::*;
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr;

fn resource_label(raw: id) -> Option<NSString> {
    if raw.is_null() {
        return None;
    }
    let ptr = msg_id(raw, sel(b"label\0"));
    if ptr.is_null() {
        None
    } else {
        Some(NSString::from_raw(ptr))
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Buffer {
    pub raw: id,
}

impl Buffer {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    pub fn len(&self) -> usize {
        msg_usize(self.raw, sel(b"length\0"))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contents(&self) -> *mut c_void {
        msg_id(self.raw, sel(b"contents\0")).0
    }

    pub fn did_modify_range(&self, range: Range) {
        msg_void_range(self.raw, sel(b"didModifyRange:\0"), range);
    }

    pub fn write<T: Copy>(&self, value: &T) {
        let size = std::mem::size_of::<T>();
        assert!(size <= self.len());
        assert!(
            !self.contents().is_null(),
            "cannot write to a private or non-CPU-visible buffer on the CPU"
        );
        unsafe {
            ptr::copy_nonoverlapping(
                value as *const T as *const u8,
                self.contents() as *mut u8,
                size,
            );
        }
    }

    pub fn write_slice<T: Copy>(&self, data: &[T]) {
        let size = std::mem::size_of_val(data);
        assert!(size <= self.len());
        assert!(
            !self.contents().is_null(),
            "cannot write to a private or non-CPU-visible buffer on the CPU"
        );
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr() as *const u8, self.contents() as *mut u8, size);
        }
    }

    pub fn read_slice<T: Copy>(&self, out: &mut [T]) {
        let size = std::mem::size_of_val(out);
        assert!(size <= self.len());
        assert!(
            !self.contents().is_null(),
            "cannot read from a private or non-CPU-visible buffer on the CPU"
        );
        unsafe {
            ptr::copy_nonoverlapping(
                self.contents() as *const u8,
                out.as_mut_ptr() as *mut u8,
                size,
            );
        }
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn gpu_address(&self) -> Result<u64, MetalError> {
        let selector = sel(b"gpuAddress\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_u64(self.raw, selector))
        } else {
            Err(MetalError::new("gpuAddress not supported on this Buffer"))
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        let val = msg_usize(self.raw, sel(b"cpuCacheMode\0"));
        match val {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn storage_mode(&self) -> StorageMode {
        let val = msg_usize(self.raw, sel(b"storageMode\0"));
        match val {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        let selector = sel(b"hazardTrackingMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HazardTrackingMode::Untracked,
                2 => HazardTrackingMode::Tracked,
                _ => HazardTrackingMode::Default,
            }
        } else {
            HazardTrackingMode::Default
        }
    }

    pub fn resource_options(&self) -> ResourceOptions {
        let selector = sel(b"resourceOptions\0");
        if responds_to_selector(self.raw, selector) {
            ResourceOptions::from_raw(msg_usize(self.raw, selector))
        } else {
            ResourceOptions::from_raw(0)
        }
    }

    pub fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState {
        let val = msg_usize_usize(self.raw, sel(b"setPurgeableState:\0"), state as usize);
        match val {
            1 => PurgeableState::KeepCurrent,
            2 => PurgeableState::NonVolatile,
            3 => PurgeableState::Volatile,
            4 => PurgeableState::Empty,
            _ => PurgeableState::KeepCurrent,
        }
    }

    pub fn heap(&self) -> Option<Heap> {
        let selector = sel(b"heap\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(Heap { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn heap_offset(&self) -> usize {
        let selector = sel(b"heapOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn allocated_size(&self) -> usize {
        let selector = sel(b"allocatedSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            self.len()
        }
    }

    pub fn make_aliasable(&self) {
        let selector = sel(b"makeAliasable\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
        }
    }

    pub fn is_aliasable(&self) -> bool {
        let selector = sel(b"isAliasable\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn set_owner_with_identity(&self, task_id_token: u32) -> i32 {
        let selector = sel(b"setOwnerWithIdentity:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, u32) -> i32 =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, task_id_token)
            }
        } else {
            0
        }
    }

    pub fn new_texture_with_descriptor(
        &self,
        descriptor: &TextureDescriptor,
        offset: usize,
        bytes_per_row: usize,
    ) -> Result<Texture, MetalError> {
        unsafe {
            let selector = sel(b"newTextureWithDescriptor:offset:bytesPerRow:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "newTextureWithDescriptor:offset:bytesPerRow: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(self.raw, selector, descriptor.raw, offset, bytes_per_row);
            if raw.is_null() {
                Err(MetalError::new("failed to create buffer-backed texture"))
            } else {
                Ok(Texture { raw })
            }
        }
    }

    pub fn add_debug_marker(&self, marker: &str, range: Range) -> Result<(), MetalError> {
        let selector = sel(b"addDebugMarker:range:\0");
        if responds_to_selector(self.raw, selector) {
            let ns_marker = NSString::new(marker);
            msg_void_id_range(self.raw, selector, ns_marker.raw(), range);
            Ok(())
        } else {
            Err(MetalError::new("addDebugMarker:range: not supported"))
        }
    }

    pub fn remove_all_debug_markers(&self) -> Result<(), MetalError> {
        let selector = sel(b"removeAllDebugMarkers\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
            Ok(())
        } else {
            Err(MetalError::new("removeAllDebugMarkers not supported"))
        }
    }

    pub fn remote_storage_buffer(&self) -> Option<Buffer> {
        let selector = sel(b"remoteStorageBuffer\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(Buffer { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn new_remote_buffer_view_for_device(&self, device: &Device) -> Result<Buffer, MetalError> {
        let selector = sel(b"newRemoteBufferViewForDevice:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newRemoteBufferViewForDevice: not supported",
            ));
        }
        let raw = msg_id_id(self.raw, selector, device.raw);
        if raw.is_null() {
            Err(MetalError::new("failed to create remote buffer view"))
        } else {
            Ok(Buffer { raw })
        }
    }

    pub fn new_tensor_with_descriptor(
        &self,
        descriptor: &TensorDescriptor,
        offset: usize,
    ) -> Result<Tensor, MetalError> {
        let selector = sel(b"newTensorWithDescriptor:offset:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newTensorWithDescriptor:offset:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, descriptor.raw, offset, &mut error)
        };
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal tensor from buffer",
            )))
        } else {
            Ok(Tensor { raw })
        }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self { raw: retain(self.raw) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct TextureDescriptor {
    pub raw: id,
}

impl TextureDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLTextureDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn texture_buffer(
        pixel_format: PixelFormat,
        width: usize,
        resource_options: ResourceOptions,
        usage: TextureUsage,
    ) -> Self {
        let selector =
            sel(b"textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:\0");
        let raw = unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            retain(f(
                class(b"MTLTextureDescriptor\0"),
                selector,
                pixel_format.as_raw(),
                width,
                resource_options.as_raw(),
                usage.as_raw(),
            ))
        };
        Self { raw }
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

    pub fn texture_3d(
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        depth: usize,
        mipmapped: bool,
    ) -> Self {
        let descriptor = Self::texture_2d(pixel_format, width, height, mipmapped);
        descriptor.set_texture_type(TextureType::D3);
        descriptor.set_depth(depth);
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
        msg_void_usize(self.raw, sel(b"setTextureType:\0"), texture_type as usize);
    }

    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
    }

    pub fn set_width(&self, width: usize) {
        msg_void_usize(self.raw, sel(b"setWidth:\0"), width);
    }

    pub fn set_height(&self, height: usize) {
        msg_void_usize(self.raw, sel(b"setHeight:\0"), height);
    }

    pub fn set_depth(&self, depth: usize) {
        msg_void_usize(self.raw, sel(b"setDepth:\0"), depth);
    }

    pub fn set_mipmap_level_count(&self, mipmap_level_count: usize) {
        msg_void_usize(self.raw, sel(b"setMipmapLevelCount:\0"), mipmap_level_count);
    }

    pub fn set_array_length(&self, array_length: usize) {
        msg_void_usize(self.raw, sel(b"setArrayLength:\0"), array_length);
    }

    pub fn set_sample_count(&self, sample_count: usize) {
        msg_void_usize(self.raw, sel(b"setSampleCount:\0"), sample_count);
    }

    pub fn set_usage(&self, usage: TextureUsage) {
        msg_void_usize(self.raw, sel(b"setUsage:\0"), usage.as_raw());
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        msg_void_usize(self.raw, sel(b"setStorageMode:\0"), storage_mode as usize);
    }

    pub fn texture_type(&self) -> TextureType {
        let val = msg_usize(self.raw, sel(b"textureType\0"));
        match val {
            0 => TextureType::D1,
            1 => TextureType::D1Array,
            2 => TextureType::D2,
            3 => TextureType::D2Array,
            4 => TextureType::D2Multisample,
            5 => TextureType::Cube,
            6 => TextureType::CubeArray,
            7 => TextureType::D3,
            8 => TextureType::D2MultisampleArray,
            9 => TextureType::TextureBuffer,
            _ => TextureType::D2,
        }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_raw(msg_usize(self.raw, sel(b"pixelFormat\0")))
    }

    pub fn width(&self) -> usize {
        msg_usize(self.raw, sel(b"width\0"))
    }

    pub fn height(&self) -> usize {
        msg_usize(self.raw, sel(b"height\0"))
    }

    pub fn depth(&self) -> usize {
        msg_usize(self.raw, sel(b"depth\0"))
    }

    pub fn mipmap_level_count(&self) -> usize {
        msg_usize(self.raw, sel(b"mipmapLevelCount\0"))
    }

    pub fn array_length(&self) -> usize {
        msg_usize(self.raw, sel(b"arrayLength\0"))
    }

    pub fn sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"sampleCount\0"))
    }

    pub fn usage(&self) -> TextureUsage {
        TextureUsage::from_raw(msg_usize(self.raw, sel(b"usage\0")))
    }

    pub fn storage_mode(&self) -> StorageMode {
        let val = msg_usize(self.raw, sel(b"storageMode\0"));
        match val {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        let val = msg_usize(self.raw, sel(b"cpuCacheMode\0"));
        match val {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn set_cpu_cache_mode(&self, cpu_cache_mode: CpuCacheMode) {
        msg_void_usize(
            self.raw,
            sel(b"setCpuCacheMode:\0"),
            cpu_cache_mode as usize,
        );
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        let selector = sel(b"hazardTrackingMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HazardTrackingMode::Untracked,
                2 => HazardTrackingMode::Tracked,
                _ => HazardTrackingMode::Default,
            }
        } else {
            HazardTrackingMode::Default
        }
    }

    pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: HazardTrackingMode) {
        let selector = sel(b"setHazardTrackingMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, hazard_tracking_mode as usize);
        }
    }

    pub fn resource_options(&self) -> ResourceOptions {
        let selector = sel(b"resourceOptions\0");
        if responds_to_selector(self.raw, selector) {
            ResourceOptions::from_raw(msg_usize(self.raw, selector))
        } else {
            ResourceOptions::from_raw(0)
        }
    }

    pub fn set_resource_options(&self, resource_options: ResourceOptions) {
        msg_void_usize(
            self.raw,
            sel(b"setResourceOptions:\0"),
            resource_options.as_raw(),
        );
    }

    pub fn allow_gpu_optimized_contents(&self) -> bool {
        let selector = sel(b"allowGPUOptimizedContents\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            true
        }
    }

    pub fn set_allow_gpu_optimized_contents(&self, allow: bool) {
        let selector = sel(b"setAllowGPUOptimizedContents:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if allow { YES } else { NO });
        }
    }

    pub fn compression_type(&self) -> TextureCompressionType {
        let selector = sel(b"compressionType\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => TextureCompressionType::Lossy,
                _ => TextureCompressionType::Lossless,
            }
        } else {
            TextureCompressionType::Lossless
        }
    }

    pub fn set_compression_type(&self, compression_type: TextureCompressionType) {
        let selector = sel(b"setCompressionType:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, compression_type as usize);
        }
    }

    pub fn swizzle(&self) -> TextureSwizzleChannels {
        let selector = sel(b"swizzle\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL) -> TextureSwizzleChannels =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector)
            }
        } else {
            TextureSwizzleChannels::default()
        }
    }

    pub fn set_swizzle(&self, swizzle: TextureSwizzleChannels) {
        let selector = sel(b"setSwizzle:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, TextureSwizzleChannels) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, swizzle);
            }
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
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Texture {
    pub raw: id,
}

impl Texture {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

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
        let raw = msg_id_usize(
            self.raw,
            sel(b"newTextureViewWithPixelFormat:\0"),
            pixel_format.as_raw(),
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal texture view"))
        } else {
            Ok(Texture { raw })
        }
    }

    pub fn new_texture_view_with_type_and_ranges(
        &self,
        pixel_format: PixelFormat,
        texture_type: TextureType,
        levels: Range,
        slices: Range,
    ) -> Result<Texture, MetalError> {
        let selector = sel(b"newTextureViewWithPixelFormat:textureType:levels:slices:\0");
        let raw = unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, Range, Range) -> id =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                pixel_format.as_raw(),
                texture_type as usize,
                levels,
                slices,
            )
        };
        if raw.is_null() {
            Err(MetalError::new("failed to create texture view with ranges"))
        } else {
            Ok(Texture { raw: retain(raw) })
        }
    }

    pub fn new_texture_view_with_swizzle(
        &self,
        pixel_format: PixelFormat,
        texture_type: TextureType,
        levels: Range,
        slices: Range,
        swizzle: TextureSwizzleChannels,
    ) -> Result<Texture, MetalError> {
        let selector = sel(b"newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "swizzled texture views are not supported on this macOS version",
            ));
        }
        let raw = unsafe {
            let f: unsafe extern "C" fn(
                id,
                SEL,
                usize,
                usize,
                Range,
                Range,
                TextureSwizzleChannels,
            ) -> id = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                pixel_format.as_raw(),
                texture_type as usize,
                levels,
                slices,
                swizzle,
            )
        };
        if raw.is_null() {
            Err(MetalError::new("failed to create swizzled texture view"))
        } else {
            Ok(Texture { raw: retain(raw) })
        }
    }

    pub fn new_texture_view_with_descriptor(
        &self,
        descriptor: &TextureViewDescriptor,
    ) -> Result<Texture, MetalError> {
        let selector = sel(b"newTextureViewWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newTextureViewWithDescriptor: not supported on this macOS version",
            ));
        }
        let raw = msg_id_id(self.raw, selector, descriptor.raw);
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create texture view with descriptor",
            ))
        } else {
            Ok(Texture { raw: retain(raw) })
        }
    }

    pub fn new_shared_texture_handle(&self) -> Option<SharedTextureHandle> {
        let selector = sel(b"newSharedTextureHandle\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(SharedTextureHandle { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn parent_texture(&self) -> Option<Texture> {
        let selector = sel(b"parentTexture\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(Texture { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn parent_relative_level(&self) -> usize {
        let selector = sel(b"parentRelativeLevel\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn parent_relative_slice(&self) -> usize {
        let selector = sel(b"parentRelativeSlice\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn buffer(&self) -> Option<Buffer> {
        let selector = sel(b"buffer\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(Buffer { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn buffer_offset(&self) -> usize {
        let selector = sel(b"bufferOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn buffer_bytes_per_row(&self) -> usize {
        let selector = sel(b"bufferBytesPerRow\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn iosurface(&self) -> Option<id> {
        let selector = sel(b"iosurface\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() { None } else { Some(ptr) }
        } else {
            None
        }
    }

    pub fn iosurface_plane(&self) -> usize {
        let selector = sel(b"iosurfacePlane\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn texture_type(&self) -> TextureType {
        let val = msg_usize(self.raw, sel(b"textureType\0"));
        match val {
            0 => TextureType::D1,
            1 => TextureType::D1Array,
            2 => TextureType::D2,
            3 => TextureType::D2Array,
            4 => TextureType::D2Multisample,
            5 => TextureType::Cube,
            6 => TextureType::CubeArray,
            7 => TextureType::D3,
            8 => TextureType::D2MultisampleArray,
            9 => TextureType::TextureBuffer,
            _ => TextureType::D2,
        }
    }

    pub fn depth(&self) -> usize {
        msg_usize(self.raw, sel(b"depth\0"))
    }

    pub fn mipmap_level_count(&self) -> usize {
        msg_usize(self.raw, sel(b"mipmapLevelCount\0"))
    }

    pub fn sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"sampleCount\0"))
    }

    pub fn array_length(&self) -> usize {
        msg_usize(self.raw, sel(b"arrayLength\0"))
    }

    pub fn usage(&self) -> TextureUsage {
        TextureUsage::from_raw(msg_usize(self.raw, sel(b"usage\0")))
    }

    pub fn is_shareable(&self) -> bool {
        let selector = sel(b"isShareable\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn is_framebuffer_only(&self) -> bool {
        let selector = sel(b"isFramebufferOnly\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn first_mipmap_in_tail(&self) -> usize {
        let selector = sel(b"firstMipmapInTail\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn tail_size_in_bytes(&self) -> usize {
        let selector = sel(b"tailSizeInBytes\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn is_sparse(&self) -> bool {
        let selector = sel(b"isSparse\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn allow_gpu_optimized_contents(&self) -> bool {
        let selector = sel(b"allowGPUOptimizedContents\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            true
        }
    }

    pub fn compression_type(&self) -> TextureCompressionType {
        let selector = sel(b"compressionType\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => TextureCompressionType::Lossy,
                _ => TextureCompressionType::Lossless,
            }
        } else {
            TextureCompressionType::Lossless
        }
    }

    pub fn swizzle(&self) -> TextureSwizzleChannels {
        let selector = sel(b"swizzle\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL) -> TextureSwizzleChannels =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector)
            }
        } else {
            TextureSwizzleChannels::default()
        }
    }

    pub fn width(&self) -> usize {
        msg_usize(self.raw, sel(b"width\0"))
    }

    pub fn height(&self) -> usize {
        msg_usize(self.raw, sel(b"height\0"))
    }

    pub fn pixel_format(&self) -> usize {
        msg_usize(self.raw, sel(b"pixelFormat\0"))
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on this Texture",
            ))
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        let val = msg_usize(self.raw, sel(b"cpuCacheMode\0"));
        match val {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn storage_mode(&self) -> StorageMode {
        let val = msg_usize(self.raw, sel(b"storageMode\0"));
        match val {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        let selector = sel(b"hazardTrackingMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HazardTrackingMode::Untracked,
                2 => HazardTrackingMode::Tracked,
                _ => HazardTrackingMode::Default,
            }
        } else {
            HazardTrackingMode::Default
        }
    }

    pub fn resource_options(&self) -> ResourceOptions {
        let selector = sel(b"resourceOptions\0");
        if responds_to_selector(self.raw, selector) {
            ResourceOptions::from_raw(msg_usize(self.raw, selector))
        } else {
            ResourceOptions::from_raw(0)
        }
    }

    pub fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState {
        let val = msg_usize_usize(self.raw, sel(b"setPurgeableState:\0"), state as usize);
        match val {
            1 => PurgeableState::KeepCurrent,
            2 => PurgeableState::NonVolatile,
            3 => PurgeableState::Volatile,
            4 => PurgeableState::Empty,
            _ => PurgeableState::KeepCurrent,
        }
    }

    pub fn heap(&self) -> Option<Heap> {
        let selector = sel(b"heap\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(Heap { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn heap_offset(&self) -> usize {
        let selector = sel(b"heapOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn allocated_size(&self) -> usize {
        let selector = sel(b"allocatedSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn make_aliasable(&self) {
        let selector = sel(b"makeAliasable\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
        }
    }

    pub fn is_aliasable(&self) -> bool {
        let selector = sel(b"isAliasable\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn set_owner_with_identity(&self, task_id_token: u32) -> i32 {
        let selector = sel(b"setOwnerWithIdentity:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, u32) -> i32 =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, task_id_token)
            }
        } else {
            0
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self { raw: retain(self.raw) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct HeapDescriptor {
    pub raw: id,
}

impl HeapDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLHeapDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn heap_type(&self) -> HeapType {
        let selector = sel(b"type\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HeapType::Placement,
                2 => HeapType::Sparse,
                _ => HeapType::Automatic,
            }
        } else {
            HeapType::Automatic
        }
    }

    pub fn set_heap_type(&self, heap_type: HeapType) {
        msg_void_usize(self.raw, sel(b"setType:\0"), heap_type as usize);
    }

    pub fn storage_mode(&self) -> StorageMode {
        let val = msg_usize(self.raw, sel(b"storageMode\0"));
        match val {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        msg_void_usize(self.raw, sel(b"setStorageMode:\0"), storage_mode as usize);
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        let val = msg_usize(self.raw, sel(b"cpuCacheMode\0"));
        match val {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn set_cpu_cache_mode(&self, cpu_cache_mode: CpuCacheMode) {
        msg_void_usize(
            self.raw,
            sel(b"setCpuCacheMode:\0"),
            cpu_cache_mode as usize,
        );
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        let selector = sel(b"hazardTrackingMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HazardTrackingMode::Untracked,
                2 => HazardTrackingMode::Tracked,
                _ => HazardTrackingMode::Default,
            }
        } else {
            HazardTrackingMode::Default
        }
    }

    pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: HazardTrackingMode) {
        msg_void_usize(
            self.raw,
            sel(b"setHazardTrackingMode:\0"),
            hazard_tracking_mode as usize,
        );
    }

    pub fn resource_options(&self) -> ResourceOptions {
        let selector = sel(b"resourceOptions\0");
        if responds_to_selector(self.raw, selector) {
            ResourceOptions::from_raw(msg_usize(self.raw, selector))
        } else {
            ResourceOptions::from_raw(0)
        }
    }

    pub fn set_resource_options(&self, resource_options: ResourceOptions) {
        msg_void_usize(
            self.raw,
            sel(b"setResourceOptions:\0"),
            resource_options.as_raw(),
        );
    }

    pub fn size(&self) -> usize {
        msg_usize(self.raw, sel(b"size\0"))
    }

    pub fn set_size(&self, size: usize) {
        msg_void_usize(self.raw, sel(b"setSize:\0"), size);
    }

    pub fn sparse_page_size(&self) -> SparsePageSize {
        let val = msg_usize(self.raw, sel(b"sparsePageSize\0"));
        match val {
            102 => SparsePageSize::Size64,
            103 => SparsePageSize::Size256,
            _ => SparsePageSize::Size16,
        }
    }

    pub fn set_sparse_page_size(&self, size: SparsePageSize) {
        msg_void_usize(self.raw, sel(b"setSparsePageSize:\0"), size as usize);
    }
}

impl Default for HeapDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HeapDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Heap {
    pub raw: id,
}

impl Heap {
    pub fn new_buffer(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        let raw = msg_id_usize_usize(
            self.raw,
            sel(b"newBufferWithLength:options:\0"),
            length,
            options.as_raw(),
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal heap buffer"))
        } else {
            Ok(Buffer { raw })
        }
    }

    pub fn new_buffer_at_offset(
        &self,
        length: usize,
        options: ResourceOptions,
        offset: usize,
    ) -> Result<Buffer, MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newBufferWithLength:options:offset:\0"),
                length,
                options.as_raw(),
                offset,
            );
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal placement heap buffer",
                ))
            } else {
                Ok(Buffer { raw })
            }
        }
    }

    pub fn new_texture(&self, descriptor: &TextureDescriptor) -> Result<Texture, MetalError> {
        let raw = msg_id_id(
            self.raw,
            sel(b"newTextureWithDescriptor:\0"),
            descriptor.raw,
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal heap texture"))
        } else {
            Ok(Texture { raw })
        }
    }

    pub fn new_texture_at_offset(
        &self,
        descriptor: &TextureDescriptor,
        offset: usize,
    ) -> Result<Texture, MetalError> {
        let raw = msg_id_id_usize(
            self.raw,
            sel(b"newTextureWithDescriptor:offset:\0"),
            descriptor.raw,
            offset,
        );
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal placement heap texture",
            ))
        } else {
            Ok(Texture { raw })
        }
    }

    pub fn new_acceleration_structure(
        &self,
        size: usize,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithSize:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_usize(self.raw, selector, size);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create heap acceleration structure",
                ))
            } else {
                Ok(AccelerationStructure { raw })
            }
        } else {
            Err(MetalError::new(
                "newAccelerationStructureWithSize: not supported on this Heap",
            ))
        }
    }

    pub fn new_acceleration_structure_at_offset(
        &self,
        size: usize,
        offset: usize,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithSize:offset:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_usize_usize(self.raw, selector, size, offset);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create heap placement acceleration structure",
                ))
            } else {
                Ok(AccelerationStructure { raw })
            }
        } else {
            Err(MetalError::new(
                "newAccelerationStructureWithSize:offset: not supported on this Heap",
            ))
        }
    }

    pub fn new_acceleration_structure_with_descriptor(
        &self,
        descriptor: &PrimitiveAccelerationStructureDescriptor,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithDescriptor:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id(self.raw, selector, descriptor.raw);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create heap acceleration structure with descriptor",
                ))
            } else {
                Ok(AccelerationStructure { raw })
            }
        } else {
            Err(MetalError::new(
                "newAccelerationStructureWithDescriptor: not supported on this Heap",
            ))
        }
    }

    pub fn new_acceleration_structure_with_descriptor_at_offset(
        &self,
        descriptor: &PrimitiveAccelerationStructureDescriptor,
        offset: usize,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithDescriptor:offset:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id_usize(self.raw, selector, descriptor.raw, offset);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create heap placement acceleration structure with descriptor",
                ))
            } else {
                Ok(AccelerationStructure { raw })
            }
        } else {
            Err(MetalError::new(
                "newAccelerationStructureWithDescriptor:offset: not supported on this Heap",
            ))
        }
    }

    pub fn new_instance_acceleration_structure_with_descriptor(
        &self,
        descriptor: &InstanceAccelerationStructureDescriptor,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithDescriptor:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id(self.raw, selector, descriptor.raw);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create heap instance acceleration structure with descriptor",
                ))
            } else {
                Ok(AccelerationStructure { raw })
            }
        } else {
            Err(MetalError::new(
                "newAccelerationStructureWithDescriptor: not supported on this Heap",
            ))
        }
    }

    pub fn new_instance_acceleration_structure_with_descriptor_at_offset(
        &self,
        descriptor: &InstanceAccelerationStructureDescriptor,
        offset: usize,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithDescriptor:offset:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id_usize(self.raw, selector, descriptor.raw, offset);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create heap placement instance acceleration structure with descriptor",
                ))
            } else {
                Ok(AccelerationStructure { raw })
            }
        } else {
            Err(MetalError::new(
                "newAccelerationStructureWithDescriptor:offset: not supported on this Heap",
            ))
        }
    }

    pub fn used_size(&self) -> usize {
        msg_usize(self.raw, sel(b"usedSize\0"))
    }

    pub fn current_allocated_size(&self) -> usize {
        msg_usize(self.raw, sel(b"currentAllocatedSize\0"))
    }

    pub fn max_available_size(&self, alignment: usize) -> usize {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> usize =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"maxAvailableSizeWithAlignment:\0"),
                alignment,
            )
        }
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn storage_mode(&self) -> StorageMode {
        let val = msg_usize(self.raw, sel(b"storageMode\0"));
        match val {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        let val = msg_usize(self.raw, sel(b"cpuCacheMode\0"));
        match val {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        let selector = sel(b"hazardTrackingMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HazardTrackingMode::Untracked,
                2 => HazardTrackingMode::Tracked,
                _ => HazardTrackingMode::Default,
            }
        } else {
            HazardTrackingMode::Default
        }
    }

    pub fn resource_options(&self) -> ResourceOptions {
        let selector = sel(b"resourceOptions\0");
        if responds_to_selector(self.raw, selector) {
            ResourceOptions::from_raw(msg_usize(self.raw, selector))
        } else {
            ResourceOptions::from_raw(0)
        }
    }

    pub fn size(&self) -> usize {
        msg_usize(self.raw, sel(b"size\0"))
    }

    pub fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState {
        let val = msg_usize_usize(self.raw, sel(b"setPurgeableState:\0"), state as usize);
        match val {
            1 => PurgeableState::KeepCurrent,
            2 => PurgeableState::NonVolatile,
            3 => PurgeableState::Volatile,
            4 => PurgeableState::Empty,
            _ => PurgeableState::KeepCurrent,
        }
    }

    pub fn heap_type(&self) -> HeapType {
        let selector = sel(b"type\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HeapType::Placement,
                2 => HeapType::Sparse,
                _ => HeapType::Automatic,
            }
        } else {
            HeapType::Automatic
        }
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ArgumentDescriptor {
    pub raw: id,
}

impl Clone for ArgumentDescriptor {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl ArgumentDescriptor {
    pub fn new() -> Self {
        let raw = retain(msg_id(
            class(b"MTLArgumentDescriptor\0"),
            sel(b"argumentDescriptor\0"),
        ));
        Self { raw }
    }

    pub fn set_data_type(&self, data_type: DataType) {
        msg_void_usize(self.raw, sel(b"setDataType:\0"), data_type as usize);
    }

    pub fn set_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setIndex:\0"), index);
    }

    pub fn set_access(&self, access: ArgumentAccess) {
        msg_void_usize(self.raw, sel(b"setAccess:\0"), access as usize);
    }

    pub fn set_texture_type(&self, texture_type: TextureType) {
        msg_void_usize(self.raw, sel(b"setTextureType:\0"), texture_type as usize);
    }

    pub fn set_array_length(&self, array_length: usize) {
        msg_void_usize(self.raw, sel(b"setArrayLength:\0"), array_length);
    }
}

impl Default for ArgumentDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ArgumentDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ArgumentEncoder {
    pub raw: id,
}

impl ArgumentEncoder {
    fn validate_range(&self, range: Range, count: usize) -> Result<(), MetalError> {
        if count != range.length {
            return Err(MetalError::new(format!(
                "array length {} does not match range length {}",
                count, range.length
            )));
        }
        Ok(())
    }

    fn validate_argument_buffer_offset(&self, offset: usize) -> Result<(), MetalError> {
        let alignment = self.alignment();
        if alignment > 0 && offset % alignment != 0 {
            return Err(MetalError::new(format!(
                "argument buffer offset {} is not aligned to required alignment {}",
                offset, alignment
            )));
        }
        Ok(())
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn encoded_length(&self) -> usize {
        msg_usize(self.raw, sel(b"encodedLength\0"))
    }

    pub fn alignment(&self) -> usize {
        msg_usize(self.raw, sel(b"alignment\0"))
    }

    pub fn set_argument_buffer(
        &self,
        buffer: Option<&Buffer>,
        offset: usize,
    ) -> Result<(), MetalError> {
        self.validate_argument_buffer_offset(offset)?;
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setArgumentBuffer:offset:\0"),
                buffer.map_or(NIL, |b| b.raw),
                offset,
            );
        }
        Ok(())
    }

    pub fn set_argument_buffer_at_array_element(
        &self,
        buffer: Option<&Buffer>,
        start_offset: usize,
        array_element: usize,
    ) -> Result<(), MetalError> {
        self.validate_argument_buffer_offset(start_offset)?;
        let selector = sel(b"setArgumentBuffer:startOffset:arrayElement:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setArgumentBuffer:startOffset:arrayElement: not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                buffer.map_or(NIL, |b| b.raw),
                start_offset,
                array_element,
            );
        }
        Ok(())
    }

    pub fn set_buffer(
        &self,
        index: usize,
        buffer: Option<&Buffer>,
        offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBuffer:offset:atIndex:\0"),
                buffer.map_or(NIL, |b| b.raw),
                offset,
                index,
            );
        }
        Ok(())
    }

    pub fn set_texture(&self, index: usize, texture: &Texture) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"setTexture:atIndex:\0"), texture.raw, index);
        }
    }

    pub fn set_sampler_state(&self, index: usize, sampler: &SamplerState) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setSamplerState:atIndex:\0"),
                sampler.raw,
                index,
            );
        }
    }

    pub fn set_bytes(&self, index: usize, bytes: &[u8]) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBytes:length:atIndex:\0"),
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                index,
            );
        }
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn set_buffers(
        &self,
        buffers: &[Buffer],
        offsets: &[usize],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, buffers.len())?;
        if offsets.len() != range.length {
            return Err(MetalError::new(format!(
                "offset array length {} does not match range length {}",
                offsets.len(),
                range.length
            )));
        }
        msg_void_ptr_ptr_range(
            self.raw,
            sel(b"setBuffers:offsets:withRange:\0"),
            buffers.as_ptr() as *const id,
            offsets.as_ptr(),
            range,
        );
        Ok(())
    }

    pub fn set_textures(&self, textures: &[Texture], range: Range) -> Result<(), MetalError> {
        self.validate_range(range, textures.len())?;
        msg_void_ptr_range(
            self.raw,
            sel(b"setTextures:withRange:\0"),
            textures.as_ptr() as *const id,
            range,
        );
        Ok(())
    }

    pub fn set_sampler_states(
        &self,
        samplers: &[SamplerState],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, samplers.len())?;
        msg_void_ptr_range(
            self.raw,
            sel(b"setSamplerStates:withRange:\0"),
            samplers.as_ptr() as *const id,
            range,
        );
        Ok(())
    }

    pub fn set_visible_function_table(
        &self,
        table: Option<&VisibleFunctionTable>,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setVisibleFunctionTable:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, table.map_or(NIL, |t| t.raw), index);
            Ok(())
        } else {
            Err(MetalError::new(
                "setVisibleFunctionTable:atIndex: not supported",
            ))
        }
    }

    pub fn set_visible_function_tables(
        &self,
        tables: &[VisibleFunctionTable],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, tables.len())?;
        let selector = sel(b"setVisibleFunctionTables:withRange:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_ptr_range(self.raw, selector, tables.as_ptr() as *const id, range);
            Ok(())
        } else {
            Err(MetalError::new(
                "setVisibleFunctionTables:withRange: not supported",
            ))
        }
    }

    pub fn set_intersection_function_table(
        &self,
        table: Option<&IntersectionFunctionTable>,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setIntersectionFunctionTable:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, table.map_or(NIL, |t| t.raw), index);
            Ok(())
        } else {
            Err(MetalError::new(
                "setIntersectionFunctionTable:atIndex: not supported",
            ))
        }
    }

    pub fn set_intersection_function_tables(
        &self,
        tables: &[IntersectionFunctionTable],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, tables.len())?;
        let selector = sel(b"setIntersectionFunctionTables:withRange:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_ptr_range(self.raw, selector, tables.as_ptr() as *const id, range);
            Ok(())
        } else {
            Err(MetalError::new(
                "setIntersectionFunctionTables:withRange: not supported",
            ))
        }
    }

    pub fn set_acceleration_structure(
        &self,
        structure: Option<&AccelerationStructure>,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setAccelerationStructure:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, structure.map_or(NIL, |a| a.raw), index);
            Ok(())
        } else {
            Err(MetalError::new(
                "setAccelerationStructure:atIndex: not supported",
            ))
        }
    }

    pub fn constant_data_at_index(&self, index: usize) -> Result<*mut c_void, MetalError> {
        let selector = sel(b"constantDataAtIndex:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("constantDataAtIndex: not supported"));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> *mut c_void =
                transmute(objc_msgSend as *const c_void);
            let ptr = f(self.raw, selector, index);
            if ptr.is_null() {
                Err(MetalError::new(format!(
                    "constantDataAtIndex: returned null for index {}",
                    index
                )))
            } else {
                Ok(ptr)
            }
        }
    }

    pub fn set_render_pipeline_state(
        &self,
        pipeline: Option<&RenderPipelineState>,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setRenderPipelineState:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, pipeline.map_or(NIL, |p| p.raw), index);
            Ok(())
        } else {
            Err(MetalError::new(
                "setRenderPipelineState:atIndex: not supported",
            ))
        }
    }

    pub fn set_render_pipeline_states(
        &self,
        pipelines: &[RenderPipelineState],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, pipelines.len())?;
        let selector = sel(b"setRenderPipelineStates:withRange:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setRenderPipelineStates:withRange: not supported",
            ));
        }
        msg_void_ptr_range(self.raw, selector, pipelines.as_ptr() as *const id, range);
        Ok(())
    }

    pub fn set_compute_pipeline_state(
        &self,
        pipeline: Option<&ComputePipelineState>,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setComputePipelineState:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, pipeline.map_or(NIL, |p| p.raw), index);
            Ok(())
        } else {
            Err(MetalError::new(
                "setComputePipelineState:atIndex: not supported",
            ))
        }
    }

    pub fn set_compute_pipeline_states(
        &self,
        pipelines: &[ComputePipelineState],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, pipelines.len())?;
        let selector = sel(b"setComputePipelineStates:withRange:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setComputePipelineStates:withRange: not supported",
            ));
        }
        msg_void_ptr_range(self.raw, selector, pipelines.as_ptr() as *const id, range);
        Ok(())
    }

    pub fn set_indirect_command_buffer(
        &self,
        buffer: Option<&IndirectCommandBuffer>,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setIndirectCommandBuffer:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, buffer.map_or(NIL, |b| b.raw), index);
            Ok(())
        } else {
            Err(MetalError::new(
                "setIndirectCommandBuffer:atIndex: not supported",
            ))
        }
    }

    pub fn set_indirect_command_buffers(
        &self,
        buffers: &[IndirectCommandBuffer],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, buffers.len())?;
        let selector = sel(b"setIndirectCommandBuffers:withRange:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "setIndirectCommandBuffers:withRange: not supported",
            ));
        }
        const STACK_CAP: usize = 32;
        if buffers.len() <= STACK_CAP {
            let mut stack = [NIL; STACK_CAP];
            for (slot, buffer) in stack.iter_mut().zip(buffers) {
                *slot = buffer.raw;
            }
            msg_void_ptr_range(self.raw, selector, stack.as_ptr(), range);
        } else {
            let raw_buffers: Vec<id> = buffers.iter().map(|buffer| buffer.raw).collect();
            msg_void_ptr_range(self.raw, selector, raw_buffers.as_ptr(), range);
        }
        Ok(())
    }

    pub fn new_argument_encoder_for_buffer_at_index(
        &self,
        index: usize,
    ) -> Result<ArgumentEncoder, MetalError> {
        let selector = sel(b"newArgumentEncoderForBufferAtIndex:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newArgumentEncoderForBufferAtIndex: not supported",
            ));
        }
        let raw = msg_id_usize(self.raw, selector, index);
        if raw.is_null() {
            Err(MetalError::new(format!(
                "newArgumentEncoderForBufferAtIndex: returned null for index {}",
                index
            )))
        } else {
            Ok(ArgumentEncoder { raw })
        }
    }
}

impl Drop for ArgumentEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct Event {
    pub raw: id,
}

impl Event {
    pub fn device(&self) -> Option<Device> {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        if ptr.is_null() {
            None
        } else {
            Some(Device { raw: ptr })
        }
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SharedEvent {
    pub raw: id,
}

impl SharedEvent {
    pub fn device(&self) -> Option<Device> {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        if ptr.is_null() {
            None
        } else {
            Some(Device { raw: ptr })
        }
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn as_event(&self) -> Event {
        Event {
            raw: retain(self.raw),
        }
    }

    pub fn signaled_value(&self) -> u64 {
        msg_u64(self.raw, sel(b"signaledValue\0"))
    }

    pub fn set_signaled_value(&self, value: u64) {
        msg_void_u64(self.raw, sel(b"setSignaledValue:\0"), value);
    }

    pub fn new_shared_event_handle(&self) -> Result<SharedEventHandle, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"newSharedEventHandle\0")));
        if raw.is_null() {
            Err(MetalError::new("failed to create new shared event handle"))
        } else {
            Ok(SharedEventHandle { raw })
        }
    }

    pub fn wait_until_signaled_value(&self, value: u64, timeout_ms: u64) -> bool {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u64, u64) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"waitUntilSignaledValue:timeoutMS:\0"),
                value,
                timeout_ms,
            ) != 0
        }
    }
}

impl Clone for SharedEvent {
    fn clone(&self) -> Self {
        Self { raw: retain(self.raw) }
    }
}

impl Drop for SharedEvent {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SharedEventHandle {
    pub raw: id,
}

impl SharedEventHandle {
    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }
}

impl Drop for SharedEventHandle {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SharedEventListener {
    pub raw: id,
}

impl SharedEventListener {
    pub fn new() -> Result<Self, MetalError> {
        let cls = class(b"MTLSharedEventListener\0");
        let obj = retain(msg_id(cls, sel(b"alloc\0")));
        let raw = msg_id(obj, sel(b"init\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create MTLSharedEventListener"))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn with_dispatch_queue(queue: *mut c_void) -> Result<Self, MetalError> {
        unsafe {
            let cls = class(b"MTLSharedEventListener\0");
            let obj = retain(msg_id(cls, sel(b"alloc\0")));
            let f: unsafe extern "C" fn(id, SEL, *mut c_void) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(obj, sel(b"initWithDispatchQueue:\0"), queue);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create MTLSharedEventListener with dispatch queue",
                ))
            } else {
                Ok(Self { raw })
            }
        }
    }
}

impl Drop for SharedEventListener {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SharedTextureHandle {
    pub raw: id,
}

impl Drop for SharedTextureHandle {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl SharedTextureHandle {
    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        resource_label(self.raw)
    }
}

#[derive(Debug)]
pub struct TextureViewDescriptor {
    pub raw: id,
}

impl Drop for TextureViewDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl TextureViewDescriptor {
    pub fn new() -> Self {
        let raw = retain(msg_id(class(b"MTLTextureViewDescriptor\0"), sel(b"new\0")));
        Self { raw }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_raw(msg_usize(self.raw, sel(b"pixelFormat\0")))
    }

    pub fn set_pixel_format(&self, format: PixelFormat) {
        msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), format.as_raw());
    }

    pub fn texture_type(&self) -> TextureType {
        let val = msg_usize(self.raw, sel(b"textureType\0"));
        match val {
            0 => TextureType::D1,
            1 => TextureType::D1Array,
            2 => TextureType::D2,
            3 => TextureType::D2Array,
            4 => TextureType::D2Multisample,
            5 => TextureType::Cube,
            6 => TextureType::CubeArray,
            7 => TextureType::D3,
            8 => TextureType::D2MultisampleArray,
            9 => TextureType::TextureBuffer,
            _ => TextureType::D2,
        }
    }

    pub fn set_texture_type(&self, texture_type: TextureType) {
        msg_void_usize(self.raw, sel(b"setTextureType:\0"), texture_type as usize);
    }

    pub fn level_range(&self) -> Range {
        msg_range(self.raw, sel(b"levelRange\0"))
    }

    pub fn set_level_range(&self, range: Range) {
        msg_void_range(self.raw, sel(b"setLevelRange:\0"), range);
    }

    pub fn slice_range(&self) -> Range {
        msg_range(self.raw, sel(b"sliceRange\0"))
    }

    pub fn set_slice_range(&self, range: Range) {
        msg_void_range(self.raw, sel(b"setSliceRange:\0"), range);
    }

    pub fn swizzle(&self) -> TextureSwizzleChannels {
        let selector = sel(b"swizzle\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL) -> TextureSwizzleChannels =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector)
            }
        } else {
            TextureSwizzleChannels::default()
        }
    }

    pub fn set_swizzle(&self, swizzle: TextureSwizzleChannels) {
        let selector = sel(b"setSwizzle:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, TextureSwizzleChannels) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, swizzle);
            }
        }
    }
}

impl Default for TextureViewDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
