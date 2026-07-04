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
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn command_types(&self) -> IndirectCommandType {
        IndirectCommandType::from_raw(msg_usize(self.raw, sel(b"commandTypes\0")))
    }

    pub fn set_command_types(&self, command_types: IndirectCommandType) {
        msg_void_usize(self.raw, sel(b"setCommandTypes:\0"), command_types.as_raw());
    }

    pub fn inherit_pipeline_state(&self) -> bool {
        let selector = sel(b"inheritPipelineState\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn set_inherit_pipeline_state(&self, inherit: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setInheritPipelineState:\0"),
            if inherit { YES } else { NO },
        );
    }

    pub fn inherit_buffers(&self) -> bool {
        msg_bool(self.raw, sel(b"inheritBuffers\0")) != NO
    }

    pub fn set_inherit_buffers(&self, inherit: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setInheritBuffers:\0"),
            if inherit { YES } else { NO },
        );
    }

    pub fn max_vertex_buffer_bind_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxVertexBufferBindCount\0"))
    }

    pub fn set_max_vertex_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxVertexBufferBindCount:\0"), count);
    }

    pub fn max_fragment_buffer_bind_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxFragmentBufferBindCount\0"))
    }

    pub fn set_max_fragment_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxFragmentBufferBindCount:\0"), count);
    }

    pub fn max_kernel_buffer_bind_count(&self) -> usize {
        let selector = sel(b"maxKernelBufferBindCount\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_max_kernel_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxKernelBufferBindCount:\0"), count);
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
            msg_usize(self.raw, sel(b"size\0"))
        }
    }

    pub fn reset(&self, range: Range) -> Result<(), MetalError> {
        self.validate_reset_range(range)?;
        msg_void_range(self.raw, sel(b"resetWithRange:\0"), range);
        Ok(())
    }

    pub fn render_command(&self, index: usize) -> Result<IndirectRenderCommand, MetalError> {
        self.validate_command_index(index)?;
        if self.config.supports_compute_commands() && !self.config.supports_render_commands() {
            return Err(MetalError::new(
                "indirect command buffer was not created with render command types",
            ));
        }
        let raw = msg_id_usize(self.raw, sel(b"indirectRenderCommandAtIndex:\0"), index);
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
        let raw = msg_id_usize(self.raw, sel(b"indirectComputeCommandAtIndex:\0"), index);
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
    pub fn set_render_pipeline_state(
        &self,
        state: &RenderPipelineState,
    ) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 && self.config.inherit_pipeline_state {
            return Err(MetalError::new(
                "cannot set render pipeline state when inheritPipelineState is enabled",
            ));
        }
        msg_void_id(self.raw, sel(b"setRenderPipelineState:\0"), state.raw);
        Ok(())
    }

    pub fn set_vertex_buffer(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 {
            if self.config.inherit_buffers {
                return Err(MetalError::new(
                    "cannot set vertex buffer when inheritBuffers is enabled",
                ));
            }
            if index >= self.config.max_vertex_buffer_bind_count {
                return Err(MetalError::new(format!(
                    "vertex buffer index {} exceeds max vertex buffer bind count {}",
                    index, self.config.max_vertex_buffer_bind_count
                )));
            }
        }
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setVertexBuffer:offset:atIndex:\0"),
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
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:\0"),
                primitive_type as usize,
                vertex_start,
                vertex_count,
                instance_count,
                base_instance,
            );
        }
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
    ) {
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
                sel(b"drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:\0"),
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
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

#[derive(Debug)]
pub struct IndirectComputeCommand {
    pub raw: id,
    config: IndirectCommandBufferConfig,
}

impl IndirectComputeCommand {
    pub fn set_compute_pipeline_state(
        &self,
        state: &ComputePipelineState,
    ) -> Result<(), MetalError> {
        if self.config.max_command_count > 0 && self.config.inherit_pipeline_state {
            return Err(MetalError::new(
                "cannot set compute pipeline state when inheritPipelineState is enabled",
            ));
        }
        msg_void_id(self.raw, sel(b"setComputePipelineState:\0"), state.raw);
        Ok(())
    }

    pub fn set_kernel_buffer(
        &self,
        index: usize,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
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
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setKernelBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
        Ok(())
    }

    pub fn dispatch_threadgroups(&self, threadgroups: Size, threads_per_threadgroup: Size) {
        msg_void_size_size(
            self.raw,
            sel(b"concurrentDispatchThreadgroups:threadsPerThreadgroup:\0"),
            threadgroups,
            threads_per_threadgroup,
        );
    }

    pub fn dispatch_threads(&self, threads: Size, threads_per_threadgroup: Size) {
        msg_void_size_size(
            self.raw,
            sel(b"concurrentDispatchThreads:threadsPerThreadgroup:\0"),
            threads,
            threads_per_threadgroup,
        );
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}
