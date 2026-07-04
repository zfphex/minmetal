use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum DepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum VisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

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

    pub fn set_viewports(&self, viewports: &[Viewport]) -> Result<(), MetalError> {
        let selector = sel(b"setViewports:count:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const Viewport, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, viewports.as_ptr(), viewports.len());
            }
            Ok(())
        } else {
            Err(MetalError::new("setViewports:count: not supported"))
        }
    }

    pub fn set_scissor_rects(&self, rects: &[ScissorRect]) -> Result<(), MetalError> {
        let selector = sel(b"setScissorRects:count:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const ScissorRect, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, rects.as_ptr(), rects.len());
            }
            Ok(())
        } else {
            Err(MetalError::new("setScissorRects:count: not supported"))
        }
    }

    pub fn set_visibility_result_mode(&self, mode: VisibilityResultMode, offset: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setVisibilityResultMode:offset:\0"),
                mode as usize,
                offset,
            );
        }
    }

    pub fn set_depth_clip_mode(&self, mode: DepthClipMode) -> Result<(), MetalError> {
        let selector = sel(b"setDepthClipMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, mode as usize);
            Ok(())
        } else {
            Err(MetalError::new("setDepthClipMode: not supported"))
        }
    }

    pub fn set_stencil_reference_value(&self, reference_value: u32) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u32) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setStencilReferenceValue:\0"),
                reference_value,
            );
        }
    }

    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, f32, f32, f32, f32) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBlendColorRed:green:blue:alpha:\0"),
                red,
                green,
                blue,
                alpha,
            );
        }
    }

    pub fn set_vertex_buffer_offset(&self, offset: usize, index: usize) -> Result<(), MetalError> {
        let selector = sel(b"setVertexBufferOffset:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, offset, index);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "setVertexBufferOffset:atIndex: not supported",
            ))
        }
    }

    pub fn set_fragment_buffer_offset(
        &self,
        offset: usize,
        index: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setFragmentBufferOffset:atIndex:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, offset, index);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "setFragmentBufferOffset:atIndex: not supported",
            ))
        }
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

    pub fn draw_patches(
        &self,
        number_of_patch_control_points: usize,
        patch_start: usize,
        patch_count: usize,
        patch_index_buffer: Option<&Buffer>,
        patch_index_buffer_offset: usize,
        instance_count: usize,
        base_instance: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(
            b"drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:\0",
        );
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, id, usize, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
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
                );
            }
            Ok(())
        } else {
            Err(MetalError::new("drawPatches: not supported"))
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
    ) -> Result<(), MetalError> {
        let selector = sel(
            b"drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:\0",
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
                );
            }
            Ok(())
        } else {
            Err(MetalError::new("drawIndexedPatches: not supported"))
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

    pub fn execute_commands_in_buffer(
        &self,
        buffer: &IndirectCommandBuffer,
        range: Range,
    ) -> Result<(), MetalError> {
        buffer.validate_reset_range(range)?;
        msg_void_id_range(
            self.raw,
            sel(b"executeCommandsInBuffer:withRange:\0"),
            buffer.raw,
            range,
        );
        Ok(())
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

    pub fn memory_barrier_with_scope_after_before(
        &self,
        scope: BarrierScope,
        after: RenderStages,
        before: RenderStages,
    ) -> Result<(), MetalError> {
        let selector = sel(b"memoryBarrierWithScope:afterStages:beforeStages:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, scope.0, after.0, before.0);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "memoryBarrierWithScope:afterStages:beforeStages: not supported",
            ))
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

    pub fn execute_commands_in_buffer(
        &self,
        buffer: &IndirectCommandBuffer,
        range: Range,
    ) -> Result<(), MetalError> {
        buffer.validate_reset_range(range)?;
        msg_void_id_range(
            self.raw,
            sel(b"executeCommandsInBuffer:withRange:\0"),
            buffer.raw,
            range,
        );
        Ok(())
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

    pub fn dispatch_type(&self) -> DispatchType {
        let val = msg_usize(self.raw, sel(b"dispatchType\0"));
        match val {
            0 => DispatchType::Serial,
            _ => DispatchType::Concurrent,
        }
    }

    pub fn set_buffer_offset(&self, offset: usize, index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"setBufferOffset:atIndex:\0"), offset, index);
        }
    }

    pub fn set_buffer_offset_with_attribute_stride(
        &self,
        offset: usize,
        stride: usize,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBufferOffset:attributeStride:atIndex:\0"),
                offset,
                stride,
                index,
            );
        }
    }

    pub fn set_buffer_with_attribute_stride(
        &self,
        buffer: &Buffer,
        offset: usize,
        stride: usize,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBuffer:offset:attributeStride:atIndex:\0"),
                buffer.raw,
                offset,
                stride,
                index,
            );
        }
    }

    pub fn set_buffers_with_attribute_strides(
        &self,
        buffers: &[Option<&Buffer>],
        offsets: &[usize],
        strides: &[usize],
        range: Range,
    ) {
        let raw_buffers: Vec<id> = buffers
            .iter()
            .map(|b| b.map_or(NIL, |buf| buf.raw))
            .collect();
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, *const usize, *const usize, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBuffers:offsets:attributeStrides:withRange:\0"),
                raw_buffers.as_ptr(),
                offsets.as_ptr(),
                strides.as_ptr(),
                range,
            );
        }
    }

    pub fn set_bytes_with_attribute_stride<T>(&self, index: usize, value: &T, stride: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setBytes:length:attributeStride:atIndex:\0"),
                value as *const T as *const c_void,
                std::mem::size_of::<T>(),
                stride,
                index,
            );
        }
    }

    pub fn set_sampler_state_with_lod_clamps(
        &self,
        sampler: &SamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, f32, f32, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setSamplerState:lodMinClamp:lodMaxClamp:atIndex:\0"),
                sampler.raw,
                lod_min_clamp,
                lod_max_clamp,
                index,
            );
        }
    }

    pub fn set_sampler_states_with_lod_clamps(
        &self,
        samplers: &[Option<&SamplerState>],
        lod_min_clamps: &[f32],
        lod_max_clamps: &[f32],
        range: Range,
    ) {
        let raw_samplers: Vec<id> = samplers
            .iter()
            .map(|s| s.map_or(NIL, |sm| sm.raw))
            .collect();
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, *const f32, *const f32, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setSamplerStates:lodMinClamps:lodMaxClamps:withRange:\0"),
                raw_samplers.as_ptr(),
                lod_min_clamps.as_ptr(),
                lod_max_clamps.as_ptr(),
                range,
            );
        }
    }

    pub fn set_threadgroup_memory_length(&self, length: usize, index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setThreadgroupMemoryLength:atIndex:\0"),
                length,
                index,
            );
        }
    }

    pub fn set_imageblock_size(&self, width: usize, height: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setImageblockWidth:height:\0"),
                width,
                height,
            );
        }
    }

    pub fn set_stage_in_region(&self, region: Region) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Region) = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"setStageInRegion:\0"), region);
        }
    }

    pub fn set_stage_in_region_with_indirect_buffer(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setStageInRegionWithIndirectBuffer:indirectBufferOffset:\0"),
            indirect_buffer.raw,
            indirect_buffer_offset,
        );
    }

    pub fn dispatch_threadgroups_with_indirect_buffer(
        &self,
        indirect_buffer: &Buffer,
        indirect_buffer_offset: usize,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, Size) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:\0"),
                indirect_buffer.raw,
                indirect_buffer_offset,
                threads_per_threadgroup,
            );
        }
    }

    pub fn use_resources(&self, resources: &[id], usage: ResourceUsage) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"useResources:count:usage:\0"),
                resources.as_ptr(),
                resources.len(),
                usage.as_raw(),
            );
        }
    }

    pub fn use_heaps(&self, heaps: &[id]) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"useHeaps:count:\0"),
                heaps.as_ptr(),
                heaps.len(),
            );
        }
    }

    pub fn execute_commands_in_buffer_indirect(
        &self,
        indirect_command_buffer: &IndirectCommandBuffer,
        indirect_range_buffer: &Buffer,
        indirect_buffer_offset: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:\0"),
                indirect_command_buffer.raw,
                indirect_range_buffer.raw,
                indirect_buffer_offset,
            );
        }
    }

    pub fn memory_barrier_with_scope(&self, scope: BarrierScope) {
        msg_void_usize(self.raw, sel(b"memoryBarrierWithScope:\0"), scope.0);
    }

    pub fn memory_barrier_with_resources(&self, resources: &[id]) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"memoryBarrierWithResources:count:\0"),
                resources.as_ptr(),
                resources.len(),
            );
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
    pub fn use_resource(&self, resource: id, usage: ResourceUsage) -> Result<(), MetalError> {
        let selector = sel(b"useResource:usage:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "useResource:usage: not supported on this ResourceStateCommandEncoder",
            ));
        }
        msg_void_id_usize(self.raw, selector, resource, usage.as_raw());
        Ok(())
    }

    pub fn use_resources(&self, resources: &[id], usage: ResourceUsage) -> Result<(), MetalError> {
        let selector = sel(b"useResources:count:usage:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "useResources:count:usage: not supported on this ResourceStateCommandEncoder",
            ));
        }
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
    }

    pub fn use_heap(&self, heap: &Heap) -> Result<(), MetalError> {
        let selector = sel(b"useHeap:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "useHeap: not supported on this ResourceStateCommandEncoder",
            ));
        }
        msg_void_id(self.raw, selector, heap.raw);
        Ok(())
    }

    pub fn use_heaps(&self, heaps: &[id]) -> Result<(), MetalError> {
        let selector = sel(b"useHeaps:count:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "useHeaps:count: not supported on this ResourceStateCommandEncoder",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, heaps.as_ptr(), heaps.len());
        }
        Ok(())
    }

    pub fn memory_barrier_with_scope(&self, scope: BarrierScope) -> Result<(), MetalError> {
        let selector = sel(b"memoryBarrierWithScope:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "memoryBarrierWithScope: not supported on this ResourceStateCommandEncoder",
            ));
        }
        msg_void_usize(self.raw, selector, scope.0);
        Ok(())
    }

    pub fn memory_barrier_with_resources(&self, resources: &[id]) -> Result<(), MetalError> {
        let selector = sel(b"memoryBarrierWithResources:count:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "memoryBarrierWithResources:count: not supported on this ResourceStateCommandEncoder",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, resources.as_ptr(), resources.len());
        }
        Ok(())
    }

    pub fn update_fence(&self, fence: &Fence) -> Result<(), MetalError> {
        let selector = sel(b"updateFence:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "updateFence: not supported on this ResourceStateCommandEncoder",
            ));
        }
        msg_void_id(self.raw, selector, fence.raw);
        Ok(())
    }

    pub fn wait_for_fence(&self, fence: &Fence) -> Result<(), MetalError> {
        let selector = sel(b"waitForFence:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "waitForFence: not supported on this ResourceStateCommandEncoder",
            ));
        }
        msg_void_id(self.raw, selector, fence.raw);
        Ok(())
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
        self.copy_texture_to_texture_with_slices(
            source,
            0,
            0,
            source_origin,
            source_size,
            destination,
            0,
            0,
            destination_origin,
        );
    }

    pub fn copy_texture_to_texture_with_slices(
        &self,
        source: &Texture,
        source_slice: usize,
        source_level: usize,
        source_origin: Origin,
        source_size: Size,
        destination: &Texture,
        destination_slice: usize,
        destination_level: usize,
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
                source_slice,
                source_level,
                source_origin,
                source_size,
                destination.raw,
                destination_slice,
                destination_level,
                destination_origin,
            );
        }
    }

    pub fn copy_textures(&self, source: &Texture, destination: &Texture) -> Result<(), MetalError> {
        let selector = sel(b"copyFromTexture:toTexture:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, id) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, source.raw, destination.raw);
            }
            Ok(())
        } else {
            Err(MetalError::new("copyFromTexture:toTexture: not supported"))
        }
    }

    pub fn copy_texture_surfaces(
        &self,
        source: &Texture,
        source_slice: usize,
        source_level: usize,
        destination: &Texture,
        destination_slice: usize,
        destination_level: usize,
        slice_count: usize,
        level_count: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(
                b"copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:\0",
            );
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "copyFromTexture:sourceSlice:sourceLevel:toTexture:... not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize, id, usize, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source.raw,
                source_slice,
                source_level,
                destination.raw,
                destination_slice,
                destination_level,
                slice_count,
                level_count,
            );
            Ok(())
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
        self.copy_buffer_to_texture_with_slices(
            source,
            source_offset,
            source_bytes_per_row,
            source_bytes_per_image,
            source_size,
            destination,
            0,
            0,
            destination_origin,
            BlitOption::NONE,
        );
    }

    pub fn copy_buffer_to_texture_with_slices(
        &self,
        source: &Buffer,
        source_offset: usize,
        source_bytes_per_row: usize,
        source_bytes_per_image: usize,
        source_size: Size,
        destination: &Texture,
        destination_slice: usize,
        destination_level: usize,
        destination_origin: Origin,
        options: BlitOption,
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
                usize,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:\0"),
                source.raw,
                source_offset,
                source_bytes_per_row,
                source_bytes_per_image,
                source_size,
                destination.raw,
                destination_slice,
                destination_level,
                destination_origin,
                options.as_raw(),
            );
        }
    }

    pub fn copy_texture_to_buffer(
        &self,
        source: &Texture,
        source_origin: Origin,
        source_size: Size,
        destination: &Buffer,
        destination_offset: usize,
        destination_bytes_per_row: usize,
        destination_bytes_per_image: usize,
    ) {
        self.copy_texture_to_buffer_with_slices(
            source,
            0,
            0,
            source_origin,
            source_size,
            destination,
            destination_offset,
            destination_bytes_per_row,
            destination_bytes_per_image,
            BlitOption::NONE,
        );
    }

    pub fn copy_texture_to_buffer_with_slices(
        &self,
        source: &Texture,
        source_slice: usize,
        source_level: usize,
        source_origin: Origin,
        source_size: Size,
        destination: &Buffer,
        destination_offset: usize,
        destination_bytes_per_row: usize,
        destination_bytes_per_image: usize,
        options: BlitOption,
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
                usize,
                usize,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:\0"),
                source.raw,
                source_slice,
                source_level,
                source_origin,
                source_size,
                destination.raw,
                destination_offset,
                destination_bytes_per_row,
                destination_bytes_per_image,
                options.as_raw(),
            );
        }
    }

    pub fn fill_buffer(&self, buffer: &Buffer, range: Range, value: u8) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range, u8) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"fillBuffer:range:value:\0"),
                buffer.raw,
                range,
                value,
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

    pub fn synchronize_texture_slice_level(
        &self,
        texture: &Texture,
        slice: usize,
        level: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"synchronizeTexture:slice:level:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, texture.raw, slice, level);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "synchronizeTexture:slice:level: not supported",
            ))
        }
    }

    pub fn optimize_contents_for_gpu_access(&self, texture: &Texture) -> Result<(), MetalError> {
        let selector = sel(b"optimizeContentsForGPUAccess:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, texture.raw);
            Ok(())
        } else {
            Err(MetalError::new(
                "optimizeContentsForGPUAccess: not supported",
            ))
        }
    }

    pub fn optimize_contents_for_gpu_access_slice_level(
        &self,
        texture: &Texture,
        slice: usize,
        level: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"optimizeContentsForGPUAccess:slice:level:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, texture.raw, slice, level);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "optimizeContentsForGPUAccess:slice:level: not supported",
            ))
        }
    }

    pub fn optimize_contents_for_cpu_access(&self, texture: &Texture) -> Result<(), MetalError> {
        let selector = sel(b"optimizeContentsForCPUAccess:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, texture.raw);
            Ok(())
        } else {
            Err(MetalError::new(
                "optimizeContentsForCPUAccess: not supported",
            ))
        }
    }

    pub fn optimize_contents_for_cpu_access_slice_level(
        &self,
        texture: &Texture,
        slice: usize,
        level: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"optimizeContentsForCPUAccess:slice:level:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, texture.raw, slice, level);
            }
            Ok(())
        } else {
            Err(MetalError::new(
                "optimizeContentsForCPUAccess:slice:level: not supported",
            ))
        }
    }

    pub fn reset_commands_in_buffer(
        &self,
        buffer: &IndirectCommandBuffer,
        range: Range,
    ) -> Result<(), MetalError> {
        buffer.validate_reset_range(range)?;
        let selector = sel(b"resetCommandsInBuffer:withRange:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_range(self.raw, selector, buffer.raw, range);
            Ok(())
        } else {
            Err(MetalError::new(
                "resetCommandsInBuffer:withRange: not supported",
            ))
        }
    }

    pub fn copy_indirect_command_buffer(
        &self,
        source: &IndirectCommandBuffer,
        source_range: Range,
        destination: &IndirectCommandBuffer,
        destination_index: usize,
    ) -> Result<(), MetalError> {
        let selector =
            sel(b"copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "copyIndirectCommandBuffer:sourceRange:destination:destinationIndex: not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                source.raw,
                source_range,
                destination.raw,
                destination_index,
            );
        }
        Ok(())
    }

    pub fn optimize_indirect_command_buffer(
        &self,
        buffer: &IndirectCommandBuffer,
        range: Range,
    ) -> Result<(), MetalError> {
        buffer.validate_reset_range(range)?;
        let selector = sel(b"optimizeIndirectCommandBuffer:withRange:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id_range(self.raw, selector, buffer.raw, range);
            Ok(())
        } else {
            Err(MetalError::new(
                "optimizeIndirectCommandBuffer:withRange: not supported",
            ))
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

impl Drop for BlitCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}
