use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct IndirectCommandBufferDescriptor {
    pub raw: id,
}

impl IndirectCommandBufferDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(
                class(b"MTLIndirectCommandBufferDescriptor\0"),
                sel(b"alloc\0"),
            );
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_command_types(&self, command_types: IndirectCommandType) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setCommandTypes:\0"), command_types.as_raw());
        }
    }

    pub fn set_inherit_pipeline_state(&self, inherit: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setInheritPipelineState:\0"),
                if inherit { YES } else { NO },
            );
        }
    }

    pub fn set_inherit_buffers(&self, inherit: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setInheritBuffers:\0"),
                if inherit { YES } else { NO },
            );
        }
    }

    pub fn set_max_vertex_buffer_bind_count(&self, count: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMaxVertexBufferBindCount:\0"), count);
        }
    }

    pub fn set_max_fragment_buffer_bind_count(&self, count: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMaxFragmentBufferBindCount:\0"), count);
        }
    }

    pub fn set_max_kernel_buffer_bind_count(&self, count: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMaxKernelBufferBindCount:\0"), count);
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
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct IndirectCommandBuffer {
    pub raw: id,
}

impl IndirectCommandBuffer {
    pub fn reset(&self, range: Range) {
        unsafe {
            msg_void_range(self.raw, sel(b"resetWithRange:\0"), range);
        }
    }

    pub fn render_command(&self, index: usize) -> Result<IndirectRenderCommand, MetalError> {
        unsafe {
            let raw = msg_id_usize(self.raw, sel(b"indirectRenderCommandAtIndex:\0"), index);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to get Metal indirect render command",
                ))
            } else {
                Ok(IndirectRenderCommand { raw })
            }
        }
    }

    pub fn compute_command(&self, index: usize) -> Result<IndirectComputeCommand, MetalError> {
        unsafe {
            let raw = msg_id_usize(self.raw, sel(b"indirectComputeCommandAtIndex:\0"), index);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to get Metal indirect compute command",
                ))
            } else {
                Ok(IndirectComputeCommand { raw })
            }
        }
    }
}

impl Drop for IndirectCommandBuffer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct IndirectRenderCommand {
    pub raw: id,
}

impl IndirectRenderCommand {
    pub fn set_render_pipeline_state(&self, state: &RenderPipelineState) {
        unsafe {
            msg_void_id(self.raw, sel(b"setRenderPipelineState:\0"), state.raw);
        }
    }

    pub fn set_vertex_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            msg_void_id_usize_usize(
                self.raw,
                sel(b"setVertexBuffer:offset:atIndex:\0"),
                buffer.raw,
                offset,
                index,
            );
        }
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
        unsafe { msg_void(self.raw, sel(b"reset\0")) };
    }
}

#[derive(Debug)]
pub struct IndirectComputeCommand {
    pub raw: id,
}

impl IndirectComputeCommand {
    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineState) {
        unsafe {
            msg_void_id(self.raw, sel(b"setComputePipelineState:\0"), state.raw);
        }
    }

    pub fn set_kernel_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            msg_void_id_usize_usize(
                self.raw,
                sel(b"setKernelBuffer:offset:atIndex:\0"),
                buffer.raw,
                offset,
                index,
            );
        }
    }

    pub fn dispatch_threadgroups(&self, threadgroups: Size, threads_per_threadgroup: Size) {
        unsafe {
            msg_void_size_size(
                self.raw,
                sel(b"concurrentDispatchThreadgroups:threadsPerThreadgroup:\0"),
                threadgroups,
                threads_per_threadgroup,
            );
        }
    }

    pub fn dispatch_threads(&self, threads: Size, threads_per_threadgroup: Size) {
        unsafe {
            msg_void_size_size(
                self.raw,
                sel(b"concurrentDispatchThreads:threadsPerThreadgroup:\0"),
                threads,
                threads_per_threadgroup,
            );
        }
    }

    pub fn reset(&self) {
        unsafe { msg_void(self.raw, sel(b"reset\0")) };
    }
}
