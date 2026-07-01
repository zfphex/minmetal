use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct RenderCommandEncoder {
    pub raw: id,
}

impl RenderCommandEncoder {
    pub fn set_render_pipeline_state(&self, state: &RenderPipelineState) {
        msg_void_id(self.raw, sel(b"setRenderPipelineState:\0"), state.raw);
    }

    pub fn set_vertex_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setVertexBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
    }

    pub fn set_vertex_texture(&self, index: usize, texture: &Texture) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVertexTexture:atIndex:\0"),
            texture.raw,
            index,
        );
    }

    pub fn set_vertex_sampler_state(&self, index: usize, sampler: &SamplerState) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVertexSamplerState:atIndex:\0"),
            sampler.raw,
            index,
        );
    }

    pub fn set_vertex_bytes<T>(&self, index: usize, value: &T) {
        msg_void_ptr_usize_usize(
            self.raw,
            sel(b"setVertexBytes:length:atIndex:\0"),
            value as *const T as *const c_void,
            std::mem::size_of::<T>(),
            index,
        );
    }

    pub fn set_fragment_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setFragmentBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
    }

    pub fn set_fragment_texture(&self, index: usize, texture: &Texture) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFragmentTexture:atIndex:\0"),
            texture.raw,
            index,
        );
    }

    pub fn set_fragment_sampler_state(&self, index: usize, sampler: &SamplerState) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFragmentSamplerState:atIndex:\0"),
            sampler.raw,
            index,
        );
    }

    pub fn set_fragment_bytes<T>(&self, index: usize, value: &T) {
        msg_void_ptr_usize_usize(
            self.raw,
            sel(b"setFragmentBytes:length:atIndex:\0"),
            value as *const T as *const c_void,
            std::mem::size_of::<T>(),
            index,
        );
    }

    pub fn set_depth_stencil_state(&self, state: &DepthStencilState) {
        msg_void_id(self.raw, sel(b"setDepthStencilState:\0"), state.raw);
    }

    pub fn set_viewport(&self, viewport: Viewport) {
        msg_void_viewport(self.raw, sel(b"setViewport:\0"), viewport);
    }

    pub fn set_scissor_rect(&self, rect: ScissorRect) {
        msg_void_scissor_rect(self.raw, sel(b"setScissorRect:\0"), rect);
    }

    pub fn set_cull_mode(&self, mode: CullMode) {
        msg_void_usize(self.raw, sel(b"setCullMode:\0"), mode as usize);
    }

    pub fn set_front_facing_winding(&self, winding: Winding) {
        msg_void_usize(self.raw, sel(b"setFrontFacingWinding:\0"), winding as usize);
    }

    pub fn set_triangle_fill_mode(&self, mode: TriangleFillMode) {
        msg_void_usize(self.raw, sel(b"setTriangleFillMode:\0"), mode as usize);
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

    pub fn draw_primitives_indirect(
        &self,
        primitive_type: PrimitiveType,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawPrimitives:indirectBuffer:indirectBufferOffset:\0"),
                primitive_type as usize,
                indirect_buffer.raw,
                indirect_buffer_offset,
            );
        }
    }

    pub fn draw_indexed_primitives_indirect(
        &self,
        primitive_type: PrimitiveType,
        index_type: IndexType,
        index_buffer: &Buffer,
        index_buffer_offset: usize,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, id, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawIndexedPrimitives:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:\0"),
                primitive_type as usize,
                index_type as usize,
                index_buffer.raw,
                index_buffer_offset,
                indirect_buffer.raw,
                indirect_buffer_offset,
            );
        }
    }

    pub fn update_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
    }

    pub fn update_fence_after_stages(
        &self,
        fence: &Fence,
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"updateFence:afterStages:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, fence.raw, stages.0);
                Ok(())
            } else {
                Err(MetalError::new("updateFence:afterStages: not supported"))
            }
        }
    }

    pub fn wait_for_fence_before_stages(
        &self,
        fence: &Fence,
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"waitForFence:beforeStages:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, fence.raw, stages.0);
                Ok(())
            } else {
                Err(MetalError::new("waitForFence:beforeStages: not supported"))
            }
        }
    }

    pub fn use_buffer(&self, buffer: &Buffer, usage: ResourceUsage) {
        msg_void_id_usize(
            self.raw,
            sel(b"useResource:usage:\0"),
            buffer.raw,
            usage.as_raw(),
        );
    }

    pub fn use_texture(&self, texture: &Texture, usage: ResourceUsage) {
        msg_void_id_usize(
            self.raw,
            sel(b"useResource:usage:\0"),
            texture.raw,
            usage.as_raw(),
        );
    }

    pub fn use_buffer_at_stages(
        &self,
        buffer: &Buffer,
        usage: ResourceUsage,
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"useResource:usage:stages:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, buffer.raw, usage.as_raw(), stages.0);
                Ok(())
            } else {
                Err(MetalError::new("useResource:usage:stages: not supported"))
            }
        }
    }

    pub fn use_texture_at_stages(
        &self,
        texture: &Texture,
        usage: ResourceUsage,
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"useResource:usage:stages:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, texture.raw, usage.as_raw(), stages.0);
                Ok(())
            } else {
                Err(MetalError::new("useResource:usage:stages: not supported"))
            }
        }
    }

    pub fn use_heap(&self, heap: &Heap) {
        msg_void_id(self.raw, sel(b"useHeap:\0"), heap.raw);
    }

    pub fn use_heap_at_stages(&self, heap: &Heap, stages: RenderStages) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"useHeap:stages:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, heap.raw, stages.0);
                Ok(())
            } else {
                Err(MetalError::new("useHeap:stages: not supported"))
            }
        }
    }

    pub fn use_buffers_at_stages(
        &self,
        buffers: &[&Buffer],
        usage: ResourceUsage,
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"useResources:count:usage:stages:\0");
            if responds_to_selector(self.raw, selector) {
                let raw_buffers: Vec<id> = buffers.iter().map(|b| b.raw).collect();
                let f: unsafe extern "C" fn(id, SEL, *const id, usize, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    raw_buffers.as_ptr(),
                    raw_buffers.len(),
                    usage.as_raw(),
                    stages.0,
                );
                Ok(())
            } else {
                Err(MetalError::new(
                    "useResources:count:usage:stages: not supported",
                ))
            }
        }
    }

    pub fn use_textures_at_stages(
        &self,
        textures: &[&Texture],
        usage: ResourceUsage,
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"useResources:count:usage:stages:\0");
            if responds_to_selector(self.raw, selector) {
                let raw_textures: Vec<id> = textures.iter().map(|t| t.raw).collect();
                let f: unsafe extern "C" fn(id, SEL, *const id, usize, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    raw_textures.as_ptr(),
                    raw_textures.len(),
                    usage.as_raw(),
                    stages.0,
                );
                Ok(())
            } else {
                Err(MetalError::new(
                    "useResources:count:usage:stages: not supported",
                ))
            }
        }
    }

    pub fn use_heaps_at_stages(
        &self,
        heaps: &[&Heap],
        stages: RenderStages,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"useHeaps:count:stages:\0");
            if responds_to_selector(self.raw, selector) {
                let raw_heaps: Vec<id> = heaps.iter().map(|h| h.raw).collect();
                let f: unsafe extern "C" fn(id, SEL, *const id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    raw_heaps.as_ptr(),
                    raw_heaps.len(),
                    stages.0,
                );
                Ok(())
            } else {
                Err(MetalError::new("useHeaps:count:stages: not supported"))
            }
        }
    }

    pub fn execute_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        msg_void_id_range(
            self.raw,
            sel(b"executeCommandsInBuffer:withRange:\0"),
            buffer.raw,
            range,
        );
    }

    pub fn set_tile_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setTileBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
    }

    pub fn set_tile_bytes<T>(&self, index: usize, value: &T) {
        msg_void_ptr_usize_usize(
            self.raw,
            sel(b"setTileBytes:length:atIndex:\0"),
            value as *const T as *const c_void,
            std::mem::size_of::<T>(),
            index,
        );
    }

    pub fn set_tile_texture(&self, index: usize, texture: &Texture) {
        msg_void_id_usize(
            self.raw,
            sel(b"setTileTexture:atIndex:\0"),
            texture.raw,
            index,
        );
    }

    pub fn set_tile_sampler_state(&self, index: usize, sampler: &SamplerState) {
        msg_void_id_usize(
            self.raw,
            sel(b"setTileSamplerState:atIndex:\0"),
            sampler.raw,
            index,
        );
    }

    pub fn dispatch_threads_per_tile(&self, threads_per_tile: Size) {
        msg_void_mtlsize(
            self.raw,
            sel(b"dispatchThreadsPerTile:\0"),
            threads_per_tile,
        );
    }

    pub fn set_threadgroup_memory_length_offset_index(
        &self,
        length: usize,
        offset: usize,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setThreadgroupMemoryLength:offset:atIndex:\0"),
                length,
                offset,
                index,
            );
        }
    }

    pub fn set_object_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setObjectBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
    }

    pub fn set_object_bytes<T>(&self, index: usize, value: &T) {
        msg_void_ptr_usize_usize(
            self.raw,
            sel(b"setObjectBytes:length:atIndex:\0"),
            value as *const T as *const c_void,
            std::mem::size_of::<T>(),
            index,
        );
    }

    pub fn set_object_texture(&self, index: usize, texture: &Texture) {
        msg_void_id_usize(
            self.raw,
            sel(b"setObjectTexture:atIndex:\0"),
            texture.raw,
            index,
        );
    }

    pub fn set_object_sampler_state(&self, index: usize, sampler: &SamplerState) {
        msg_void_id_usize(
            self.raw,
            sel(b"setObjectSamplerState:atIndex:\0"),
            sampler.raw,
            index,
        );
    }

    pub fn set_mesh_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setMeshBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
    }

    pub fn set_mesh_bytes<T>(&self, index: usize, value: &T) {
        msg_void_ptr_usize_usize(
            self.raw,
            sel(b"setMeshBytes:length:atIndex:\0"),
            value as *const T as *const c_void,
            std::mem::size_of::<T>(),
            index,
        );
    }

    pub fn set_mesh_texture(&self, index: usize, texture: &Texture) {
        msg_void_id_usize(
            self.raw,
            sel(b"setMeshTexture:atIndex:\0"),
            texture.raw,
            index,
        );
    }

    pub fn set_mesh_sampler_state(&self, index: usize, sampler: &SamplerState) {
        msg_void_id_usize(
            self.raw,
            sel(b"setMeshSamplerState:atIndex:\0"),
            sampler.raw,
            index,
        );
    }

    pub fn draw_mesh_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Size, Size, Size) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawMeshThreadgroups:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:\0"),
                threadgroups_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    pub fn draw_mesh_threads(
        &self,
        threads_per_grid: Size,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Size, Size, Size) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawMeshThreads:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:\0"),
                threads_per_grid,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    pub fn draw_mesh_threadgroups_indirect(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, Size, Size) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawMeshThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:\0"),
                indirect_buffer.raw,
                indirect_buffer_offset,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    pub fn set_vertex_visible_function_table(&self, table: &VisibleFunctionTable, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVertexVisibleFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_vertex_intersection_function_table(
        &self,
        table: &IntersectionFunctionTable,
        index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVertexIntersectionFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_vertex_acceleration_structure(
        &self,
        structure: &AccelerationStructure,
        index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVertexAccelerationStructure:atBufferIndex:\0"),
            structure.raw,
            index,
        );
    }

    pub fn set_fragment_visible_function_table(&self, table: &VisibleFunctionTable, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFragmentVisibleFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_fragment_intersection_function_table(
        &self,
        table: &IntersectionFunctionTable,
        index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFragmentIntersectionFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_fragment_acceleration_structure(
        &self,
        structure: &AccelerationStructure,
        index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFragmentAccelerationStructure:atBufferIndex:\0"),
            structure.raw,
            index,
        );
    }

    pub fn set_tile_visible_function_table(&self, table: &VisibleFunctionTable, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setTileVisibleFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_tile_intersection_function_table(
        &self,
        table: &IntersectionFunctionTable,
        index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setTileIntersectionFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_tile_acceleration_structure(&self, structure: &AccelerationStructure, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setTileAccelerationStructure:atBufferIndex:\0"),
            structure.raw,
            index,
        );
    }

    pub fn set_vertex_buffers(&self, buffers: &[Option<&Buffer>], offsets: &[usize], range: Range) {
        let raw_buffers: Vec<id> = buffers
            .iter()
            .map(|b| b.map_or(NIL, |buf| buf.raw))
            .collect();
        msg_void_ptr_ptr_range(
            self.raw,
            sel(b"setVertexBuffers:offsets:withRange:\0"),
            raw_buffers.as_ptr(),
            offsets.as_ptr(),
            range,
        );
    }

    pub fn set_vertex_textures(&self, textures: &[Option<&Texture>], range: Range) {
        let raw_textures: Vec<id> = textures
            .iter()
            .map(|t| t.map_or(NIL, |tex| tex.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setVertexTextures:withRange:\0"),
            raw_textures.as_ptr(),
            range,
        );
    }

    pub fn set_vertex_sampler_states(&self, samplers: &[Option<&SamplerState>], range: Range) {
        let raw_samplers: Vec<id> = samplers
            .iter()
            .map(|s| s.map_or(NIL, |sm| sm.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setVertexSamplerStates:withRange:\0"),
            raw_samplers.as_ptr(),
            range,
        );
    }

    pub fn set_fragment_buffers(
        &self,
        buffers: &[Option<&Buffer>],
        offsets: &[usize],
        range: Range,
    ) {
        let raw_buffers: Vec<id> = buffers
            .iter()
            .map(|b| b.map_or(NIL, |buf| buf.raw))
            .collect();
        msg_void_ptr_ptr_range(
            self.raw,
            sel(b"setFragmentBuffers:offsets:withRange:\0"),
            raw_buffers.as_ptr(),
            offsets.as_ptr(),
            range,
        );
    }

    pub fn set_fragment_textures(&self, textures: &[Option<&Texture>], range: Range) {
        let raw_textures: Vec<id> = textures
            .iter()
            .map(|t| t.map_or(NIL, |tex| tex.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setFragmentTextures:withRange:\0"),
            raw_textures.as_ptr(),
            range,
        );
    }

    pub fn set_fragment_sampler_states(&self, samplers: &[Option<&SamplerState>], range: Range) {
        let raw_samplers: Vec<id> = samplers
            .iter()
            .map(|s| s.map_or(NIL, |sm| sm.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setFragmentSamplerStates:withRange:\0"),
            raw_samplers.as_ptr(),
            range,
        );
    }

    pub fn set_vertex_visible_function_tables(
        &self,
        tables: &[Option<&VisibleFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setVertexVisibleFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn set_vertex_intersection_function_tables(
        &self,
        tables: &[Option<&IntersectionFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setVertexIntersectionFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn set_fragment_visible_function_tables(
        &self,
        tables: &[Option<&VisibleFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setFragmentVisibleFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn set_fragment_intersection_function_tables(
        &self,
        tables: &[Option<&IntersectionFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setFragmentIntersectionFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn set_tile_visible_function_tables(
        &self,
        tables: &[Option<&VisibleFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setTileVisibleFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn set_tile_intersection_function_tables(
        &self,
        tables: &[Option<&IntersectionFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setTileIntersectionFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for RenderCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ComputeCommandEncoder {
    pub raw: id,
}

impl ComputeCommandEncoder {
    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineState) {
        msg_void_id(self.raw, sel(b"setComputePipelineState:\0"), state.raw);
    }

    pub fn set_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setBuffer:offset:atIndex:\0"),
            buffer.raw,
            offset,
            index,
        );
    }

    pub fn set_texture(&self, index: usize, texture: &Texture) {
        msg_void_id_usize(self.raw, sel(b"setTexture:atIndex:\0"), texture.raw, index);
    }

    pub fn set_sampler_state(&self, index: usize, sampler: &SamplerState) {
        msg_void_id_usize(
            self.raw,
            sel(b"setSamplerState:atIndex:\0"),
            sampler.raw,
            index,
        );
    }

    pub fn set_bytes<T>(&self, index: usize, value: &T) {
        msg_void_ptr_usize_usize(
            self.raw,
            sel(b"setBytes:length:atIndex:\0"),
            value as *const T as *const c_void,
            std::mem::size_of::<T>(),
            index,
        );
    }

    pub fn dispatch_threadgroups(&self, threadgroups: Size, threads_per_threadgroup: Size) {
        msg_void_size_size(
            self.raw,
            sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
            threadgroups,
            threads_per_threadgroup,
        );
    }

    pub fn dispatch_threads(&self, threads: Size, threads_per_threadgroup: Size) {
        msg_void_size_size(
            self.raw,
            sel(b"dispatchThreads:threadsPerThreadgroup:\0"),
            threads,
            threads_per_threadgroup,
        );
    }

    pub fn update_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
    }

    pub fn use_buffer(&self, buffer: &Buffer, usage: ResourceUsage) {
        msg_void_id_usize(
            self.raw,
            sel(b"useResource:usage:\0"),
            buffer.raw,
            usage.as_raw(),
        );
    }

    pub fn use_texture(&self, texture: &Texture, usage: ResourceUsage) {
        msg_void_id_usize(
            self.raw,
            sel(b"useResource:usage:\0"),
            texture.raw,
            usage.as_raw(),
        );
    }

    pub fn use_heap(&self, heap: &Heap) {
        msg_void_id(self.raw, sel(b"useHeap:\0"), heap.raw);
    }

    pub fn execute_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        msg_void_id_range(
            self.raw,
            sel(b"executeCommandsInBuffer:withRange:\0"),
            buffer.raw,
            range,
        );
    }

    pub fn set_acceleration_structure(&self, structure: &AccelerationStructure, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setAccelerationStructure:atBufferIndex:\0"),
            structure.raw,
            index,
        );
    }

    pub fn set_visible_function_table(&self, table: &VisibleFunctionTable, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVisibleFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
    }

    pub fn set_intersection_function_table(&self, table: &IntersectionFunctionTable, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setIntersectionFunctionTable:atBufferIndex:\0"),
            table.raw,
            index,
        );
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

    pub fn set_visible_function_tables(
        &self,
        tables: &[Option<&VisibleFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setVisibleFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn set_intersection_function_tables(
        &self,
        tables: &[Option<&IntersectionFunctionTable>],
        range: Range,
    ) {
        let raw_tables: Vec<id> = tables
            .iter()
            .map(|t| t.map_or(NIL, |tbl| tbl.raw))
            .collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setIntersectionFunctionTables:withBufferRange:\0"),
            raw_tables.as_ptr(),
            range,
        );
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for ComputeCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ResourceStateCommandEncoder {
    pub raw: id,
}

impl ResourceStateCommandEncoder {
    pub fn update_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for ResourceStateCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
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
        msg_void_id(self.raw, sel(b"generateMipmapsForTexture:\0"), texture.raw);
    }

    pub fn synchronize_resource(&self, resource: &Buffer) {
        msg_void_id(self.raw, sel(b"synchronizeResource:\0"), resource.raw);
    }

    pub fn synchronize_texture(&self, texture: &Texture) {
        msg_void_id(self.raw, sel(b"synchronizeResource:\0"), texture.raw);
    }

    pub fn update_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"updateFence:\0"), fence.raw);
    }

    pub fn wait_for_fence(&self, fence: &Fence) {
        msg_void_id(self.raw, sel(b"waitForFence:\0"), fence.raw);
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for BlitCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}
