use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndirectCommandBufferConfig {
    pub max_command_count: usize,
    pub command_types: IndirectCommandType,
    pub inherit_pipeline_state: bool,
    pub inherit_buffers: bool,
    pub max_vertex_buffer_bind_count: usize,
    pub max_fragment_buffer_bind_count: usize,
    pub max_kernel_buffer_bind_count: usize,
    pub support_dynamic_attribute_stride: bool,
    pub max_object_buffer_bind_count: usize,
    pub max_mesh_buffer_bind_count: usize,
}

impl Default for IndirectCommandBufferConfig {
    fn default() -> Self {
        Self {
            max_command_count: 0,
            command_types: IndirectCommandType::from_raw(0),
            inherit_pipeline_state: false,
            inherit_buffers: false,
            max_vertex_buffer_bind_count: 0,
            max_fragment_buffer_bind_count: 0,
            max_kernel_buffer_bind_count: 0,
            support_dynamic_attribute_stride: false,
            max_object_buffer_bind_count: 0,
            max_mesh_buffer_bind_count: 0,
        }
    }
}

impl IndirectCommandBufferConfig {
    pub fn supports_render_commands(self) -> bool {
        self.max_command_count > 0 && self.command_types.is_render()
    }

    pub fn supports_compute_commands(self) -> bool {
        self.max_command_count > 0 && self.command_types.is_compute()
    }
}

#[derive(Debug)]
pub struct IndirectCommandBufferDescriptor {
    pub raw: id,
}

impl IndirectCommandBufferDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLIndirectCommandBufferDescriptor\0"),
            sel!(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel!(b"init\0")),
        }
    }

    pub fn command_types(&self) -> IndirectCommandType {
        IndirectCommandType::from_raw(msg_usize(self.raw, sel!(b"commandTypes\0")))
    }

    pub fn set_command_types(&self, command_types: IndirectCommandType) {
        msg_void_usize(self.raw, sel!(b"setCommandTypes:\0"), command_types.as_raw());
    }

    pub fn inherit_pipeline_state(&self) -> bool {
        let selector = sel!(b"inheritPipelineState\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn set_inherit_pipeline_state(&self, inherit: bool) {
        msg_void_bool(
            self.raw,
            sel!(b"setInheritPipelineState:\0"),
            if inherit { YES } else { NO },
        );
    }

    pub fn inherit_buffers(&self) -> bool {
        msg_bool(self.raw, sel!(b"inheritBuffers\0")) != NO
    }

    pub fn set_inherit_buffers(&self, inherit: bool) {
        msg_void_bool(
            self.raw,
            sel!(b"setInheritBuffers:\0"),
            if inherit { YES } else { NO },
        );
    }

    pub fn max_vertex_buffer_bind_count(&self) -> usize {
        msg_usize(self.raw, sel!(b"maxVertexBufferBindCount\0"))
    }

    pub fn set_max_vertex_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel!(b"setMaxVertexBufferBindCount:\0"), count);
    }

    pub fn max_fragment_buffer_bind_count(&self) -> usize {
        msg_usize(self.raw, sel!(b"maxFragmentBufferBindCount\0"))
    }

    pub fn set_max_fragment_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel!(b"setMaxFragmentBufferBindCount:\0"), count);
    }

    pub fn max_kernel_buffer_bind_count(&self) -> usize {
        let selector = sel!(b"maxKernelBufferBindCount\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_max_kernel_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel!(b"setMaxKernelBufferBindCount:\0"), count);
    }

    pub fn inherit_depth_stencil_state(&self) -> bool {
        let selector = sel!(b"inheritDepthStencilState\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            true
        }
    }

    pub fn set_inherit_depth_stencil_state(&self, inherit: bool) {
        let selector = sel!(b"setInheritDepthStencilState:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if inherit { YES } else { NO });
        }
    }

    pub fn inherit_depth_bias(&self) -> bool {
        let selector = sel!(b"inheritDepthBias\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            true
        }
    }

    pub fn set_inherit_depth_bias(&self, inherit: bool) {
        let selector = sel!(b"setInheritDepthBias:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if inherit { YES } else { NO });
        }
    }

    pub fn inherit_depth_clip_mode(&self) -> bool {
        let selector = sel!(b"inheritDepthClipMode\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            true
        }
    }

    pub fn set_inherit_depth_clip_mode(&self, inherit: bool) {
        let selector = sel!(b"setInheritDepthClipMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if inherit { YES } else { NO });
        }
    }

    pub fn inherit_cull_mode(&self) -> bool {
        let selector = sel!(b"inheritCullMode\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            true
        }
    }

    pub fn set_inherit_cull_mode(&self, inherit: bool) {
        let selector = sel!(b"setInheritCullMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if inherit { YES } else { NO });
        }
    }

    pub fn inherit_front_facing_winding(&self) -> bool {
        let selector = sel!(b"inheritFrontFacingWinding\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            true
        }
    }

    pub fn set_inherit_front_facing_winding(&self, inherit: bool) {
        let selector = sel!(b"setInheritFrontFacingWinding:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if inherit { YES } else { NO });
        }
    }

    pub fn inherit_triangle_fill_mode(&self) -> bool {
        let selector = sel!(b"inheritTriangleFillMode\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            true
        }
    }

    pub fn set_inherit_triangle_fill_mode(&self, inherit: bool) {
        let selector = sel!(b"setInheritTriangleFillMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if inherit { YES } else { NO });
        }
    }

    pub fn support_ray_tracing(&self) -> bool {
        let selector = sel!(b"supportRayTracing\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn set_support_ray_tracing(&self, support: bool) {
        let selector = sel!(b"setSupportRayTracing:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if support { YES } else { NO });
        }
    }

    pub fn support_dynamic_attribute_stride(&self) -> bool {
        let selector = sel!(b"supportDynamicAttributeStride\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn set_support_dynamic_attribute_stride(&self, support: bool) {
        let selector = sel!(b"setSupportDynamicAttributeStride:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if support { YES } else { NO });
        }
    }

    pub fn max_object_buffer_bind_count(&self) -> usize {
        let selector = sel!(b"maxObjectBufferBindCount\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_max_object_buffer_bind_count(&self, count: usize) {
        let selector = sel!(b"setMaxObjectBufferBindCount:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, count);
        }
    }

    pub fn max_mesh_buffer_bind_count(&self) -> usize {
        let selector = sel!(b"maxMeshBufferBindCount\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_max_mesh_buffer_bind_count(&self, count: usize) {
        let selector = sel!(b"setMaxMeshBufferBindCount:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, count);
        }
    }

    pub fn validate(&self) -> Result<(), MetalError> {
        let command_types = self.command_types();
        if command_types.as_raw() == 0 {
            return Err(MetalError::new(
                "indirect command buffer descriptor command types must not be empty",
            ));
        }
        if command_types.is_render() && command_types.is_compute() {
            return Err(MetalError::new(
                "indirect command buffer descriptor cannot mix draw and dispatch command types",
            ));
        }
        Ok(())
    }

    pub fn snapshot_config(&self, max_command_count: usize) -> IndirectCommandBufferConfig {
        IndirectCommandBufferConfig {
            max_command_count,
            command_types: self.command_types(),
            inherit_pipeline_state: self.inherit_pipeline_state(),
            inherit_buffers: self.inherit_buffers(),
            max_vertex_buffer_bind_count: self.max_vertex_buffer_bind_count(),
            max_fragment_buffer_bind_count: self.max_fragment_buffer_bind_count(),
            max_kernel_buffer_bind_count: self.max_kernel_buffer_bind_count(),
            support_dynamic_attribute_stride: self.support_dynamic_attribute_stride(),
            max_object_buffer_bind_count: self.max_object_buffer_bind_count(),
            max_mesh_buffer_bind_count: self.max_mesh_buffer_bind_count(),
        }
    }
}

impl Default for IndirectCommandBufferDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IndirectCommandBufferDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IndirectCommandBuffer {
    pub raw: id,
    config: IndirectCommandBufferConfig,
}

impl IndirectCommandBuffer {
    pub(crate) fn with_config(raw: id, config: IndirectCommandBufferConfig) -> Self {
        Self { raw, config }
    }

    pub fn from_raw(raw: id) -> Self {
        Self {
            raw,
            config: IndirectCommandBufferConfig::default(),
        }
    }

    pub fn config(&self) -> IndirectCommandBufferConfig {
        self.config
    }

    pub fn validate_command_index(&self, index: usize) -> Result<(), MetalError> {
        if self.raw.is_null() {
            return Err(MetalError::new("indirect command buffer is null"));
        }
        if self.config.max_command_count > 0 && index >= self.config.max_command_count {
            return Err(MetalError::new(format!(
                "indirect command index {} is out of bounds for max command count {}",
                index, self.config.max_command_count
            )));
        }
        Ok(())
    }

    pub fn validate_reset_range(&self, range: Range) -> Result<(), MetalError> {
        if self.raw.is_null() {
            return Err(MetalError::new("indirect command buffer is null"));
        }
        if self.config.max_command_count > 0 {
            let end = range.location.saturating_add(range.length);
            if end > self.config.max_command_count {
                return Err(MetalError::new(format!(
                    "indirect command buffer reset range [{}, {}) exceeds max command count {}",
                    range.location, end, self.config.max_command_count
                )));
            }
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        if self.raw.is_null() {
            0
        } else {
            msg_usize(self.raw, sel!(b"size\0"))
        }
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel!(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on this IndirectCommandBuffer",
            ))
        }
    }

    pub fn reset(&self, range: Range) -> Result<(), MetalError> {
        self.validate_reset_range(range)?;
        msg_void_range(self.raw, sel!(b"resetWithRange:\0"), range);
        Ok(())
    }

    pub fn render_command(&self, index: usize) -> Result<IndirectRenderCommand, MetalError> {
        self.validate_command_index(index)?;
        if self.config.supports_compute_commands() && !self.config.supports_render_commands() {
            return Err(MetalError::new(
                "indirect command buffer was not created with render command types",
            ));
        }
        let raw = msg_id_usize(self.raw, sel!(b"indirectRenderCommandAtIndex:\0"), index);
        if raw.is_null() {
            Err(MetalError::new(
                "failed to get Metal indirect render command",
            ))
        } else {
            Ok(IndirectRenderCommand {
                raw,
                config: self.config,
            })
        }
    }

    pub fn compute_command(&self, index: usize) -> Result<IndirectComputeCommand, MetalError> {
        self.validate_command_index(index)?;
        if self.config.supports_render_commands() && !self.config.supports_compute_commands() {
            return Err(MetalError::new(
                "indirect command buffer was not created with compute command types",
            ));
        }
        let raw = msg_id_usize(self.raw, sel!(b"indirectComputeCommandAtIndex:\0"), index);
        if raw.is_null() {
            Err(MetalError::new(
                "failed to get Metal indirect compute command",
            ))
        } else {
            Ok(IndirectComputeCommand {
                raw,
                config: self.config,
            })
        }
    }
}

impl Drop for IndirectCommandBuffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IndirectRenderCommand {
    pub raw: id,
    config: IndirectCommandBufferConfig,
}

impl IndirectRenderCommand {
    fn validate_command_type(&self, command_type: IndirectCommandType) -> Result<(), MetalError> {
        if self.config.max_command_count > 0
            && self.config.command_types.as_raw() & command_type.as_raw() == 0
        {
            return Err(MetalError::new(format!(
                "indirect render command type {:?} was not enabled in the indirect command buffer descriptor",
                command_type.as_raw()
            )));
        }
        Ok(())
    }

    fn validate_buffer_bind(
        &self,
        index: usize,
        max_count: usize,
        kind: &str,
    ) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 {
            if self.config.inherit_buffers {
                return Err(MetalError::new(format!(
                    "cannot set {kind} buffer when inheritBuffers is enabled"
                )));
            }
            if index >= max_count {
                return Err(MetalError::new(format!(
                    "{kind} buffer index {index} exceeds max {kind} buffer bind count {max_count}"
                )));
            }
        }
        Ok(())
    }

    fn validate_dynamic_attribute_stride(&self) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 && !self.config.support_dynamic_attribute_stride {
            return Err(MetalError::new(
                "cannot set attribute stride when supportDynamicAttributeStride is disabled",
            ));
        }
        Ok(())
    }

    pub fn set_render_pipeline_state(&self, state: &RenderPipelineState) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 && self.config.inherit_pipeline_state {
            return Err(MetalError::new(
                "cannot set render pipeline state when inheritPipelineState is enabled",
            ));
        }
        msg_void_id(self.raw, sel!(b"setRenderPipelineState:\0"), state.raw);
        Ok(())
    }

    pub fn set_vertex_buffer(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
        self.validate_buffer_bind(index, self.config.max_vertex_buffer_bind_count, "vertex")?;
        msg_void_id_usize_usize(
            self.raw,
            sel!(b"setVertexBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
        Ok(())
    }

    pub fn set_vertex_buffer_with_attribute_stride(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
        attribute_stride: usize,
    ) -> Result<(), MetalError> {
        self.validate_dynamic_attribute_stride()?;
        self.validate_buffer_bind(index, self.config.max_vertex_buffer_bind_count, "vertex")?;
        let selector = sel!(b"setVertexBuffer:offset:attributeStride:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    buffer.raw,
                    offset,
                    attribute_stride,
                    index,
                );
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "setVertexBuffer:offset:attributeStride:atIndex: not supported",
            ))
        }
    }

    pub fn set_fragment_buffer(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
        self.validate_buffer_bind(
            index,
            self.config.max_fragment_buffer_bind_count,
            "fragment",
        )?;
        msg_void_id_usize_usize(
            self.raw,
            sel!(b"setFragmentBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
        Ok(())
    }

    pub fn draw_primitives(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
        base_instance: usize,
    ) -> Result<(), MetalError> {
        self.validate_command_type(IndirectCommandType::DRAW)?;
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel!(b"drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:\0"),
                primitive_type as usize,
                vertex_start,
                vertex_count,
                instance_count,
                base_instance,
            );
        }
        Ok(())
    }

    pub fn draw_indexed_primitives(
        &self,
        primitive_type: PrimitiveType,
        index_count: usize,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: usize,
        instance_count: usize,
        base_vertex: isize,
        base_instance: usize,
    ) -> Result<(), MetalError> {
        self.validate_command_type(IndirectCommandType::DRAW_INDEXED)?;
        unsafe {
            let f: unsafe extern "C" fn(
                id,
                SEL,
                usize,
                usize,
                usize,
                id,
                usize,
                usize,
                isize,
                usize,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel!(b"drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:\0"),
                primitive_type as usize,
                index_count,
                index_type as usize,
                index_buffer.raw,
                index_buffer_offset,
                instance_count,
                base_vertex,
                base_instance,
            );
        }
        Ok(())
    }

    pub fn draw_patches(
        &self,
        number_of_patch_control_points: usize,
        patch_start: usize,
        patch_count: usize,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: usize,
        instance_count: usize,
        base_instance: usize,
        tessellation_factor_buffer: &Buffer,
        tessellation_factor_buffer_offset: usize,
        tessellation_factor_buffer_instance_stride: usize,
    ) -> Result<(), MetalError> {
        self.validate_command_type(IndirectCommandType::DRAW_PATCH)?;
        let selector = sel!(
            b"drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:\0",
        );
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(
                    id,
                    SEL,
                    usize,
                    usize,
                    usize,
                    id,
                    usize,
                    usize,
                    usize,
                    id,
                    usize,
                    usize,
                ) = transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    number_of_patch_control_points,
                    patch_start,
                    patch_count,
                    patch_index_buffer.map_or(NIL, |b| b.raw),
                    patch_index_buffer_offset,
                    instance_count,
                    base_instance,
                    tessellation_factor_buffer.raw,
                    tessellation_factor_buffer_offset,
                    tessellation_factor_buffer_instance_stride,
                );
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "drawPatches: not supported on this indirect render command",
            ))
        }
    }

    pub fn draw_indexed_patches(
        &self,
        number_of_patch_control_points: usize,
        patch_start: usize,
        patch_count: usize,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: usize,
        control_point_index_buffer: &Buffer,
        control_point_index_buffer_offset: usize,
        instance_count: usize,
        base_instance: usize,
        tessellation_factor_buffer: &Buffer,
        tessellation_factor_buffer_offset: usize,
        tessellation_factor_buffer_instance_stride: usize,
    ) -> Result<(), MetalError> {
        self.validate_command_type(IndirectCommandType::DRAW_INDEXED_PATCH)?;
        let selector = sel!(
            b"drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:\0",
        );
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(
                    id,
                    SEL,
                    usize,
                    usize,
                    usize,
                    id,
                    usize,
                    id,
                    usize,
                    usize,
                    usize,
                    id,
                    usize,
                    usize,
                ) = transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    number_of_patch_control_points,
                    patch_start,
                    patch_count,
                    patch_index_buffer.map_or(NIL, |b| b.raw),
                    patch_index_buffer_offset,
                    control_point_index_buffer.raw,
                    control_point_index_buffer_offset,
                    instance_count,
                    base_instance,
                    tessellation_factor_buffer.raw,
                    tessellation_factor_buffer_offset,
                    tessellation_factor_buffer_instance_stride,
                );
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "drawIndexedPatches: not supported on this indirect render command",
            ))
        }
    }

    pub fn set_barrier(&self) -> Result<(), MetalError> {
        let selector = sel!(b"setBarrier\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
            Ok(())
        } else {
            Err(MetalError::new(
                "setBarrier not supported on this indirect render command",
            ))
        }
    }

    pub fn clear_barrier(&self) -> Result<(), MetalError> {
        let selector = sel!(b"clearBarrier\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
            Ok(())
        } else {
            Err(MetalError::new(
                "clearBarrier not supported on this indirect render command",
            ))
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel!(b"reset\0"));
    }
}

#[derive(Debug)]
pub struct IndirectComputeCommand {
    pub raw: id,
    config: IndirectCommandBufferConfig,
}

impl IndirectComputeCommand {
    fn validate_command_type(&self, command_type: IndirectCommandType) -> Result<(), MetalError> {
        if self.config.max_command_count > 0
            && self.config.command_types.as_raw() & command_type.as_raw() == 0
        {
            return Err(MetalError::new(format!(
                "indirect compute command type {:?} was not enabled in the indirect command buffer descriptor",
                command_type.as_raw()
            )));
        }
        Ok(())
    }

    fn validate_kernel_buffer_bind(&self, index: usize) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 {
            if self.config.inherit_buffers {
                return Err(MetalError::new(
                    "cannot set kernel buffer when inheritBuffers is enabled",
                ));
            }
            if index >= self.config.max_kernel_buffer_bind_count {
                return Err(MetalError::new(format!(
                    "kernel buffer index {} exceeds max kernel buffer bind count {}",
                    index, self.config.max_kernel_buffer_bind_count
                )));
            }
        }
        Ok(())
    }

    fn validate_dynamic_attribute_stride(&self) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 && !self.config.support_dynamic_attribute_stride {
            return Err(MetalError::new(
                "cannot set attribute stride when supportDynamicAttributeStride is disabled",
            ));
        }
        Ok(())
    }

    pub fn set_compute_pipeline_state(
        &self,
        state: &ComputePipelineState,
    ) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 && self.config.inherit_pipeline_state {
            return Err(MetalError::new(
                "cannot set compute pipeline state when inheritPipelineState is enabled",
            ));
        }
        msg_void_id(self.raw, sel!(b"setComputePipelineState:\0"), state.raw);
        Ok(())
    }

    pub fn set_kernel_buffer(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
        self.validate_kernel_buffer_bind(index)?;
        msg_void_id_usize_usize(
            self.raw,
            sel!(b"setKernelBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
        Ok(())
    }

    pub fn set_kernel_buffer_with_attribute_stride(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
        attribute_stride: usize,
    ) -> Result<(), MetalError> {
        self.validate_dynamic_attribute_stride()?;
        self.validate_kernel_buffer_bind(index)?;
        let selector = sel!(b"setKernelBuffer:offset:attributeStride:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    buffer.raw,
                    offset,
                    attribute_stride,
                    index,
                );
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "setKernelBuffer:offset:attributeStride:atIndex: not supported",
            ))
        }
    }

    pub fn set_barrier(&self) -> Result<(), MetalError> {
        let selector = sel!(b"setBarrier\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
            Ok(())
        } else {
            Err(MetalError::new(
                "setBarrier not supported on this indirect compute command",
            ))
        }
    }

    pub fn clear_barrier(&self) -> Result<(), MetalError> {
        let selector = sel!(b"clearBarrier\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
            Ok(())
        } else {
            Err(MetalError::new(
                "clearBarrier not supported on this indirect compute command",
            ))
        }
    }

    pub fn set_threadgroup_memory_length(
        &self,
        length: usize,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel!(b"setThreadgroupMemoryLength:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, length, index);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "setThreadgroupMemoryLength:atIndex: not supported on this indirect compute command",
            ))
        }
    }

    pub fn set_stage_in_region(&self, region: Region) -> Result<(), MetalError> {
        let selector = sel!(b"setStageInRegion:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, Region) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, region);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "setStageInRegion: not supported on this indirect compute command",
            ))
        }
    }

    pub fn dispatch_threadgroups(
        &self,
        threadgroups: Size,
        threads_per_threadgroup: Size,
    ) -> Result<(), MetalError> {
        self.validate_command_type(IndirectCommandType::CONCURRENT_DISPATCH)?;
        msg_void_size_size(
            self.raw,
            sel!(b"concurrentDispatchThreadgroups:threadsPerThreadgroup:\0"),
            threadgroups,
            threads_per_threadgroup,
        );
        Ok(())
    }

    pub fn dispatch_threads(
        &self,
        threads: Size,
        threads_per_threadgroup: Size,
    ) -> Result<(), MetalError> {
        self.validate_command_type(IndirectCommandType::CONCURRENT_DISPATCH_THREADS)?;
        msg_void_size_size(
            self.raw,
            sel!(b"concurrentDispatchThreads:threadsPerThreadgroup:\0"),
            threads,
            threads_per_threadgroup,
        );
        Ok(())
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel!(b"reset\0"));
    }
}
