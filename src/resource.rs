use crate::*;
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr;

#[derive(Debug)]
pub struct Buffer {
    pub raw: id,
}

impl Buffer {
    pub fn len(&self) -> usize {
        msg_usize(self.raw, sel(b"length\0"))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contents(&self) -> *mut c_void {
        msg_id(self.raw, sel(b"contents\0"))
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

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
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
pub struct Texture {
    pub raw: id,
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

    pub fn width(&self) -> usize {
        msg_usize(self.raw, sel(b"width\0"))
    }

    pub fn height(&self) -> usize {
        msg_usize(self.raw, sel(b"height\0"))
    }

    pub fn pixel_format(&self) -> usize {
        msg_usize(self.raw, sel(b"pixelFormat\0"))
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
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

    pub fn set_heap_type(&self, heap_type: HeapType) {
        msg_void_usize(self.raw, sel(b"setType:\0"), heap_type as usize);
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        msg_void_usize(self.raw, sel(b"setStorageMode:\0"), storage_mode as usize);
    }

    pub fn set_cpu_cache_mode(&self, cpu_cache_mode: CpuCacheMode) {
        msg_void_usize(
            self.raw,
            sel(b"setCpuCacheMode:\0"),
            cpu_cache_mode as usize,
        );
    }

    pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: HazardTrackingMode) {
        msg_void_usize(
            self.raw,
            sel(b"setHazardTrackingMode:\0"),
            hazard_tracking_mode as usize,
        );
    }

    pub fn set_size(&self, size: usize) {
        msg_void_usize(self.raw, sel(b"setSize:\0"), size);
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

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ArgumentDescriptor {
    pub raw: id,
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
    pub fn encoded_length(&self) -> usize {
        msg_usize(self.raw, sel(b"encodedLength\0"))
    }

    pub fn alignment(&self) -> usize {
        msg_usize(self.raw, sel(b"alignment\0"))
    }

    pub fn set_argument_buffer(&self, buffer: &Buffer, offset: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setArgumentBuffer:offset:\0"),
                buffer.raw,
                offset,
            );
        }
    }

    pub fn set_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBuffer:offset:atIndex:\0"),
                buffer.raw,
                offset,
                index,
            );
        }
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

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn set_buffers(&self, buffers: &[Option<&Buffer>], offsets: &[usize], range: Range) {
        let raw_buffers: Vec<id> = buffers
            .iter()
            .map(|b| b.map_or(NIL, |buf| buf.raw))
            .collect();
        msg_void_ptr_ptr_range(
            self.raw,
            sel(b"setBuffers:offsets:withRange:\0"),
            raw_buffers.as_ptr(),
            offsets.as_ptr(),
            range,
        );
    }

    pub fn set_textures(&self, textures: &[Option<&Texture>], range: Range) {
        let raw_textures: Vec<id> = textures
            .iter()
            .map(|t| t.map_or(NIL, |tex| tex.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setTextures:withRange:\0"),
            raw_textures.as_ptr(),
            range,
        );
    }

    pub fn set_sampler_states(&self, samplers: &[Option<&SamplerState>], range: Range) {
        let raw_samplers: Vec<id> = samplers
            .iter()
            .map(|s| s.map_or(NIL, |sm| sm.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setSamplerStates:withRange:\0"),
            raw_samplers.as_ptr(),
            range,
        );
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
        tables: &[Option<&VisibleFunctionTable>],
        range: Range,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setVisibleFunctionTables:withRange:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_tables: Vec<id> = tables
                .iter()
                .map(|t| t.map_or(NIL, |tbl| tbl.raw))
                .collect();
            msg_void_ptr_range(self.raw, selector, raw_tables.as_ptr(), range);
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
        tables: &[Option<&IntersectionFunctionTable>],
        range: Range,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setIntersectionFunctionTables:withRange:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_tables: Vec<id> = tables
                .iter()
                .map(|t| t.map_or(NIL, |tbl| tbl.raw))
                .collect();
            msg_void_ptr_range(self.raw, selector, raw_tables.as_ptr(), range);
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
}

impl Drop for ArgumentEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SharedEvent {
    pub raw: id,
}

impl SharedEvent {
    pub fn signaled_value(&self) -> u64 {
        msg_u64(self.raw, sel(b"signaledValue\0"))
    }

    pub fn set_signaled_value(&self, value: u64) {
        msg_void_u64(self.raw, sel(b"setSignaledValue:\0"), value);
    }
}

impl Drop for SharedEvent {
    fn drop(&mut self) {
        release(self.raw);
    }
}
