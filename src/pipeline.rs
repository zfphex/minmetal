use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct FunctionConstantValues {
    pub raw: id,
}

impl FunctionConstantValues {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLFunctionConstantValues\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_bool(&self, index: usize, value: bool) {
        let value: bool = value;
        self.set_raw(
            index,
            DataType::Bool,
            &value as *const bool as *const c_void,
        );
    }

    pub fn set_u32(&self, index: usize, value: u32) {
        self.set_raw(index, DataType::UInt, &value as *const u32 as *const c_void);
    }

    pub fn set_i32(&self, index: usize, value: i32) {
        self.set_raw(index, DataType::Int, &value as *const i32 as *const c_void);
    }

    pub fn set_f32(&self, index: usize, value: f32) {
        self.set_raw(
            index,
            DataType::Float,
            &value as *const f32 as *const c_void,
        );
    }

    pub fn set_bytes(&self, index: usize, data_type: DataType, bytes: &[u8]) {
        self.set_raw(index, data_type, bytes.as_ptr() as *const c_void);
    }

    fn set_raw(&self, index: usize, data_type: DataType, ptr: *const c_void) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setConstantValue:type:atIndex:\0"),
                ptr,
                data_type as usize,
                index,
            );
        }
    }
}

impl Default for FunctionConstantValues {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FunctionConstantValues {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct BinaryArchiveDescriptor {
    pub raw: id,
}

impl BinaryArchiveDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLBinaryArchiveDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }
}

impl Default for BinaryArchiveDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BinaryArchiveDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct BinaryArchive {
    pub raw: id,
}

impl BinaryArchive {
    pub fn add_render_pipeline_functions(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(
                self.raw,
                sel(b"addRenderPipelineFunctionsWithDescriptor:error:\0"),
                descriptor.raw,
                &mut error,
            );
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add render pipeline functions to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_compute_pipeline_functions(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(
                self.raw,
                sel(b"addComputePipelineFunctionsWithDescriptor:error:\0"),
                descriptor.raw,
                &mut error,
            );
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add compute pipeline functions to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for BinaryArchive {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ComputePipelineDescriptor {
    pub raw: id,
}

impl ComputePipelineDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLComputePipelineDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_compute_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setComputeFunction:\0"), function.raw);
    }

    pub fn set_binary_archives(&self, archives: &[&BinaryArchive]) {
        let raw: Vec<id> = archives.iter().map(|archive| archive.raw).collect();
        let array = ns_array_from_ids(&raw);
        msg_void_id(self.raw, sel(b"setBinaryArchives:\0"), array);
    }

    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setSupportIndirectCommandBuffers:\0"),
            if support { YES } else { NO },
        );
    }

    pub fn linked_functions(&self) -> LinkedFunctions {
        let lf = msg_id(self.raw, sel(b"linkedFunctions\0"));
        LinkedFunctions { raw: retain(lf) }
    }

    pub fn set_linked_functions(&self, linked_functions: &LinkedFunctions) {
        msg_void_id(
            self.raw,
            sel(b"setLinkedFunctions:\0"),
            linked_functions.raw,
        );
    }

    pub fn support_adding_binary_functions(&self) -> bool {
        msg_bool(self.raw, sel(b"supportAddingBinaryFunctions\0")) != NO
    }

    pub fn set_support_adding_binary_functions(&self, support: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setSupportAddingBinaryFunctions:\0"),
            if support { YES } else { NO },
        );
    }

    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTotalThreadsPerThreadgroup\0"))
    }

    pub fn set_max_total_threads_per_threadgroup(&self, max: usize) {
        msg_void_usize(self.raw, sel(b"setMaxTotalThreadsPerThreadgroup:\0"), max);
    }

    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        msg_bool(
            self.raw,
            sel(b"threadGroupSizeIsMultipleOfThreadExecutionWidth\0"),
        ) != NO
    }

    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setThreadGroupSizeIsMultipleOfThreadExecutionWidth:\0"),
            if value { YES } else { NO },
        );
    }
}

impl Default for ComputePipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ComputePipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ComputePipelineState {
    pub raw: id,
}

impl Drop for ComputePipelineState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl ComputePipelineState {
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTotalThreadsPerThreadgroup\0"))
    }

    pub fn thread_execution_width(&self) -> usize {
        msg_usize(self.raw, sel(b"threadExecutionWidth\0"))
    }

    pub fn static_threadgroup_memory_length(&self) -> usize {
        let selector = sel(b"staticThreadgroupMemoryLength\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn support_indirect_command_buffers(&self) -> bool {
        let selector = sel(b"supportIndirectCommandBuffers\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on ComputePipelineState",
            ))
        }
    }

    pub fn function_handle_with_function(
        &self,
        function: &Function,
    ) -> Result<FunctionHandle, MetalError> {
        let selector = sel(b"functionHandleWithFunction:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = retain(msg_id_id(self.raw, selector, function.raw));
            if raw.is_null() {
                Err(MetalError::new("failed to get function handle"))
            } else {
                Ok(FunctionHandle { raw })
            }
        } else {
            Err(MetalError::new(
                "functionHandleWithFunction: not supported on ComputePipelineState",
            ))
        }
    }

    pub fn new_visible_function_table(
        &self,
        descriptor: &VisibleFunctionTableDescriptor,
    ) -> Result<VisibleFunctionTable, MetalError> {
        let selector = sel(b"newVisibleFunctionTableWithDescriptor:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id(self.raw, selector, descriptor.raw);
            if raw.is_null() {
                Err(MetalError::new("failed to create visible function table"))
            } else {
                Ok(VisibleFunctionTable { raw })
            }
        } else {
            Err(MetalError::new(
                "newVisibleFunctionTableWithDescriptor: not supported on ComputePipelineState",
            ))
        }
    }

    pub fn new_intersection_function_table(
        &self,
        descriptor: &IntersectionFunctionTableDescriptor,
    ) -> Result<IntersectionFunctionTable, MetalError> {
        let selector = sel(b"newIntersectionFunctionTableWithDescriptor:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id(self.raw, selector, descriptor.raw);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create intersection function table",
                ))
            } else {
                Ok(IntersectionFunctionTable { raw })
            }
        } else {
            Err(MetalError::new(
                "newIntersectionFunctionTableWithDescriptor: not supported on ComputePipelineState",
            ))
        }
    }
}

#[derive(Debug)]
pub struct VertexDescriptor {
    pub raw: id,
}

impl VertexDescriptor {
    pub fn new() -> Self {
        let raw = retain(msg_id(
            class(b"MTLVertexDescriptor\0"),
            sel(b"vertexDescriptor\0"),
        ));
        Self { raw }
    }

    pub fn set_attribute(
        &self,
        index: usize,
        format: VertexFormat,
        offset: usize,
        buffer_index: usize,
    ) {
        let attributes = msg_id(self.raw, sel(b"attributes\0"));
        let attribute = msg_id_usize(attributes, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(attribute, sel(b"setFormat:\0"), format as usize);
        msg_void_usize(attribute, sel(b"setOffset:\0"), offset);
        msg_void_usize(attribute, sel(b"setBufferIndex:\0"), buffer_index);
    }

    pub fn set_layout(
        &self,
        index: usize,
        stride: usize,
        step_function: VertexStepFunction,
        step_rate: usize,
    ) {
        let layouts = msg_id(self.raw, sel(b"layouts\0"));
        let layout = msg_id_usize(layouts, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(layout, sel(b"setStride:\0"), stride);
        msg_void_usize(layout, sel(b"setStepFunction:\0"), step_function as usize);
        msg_void_usize(layout, sel(b"setStepRate:\0"), step_rate);
    }
}

impl Default for VertexDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VertexDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct RenderPipelineDescriptor {
    pub raw: id,
}

impl RenderPipelineDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLRenderPipelineDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_vertex_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setVertexFunction:\0"), function.raw);
    }

    pub fn set_fragment_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setFragmentFunction:\0"), function.raw);
    }

    pub fn set_color_attachment_pixel_format(&self, index: usize, pixel_format: PixelFormat) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(attachment, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
    }

    pub fn set_vertex_descriptor(&self, vertex_descriptor: &VertexDescriptor) {
        msg_void_id(
            self.raw,
            sel(b"setVertexDescriptor:\0"),
            vertex_descriptor.raw,
        );
    }

    pub fn set_sample_count(&self, sample_count: usize) {
        msg_void_usize(self.raw, sel(b"setSampleCount:\0"), sample_count);
    }

    pub fn set_raster_sample_count(&self, raster_sample_count: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setRasterSampleCount:\0"),
            raster_sample_count,
        );
    }

    pub fn set_depth_attachment_pixel_format(&self, pixel_format: PixelFormat) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthAttachmentPixelFormat:\0"),
            pixel_format.as_raw(),
        );
    }

    pub fn set_stencil_attachment_pixel_format(&self, pixel_format: PixelFormat) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilAttachmentPixelFormat:\0"),
            pixel_format.as_raw(),
        );
    }

    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setAlphaToCoverageEnabled:\0"),
            if enabled { YES } else { NO },
        );
    }

    pub fn set_color_attachment_blending(
        &self,
        index: usize,
        enabled: bool,
        source_rgb: BlendFactor,
        destination_rgb: BlendFactor,
        rgb_operation: BlendOperation,
        source_alpha: BlendFactor,
        destination_alpha: BlendFactor,
        alpha_operation: BlendOperation,
    ) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_bool(
            attachment,
            sel(b"setBlendingEnabled:\0"),
            if enabled { YES } else { NO },
        );
        msg_void_usize(
            attachment,
            sel(b"setSourceRGBBlendFactor:\0"),
            source_rgb as usize,
        );
        msg_void_usize(
            attachment,
            sel(b"setDestinationRGBBlendFactor:\0"),
            destination_rgb as usize,
        );
        msg_void_usize(
            attachment,
            sel(b"setRgbBlendOperation:\0"),
            rgb_operation as usize,
        );
        msg_void_usize(
            attachment,
            sel(b"setSourceAlphaBlendFactor:\0"),
            source_alpha as usize,
        );
        msg_void_usize(
            attachment,
            sel(b"setDestinationAlphaBlendFactor:\0"),
            destination_alpha as usize,
        );
        msg_void_usize(
            attachment,
            sel(b"setAlphaBlendOperation:\0"),
            alpha_operation as usize,
        );
    }

    pub fn set_color_attachment_write_mask(&self, index: usize, mask: ColorWriteMask) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(attachment, sel(b"setWriteMask:\0"), mask.as_raw());
    }

    pub fn set_binary_archives(&self, archives: &[&BinaryArchive]) {
        let raw: Vec<id> = archives.iter().map(|archive| archive.raw).collect();
        let array = ns_array_from_ids(&raw);
        msg_void_id(self.raw, sel(b"setBinaryArchives:\0"), array);
    }

    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setSupportIndirectCommandBuffers:\0"),
            if support { YES } else { NO },
        );
    }

    pub fn linked_functions(&self) -> LinkedFunctions {
        let lf = msg_id(self.raw, sel(b"linkedFunctions\0"));
        LinkedFunctions { raw: retain(lf) }
    }

    pub fn set_linked_functions(&self, linked_functions: &LinkedFunctions) {
        msg_void_id(
            self.raw,
            sel(b"setLinkedFunctions:\0"),
            linked_functions.raw,
        );
    }

    pub fn support_adding_binary_functions(&self) -> bool {
        let selector = sel(b"supportAddingBinaryFunctions\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn set_support_adding_binary_functions(&self, support: bool) {
        let selector = sel(b"setSupportAddingBinaryFunctions:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if support { YES } else { NO });
        }
    }

    pub fn max_call_stack_depth(&self) -> usize {
        let selector = sel(b"maxCallStackDepth\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_max_call_stack_depth(&self, depth: usize) {
        let selector = sel(b"setMaxCallStackDepth:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, depth);
        }
    }
}

impl Default for RenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RenderPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct RenderPipelineState {
    pub raw: id,
}

impl Drop for RenderPipelineState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl RenderPipelineState {
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        let selector = sel(b"maxTotalThreadsPerThreadgroup\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn threadgroup_size_matches_tile_size(&self) -> bool {
        let selector = sel(b"threadgroupSizeMatchesTileSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn max_total_threads_per_object_threadgroup(&self) -> usize {
        let selector = sel(b"maxTotalThreadsPerObjectThreadgroup\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn max_total_threads_per_mesh_threadgroup(&self) -> usize {
        let selector = sel(b"maxTotalThreadsPerMeshThreadgroup\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn object_thread_execution_width(&self) -> usize {
        let selector = sel(b"objectThreadExecutionWidth\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn mesh_thread_execution_width(&self) -> usize {
        let selector = sel(b"meshThreadExecutionWidth\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn max_total_threadgroups_per_mesh_grid(&self) -> usize {
        let selector = sel(b"maxTotalThreadgroupsPerMeshGrid\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on RenderPipelineState",
            ))
        }
    }

    pub fn function_handle_with_function(
        &self,
        function: &Function,
        stage: RenderStages,
    ) -> Result<FunctionHandle, MetalError> {
        let selector = sel(b"functionHandleWithFunction:stage:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = retain(msg_id_id_usize(self.raw, selector, function.raw, stage.0));
            if raw.is_null() {
                Err(MetalError::new("failed to get function handle"))
            } else {
                Ok(FunctionHandle { raw })
            }
        } else {
            Err(MetalError::new(
                "functionHandleWithFunction:stage: not supported on RenderPipelineState",
            ))
        }
    }

    pub fn new_visible_function_table(
        &self,
        descriptor: &VisibleFunctionTableDescriptor,
        stage: RenderStages,
    ) -> Result<VisibleFunctionTable, MetalError> {
        let selector = sel(b"newVisibleFunctionTableWithDescriptor:stage:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id_usize(self.raw, selector, descriptor.raw, stage.0);
            if raw.is_null() {
                Err(MetalError::new("failed to create visible function table"))
            } else {
                Ok(VisibleFunctionTable { raw })
            }
        } else {
            Err(MetalError::new(
                "newVisibleFunctionTableWithDescriptor:stage: not supported on RenderPipelineState",
            ))
        }
    }

    pub fn new_intersection_function_table(
        &self,
        descriptor: &IntersectionFunctionTableDescriptor,
        stage: RenderStages,
    ) -> Result<IntersectionFunctionTable, MetalError> {
        let selector = sel(b"newIntersectionFunctionTableWithDescriptor:stage:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id_usize(self.raw, selector, descriptor.raw, stage.0);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create intersection function table",
                ))
            } else {
                Ok(IntersectionFunctionTable { raw })
            }
        } else {
            Err(MetalError::new(
                "newIntersectionFunctionTableWithDescriptor:stage: not supported on RenderPipelineState",
            ))
        }
    }
}

#[derive(Debug)]
pub struct StencilDescriptor {
    pub raw: id,
}

impl StencilDescriptor {
    fn borrowed(raw: id) -> Self {
        Self { raw }
    }

    pub fn set_stencil_compare_function(&self, compare_function: CompareFunction) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilCompareFunction:\0"),
            compare_function as usize,
        );
    }

    pub fn set_stencil_failure_operation(&self, operation: StencilOperation) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilFailureOperation:\0"),
            operation as usize,
        );
    }

    pub fn set_depth_failure_operation(&self, operation: StencilOperation) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthFailureOperation:\0"),
            operation as usize,
        );
    }

    pub fn set_depth_stencil_pass_operation(&self, operation: StencilOperation) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthStencilPassOperation:\0"),
            operation as usize,
        );
    }

    pub fn set_read_mask(&self, mask: u32) {
        msg_void_usize(self.raw, sel(b"setReadMask:\0"), mask as usize);
    }

    pub fn set_write_mask(&self, mask: u32) {
        msg_void_usize(self.raw, sel(b"setWriteMask:\0"), mask as usize);
    }
}

#[derive(Debug)]
pub struct DepthStencilDescriptor {
    pub raw: id,
}

impl DepthStencilDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLDepthStencilDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_depth_compare_function(&self, compare_function: CompareFunction) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthCompareFunction:\0"),
            compare_function as usize,
        );
    }

    pub fn set_depth_write_enabled(&self, enabled: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setDepthWriteEnabled:\0"),
            if enabled { YES } else { NO },
        );
    }

    pub fn front_face_stencil(&self) -> StencilDescriptor {
        StencilDescriptor::borrowed(msg_id(self.raw, sel(b"frontFaceStencil\0")))
    }

    pub fn back_face_stencil(&self) -> StencilDescriptor {
        StencilDescriptor::borrowed(msg_id(self.raw, sel(b"backFaceStencil\0")))
    }
}

impl Default for DepthStencilDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DepthStencilDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct DepthStencilState {
    pub raw: id,
}

impl Drop for DepthStencilState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SamplerDescriptor {
    pub raw: id,
}

impl SamplerDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLSamplerDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_min_filter(&self, filter: SamplerMinMagFilter) {
        msg_void_usize(self.raw, sel(b"setMinFilter:\0"), filter as usize);
    }

    pub fn set_mag_filter(&self, filter: SamplerMinMagFilter) {
        msg_void_usize(self.raw, sel(b"setMagFilter:\0"), filter as usize);
    }

    pub fn set_mip_filter(&self, filter: SamplerMipFilter) {
        msg_void_usize(self.raw, sel(b"setMipFilter:\0"), filter as usize);
    }

    pub fn set_address_mode(&self, mode: SamplerAddressMode) {
        self.set_s_address_mode(mode);
        self.set_t_address_mode(mode);
        self.set_r_address_mode(mode);
    }

    pub fn set_s_address_mode(&self, mode: SamplerAddressMode) {
        msg_void_usize(self.raw, sel(b"setSAddressMode:\0"), mode as usize);
    }

    pub fn set_t_address_mode(&self, mode: SamplerAddressMode) {
        msg_void_usize(self.raw, sel(b"setTAddressMode:\0"), mode as usize);
    }

    pub fn set_r_address_mode(&self, mode: SamplerAddressMode) {
        msg_void_usize(self.raw, sel(b"setRAddressMode:\0"), mode as usize);
    }

    pub fn set_compare_function(&self, compare_function: CompareFunction) {
        msg_void_usize(
            self.raw,
            sel(b"setCompareFunction:\0"),
            compare_function as usize,
        );
    }

    pub fn set_max_anisotropy(&self, max_anisotropy: usize) {
        msg_void_usize(self.raw, sel(b"setMaxAnisotropy:\0"), max_anisotropy);
    }

    pub fn set_lod_min_clamp(&self, value: f64) {
        msg_void_f64(self.raw, sel(b"setLodMinClamp:\0"), value);
    }

    pub fn set_lod_max_clamp(&self, value: f64) {
        msg_void_f64(self.raw, sel(b"setLodMaxClamp:\0"), value);
    }
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SamplerDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct SamplerState {
    pub raw: id,
}

impl Drop for SamplerState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl SamplerState {
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
                "gpuResourceID not supported on this SamplerState",
            ))
        }
    }
}

#[derive(Debug)]
pub struct Fence {
    pub raw: id,
}

impl Drop for Fence {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct FunctionDescriptor {
    pub raw: id,
}

impl FunctionDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLFunctionDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn name(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
    }

    pub fn set_name(&self, name: &str) {
        let ns_name = NSString::new(name);
        msg_void_id(self.raw, sel(b"setName:\0"), ns_name.raw());
    }

    pub fn specialized_name(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"specializedName\0")))
    }

    pub fn set_specialized_name(&self, name: &str) {
        let ns_name = NSString::new(name);
        msg_void_id(self.raw, sel(b"setSpecializedName:\0"), ns_name.raw());
    }

    pub fn constant_values(&self) -> Option<FunctionConstantValues> {
        let cv = msg_id(self.raw, sel(b"constantValues\0"));
        (!cv.is_null()).then_some(FunctionConstantValues { raw: retain(cv) })
    }

    pub fn set_constant_values(&self, constant_values: Option<&FunctionConstantValues>) {
        msg_void_id(
            self.raw,
            sel(b"setConstantValues:\0"),
            constant_values.map_or(NIL, |cv| cv.raw),
        );
    }

    pub fn options(&self) -> FunctionOptions {
        FunctionOptions(msg_usize(self.raw, sel(b"options\0")))
    }

    pub fn set_options(&self, options: FunctionOptions) {
        msg_void_usize(self.raw, sel(b"setOptions:\0"), options.0);
    }

    pub fn set_binary_archives(&self, archives: &[&BinaryArchive]) -> Result<(), MetalError> {
        let selector = sel(b"setBinaryArchives:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_archives: Vec<id> = archives.iter().map(|a| a.raw).collect();
            let array = ns_array_from_ids(&raw_archives);
            msg_void_id(self.raw, selector, array);
            Ok(())
        } else {
            Err(MetalError::new(
                "setBinaryArchives: not supported on FunctionDescriptor",
            ))
        }
    }
}

impl Default for FunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IntersectionFunctionDescriptor {
    pub raw: id,
}

impl IntersectionFunctionDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLIntersectionFunctionDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn base(&self) -> FunctionDescriptor {
        FunctionDescriptor { raw: self.raw }
    }
}

impl Default for IntersectionFunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IntersectionFunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct LinkedFunctions {
    pub raw: id,
}

impl LinkedFunctions {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLLinkedFunctions\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_functions(&self, functions: &[&Function]) {
        let raw_functions: Vec<id> = functions.iter().map(|f| f.raw).collect();
        let array = ns_array_from_ids(&raw_functions);
        msg_void_id(self.raw, sel(b"setFunctions:\0"), array);
    }

    pub fn set_binary_functions(&self, functions: &[&Function]) {
        let raw_functions: Vec<id> = functions.iter().map(|f| f.raw).collect();
        let array = ns_array_from_ids(&raw_functions);
        msg_void_id(self.raw, sel(b"setBinaryFunctions:\0"), array);
    }

    pub fn set_private_functions(&self, functions: &[&Function]) {
        let selector = sel(b"setPrivateFunctions:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_functions: Vec<id> = functions.iter().map(|f| f.raw).collect();
            let array = ns_array_from_ids(&raw_functions);
            msg_void_id(self.raw, selector, array);
        }
    }
}

impl Default for LinkedFunctions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LinkedFunctions {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct DynamicLibrary {
    pub raw: id,
}

impl DynamicLibrary {
    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn install_name(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"installName\0")))
    }

    pub fn serialize_to_url(&self, url_path: &str) -> Result<(), MetalError> {
        unsafe {
            let url = ns_url_from_path(url_path);
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(self.raw, sel(b"serializeToURL:error:\0"), url, &mut error);
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to serialize dynamic library",
                )))
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct FunctionHandle {
    pub raw: id,
}

impl FunctionHandle {
    pub fn name(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
    }
}

impl Drop for FunctionHandle {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct FunctionLogDebugLocation {
    pub raw: id,
}

impl FunctionLogDebugLocation {
    pub fn function_name(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"functionName\0")))
    }

    pub fn url_path(&self) -> Option<String> {
        let url = msg_id(self.raw, sel(b"URL\0"));
        if url.is_null() {
            None
        } else {
            ns_string_to_string(msg_id(url, sel(b"path\0")))
        }
    }

    pub fn line(&self) -> usize {
        msg_usize(self.raw, sel(b"line\0"))
    }

    pub fn column(&self) -> usize {
        msg_usize(self.raw, sel(b"column\0"))
    }
}

impl Drop for FunctionLogDebugLocation {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct FunctionLog {
    pub raw: id,
}

impl FunctionLog {
    pub fn log_type(&self) -> usize {
        msg_usize(self.raw, sel(b"type\0"))
    }

    pub fn encoder_label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"encoderLabel\0")))
    }

    pub fn function(&self) -> Option<Function> {
        let f = msg_id(self.raw, sel(b"function\0"));
        (!f.is_null()).then_some(Function { raw: retain(f) })
    }

    pub fn debug_location(&self) -> Option<FunctionLogDebugLocation> {
        let loc = msg_id(self.raw, sel(b"debugLocation\0"));
        (!loc.is_null()).then_some(FunctionLogDebugLocation { raw: retain(loc) })
    }
}

impl Drop for FunctionLog {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct LogStateDescriptor {
    pub raw: id,
}

impl LogStateDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLLogStateDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new("MTLLogStateDescriptor is not available"));
        }
        let raw = retain(msg_id(class_ptr, sel(b"alloc\0")));
        let init_raw = msg_id(raw, sel(b"init\0"));
        if init_raw.is_null() {
            Err(MetalError::new(
                "failed to initialize MTLLogStateDescriptor",
            ))
        } else {
            Ok(Self { raw: init_raw })
        }
    }

    pub fn level(&self) -> LogLevel {
        unsafe { std::mem::transmute(msg_usize(self.raw, sel(b"level\0"))) }
    }

    pub fn set_level(&self, level: LogLevel) {
        msg_void_usize(self.raw, sel(b"setLevel:\0"), level as usize);
    }

    pub fn buffer_size(&self) -> usize {
        msg_usize(self.raw, sel(b"bufferSize\0"))
    }

    pub fn set_buffer_size(&self, size: usize) {
        msg_void_usize(self.raw, sel(b"setBufferSize:\0"), size);
    }
}

impl Drop for LogStateDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct LogState {
    pub raw: id,
}

impl Drop for LogState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct VisibleFunctionTableDescriptor {
    pub raw: id,
}

impl VisibleFunctionTableDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLVisibleFunctionTableDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn function_count(&self) -> usize {
        msg_usize(self.raw, sel(b"functionCount\0"))
    }

    pub fn set_function_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setFunctionCount:\0"), count);
    }
}

impl Default for VisibleFunctionTableDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VisibleFunctionTableDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct VisibleFunctionTable {
    pub raw: id,
}

impl VisibleFunctionTable {
    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new("gpuResourceID not supported"))
        }
    }

    pub fn set_function(&self, function: Option<&FunctionHandle>, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFunction:atIndex:\0"),
            function.map_or(NIL, |f| f.raw),
            index,
        );
    }

    pub fn set_functions(&self, functions: &[Option<&FunctionHandle>], range: Range) {
        let raw_functions: Vec<id> = functions.iter().map(|f| f.map_or(NIL, |h| h.raw)).collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setFunctions:withRange:\0"),
            raw_functions.as_ptr(),
            range,
        );
    }
}

impl Drop for VisibleFunctionTable {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IntersectionFunctionTableDescriptor {
    pub raw: id,
}

impl IntersectionFunctionTableDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLIntersectionFunctionTableDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn function_count(&self) -> usize {
        msg_usize(self.raw, sel(b"functionCount\0"))
    }

    pub fn set_function_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setFunctionCount:\0"), count);
    }
}

impl Default for IntersectionFunctionTableDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IntersectionFunctionTableDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IntersectionFunctionTable {
    pub raw: id,
}

impl IntersectionFunctionTable {
    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new("gpuResourceID not supported"))
        }
    }

    pub fn set_function(&self, function: Option<&FunctionHandle>, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"setFunction:atIndex:\0"),
            function.map_or(NIL, |f| f.raw),
            index,
        );
    }

    pub fn set_functions(&self, functions: &[Option<&FunctionHandle>], range: Range) {
        let raw_functions: Vec<id> = functions.iter().map(|f| f.map_or(NIL, |h| h.raw)).collect();
        msg_void_ptr_range(
            self.raw,
            sel(b"setFunctions:withRange:\0"),
            raw_functions.as_ptr(),
            range,
        );
    }

    pub fn set_buffer(&self, buffer: Option<&Buffer>, offset: usize, index: usize) {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setBuffer:offset:atIndex:\0"),
            buffer.map_or(NIL, |b| b.raw),
            offset,
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

    pub fn set_opaque_triangle_intersection_function(
        &self,
        signature: IntersectionFunctionSignature,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:\0"),
                signature.0,
                index,
            );
        }
    }

    pub fn set_opaque_triangle_intersection_functions(
        &self,
        signature: IntersectionFunctionSignature,
        range: Range,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setOpaqueTriangleIntersectionFunctionWithSignature:withRange:\0"),
                signature.0,
                range,
            );
        }
    }

    pub fn set_opaque_curve_intersection_function(
        &self,
        signature: IntersectionFunctionSignature,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setOpaqueCurveIntersectionFunctionWithSignature:atIndex:\0"),
                signature.0,
                index,
            );
        }
    }

    pub fn set_opaque_curve_intersection_functions(
        &self,
        signature: IntersectionFunctionSignature,
        range: Range,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setOpaqueCurveIntersectionFunctionWithSignature:withRange:\0"),
                signature.0,
                range,
            );
        }
    }

    pub fn set_visible_function_table(
        &self,
        table: Option<&VisibleFunctionTable>,
        buffer_index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setVisibleFunctionTable:atBufferIndex:\0"),
            table.map_or(NIL, |t| t.raw),
            buffer_index,
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
}

impl Drop for IntersectionFunctionTable {
    fn drop(&mut self) {
        release(self.raw);
    }
}
