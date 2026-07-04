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
    pub const EXTENDED_LIMITS: Self = Self(4);
    pub const PREFER_FAST_INTERSECTION: Self = Self(16);
    pub const MINIMIZE_MEMORY: Self = Self(32);

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum MotionBorderMode {
    Clamp = 0,
    Vanish = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum MatrixLayout {
    ColumnMajor = 0,
    RowMajor = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TransformType {
    PackedFloat4x3 = 0,
    Component = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum CurveType {
    Round = 0,
    Flat = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum CurveBasis {
    BSpline = 0,
    CatmullRom = 1,
    Linear = 2,
    Bezier = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum CurveEndCaps {
    None = 0,
    Disk = 1,
    Sphere = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum AccelerationStructureInstanceDescriptorType {
    Default = 0,
    UserID = 1,
    Motion = 2,
    Indirect = 3,
    IndirectMotion = 4,
}

fn acceleration_structure_usage(raw: id) -> AccelerationStructureUsage {
    AccelerationStructureUsage(msg_usize(raw, sel(b"usage\0")))
}

fn set_acceleration_structure_usage(raw: id, usage: AccelerationStructureUsage) {
    msg_void_usize(raw, sel(b"setUsage:\0"), usage.as_raw());
}

fn geometry_descriptor_buffer(raw: id, selector: &[u8]) -> Option<Buffer> {
    let ptr = msg_id(raw, sel(selector));
    if ptr.is_null() {
        None
    } else {
        Some(Buffer { raw: retain(ptr) })
    }
}

fn geometry_descriptor_set_buffer(raw: id, selector: &[u8], buffer: Option<&Buffer>) {
    msg_void_id(
        raw,
        sel(selector),
        buffer.map_or(NIL, |b| b.raw),
    );
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

    pub fn intersection_function_table_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"intersectionFunctionTableOffset\0"))
    }

    pub fn set_intersection_function_table_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setIntersectionFunctionTableOffset:\0"), offset);
    }

    pub fn opaque(&self) -> bool {
        msg_bool(self.raw, sel(b"isOpaque\0")) != NO
    }

    pub fn allow_duplicate_intersection_function_invocation(&self) -> bool {
        msg_bool(
            self.raw,
            sel(b"allowDuplicateIntersectionFunctionInvocation\0"),
        ) != NO
    }

    pub fn set_allow_duplicate_intersection_function_invocation(&self, allow: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setAllowDuplicateIntersectionFunctionInvocation:\0"),
            if allow { YES } else { NO },
        );
    }

    pub fn label(&self) -> Option<String> {
        let selector = sel(b"label\0");
        if responds_to_selector(self.raw, selector) {
            ns_string_to_string(msg_id(self.raw, selector))
        } else {
            None
        }
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn primitive_data_buffer(&self) -> Option<Buffer> {
        let selector = sel(b"primitiveDataBuffer\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_buffer(self.raw, b"primitiveDataBuffer\0")
        } else {
            None
        }
    }

    pub fn set_primitive_data_buffer(&self, buffer: Option<&Buffer>) {
        let selector = sel(b"setPrimitiveDataBuffer:\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_set_buffer(self.raw, b"setPrimitiveDataBuffer:\0", buffer);
        }
    }

    pub fn primitive_data_buffer_offset(&self) -> usize {
        let selector = sel(b"primitiveDataBufferOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_primitive_data_buffer_offset(&self, offset: usize) {
        let selector = sel(b"setPrimitiveDataBufferOffset:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, offset);
        }
    }

    pub fn primitive_data_stride(&self) -> usize {
        let selector = sel(b"primitiveDataStride\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_primitive_data_stride(&self, stride: usize) {
        let selector = sel(b"setPrimitiveDataStride:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, stride);
        }
    }

    pub fn primitive_data_element_size(&self) -> usize {
        let selector = sel(b"primitiveDataElementSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_primitive_data_element_size(&self, size: usize) {
        let selector = sel(b"setPrimitiveDataElementSize:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, size);
        }
    }

    pub fn vertex_buffer(&self) -> Option<Buffer> {
        geometry_descriptor_buffer(self.raw, b"vertexBuffer\0")
    }

    pub fn vertex_buffer_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"vertexBufferOffset\0"))
    }

    pub fn vertex_stride(&self) -> usize {
        msg_usize(self.raw, sel(b"vertexStride\0"))
    }

    pub fn vertex_format(&self) -> VertexFormat {
        let selector = sel(b"vertexFormat\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                28 => VertexFormat::Float,
                29 => VertexFormat::Float2,
                30 => VertexFormat::Float3,
                31 => VertexFormat::Float4,
                _ => VertexFormat::Float3,
            }
        } else {
            VertexFormat::Float3
        }
    }

    pub fn index_buffer(&self) -> Option<Buffer> {
        geometry_descriptor_buffer(self.raw, b"indexBuffer\0")
    }

    pub fn index_buffer_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"indexBufferOffset\0"))
    }

    pub fn index_type(&self) -> IndexType {
        match msg_usize(self.raw, sel(b"indexType\0")) {
            1 => IndexType::UInt16,
            _ => IndexType::UInt32,
        }
    }

    pub fn triangle_count(&self) -> usize {
        msg_usize(self.raw, sel(b"triangleCount\0"))
    }

    pub fn transformation_matrix_buffer(&self) -> Option<Buffer> {
        let selector = sel(b"transformationMatrixBuffer\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_buffer(self.raw, b"transformationMatrixBuffer\0")
        } else {
            None
        }
    }

    pub fn set_transformation_matrix_buffer(&self, buffer: Option<&Buffer>) {
        let selector = sel(b"setTransformationMatrixBuffer:\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_set_buffer(self.raw, b"setTransformationMatrixBuffer:\0", buffer);
        }
    }

    pub fn transformation_matrix_buffer_offset(&self) -> usize {
        let selector = sel(b"transformationMatrixBufferOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_transformation_matrix_buffer_offset(&self, offset: usize) {
        let selector = sel(b"setTransformationMatrixBufferOffset:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, offset);
        }
    }

    pub fn transformation_matrix_layout(&self) -> MatrixLayout {
        let selector = sel(b"transformationMatrixLayout\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                1 => MatrixLayout::RowMajor,
                _ => MatrixLayout::ColumnMajor,
            }
        } else {
            MatrixLayout::ColumnMajor
        }
    }

    pub fn set_transformation_matrix_layout(&self, layout: MatrixLayout) {
        let selector = sel(b"setTransformationMatrixLayout:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, layout as usize);
        }
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

    pub fn intersection_function_table_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"intersectionFunctionTableOffset\0"))
    }

    pub fn set_intersection_function_table_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setIntersectionFunctionTableOffset:\0"), offset);
    }

    pub fn opaque(&self) -> bool {
        msg_bool(self.raw, sel(b"isOpaque\0")) != NO
    }

    pub fn allow_duplicate_intersection_function_invocation(&self) -> bool {
        msg_bool(
            self.raw,
            sel(b"allowDuplicateIntersectionFunctionInvocation\0"),
        ) != NO
    }

    pub fn set_allow_duplicate_intersection_function_invocation(&self, allow: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setAllowDuplicateIntersectionFunctionInvocation:\0"),
            if allow { YES } else { NO },
        );
    }

    pub fn label(&self) -> Option<String> {
        let selector = sel(b"label\0");
        if responds_to_selector(self.raw, selector) {
            ns_string_to_string(msg_id(self.raw, selector))
        } else {
            None
        }
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn primitive_data_buffer(&self) -> Option<Buffer> {
        let selector = sel(b"primitiveDataBuffer\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_buffer(self.raw, b"primitiveDataBuffer\0")
        } else {
            None
        }
    }

    pub fn set_primitive_data_buffer(&self, buffer: Option<&Buffer>) {
        let selector = sel(b"setPrimitiveDataBuffer:\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_set_buffer(self.raw, b"setPrimitiveDataBuffer:\0", buffer);
        }
    }

    pub fn primitive_data_buffer_offset(&self) -> usize {
        let selector = sel(b"primitiveDataBufferOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_primitive_data_buffer_offset(&self, offset: usize) {
        let selector = sel(b"setPrimitiveDataBufferOffset:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, offset);
        }
    }

    pub fn primitive_data_stride(&self) -> usize {
        let selector = sel(b"primitiveDataStride\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_primitive_data_stride(&self, stride: usize) {
        let selector = sel(b"setPrimitiveDataStride:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, stride);
        }
    }

    pub fn primitive_data_element_size(&self) -> usize {
        let selector = sel(b"primitiveDataElementSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_primitive_data_element_size(&self, size: usize) {
        let selector = sel(b"setPrimitiveDataElementSize:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, size);
        }
    }

    pub fn bounding_box_buffer(&self) -> Option<Buffer> {
        geometry_descriptor_buffer(self.raw, b"boundingBoxBuffer\0")
    }

    pub fn bounding_box_buffer_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"boundingBoxBufferOffset\0"))
    }

    pub fn bounding_box_stride(&self) -> usize {
        msg_usize(self.raw, sel(b"boundingBoxStride\0"))
    }

    pub fn bounding_box_count(&self) -> usize {
        msg_usize(self.raw, sel(b"boundingBoxCount\0"))
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

    pub fn usage(&self) -> AccelerationStructureUsage {
        acceleration_structure_usage(self.raw)
    }

    pub fn set_usage(&self, usage: AccelerationStructureUsage) {
        set_acceleration_structure_usage(self.raw, usage);
    }

    pub fn geometry_descriptors(&self) -> Vec<AccelerationStructureGeometryDescriptor> {
        let array = msg_id(self.raw, sel(b"geometryDescriptors\0"));
        if array.is_null() {
            return Vec::new();
        }
        let count = msg_usize(array, sel(b"count\0"));
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let item = retain(msg_id_usize(array, sel(b"objectAtIndex:\0"), i));
            if !item.is_null() {
                result.push(AccelerationStructureGeometryDescriptor { raw: item });
            }
        }
        result
    }

    pub fn motion_start_border_mode(&self) -> MotionBorderMode {
        let selector = sel(b"motionStartBorderMode\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                1 => MotionBorderMode::Vanish,
                _ => MotionBorderMode::Clamp,
            }
        } else {
            MotionBorderMode::Clamp
        }
    }

    pub fn set_motion_start_border_mode(&self, mode: MotionBorderMode) {
        let selector = sel(b"setMotionStartBorderMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, mode as usize);
        }
    }

    pub fn motion_end_border_mode(&self) -> MotionBorderMode {
        let selector = sel(b"motionEndBorderMode\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                1 => MotionBorderMode::Vanish,
                _ => MotionBorderMode::Clamp,
            }
        } else {
            MotionBorderMode::Clamp
        }
    }

    pub fn set_motion_end_border_mode(&self, mode: MotionBorderMode) {
        let selector = sel(b"setMotionEndBorderMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, mode as usize);
        }
    }

    pub fn motion_start_time(&self) -> f32 {
        let selector = sel(b"motionStartTime\0");
        if responds_to_selector(self.raw, selector) {
            msg_f32(self.raw, selector)
        } else {
            0.0
        }
    }

    pub fn set_motion_start_time(&self, time: f32) {
        let selector = sel(b"setMotionStartTime:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_f32(self.raw, selector, time);
        }
    }

    pub fn motion_end_time(&self) -> f32 {
        let selector = sel(b"motionEndTime\0");
        if responds_to_selector(self.raw, selector) {
            msg_f32(self.raw, selector)
        } else {
            1.0
        }
    }

    pub fn set_motion_end_time(&self, time: f32) {
        let selector = sel(b"setMotionEndTime:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_f32(self.raw, selector, time);
        }
    }

    pub fn motion_keyframe_count(&self) -> usize {
        let selector = sel(b"motionKeyframeCount\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            1
        }
    }

    pub fn set_motion_keyframe_count(&self, count: usize) {
        let selector = sel(b"setMotionKeyframeCount:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, count);
        }
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

    pub fn usage(&self) -> AccelerationStructureUsage {
        acceleration_structure_usage(self.raw)
    }

    pub fn set_usage(&self, usage: AccelerationStructureUsage) {
        set_acceleration_structure_usage(self.raw, usage);
    }

    pub fn instance_descriptor_buffer(&self) -> Option<Buffer> {
        geometry_descriptor_buffer(self.raw, b"instanceDescriptorBuffer\0")
    }

    pub fn instance_descriptor_buffer_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"instanceDescriptorBufferOffset\0"))
    }

    pub fn instance_descriptor_stride(&self) -> usize {
        msg_usize(self.raw, sel(b"instanceDescriptorStride\0"))
    }

    pub fn instance_count(&self) -> usize {
        msg_usize(self.raw, sel(b"instanceCount\0"))
    }

    pub fn instanced_acceleration_structures(&self) -> Vec<AccelerationStructure> {
        let array = msg_id(self.raw, sel(b"instancedAccelerationStructures\0"));
        if array.is_null() {
            return Vec::new();
        }
        let count = msg_usize(array, sel(b"count\0"));
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let item = retain(msg_id_usize(array, sel(b"objectAtIndex:\0"), i));
            if !item.is_null() {
                result.push(AccelerationStructure { raw: item });
            }
        }
        result
    }

    pub fn instance_descriptor_type(&self) -> AccelerationStructureInstanceDescriptorType {
        let selector = sel(b"instanceDescriptorType\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                1 => AccelerationStructureInstanceDescriptorType::UserID,
                2 => AccelerationStructureInstanceDescriptorType::Motion,
                3 => AccelerationStructureInstanceDescriptorType::Indirect,
                4 => AccelerationStructureInstanceDescriptorType::IndirectMotion,
                _ => AccelerationStructureInstanceDescriptorType::Default,
            }
        } else {
            AccelerationStructureInstanceDescriptorType::Default
        }
    }

    pub fn set_instance_descriptor_type(
        &self,
        descriptor_type: AccelerationStructureInstanceDescriptorType,
    ) {
        let selector = sel(b"setInstanceDescriptorType:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, descriptor_type as usize);
        }
    }

    pub fn motion_transform_buffer(&self) -> Option<Buffer> {
        let selector = sel(b"motionTransformBuffer\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_buffer(self.raw, b"motionTransformBuffer\0")
        } else {
            None
        }
    }

    pub fn set_motion_transform_buffer(&self, buffer: Option<&Buffer>) {
        let selector = sel(b"setMotionTransformBuffer:\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_set_buffer(self.raw, b"setMotionTransformBuffer:\0", buffer);
        }
    }

    pub fn motion_transform_buffer_offset(&self) -> usize {
        let selector = sel(b"motionTransformBufferOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_motion_transform_buffer_offset(&self, offset: usize) {
        let selector = sel(b"setMotionTransformBufferOffset:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, offset);
        }
    }

    pub fn motion_transform_count(&self) -> usize {
        let selector = sel(b"motionTransformCount\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_motion_transform_count(&self, count: usize) {
        let selector = sel(b"setMotionTransformCount:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, count);
        }
    }

    pub fn instance_transformation_matrix_layout(&self) -> MatrixLayout {
        let selector = sel(b"instanceTransformationMatrixLayout\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                1 => MatrixLayout::RowMajor,
                _ => MatrixLayout::ColumnMajor,
            }
        } else {
            MatrixLayout::ColumnMajor
        }
    }

    pub fn set_instance_transformation_matrix_layout(&self, layout: MatrixLayout) {
        let selector = sel(b"setInstanceTransformationMatrixLayout:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, layout as usize);
        }
    }

    pub fn motion_transform_type(&self) -> TransformType {
        let selector = sel(b"motionTransformType\0");
        if responds_to_selector(self.raw, selector) {
            match msg_usize(self.raw, selector) {
                1 => TransformType::Component,
                _ => TransformType::PackedFloat4x3,
            }
        } else {
            TransformType::PackedFloat4x3
        }
    }

    pub fn set_motion_transform_type(&self, transform_type: TransformType) {
        let selector = sel(b"setMotionTransformType:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, transform_type as usize);
        }
    }

    pub fn motion_transform_stride(&self) -> usize {
        let selector = sel(b"motionTransformStride\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_motion_transform_stride(&self, stride: usize) {
        let selector = sel(b"setMotionTransformStride:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, stride);
        }
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
pub struct MotionKeyframeData {
    pub raw: id,
}

impl MotionKeyframeData {
    pub fn new() -> Self {
        let class_ptr = class(b"MTLMotionKeyframeData\0");
        if class_ptr.is_null() {
            return Self { raw: NIL };
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn data() -> Self {
        let class_ptr = class(b"MTLMotionKeyframeData\0");
        if class_ptr.is_null() {
            return Self { raw: NIL };
        }
        Self {
            raw: retain(msg_id(class_ptr, sel(b"data\0"))),
        }
    }

    pub fn buffer(&self) -> Option<Buffer> {
        geometry_descriptor_buffer(self.raw, b"buffer\0")
    }

    pub fn set_buffer(&self, buffer: Option<&Buffer>) {
        geometry_descriptor_set_buffer(self.raw, b"setBuffer:\0", buffer);
    }

    pub fn offset(&self) -> usize {
        msg_usize(self.raw, sel(b"offset\0"))
    }

    pub fn set_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setOffset:\0"), offset);
    }
}

impl Default for MotionKeyframeData {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MotionKeyframeData {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructureMotionTriangleGeometryDescriptor {
    pub raw: id,
}

impl AccelerationStructureMotionTriangleGeometryDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLAccelerationStructureMotionTriangleGeometryDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_vertex_buffers(&self, keyframes: &[&MotionKeyframeData]) {
        let raw_keyframes: Vec<id> = keyframes.iter().map(|k| k.raw).collect();
        let array = ns_array_from_ids(&raw_keyframes);
        msg_void_id(self.raw, sel(b"setVertexBuffers:\0"), array);
    }

    pub fn set_vertex_format(&self, format: VertexFormat) {
        msg_void_usize(self.raw, sel(b"setVertexFormat:\0"), format as usize);
    }

    pub fn set_vertex_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setVertexStride:\0"), stride);
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

    pub fn set_transformation_matrix_buffer(&self, buffer: Option<&Buffer>) {
        let selector = sel(b"setTransformationMatrixBuffer:\0");
        if responds_to_selector(self.raw, selector) {
            geometry_descriptor_set_buffer(self.raw, b"setTransformationMatrixBuffer:\0", buffer);
        }
    }

    pub fn set_transformation_matrix_buffer_offset(&self, offset: usize) {
        let selector = sel(b"setTransformationMatrixBufferOffset:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, offset);
        }
    }

    pub fn set_transformation_matrix_layout(&self, layout: MatrixLayout) {
        let selector = sel(b"setTransformationMatrixLayout:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, layout as usize);
        }
    }
}

impl Default for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AccelerationStructureMotionTriangleGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    pub raw: id,
}

impl AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_bounding_box_buffers(&self, keyframes: &[&MotionKeyframeData]) {
        let raw_keyframes: Vec<id> = keyframes.iter().map(|k| k.raw).collect();
        let array = ns_array_from_ids(&raw_keyframes);
        msg_void_id(self.raw, sel(b"setBoundingBoxBuffers:\0"), array);
    }

    pub fn set_bounding_box_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setBoundingBoxStride:\0"), stride);
    }

    pub fn set_bounding_box_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setBoundingBoxCount:\0"), count);
    }
}

impl Default for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructureCurveGeometryDescriptor {
    pub raw: id,
}

impl AccelerationStructureCurveGeometryDescriptor {
    pub fn new() -> Self {
        let class_ptr = class(b"MTLAccelerationStructureCurveGeometryDescriptor\0");
        if class_ptr.is_null() {
            return Self { raw: NIL };
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_control_point_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setControlPointBuffer:\0"), buffer.raw);
    }

    pub fn set_control_point_buffer_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setControlPointBufferOffset:\0"), offset);
    }

    pub fn set_control_point_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setControlPointCount:\0"), count);
    }

    pub fn set_control_point_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setControlPointStride:\0"), stride);
    }

    pub fn set_control_point_format(&self, format: AttributeFormat) {
        msg_void_usize(self.raw, sel(b"setControlPointFormat:\0"), format as usize);
    }

    pub fn set_radius_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setRadiusBuffer:\0"), buffer.raw);
    }

    pub fn set_radius_buffer_offset(&self, offset: usize) {
        msg_void_usize(self.raw, sel(b"setRadiusBufferOffset:\0"), offset);
    }

    pub fn set_radius_format(&self, format: AttributeFormat) {
        msg_void_usize(self.raw, sel(b"setRadiusFormat:\0"), format as usize);
    }

    pub fn set_radius_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setRadiusStride:\0"), stride);
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

    pub fn set_segment_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setSegmentCount:\0"), count);
    }

    pub fn set_segment_control_point_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setSegmentControlPointCount:\0"), count);
    }

    pub fn set_curve_type(&self, curve_type: CurveType) {
        msg_void_usize(self.raw, sel(b"setCurveType:\0"), curve_type as usize);
    }

    pub fn set_curve_basis(&self, curve_basis: CurveBasis) {
        msg_void_usize(self.raw, sel(b"setCurveBasis:\0"), curve_basis as usize);
    }

    pub fn set_curve_end_caps(&self, end_caps: CurveEndCaps) {
        msg_void_usize(self.raw, sel(b"setCurveEndCaps:\0"), end_caps as usize);
    }
}

impl Default for AccelerationStructureCurveGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AccelerationStructureCurveGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct AccelerationStructureMotionCurveGeometryDescriptor {
    pub raw: id,
}

impl AccelerationStructureMotionCurveGeometryDescriptor {
    pub fn new() -> Self {
        let class_ptr = class(b"MTLAccelerationStructureMotionCurveGeometryDescriptor\0");
        if class_ptr.is_null() {
            return Self { raw: NIL };
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_control_point_buffers(&self, keyframes: &[&MotionKeyframeData]) {
        let raw_keyframes: Vec<id> = keyframes.iter().map(|k| k.raw).collect();
        let array = ns_array_from_ids(&raw_keyframes);
        msg_void_id(self.raw, sel(b"setControlPointBuffers:\0"), array);
    }

    pub fn set_radius_buffers(&self, keyframes: &[&MotionKeyframeData]) {
        let raw_keyframes: Vec<id> = keyframes.iter().map(|k| k.raw).collect();
        let array = ns_array_from_ids(&raw_keyframes);
        msg_void_id(self.raw, sel(b"setRadiusBuffers:\0"), array);
    }

    pub fn set_control_point_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setControlPointCount:\0"), count);
    }

    pub fn set_control_point_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setControlPointStride:\0"), stride);
    }

    pub fn set_control_point_format(&self, format: AttributeFormat) {
        msg_void_usize(self.raw, sel(b"setControlPointFormat:\0"), format as usize);
    }

    pub fn set_radius_format(&self, format: AttributeFormat) {
        msg_void_usize(self.raw, sel(b"setRadiusFormat:\0"), format as usize);
    }

    pub fn set_radius_stride(&self, stride: usize) {
        msg_void_usize(self.raw, sel(b"setRadiusStride:\0"), stride);
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

    pub fn set_segment_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setSegmentCount:\0"), count);
    }

    pub fn set_segment_control_point_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setSegmentControlPointCount:\0"), count);
    }

    pub fn set_curve_type(&self, curve_type: CurveType) {
        msg_void_usize(self.raw, sel(b"setCurveType:\0"), curve_type as usize);
    }

    pub fn set_curve_basis(&self, curve_basis: CurveBasis) {
        msg_void_usize(self.raw, sel(b"setCurveBasis:\0"), curve_basis as usize);
    }

    pub fn set_curve_end_caps(&self, end_caps: CurveEndCaps) {
        msg_void_usize(self.raw, sel(b"setCurveEndCaps:\0"), end_caps as usize);
    }
}

impl Default for AccelerationStructureMotionCurveGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AccelerationStructureMotionCurveGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IndirectInstanceAccelerationStructureDescriptor {
    pub raw: id,
}

impl IndirectInstanceAccelerationStructureDescriptor {
    pub fn new() -> Self {
        let class_ptr = class(b"MTLIndirectInstanceAccelerationStructureDescriptor\0");
        if class_ptr.is_null() {
            return Self { raw: NIL };
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn usage(&self) -> AccelerationStructureUsage {
        acceleration_structure_usage(self.raw)
    }

    pub fn set_usage(&self, usage: AccelerationStructureUsage) {
        set_acceleration_structure_usage(self.raw, usage);
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

    pub fn set_max_instance_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxInstanceCount:\0"), count);
    }

    pub fn set_instance_count_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setInstanceCountBuffer:\0"), buffer.raw);
    }

    pub fn set_instance_count_buffer_offset(&self, offset: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setInstanceCountBufferOffset:\0"),
            offset,
        );
    }

    pub fn set_instance_descriptor_type(
        &self,
        descriptor_type: AccelerationStructureInstanceDescriptorType,
    ) {
        msg_void_usize(
            self.raw,
            sel(b"setInstanceDescriptorType:\0"),
            descriptor_type as usize,
        );
    }

    pub fn set_motion_transform_buffer(&self, buffer: Option<&Buffer>) {
        geometry_descriptor_set_buffer(self.raw, b"setMotionTransformBuffer:\0", buffer);
    }

    pub fn set_motion_transform_buffer_offset(&self, offset: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMotionTransformBufferOffset:\0"),
            offset,
        );
    }

    pub fn set_max_motion_transform_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxMotionTransformCount:\0"), count);
    }

    pub fn set_motion_transform_count_buffer(&self, buffer: &Buffer) {
        msg_void_id(self.raw, sel(b"setMotionTransformCountBuffer:\0"), buffer.raw);
    }

    pub fn set_motion_transform_count_buffer_offset(&self, offset: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMotionTransformCountBufferOffset:\0"),
            offset,
        );
    }

    pub fn set_instance_transformation_matrix_layout(&self, layout: MatrixLayout) {
        let selector = sel(b"setInstanceTransformationMatrixLayout:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, layout as usize);
        }
    }

    pub fn set_motion_transform_type(&self, transform_type: TransformType) {
        let selector = sel(b"setMotionTransformType:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, transform_type as usize);
        }
    }

    pub fn set_motion_transform_stride(&self, stride: usize) {
        let selector = sel(b"setMotionTransformStride:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, stride);
        }
    }
}

impl Default for IndirectInstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IndirectInstanceAccelerationStructureDescriptor {
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

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
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
            self.size()
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

    pub fn acceleration_structure_command_encoder_with_descriptor(
        &self,
        descriptor: &AccelerationStructurePassDescriptor,
    ) -> Result<AccelerationStructureCommandEncoder, MetalError> {
        let selector = sel(b"accelerationStructureCommandEncoderWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "accelerationStructureCommandEncoderWithDescriptor: not supported on this CommandBuffer",
            ));
        }
        let raw = retain(msg_id_id(self.raw, selector, descriptor.raw));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create AccelerationStructureCommandEncoder with descriptor",
            ))
        } else {
            Ok(AccelerationStructureCommandEncoder { raw })
        }
    }
}

impl AccelerationStructureCommandEncoder {
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) -> Result<(), MetalError> {
        sample_buffer.validate_sample_index(sample_index)?;
        unsafe {
            let selector = sel(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "sampleCountersInBuffer:atSampleIndex:withBarrier: not supported on this AccelerationStructureCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, BOOL) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                sample_buffer.raw,
                sample_index,
                if barrier { YES } else { NO },
            );
            Ok(())
        }
    }

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

    pub fn refit_primitive_acceleration_structure_with_options(
        &self,
        source: &AccelerationStructure,
        descriptor: &PrimitiveAccelerationStructureDescriptor,
        destination: Option<&AccelerationStructure>,
        scratch_buffer: Option<&Buffer>,
        scratch_buffer_offset: usize,
        options: AccelerationStructureRefitOptions,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(
                b"refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:\0",
            );
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "refitAccelerationStructure:...:options: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, id, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source.raw,
                descriptor.raw,
                destination.map_or(NIL, |d| d.raw),
                scratch_buffer.map_or(NIL, |b| b.raw),
                scratch_buffer_offset,
                options.as_raw(),
            );
            Ok(())
        }
    }

    pub fn refit_instance_acceleration_structure_with_options(
        &self,
        source: &AccelerationStructure,
        descriptor: &InstanceAccelerationStructureDescriptor,
        destination: Option<&AccelerationStructure>,
        scratch_buffer: Option<&Buffer>,
        scratch_buffer_offset: usize,
        options: AccelerationStructureRefitOptions,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(
                b"refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:\0",
            );
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "refitAccelerationStructure:...:options: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, id, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source.raw,
                descriptor.raw,
                destination.map_or(NIL, |d| d.raw),
                scratch_buffer.map_or(NIL, |b| b.raw),
                scratch_buffer_offset,
                options.as_raw(),
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

    pub fn write_compacted_acceleration_structure_size_with_data_type(
        &self,
        structure: &AccelerationStructure,
        buffer: &Buffer,
        offset: usize,
        size_data_type: DataType,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(
                b"writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:\0",
            );
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                structure.raw,
                buffer.raw,
                offset,
                size_data_type as usize,
            );
            Ok(())
        }
    }

    pub fn update_fence(&self, fence: &Fence) -> Result<(), MetalError> {
        let selector = sel(b"updateFence:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, fence.raw);
            Ok(())
        } else {
            Err(MetalError::new("updateFence: not supported"))
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence) -> Result<(), MetalError> {
        let selector = sel(b"waitForFence:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, fence.raw);
            Ok(())
        } else {
            Err(MetalError::new("waitForFence: not supported"))
        }
    }

    pub fn use_buffer(&self, buffer: &Buffer, usage: ResourceUsage) -> Result<(), MetalError> {
        let selector = sel(b"useResource:usage:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, buffer.raw, usage.as_raw());
            Ok(())
        } else {
            Err(MetalError::new("useResource:usage: not supported"))
        }
    }

    pub fn use_texture(&self, texture: &Texture, usage: ResourceUsage) -> Result<(), MetalError> {
        let selector = sel(b"useResource:usage:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_usize(self.raw, selector, texture.raw, usage.as_raw());
            Ok(())
        } else {
            Err(MetalError::new("useResource:usage: not supported"))
        }
    }

    pub fn use_resources(&self, resources: &[id], usage: ResourceUsage) -> Result<(), MetalError> {
        let selector = sel(b"useResources:count:usage:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    resources.as_ptr(),
                    resources.len(),
                    usage.as_raw(),
                );
            }
            Ok(())
        } else {
            Err(MetalError::new("useResources:count:usage: not supported"))
        }
    }

    pub fn use_heap(&self, heap: &Heap) -> Result<(), MetalError> {
        let selector = sel(b"useHeap:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, heap.raw);
            Ok(())
        } else {
            Err(MetalError::new("useHeap: not supported"))
        }
    }

    pub fn use_heaps(&self, heaps: &[id]) -> Result<(), MetalError> {
        let selector = sel(b"useHeaps:count:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, heaps.as_ptr(), heaps.len());
            }
            Ok(())
        } else {
            Err(MetalError::new("useHeaps:count: not supported"))
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn insert_debug_signpost(&self, string: &str) {
        let ns_str = NSString::new(string);
        msg_void_id(self.raw, sel(b"insertDebugSignpost:\0"), ns_str.raw());
    }

    pub fn push_debug_group(&self, string: &str) {
        let ns_str = NSString::new(string);
        msg_void_id(self.raw, sel(b"pushDebugGroup:\0"), ns_str.raw());
    }

    pub fn pop_debug_group(&self) {
        msg_void(self.raw, sel(b"popDebugGroup\0"));
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}
