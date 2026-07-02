use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct RenderPassAttachmentDescriptor {
    pub raw: id,
}

impl RenderPassAttachmentDescriptor {
    pub fn texture(&self) -> Option<Texture> {
        let t = msg_id(self.raw, sel(b"texture\0"));
        (!t.is_null()).then_some(Texture { raw: retain(t) })
    }

    pub fn set_texture(&self, texture: Option<&Texture>) {
        msg_void_id(
            self.raw,
            sel(b"setTexture:\0"),
            texture.map_or(NIL, |t| t.raw),
        );
    }

    pub fn level(&self) -> usize {
        msg_usize(self.raw, sel(b"level\0"))
    }

    pub fn set_level(&self, level: usize) {
        msg_void_usize(self.raw, sel(b"setLevel:\0"), level);
    }

    pub fn slice(&self) -> usize {
        msg_usize(self.raw, sel(b"slice\0"))
    }

    pub fn set_slice(&self, slice: usize) {
        msg_void_usize(self.raw, sel(b"setSlice:\0"), slice);
    }

    pub fn depth_plane(&self) -> usize {
        msg_usize(self.raw, sel(b"depthPlane\0"))
    }

    pub fn set_depth_plane(&self, depth_plane: usize) {
        msg_void_usize(self.raw, sel(b"setDepthPlane:\0"), depth_plane);
    }

    pub fn resolve_texture(&self) -> Option<Texture> {
        let t = msg_id(self.raw, sel(b"resolveTexture\0"));
        (!t.is_null()).then_some(Texture { raw: retain(t) })
    }

    pub fn set_resolve_texture(&self, texture: Option<&Texture>) {
        msg_void_id(
            self.raw,
            sel(b"setResolveTexture:\0"),
            texture.map_or(NIL, |t| t.raw),
        );
    }

    pub fn resolve_level(&self) -> usize {
        msg_usize(self.raw, sel(b"resolveLevel\0"))
    }

    pub fn set_resolve_level(&self, level: usize) {
        msg_void_usize(self.raw, sel(b"setResolveLevel:\0"), level);
    }

    pub fn resolve_slice(&self) -> usize {
        msg_usize(self.raw, sel(b"resolveSlice\0"))
    }

    pub fn set_resolve_slice(&self, slice: usize) {
        msg_void_usize(self.raw, sel(b"setResolveSlice:\0"), slice);
    }

    pub fn resolve_depth_plane(&self) -> usize {
        msg_usize(self.raw, sel(b"resolveDepthPlane\0"))
    }

    pub fn set_resolve_depth_plane(&self, depth_plane: usize) {
        msg_void_usize(self.raw, sel(b"setResolveDepthPlane:\0"), depth_plane);
    }

    pub fn load_action(&self) -> Result<LoadAction, MetalError> {
        let val = msg_usize(self.raw, sel(b"loadAction\0"));
        LoadAction::from_raw(val).ok_or_else(|| {
            MetalError::new(format!("invalid MTLLoadAction value from Metal: {}", val))
        })
    }

    pub fn set_load_action(&self, load_action: LoadAction) {
        msg_void_usize(self.raw, sel(b"setLoadAction:\0"), load_action as usize);
    }

    pub fn store_action(&self) -> Result<StoreAction, MetalError> {
        let val = msg_usize(self.raw, sel(b"storeAction\0"));
        StoreAction::from_raw(val).ok_or_else(|| {
            MetalError::new(format!("invalid MTLStoreAction value from Metal: {}", val))
        })
    }

    pub fn set_store_action(&self, store_action: StoreAction) {
        msg_void_usize(self.raw, sel(b"setStoreAction:\0"), store_action as usize);
    }

    pub fn store_action_options(&self) -> Result<StoreActionOptions, MetalError> {
        let selector = sel(b"storeActionOptions\0");
        if responds_to_selector(self.raw, selector) {
            Ok(StoreActionOptions(msg_usize(self.raw, selector)))
        } else {
            Err(MetalError::new("storeActionOptions not supported"))
        }
    }

    pub fn set_store_action_options(&self, options: StoreActionOptions) -> Result<(), MetalError> {
        let selector = sel(b"setStoreActionOptions:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, options.0);
            Ok(())
        } else {
            Err(MetalError::new("setStoreActionOptions: not supported"))
        }
    }
}

#[derive(Debug)]
pub struct RenderPassColorAttachmentDescriptor {
    pub raw: id,
}

impl RenderPassColorAttachmentDescriptor {
    pub fn base(&self) -> RenderPassAttachmentDescriptor {
        RenderPassAttachmentDescriptor { raw: self.raw }
    }

    pub fn clear_color(&self) -> ClearColor {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> ClearColor =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"clearColor\0"))
        }
    }

    pub fn set_clear_color(&self, color: ClearColor) {
        msg_void_clear_color(self.raw, sel(b"setClearColor:\0"), color);
    }
}

#[derive(Debug)]
pub struct RenderPassDepthAttachmentDescriptor {
    pub raw: id,
}

impl RenderPassDepthAttachmentDescriptor {
    pub fn base(&self) -> RenderPassAttachmentDescriptor {
        RenderPassAttachmentDescriptor { raw: self.raw }
    }

    pub fn clear_depth(&self) -> f64 {
        msg_f64(self.raw, sel(b"clearDepth\0"))
    }

    pub fn set_clear_depth(&self, depth: f64) {
        msg_void_f64(self.raw, sel(b"setClearDepth:\0"), depth);
    }
}

#[derive(Debug)]
pub struct RenderPassStencilAttachmentDescriptor {
    pub raw: id,
}

impl RenderPassStencilAttachmentDescriptor {
    pub fn base(&self) -> RenderPassAttachmentDescriptor {
        RenderPassAttachmentDescriptor { raw: self.raw }
    }

    pub fn clear_stencil(&self) -> u32 {
        msg_usize(self.raw, sel(b"clearStencil\0")) as u32
    }

    pub fn set_clear_stencil(&self, stencil: u32) {
        msg_void_usize(self.raw, sel(b"setClearStencil:\0"), stencil as usize);
    }
}

#[derive(Debug)]
pub struct RenderPassSampleBufferAttachmentDescriptor {
    pub raw: id,
}

impl RenderPassSampleBufferAttachmentDescriptor {
    pub fn sample_buffer(&self) -> Option<CounterSampleBuffer> {
        let sb = msg_id(self.raw, sel(b"sampleBuffer\0"));
        (!sb.is_null()).then_some(CounterSampleBuffer { raw: retain(sb) })
    }

    pub fn set_sample_buffer(&self, buffer: Option<&CounterSampleBuffer>) {
        msg_void_id(
            self.raw,
            sel(b"setSampleBuffer:\0"),
            buffer.map_or(NIL, |b| b.raw),
        );
    }

    pub fn start_of_vertex_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"startOfVertexSampleIndex\0"))
    }

    pub fn set_start_of_vertex_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setStartOfVertexSampleIndex:\0"), index);
    }

    pub fn end_of_vertex_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"endOfVertexSampleIndex\0"))
    }

    pub fn set_end_of_vertex_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setEndOfVertexSampleIndex:\0"), index);
    }

    pub fn start_of_fragment_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"startOfFragmentSampleIndex\0"))
    }

    pub fn set_start_of_fragment_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setStartOfFragmentSampleIndex:\0"), index);
    }

    pub fn end_of_fragment_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"endOfFragmentSampleIndex\0"))
    }

    pub fn set_end_of_fragment_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setEndOfFragmentSampleIndex:\0"), index);
    }
}

#[derive(Debug)]
pub struct RenderPassSampleBufferAttachmentDescriptorArray {
    pub raw: id,
}

impl RenderPassSampleBufferAttachmentDescriptorArray {
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> RenderPassSampleBufferAttachmentDescriptor {
        let attachment = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        RenderPassSampleBufferAttachmentDescriptor { raw: attachment }
    }
}

#[derive(Debug)]
pub struct RenderPassDescriptor {
    pub raw: id,
}

impl RenderPassDescriptor {
    pub fn new() -> Self {
        let raw = retain(msg_id(
            class(b"MTLRenderPassDescriptor\0"),
            sel(b"renderPassDescriptor\0"),
        ));
        Self { raw }
    }

    pub fn color_attachment(&self, index: usize) -> RenderPassColorAttachmentDescriptor {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        RenderPassColorAttachmentDescriptor { raw: attachment }
    }

    pub fn depth_attachment(&self) -> RenderPassDepthAttachmentDescriptor {
        let attachment = msg_id(self.raw, sel(b"depthAttachment\0"));
        RenderPassDepthAttachmentDescriptor { raw: attachment }
    }

    pub fn stencil_attachment(&self) -> RenderPassStencilAttachmentDescriptor {
        let attachment = msg_id(self.raw, sel(b"stencilAttachment\0"));
        RenderPassStencilAttachmentDescriptor { raw: attachment }
    }

    pub fn set_color_attachment(
        &self,
        index: usize,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_color: ClearColor,
    ) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
        msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
        msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
        msg_void_clear_color(attachment, sel(b"setClearColor:\0"), clear_color);
    }

    pub fn set_color_attachment_resolve_texture(&self, index: usize, texture: &Texture) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_id(attachment, sel(b"setResolveTexture:\0"), texture.raw);
    }

    pub fn set_depth_attachment(
        &self,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_depth: f64,
    ) {
        let attachment = msg_id(self.raw, sel(b"depthAttachment\0"));
        msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
        msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
        msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
        msg_void_f64(attachment, sel(b"setClearDepth:\0"), clear_depth);
    }

    pub fn set_depth_resolve_texture(&self, texture: &Texture) {
        let attachment = msg_id(self.raw, sel(b"depthAttachment\0"));
        msg_void_id(attachment, sel(b"setResolveTexture:\0"), texture.raw);
    }

    pub fn set_stencil_attachment(
        &self,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_stencil: u32,
    ) {
        let attachment = msg_id(self.raw, sel(b"stencilAttachment\0"));
        msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
        msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
        msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
        msg_void_usize(
            attachment,
            sel(b"setClearStencil:\0"),
            clear_stencil as usize,
        );
    }

    pub fn visibility_result_buffer(&self) -> Option<Buffer> {
        let b = msg_id(self.raw, sel(b"visibilityResultBuffer\0"));
        (!b.is_null()).then_some(Buffer { raw: retain(b) })
    }

    pub fn set_visibility_result_buffer(&self, buffer: Option<&Buffer>) {
        msg_void_id(
            self.raw,
            sel(b"setVisibilityResultBuffer:\0"),
            buffer.map_or(NIL, |b| b.raw),
        );
    }

    pub fn render_target_array_length(&self) -> usize {
        msg_usize(self.raw, sel(b"renderTargetArrayLength\0"))
    }

    pub fn set_render_target_array_length(&self, length: usize) {
        msg_void_usize(self.raw, sel(b"setRenderTargetArrayLength:\0"), length);
    }

    pub fn imageblock_sample_length(&self) -> usize {
        msg_usize(self.raw, sel(b"imageblockSampleLength\0"))
    }

    pub fn set_imageblock_sample_length(&self, length: usize) {
        msg_void_usize(self.raw, sel(b"setImageblockSampleLength:\0"), length);
    }

    pub fn tile_width(&self) -> usize {
        msg_usize(self.raw, sel(b"tileWidth\0"))
    }

    pub fn set_tile_width(&self, width: usize) {
        msg_void_usize(self.raw, sel(b"setTileWidth:\0"), width);
    }

    pub fn tile_height(&self) -> usize {
        msg_usize(self.raw, sel(b"tileHeight\0"))
    }

    pub fn set_tile_height(&self, height: usize) {
        msg_void_usize(self.raw, sel(b"setTileHeight:\0"), height);
    }

    pub fn sample_buffer_attachments(&self) -> RenderPassSampleBufferAttachmentDescriptorArray {
        let array = msg_id(self.raw, sel(b"sampleBufferAttachments\0"));
        RenderPassSampleBufferAttachmentDescriptorArray { raw: array }
    }

    pub fn rasterization_rate_map(&self) -> Option<RasterizationRateMap> {
        let selector = sel(b"rasterizationRateMap\0");
        if !responds_to_selector(self.raw, selector) {
            return None;
        }
        let map = msg_id(self.raw, selector);
        if map.is_null() {
            None
        } else {
            Some(RasterizationRateMap { raw: retain(map) })
        }
    }

    pub fn set_rasterization_rate_map(
        &self,
        map: Option<&RasterizationRateMap>,
    ) -> Result<(), MetalError> {
        let selector = sel(b"setRasterizationRateMap:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("setRasterizationRateMap: is not supported"));
        }
        msg_void_id(self.raw, selector, map.map_or(NIL, |m| m.raw));
        Ok(())
    }
}

impl Default for RenderPassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RenderPassDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ComputePassSampleBufferAttachmentDescriptor {
    pub raw: id,
}

impl ComputePassSampleBufferAttachmentDescriptor {
    pub fn sample_buffer(&self) -> Option<CounterSampleBuffer> {
        let sb = msg_id(self.raw, sel(b"sampleBuffer\0"));
        (!sb.is_null()).then_some(CounterSampleBuffer { raw: retain(sb) })
    }

    pub fn set_sample_buffer(&self, buffer: Option<&CounterSampleBuffer>) {
        msg_void_id(
            self.raw,
            sel(b"setSampleBuffer:\0"),
            buffer.map_or(NIL, |b| b.raw),
        );
    }

    pub fn start_of_encoder_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"startOfEncoderSampleIndex\0"))
    }

    pub fn set_start_of_encoder_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setStartOfEncoderSampleIndex:\0"), index);
    }

    pub fn end_of_encoder_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"endOfEncoderSampleIndex\0"))
    }

    pub fn set_end_of_encoder_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setEndOfEncoderSampleIndex:\0"), index);
    }
}

#[derive(Debug)]
pub struct ComputePassSampleBufferAttachmentDescriptorArray {
    pub raw: id,
}

impl ComputePassSampleBufferAttachmentDescriptorArray {
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> ComputePassSampleBufferAttachmentDescriptor {
        let attachment = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        ComputePassSampleBufferAttachmentDescriptor { raw: attachment }
    }
}

#[derive(Debug)]
pub struct ComputePassDescriptor {
    pub raw: id,
}

impl ComputePassDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLComputePassDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new("MTLComputePassDescriptor is not available"));
        }
        let raw = retain(msg_id(class_ptr, sel(b"computePassDescriptor\0")));
        if raw.is_null() {
            Err(MetalError::new("failed to create MTLComputePassDescriptor"))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn dispatch_type(&self) -> Result<DispatchType, MetalError> {
        let raw = msg_usize(self.raw, sel(b"dispatchType\0"));
        DispatchType::from_raw(raw).ok_or_else(|| {
            MetalError::new(format!("invalid MTLDispatchType value from Metal: {}", raw))
        })
    }

    pub fn set_dispatch_type(&self, dispatch_type: DispatchType) {
        msg_void_usize(self.raw, sel(b"setDispatchType:\0"), dispatch_type as usize);
    }

    pub fn sample_buffer_attachments(&self) -> ComputePassSampleBufferAttachmentDescriptorArray {
        let array = msg_id(self.raw, sel(b"sampleBufferAttachments\0"));
        ComputePassSampleBufferAttachmentDescriptorArray { raw: array }
    }
}

impl Drop for ComputePassDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct BlitPassSampleBufferAttachmentDescriptor {
    pub raw: id,
}

impl BlitPassSampleBufferAttachmentDescriptor {
    pub fn sample_buffer(&self) -> Option<CounterSampleBuffer> {
        let sb = msg_id(self.raw, sel(b"sampleBuffer\0"));
        (!sb.is_null()).then_some(CounterSampleBuffer { raw: retain(sb) })
    }

    pub fn set_sample_buffer(&self, buffer: Option<&CounterSampleBuffer>) {
        msg_void_id(
            self.raw,
            sel(b"setSampleBuffer:\0"),
            buffer.map_or(NIL, |b| b.raw),
        );
    }

    pub fn start_of_encoder_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"startOfEncoderSampleIndex\0"))
    }

    pub fn set_start_of_encoder_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setStartOfEncoderSampleIndex:\0"), index);
    }

    pub fn end_of_encoder_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"endOfEncoderSampleIndex\0"))
    }

    pub fn set_end_of_encoder_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setEndOfEncoderSampleIndex:\0"), index);
    }
}

#[derive(Debug)]
pub struct BlitPassSampleBufferAttachmentDescriptorArray {
    pub raw: id,
}

impl BlitPassSampleBufferAttachmentDescriptorArray {
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> BlitPassSampleBufferAttachmentDescriptor {
        let attachment = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        BlitPassSampleBufferAttachmentDescriptor { raw: attachment }
    }
}

#[derive(Debug)]
pub struct BlitPassDescriptor {
    pub raw: id,
}

impl BlitPassDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLBlitPassDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new("MTLBlitPassDescriptor is not available"));
        }
        let raw = retain(msg_id(class_ptr, sel(b"blitPassDescriptor\0")));
        if raw.is_null() {
            Err(MetalError::new("failed to create MTLBlitPassDescriptor"))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn sample_buffer_attachments(&self) -> BlitPassSampleBufferAttachmentDescriptorArray {
        let array = msg_id(self.raw, sel(b"sampleBufferAttachments\0"));
        BlitPassSampleBufferAttachmentDescriptorArray { raw: array }
    }
}

impl Drop for BlitPassDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ResourceStatePassSampleBufferAttachmentDescriptor {
    pub raw: id,
}

impl ResourceStatePassSampleBufferAttachmentDescriptor {
    pub fn sample_buffer(&self) -> Option<CounterSampleBuffer> {
        let sb = msg_id(self.raw, sel(b"sampleBuffer\0"));
        (!sb.is_null()).then_some(CounterSampleBuffer { raw: retain(sb) })
    }

    pub fn set_sample_buffer(&self, buffer: Option<&CounterSampleBuffer>) {
        msg_void_id(
            self.raw,
            sel(b"setSampleBuffer:\0"),
            buffer.map_or(NIL, |b| b.raw),
        );
    }

    pub fn start_of_encoder_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"startOfEncoderSampleIndex\0"))
    }

    pub fn set_start_of_encoder_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setStartOfEncoderSampleIndex:\0"), index);
    }

    pub fn end_of_encoder_sample_index(&self) -> usize {
        msg_usize(self.raw, sel(b"endOfEncoderSampleIndex\0"))
    }

    pub fn set_end_of_encoder_sample_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setEndOfEncoderSampleIndex:\0"), index);
    }
}

#[derive(Debug)]
pub struct ResourceStatePassSampleBufferAttachmentDescriptorArray {
    pub raw: id,
}

impl ResourceStatePassSampleBufferAttachmentDescriptorArray {
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> ResourceStatePassSampleBufferAttachmentDescriptor {
        let attachment = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        ResourceStatePassSampleBufferAttachmentDescriptor { raw: attachment }
    }
}

#[derive(Debug)]
pub struct ResourceStatePassDescriptor {
    pub raw: id,
}

impl ResourceStatePassDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLResourceStatePassDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new(
                "MTLResourceStatePassDescriptor is not available",
            ));
        }
        let raw = retain(msg_id(class_ptr, sel(b"resourceStatePassDescriptor\0")));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create MTLResourceStatePassDescriptor",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn sample_buffer_attachments(
        &self,
    ) -> ResourceStatePassSampleBufferAttachmentDescriptorArray {
        let array = msg_id(self.raw, sel(b"sampleBufferAttachments\0"));
        ResourceStatePassSampleBufferAttachmentDescriptorArray { raw: array }
    }
}

impl Drop for ResourceStatePassDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ParallelRenderCommandEncoder {
    pub raw: id,
}

impl ParallelRenderCommandEncoder {
    pub fn render_command_encoder(&self) -> Result<RenderCommandEncoder, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"renderCommandEncoder\0")));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create child RenderCommandEncoder",
            ))
        } else {
            Ok(RenderCommandEncoder { raw })
        }
    }

    pub fn set_color_store_action(&self, store_action: StoreAction, index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setColorStoreAction:atIndex:\0"),
                store_action as usize,
                index,
            );
        }
    }

    pub fn set_depth_store_action(&self, store_action: StoreAction) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthStoreAction:\0"),
            store_action as usize,
        );
    }

    pub fn set_stencil_store_action(&self, store_action: StoreAction) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilStoreAction:\0"),
            store_action as usize,
        );
    }

    pub fn set_color_store_action_options(&self, options: StoreActionOptions, index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setColorStoreActionOptions:atIndex:\0"),
                options.0,
                index,
            );
        }
    }

    pub fn set_depth_store_action_options(&self, options: StoreActionOptions) {
        msg_void_usize(self.raw, sel(b"setDepthStoreActionOptions:\0"), options.0);
    }

    pub fn set_stencil_store_action_options(&self, options: StoreActionOptions) {
        msg_void_usize(self.raw, sel(b"setStencilStoreActionOptions:\0"), options.0);
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for ParallelRenderCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}
