use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AccelerationStructureSizes {
    pub acceleration_structure_size: usize,
    pub build_scratch_buffer_size: usize,
    pub refit_scratch_buffer_size: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccelerationStructureGeometryFlags(pub usize);
impl AccelerationStructureGeometryFlags {
    pub const NONE: Self = Self(0);
    pub const OPAQUE: Self = Self(1);
    pub const NON_OPAQUE: Self = Self(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccelerationStructureUsage(pub usize);
impl AccelerationStructureUsage {
    pub const NONE: Self = Self(0);
    pub const REFIT: Self = Self(1);
    pub const PREFER_FAST_BUILD: Self = Self(2);
    pub const PREFER_FAST_INTERSECTION: Self = Self(4);
}

#[derive(Debug)]
pub struct AccelerationStructureGeometryDescriptor {
    pub raw: id,
}

impl Drop for AccelerationStructureGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructureTriangleGeometryDescriptor {
    pub raw: id,
}

impl AccelerationStructureTriangleGeometryDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLAccelerationStructureTriangleGeometryDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_vertex_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setVertexBuffer:\0"), buffer.raw);
    }

    pub fn set_vertex_buffer_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setVertexBufferOffset:\0"), offset);
    }

    pub fn set_vertex_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setVertexStride:\0"), stride);
    }

    pub fn set_vertex_format(&self, format: VertexFormat) {
        msg_void_usize(self.raw, sel(b"setVertexFormat:\0"), format as usize);
    }

    pub fn set_index_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setIndexBuffer:\0"), buffer.raw);
    }

    pub fn set_index_buffer_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setIndexBufferOffset:\0"), offset);
    }

    pub fn set_index_type(&self, index_type: IndexType) {
        msg_void_usize(self.raw, sel(b"setIndexType:\0"), index_type as usize);
    }

    pub fn set_triangle_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setTriangleCount:\0"), count);
    }

    pub fn set_opaque(&self, opaque: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setOpaque:\0"),
            if opaque { YES } else { NO },
        );
    }
}

impl Default for AccelerationStructureTriangleGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AccelerationStructureTriangleGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructureBoundingBoxGeometryDescriptor {
    pub raw: id,
}

impl AccelerationStructureBoundingBoxGeometryDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLAccelerationStructureBoundingBoxGeometryDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_bounding_box_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setBoundingBoxBuffer:\0"), buffer.raw);
    }

    pub fn set_bounding_box_buffer_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setBoundingBoxBufferOffset:\0"), offset);
    }

    pub fn set_bounding_box_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setBoundingBoxStride:\0"), stride);
    }

    pub fn set_bounding_box_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setBoundingBoxCount:\0"), count);
    }

    pub fn set_opaque(&self, opaque: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setOpaque:\0"),
            if opaque { YES } else { NO },
        );
    }
}

impl Default for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AccelerationStructureBoundingBoxGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct PrimitiveAccelerationStructureDescriptor {
    pub raw: id,
}

impl PrimitiveAccelerationStructureDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLPrimitiveAccelerationStructureDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_geometry_descriptors(
        &self,
        descriptors: &[&AccelerationStructureTriangleGeometryDescriptor],
    ) {
        let raw_descriptors: Vec<id> = descriptors.iter().map(|d| d.raw).collect();
        let array = ns_array_from_ids(&raw_descriptors);
        msg_void_id(self.raw, sel(b"setGeometryDescriptors:\0"), array);
    }

    pub fn set_bounding_box_geometry_descriptors(
        &self,
        descriptors: &[&AccelerationStructureBoundingBoxGeometryDescriptor],
    ) {
        let raw_descriptors: Vec<id> = descriptors.iter().map(|d| d.raw).collect();
        let array = ns_array_from_ids(&raw_descriptors);
        msg_void_id(self.raw, sel(b"setGeometryDescriptors:\0"), array);
    }
}

impl Default for PrimitiveAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PrimitiveAccelerationStructureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct InstanceAccelerationStructureDescriptor {
    pub raw: id,
}

impl InstanceAccelerationStructureDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLInstanceAccelerationStructureDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_instance_descriptor_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setInstanceDescriptorBuffer:\0"), buffer.raw);
    }

    pub fn set_instance_descriptor_buffer_offset(&self, offset: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setInstanceDescriptorBufferOffset:\0"),
            offset,
        );
    }

    pub fn set_instance_descriptor_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setInstanceDescriptorStride:\0"), stride);
    }

    pub fn set_instance_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setInstanceCount:\0"), count);
    }

    pub fn set_instanced_acceleration_structures(&self, structures: &[&AccelerationStructure]) {
        let raw_structures: Vec<id> = structures.iter().map(|s| s.raw).collect();
        let array = ns_array_from_ids(&raw_structures);
        msg_void_id(
            self.raw,
            sel(b"setInstancedAccelerationStructures:\0"),
            array,
        );
    }
}

impl Default for InstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for InstanceAccelerationStructureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructure {
    pub raw: id,
}

impl Drop for AccelerationStructure {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl AccelerationStructure {
    pub fn size(&self) -> usize {
        msg_usize(self.raw, sel(b"size\0"))
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on AccelerationStructure",
            ))
        }
    }
}

#[derive(Debug)]
pub struct AccelerationStructureCommandEncoder {
    pub raw: id,
}

impl Drop for AccelerationStructureCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl Device {
    pub fn supports_raytracing(&self) -> bool {
        let selector = sel(b"supportsRaytracing\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn acceleration_structure_sizes(
        &self,
        descriptor: &PrimitiveAccelerationStructureDescriptor,
    ) -> Result<AccelerationStructureSizes, MetalError> {
        unsafe {
            let selector = sel(b"accelerationStructureSizesWithDescriptor:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "accelerationStructureSizesWithDescriptor: not supported on this Device",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id) -> AccelerationStructureSizes =
                transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector, descriptor.raw))
        }
    }

    pub fn instance_acceleration_structure_sizes(
        &self,
        descriptor: &InstanceAccelerationStructureDescriptor,
    ) -> Result<AccelerationStructureSizes, MetalError> {
        unsafe {
            let selector = sel(b"accelerationStructureSizesWithDescriptor:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "accelerationStructureSizesWithDescriptor: not supported on this Device",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id) -> AccelerationStructureSizes =
                transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector, descriptor.raw))
        }
    }

    pub fn new_acceleration_structure(
        &self,
        size: usize,
    ) -> Result<AccelerationStructure, MetalError> {
        let selector = sel(b"newAccelerationStructureWithSize:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newAccelerationStructureWithSize: not supported on this Device",
            ));
        }
        let raw = msg_id_usize(self.raw, selector, size);
        if raw.is_null() {
            Err(MetalError::new("failed to create AccelerationStructure"))
        } else {
            Ok(AccelerationStructure { raw })
        }
    }
}

impl CommandBuffer {
    pub fn acceleration_structure_command_encoder(
        &self,
    ) -> Result<AccelerationStructureCommandEncoder, MetalError> {
        let selector = sel(b"accelerationStructureCommandEncoder\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "accelerationStructureCommandEncoder: not supported on this CommandBuffer",
            ));
        }
        let raw = retain(msg_id(self.raw, selector));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create AccelerationStructureCommandEncoder",
            ))
        } else {
            Ok(AccelerationStructureCommandEncoder { raw })
        }
    }
}

impl AccelerationStructureCommandEncoder {
    pub fn build_acceleration_structure(
        &self,
        structure: &AccelerationStructure,
        descriptor: &PrimitiveAccelerationStructureDescriptor,
        scratch_buffer: &Buffer,
        scratch_buffer_offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector =
                sel(b"buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                structure.raw,
                descriptor.raw,
                scratch_buffer.raw,
                scratch_buffer_offset,
            );
            Ok(())
        }
    }

    pub fn build_instance_acceleration_structure(
        &self,
        structure: &AccelerationStructure,
        descriptor: &InstanceAccelerationStructureDescriptor,
        scratch_buffer: &Buffer,
        scratch_buffer_offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector =
                sel(b"buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                structure.raw,
                descriptor.raw,
                scratch_buffer.raw,
                scratch_buffer_offset,
            );
            Ok(())
        }
    }

    pub fn refit_primitive_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        descriptor: &PrimitiveAccelerationStructureDescriptor,
        destination: Option<&AccelerationStructure>,
        scratch_buffer: Option<&Buffer>,
        scratch_buffer_offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "refitAccelerationStructure:... not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, id, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source.raw,
                descriptor.raw,
                destination.map_or(NIL, |d| d.raw),
                scratch_buffer.map_or(NIL, |b| b.raw),
                scratch_buffer_offset,
            );
            Ok(())
        }
    }

    pub fn refit_instance_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        descriptor: &InstanceAccelerationStructureDescriptor,
        destination: Option<&AccelerationStructure>,
        scratch_buffer: Option<&Buffer>,
        scratch_buffer_offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "refitAccelerationStructure:... not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, id, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source.raw,
                descriptor.raw,
                destination.map_or(NIL, |d| d.raw),
                scratch_buffer.map_or(NIL, |b| b.raw),
                scratch_buffer_offset,
            );
            Ok(())
        }
    }

    pub fn copy_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        destination: &AccelerationStructure,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"copyAccelerationStructure:toAccelerationStructure:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "copyAccelerationStructure:toAccelerationStructure: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id) = transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, source.raw, destination.raw);
            Ok(())
        }
    }

    pub fn copy_and_compact_acceleration_structure(
        &self,
        source: &AccelerationStructure,
        destination: &AccelerationStructure,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"copyAndCompactAccelerationStructure:toAccelerationStructure:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "copyAndCompactAccelerationStructure:toAccelerationStructure: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id) = transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, source.raw, destination.raw);
            Ok(())
        }
    }

    pub fn write_compacted_acceleration_structure_size(
        &self,
        structure: &AccelerationStructure,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"writeCompactedAccelerationStructureSize:toBuffer:offset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "writeCompactedAccelerationStructureSize:toBuffer:offset: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, structure.raw, buffer.raw, offset);
            Ok(())
        }
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}
