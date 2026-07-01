use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct RenderCommandEncoder {
    pub raw: id,
}

impl RenderCommandEncoder {
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

    pub fn set_vertex_texture(&self, index: usize, texture: &Texture) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"setVertexTexture:atIndex:\0"),
                texture.raw,
                index,
            );
        }
    }

    pub fn set_vertex_sampler_state(&self, index: usize, sampler: &SamplerState) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"setVertexSamplerState:atIndex:\0"),
                sampler.raw,
                index,
            );
        }
    }

    pub fn set_vertex_bytes<T>(&self, index: usize, value: &T) {
        unsafe {
            msg_void_ptr_usize_usize(
                self.raw,
                sel(b"setVertexBytes:length:atIndex:\0"),
                value as *const T as *const c_void,
                std::mem::size_of::<T>(),
                index,
            );
        }
    }

    pub fn set_fragment_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            msg_void_id_usize_usize(
                self.raw,
                sel(b"setFragmentBuffer:offset:atIndex:\0"),
                buffer.raw,
                offset,
                index,
            );
        }
    }

    pub fn set_fragment_texture(&self, index: usize, texture: &Texture) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"setFragmentTexture:atIndex:\0"),
                texture.raw,
                index,
            );
        }
    }

    pub fn set_fragment_sampler_state(&self, index: usize, sampler: &SamplerState) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"setFragmentSamplerState:atIndex:\0"),
                sampler.raw,
                index,
            );
        }
    }

    pub fn set_fragment_bytes<T>(&self, index: usize, value: &T) {
        unsafe {
            msg_void_ptr_usize_usize(
                self.raw,
                sel(b"setFragmentBytes:length:atIndex:\0"),
                value as *const T as *const c_void,
                std::mem::size_of::<T>(),
                index,
            );
        }
    }

    pub fn set_depth_stencil_state(&self, state: &DepthStencilState) {
        unsafe {
            msg_void_id(self.raw, sel(b"setDepthStencilState:\0"), state.raw);
        }
    }

    pub fn set_viewport(&self, viewport: Viewport) {
        unsafe {
            msg_void_viewport(self.raw, sel(b"setViewport:\0"), viewport);
        }
    }

    pub fn set_scissor_rect(&self, rect: ScissorRect) {
        unsafe {
            msg_void_scissor_rect(self.raw, sel(b"setScissorRect:\0"), rect);
        }
    }

    pub fn set_cull_mode(&self, mode: CullMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setCullMode:\0"), mode as usize);
        }
    }

    pub fn set_front_facing_winding(&self, winding: Winding) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setFrontFacingWinding:\0"), winding as usize);
        }
    }

    pub fn set_triangle_fill_mode(&self, mode: TriangleFillMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setTriangleFillMode:\0"), mode as usize);
        }
    }

    pub fn set_depth_bias(&self, bias: f32, slope_scale: f32, clamp: f32) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, f32, f32, f32) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setDepthBias:slopeScale:clamp:\0"),
                bias,
                slope_scale,
                clamp,
            );
        }
    }

    pub fn draw_primitives(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawPrimitives:vertexStart:vertexCount:\0"),
                primitive_type as usize,
                vertex_start,
                vertex_count,
            );
        }
    }

    pub fn draw_primitives_instanced(
        &self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawPrimitives:vertexStart:vertexCount:instanceCount:\0"),
                primitive_type as usize,
                vertex_start,
                vertex_count,
                instance_count,
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
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:\0"),
                primitive_type as usize,
                index_count,
                index_type as usize,
                index_buffer.raw,
                index_buffer_offset,
            );
        }
    }

    pub fn draw_indexed_primitives_instanced(
        &self,
        primitive_type: PrimitiveType,
        index_count: usize,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: usize,
        instance_count: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:\0"),
                primitive_type as usize,
                index_count,
                index_type as usize,
                index_buffer.raw,
                index_buffer_offset,
                instance_count,
            );
        }
    }

    pub fn update_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
        }
    }

    pub fn use_buffer(&self, buffer: &Buffer, usage: ResourceUsage) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"useResource:usage:\0"),
                buffer.raw,
                usage.as_raw(),
            );
        }
    }

    pub fn use_texture(&self, texture: &Texture, usage: ResourceUsage) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"useResource:usage:\0"),
                texture.raw,
                usage.as_raw(),
            );
        }
    }

    pub fn use_heap(&self, heap: &Heap) {
        unsafe {
            msg_void_id(self.raw, sel(b"useHeap:\0"), heap.raw);
        }
    }

    pub fn execute_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        unsafe {
            msg_void_id_range(
                self.raw,
                sel(b"executeCommandsInBuffer:withRange:\0"),
                buffer.raw,
                range,
            );
        }
    }

    pub fn end_encoding(&self) {
        unsafe { msg_void(self.raw, sel(b"endEncoding\0")) };
    }
}

impl Drop for RenderCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct ComputeCommandEncoder {
    pub raw: id,
}

impl ComputeCommandEncoder {
    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineState) {
        unsafe {
            msg_void_id(self.raw, sel(b"setComputePipelineState:\0"), state.raw);
        }
    }

    pub fn set_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            msg_void_id_usize_usize(
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
            msg_void_id_usize(self.raw, sel(b"setTexture:atIndex:\0"), texture.raw, index);
        }
    }

    pub fn set_sampler_state(&self, index: usize, sampler: &SamplerState) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"setSamplerState:atIndex:\0"),
                sampler.raw,
                index,
            );
        }
    }

    pub fn set_bytes<T>(&self, index: usize, value: &T) {
        unsafe {
            msg_void_ptr_usize_usize(
                self.raw,
                sel(b"setBytes:length:atIndex:\0"),
                value as *const T as *const c_void,
                std::mem::size_of::<T>(),
                index,
            );
        }
    }

    pub fn dispatch_threadgroups(&self, threadgroups: Size, threads_per_threadgroup: Size) {
        unsafe {
            msg_void_size_size(
                self.raw,
                sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                threadgroups,
                threads_per_threadgroup,
            );
        }
    }

    pub fn dispatch_threads(&self, threads: Size, threads_per_threadgroup: Size) {
        unsafe {
            msg_void_size_size(
                self.raw,
                sel(b"dispatchThreads:threadsPerThreadgroup:\0"),
                threads,
                threads_per_threadgroup,
            );
        }
    }

    pub fn update_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
        }
    }

    pub fn use_buffer(&self, buffer: &Buffer, usage: ResourceUsage) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"useResource:usage:\0"),
                buffer.raw,
                usage.as_raw(),
            );
        }
    }

    pub fn use_texture(&self, texture: &Texture, usage: ResourceUsage) {
        unsafe {
            msg_void_id_usize(
                self.raw,
                sel(b"useResource:usage:\0"),
                texture.raw,
                usage.as_raw(),
            );
        }
    }

    pub fn use_heap(&self, heap: &Heap) {
        unsafe {
            msg_void_id(self.raw, sel(b"useHeap:\0"), heap.raw);
        }
    }

    pub fn execute_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        unsafe {
            msg_void_id_range(
                self.raw,
                sel(b"executeCommandsInBuffer:withRange:\0"),
                buffer.raw,
                range,
            );
        }
    }

    pub fn end_encoding(&self) {
        unsafe { msg_void(self.raw, sel(b"endEncoding\0")) };
    }
}

impl Drop for ComputeCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct ResourceStateCommandEncoder {
    pub raw: id,
}

impl ResourceStateCommandEncoder {
    pub fn update_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
        }
    }

    pub fn end_encoding(&self) {
        unsafe { msg_void(self.raw, sel(b"endEncoding\0")) };
    }
}

impl Drop for ResourceStateCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct BlitCommandEncoder {
    pub raw: id,
}

impl BlitCommandEncoder {
    pub fn copy_texture_to_texture(
        &self,
        source: &Texture,
        source_origin: Origin,
        source_size: Size,
        destination: &Texture,
        destination_origin: Origin,
    ) {
        unsafe {
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
                sel(b"copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:\0"),
                source.raw,
                0,
                0,
                source_origin,
                source_size,
                destination.raw,
                0,
                0,
                destination_origin,
            );
        }
    }

    pub fn copy_buffer_to_buffer(
        &self,
        source: &Buffer,
        source_offset: usize,
        destination: &Buffer,
        destination_offset: usize,
        size: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:\0"),
                source.raw,
                source_offset,
                destination.raw,
                destination_offset,
                size,
            );
        }
    }

    pub fn copy_buffer_to_texture(
        &self,
        source: &Buffer,
        source_offset: usize,
        source_bytes_per_row: usize,
        source_bytes_per_image: usize,
        source_size: Size,
        destination: &Texture,
        destination_origin: Origin,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(
                id,
                SEL,
                id,
                usize,
                usize,
                usize,
                Size,
                id,
                usize,
                usize,
                Origin,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:\0"),
                source.raw,
                source_offset,
                source_bytes_per_row,
                source_bytes_per_image,
                source_size,
                destination.raw,
                0,
                0,
                destination_origin,
            );
        }
    }

    pub fn generate_mipmaps(&self, texture: &Texture) {
        unsafe {
            msg_void_id(self.raw, sel(b"generateMipmapsForTexture:\0"), texture.raw);
        }
    }

    pub fn synchronize_resource(&self, resource: &Buffer) {
        unsafe {
            msg_void_id(self.raw, sel(b"synchronizeResource:\0"), resource.raw);
        }
    }

    pub fn synchronize_texture(&self, texture: &Texture) {
        unsafe {
            msg_void_id(self.raw, sel(b"synchronizeResource:\0"), texture.raw);
        }
    }

    pub fn update_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe {
            msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
        }
    }

    pub fn end_encoding(&self) {
        unsafe { msg_void(self.raw, sel(b"endEncoding\0")) };
    }
}

impl Drop for BlitCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}
