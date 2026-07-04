use crate::*;
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr;

#[derive(Debug)]
pub struct Device {
    pub raw: id,
}

impl Device {
    pub fn system_default() -> Option<Self> {
        unsafe {
            let raw = MTLCreateSystemDefaultDevice();
            (!raw.is_null()).then_some(Self { raw })
        }
    }

    pub fn required_system_default() -> Result<Self, MetalError> {
        Self::system_default()
            .ok_or_else(|| MetalError::new("no system default Metal device found"))
    }

    pub fn name(&self) -> String {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
            .unwrap_or_else(|| "Unknown Metal Device".to_string())
    }

    pub fn new_command_queue(&self) -> Result<CommandQueue, MetalError> {
        let raw = msg_id(self.raw, sel(b"newCommandQueue\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal command queue"))
        } else {
            Ok(CommandQueue { raw })
        }
    }

    pub fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: usize,
    ) -> Result<CommandQueue, MetalError> {
        let selector = sel(b"newCommandQueueWithMaxCommandBufferCount:\0");
        let raw = msg_id_usize(self.raw, selector, max_command_buffer_count);
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal command queue with max command buffer count",
            ))
        } else {
            Ok(CommandQueue { raw })
        }
    }

    pub fn new_command_queue_with_descriptor(
        &self,
        descriptor: &CommandQueueDescriptor,
    ) -> Result<CommandQueue, MetalError> {
        let selector = sel(b"newCommandQueueWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newCommandQueueWithDescriptor: is not supported on this macOS version",
            ));
        }
        let raw = msg_id_id(self.raw, selector, descriptor.raw);
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal command queue with descriptor",
            ))
        } else {
            Ok(CommandQueue { raw })
        }
    }

    pub fn new_library_with_source(&self, source: &str) -> Result<Library, MetalError> {
        unsafe {
            let source = NSString::new(source);
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newLibraryWithSource:options:error:\0"),
                source.raw(),
                NIL,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to compile Metal shader source",
                )))
            } else {
                Ok(Library { raw })
            }
        }
    }

    pub fn new_library_with_source_and_options(
        &self,
        source: &str,
        options: &CompileOptions,
    ) -> Result<Library, MetalError> {
        unsafe {
            let source = NSString::new(source);
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newLibraryWithSource:options:error:\0"),
                source.raw(),
                options.raw,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to compile Metal shader source with options",
                )))
            } else {
                Ok(Library { raw })
            }
        }
    }

    pub fn new_library_with_stitched_descriptor(
        &self,
        descriptor: &StitchedLibraryDescriptor,
    ) -> Result<Library, MetalError> {
        let selector = sel(b"newLibraryWithStitchedDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newLibraryWithStitchedDescriptor:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create stitched Metal library",
            )))
        } else {
            Ok(Library { raw })
        }
    }

    pub fn new_library_with_url_path(&self, path: &str) -> Result<Library, MetalError> {
        let selector = sel(b"newLibraryWithURL:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newLibraryWithURL:error: is not supported on this macOS version",
            ));
        }
        let ns_url = ns_url_from_path(path);
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, ns_url, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                &format!("failed to load Metal library from URL: {}", path),
            )))
        } else {
            Ok(Library { raw })
        }
    }

    pub fn new_library_with_file(&self, path: &str) -> Result<Library, MetalError> {
        let selector_url = sel(b"newLibraryWithURL:error:\0");
        if responds_to_selector(self.raw, selector_url) {
            self.new_library_with_url_path(path)
        } else {
            let ns_path = NSString::new(path);
            let mut error = NIL;
            let selector_file = sel(b"newLibraryWithFile:error:\0");
            let raw = msg_id_id_err(self.raw, selector_file, ns_path.raw(), &mut error);
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    &format!("failed to load Metal library from file: {}", path),
                )))
            } else {
                Ok(Library { raw })
            }
        }
    }

    pub fn new_default_library(&self) -> Result<Library, MetalError> {
        let raw = msg_id(self.raw, sel(b"newDefaultLibrary\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create default Metal library"))
        } else {
            Ok(Library { raw })
        }
    }

    pub fn new_default_library_with_bundle(&self, raw_bundle: id) -> Result<Library, MetalError> {
        let selector = sel(b"newDefaultLibraryWithBundle:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newDefaultLibraryWithBundle:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, raw_bundle, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create default Metal library with bundle",
            )))
        } else {
            Ok(Library { raw })
        }
    }

    pub fn new_render_pipeline_state(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<RenderPipelineState, MetalError> {
        let mut error = NIL;
        let raw = msg_id_id_err(
            self.raw,
            sel(b"newRenderPipelineStateWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        );
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal render pipeline state",
            )))
        } else {
            Ok(RenderPipelineState { raw })
        }
    }

    pub fn new_compute_pipeline_state_with_function(
        &self,
        function: &Function,
    ) -> Result<ComputePipelineState, MetalError> {
        let mut error = NIL;
        let raw = msg_id_id_err(
            self.raw,
            sel(b"newComputePipelineStateWithFunction:error:\0"),
            function.raw,
            &mut error,
        );
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal compute pipeline state with function",
            )))
        } else {
            Ok(ComputePipelineState { raw })
        }
    }

    pub fn new_compute_pipeline_state(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<ComputePipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newComputePipelineStateWithDescriptor:options:reflection:error:\0"),
                descriptor.raw,
                0,
                ptr::null_mut(),
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal compute pipeline state with descriptor",
                )))
            } else {
                Ok(ComputePipelineState { raw })
            }
        }
    }

    pub fn new_depth_stencil_state(
        &self,
        descriptor: &DepthStencilDescriptor,
    ) -> Result<DepthStencilState, MetalError> {
        let raw = msg_id_id(
            self.raw,
            sel(b"newDepthStencilStateWithDescriptor:\0"),
            descriptor.raw,
        );
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal depth stencil state",
            ))
        } else {
            Ok(DepthStencilState { raw })
        }
    }

    pub fn new_sampler_state(
        &self,
        descriptor: &SamplerDescriptor,
    ) -> Result<SamplerState, MetalError> {
        let raw = msg_id_id(
            self.raw,
            sel(b"newSamplerStateWithDescriptor:\0"),
            descriptor.raw,
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal sampler state"))
        } else {
            Ok(SamplerState { raw })
        }
    }

    pub fn new_fence(&self) -> Result<Fence, MetalError> {
        let raw = msg_id(self.raw, sel(b"newFence\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal fence"))
        } else {
            Ok(Fence { raw })
        }
    }

    pub fn new_shared_event(&self) -> Result<SharedEvent, MetalError> {
        let raw = msg_id(self.raw, sel(b"newSharedEvent\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal shared event"))
        } else {
            Ok(SharedEvent { raw })
        }
    }

    pub fn new_indirect_command_buffer(
        &self,
        descriptor: &IndirectCommandBufferDescriptor,
        max_command_count: usize,
        options: IndirectCommandBufferOptions,
    ) -> Result<IndirectCommandBuffer, MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newIndirectCommandBufferWithDescriptor:maxCommandCount:options:\0"),
                descriptor.raw,
                max_command_count,
                options.as_raw(),
            );
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal indirect command buffer",
                ))
            } else {
                Ok(IndirectCommandBuffer { raw })
            }
        }
    }

    pub fn new_binary_archive(
        &self,
        descriptor: &BinaryArchiveDescriptor,
    ) -> Result<BinaryArchive, MetalError> {
        let mut error = NIL;
        let raw = msg_id_id_err(
            self.raw,
            sel(b"newBinaryArchiveWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        );
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal binary archive",
            )))
        } else {
            Ok(BinaryArchive { raw })
        }
    }

    pub fn new_buffer(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        let raw = msg_id_usize_usize(
            self.raw,
            sel(b"newBufferWithLength:options:\0"),
            length,
            options.as_raw(),
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal buffer"))
        } else {
            Ok(Buffer { raw })
        }
    }

    pub fn new_heap(&self, descriptor: &HeapDescriptor) -> Result<Heap, MetalError> {
        let raw = msg_id_id(self.raw, sel(b"newHeapWithDescriptor:\0"), descriptor.raw);
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal heap"))
        } else {
            Ok(Heap { raw })
        }
    }

    pub fn heap_buffer_size_and_align(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> SizeAndAlign {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) -> SizeAndAlign =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"heapBufferSizeAndAlignWithLength:options:\0"),
                length,
                options.as_raw(),
            )
        }
    }

    pub fn heap_texture_size_and_align(&self, descriptor: &TextureDescriptor) -> SizeAndAlign {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id) -> SizeAndAlign =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"heapTextureSizeAndAlignWithDescriptor:\0"),
                descriptor.raw,
            )
        }
    }

    pub fn new_argument_encoder(
        &self,
        descriptors: &[&ArgumentDescriptor],
    ) -> Result<ArgumentEncoder, MetalError> {
        let raw_descriptors: Vec<id> = descriptors
            .iter()
            .map(|descriptor| descriptor.raw)
            .collect();
        let array = ns_array_from_ids(&raw_descriptors);
        let raw = msg_id_id(self.raw, sel(b"newArgumentEncoderWithArguments:\0"), array);
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal argument encoder"))
        } else {
            Ok(ArgumentEncoder { raw })
        }
    }

    pub fn new_buffer_with_data<T>(
        &self,
        data: &[T],
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        let length = std::mem::size_of_val(data);
        let raw = msg_id_ptr_usize_usize(
            self.raw,
            sel(b"newBufferWithBytes:length:options:\0"),
            data.as_ptr() as *const c_void,
            length,
            options.as_raw(),
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal buffer from data"))
        } else {
            Ok(Buffer { raw })
        }
    }

    pub fn new_texture(&self, descriptor: &TextureDescriptor) -> Result<Texture, MetalError> {
        let raw = msg_id_id(
            self.raw,
            sel(b"newTextureWithDescriptor:\0"),
            descriptor.raw,
        );
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal texture"))
        } else {
            Ok(Texture { raw })
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CommandQueue {
    pub raw: id,
}

impl CommandQueue {
    pub fn command_buffer(&self) -> Result<CommandBuffer, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"commandBuffer\0")));
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal command buffer"))
        } else {
            Ok(CommandBuffer { raw })
        }
    }

    pub fn command_buffer_with_unretained_references(&self) -> Result<CommandBuffer, MetalError> {
        let raw = retain(msg_id(
            self.raw,
            sel(b"commandBufferWithUnretainedReferences\0"),
        ));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal command buffer with unretained references",
            ))
        } else {
            Ok(CommandBuffer { raw })
        }
    }

    pub fn command_buffer_with_descriptor(
        &self,
        descriptor: &CommandBufferDescriptor,
    ) -> Result<CommandBuffer, MetalError> {
        let selector = sel(b"commandBufferWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "commandBufferWithDescriptor: is not supported on this macOS version",
            ));
        }
        let raw = retain(msg_id_id(self.raw, selector, descriptor.raw));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal command buffer with descriptor",
            ))
        } else {
            Ok(CommandBuffer { raw })
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

    pub fn add_residency_set(&self, residency_set: &ResidencySet) {
        let selector = sel(b"addResidencySet:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, residency_set.raw);
        }
    }

    pub fn add_residency_sets(&self, residency_sets: &[&ResidencySet]) {
        let selector = sel(b"addResidencySets:count:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_sets: Vec<id> = residency_sets.iter().map(|s| s.raw).collect();
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, raw_sets.as_ptr(), raw_sets.len());
            }
        }
    }

    pub fn remove_residency_set(&self, residency_set: &ResidencySet) {
        let selector = sel(b"removeResidencySet:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, residency_set.raw);
        }
    }

    pub fn remove_residency_sets(&self, residency_sets: &[&ResidencySet]) {
        let selector = sel(b"removeResidencySets:count:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_sets: Vec<id> = residency_sets.iter().map(|s| s.raw).collect();
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, raw_sets.as_ptr(), raw_sets.len());
            }
        }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    pub raw: id,
}

impl CommandBuffer {
    pub fn render_command_encoder(
        &self,
        descriptor: &RenderPassDescriptor,
    ) -> Result<RenderCommandEncoder, MetalError> {
        let raw = retain(msg_id_id(
            self.raw,
            sel(b"renderCommandEncoderWithDescriptor:\0"),
            descriptor.raw,
        ));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal render command encoder",
            ))
        } else {
            Ok(RenderCommandEncoder { raw })
        }
    }

    pub fn parallel_render_command_encoder(
        &self,
        descriptor: &RenderPassDescriptor,
    ) -> Result<ParallelRenderCommandEncoder, MetalError> {
        let raw = retain(msg_id_id(
            self.raw,
            sel(b"parallelRenderCommandEncoderWithDescriptor:\0"),
            descriptor.raw,
        ));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal parallel render command encoder",
            ))
        } else {
            Ok(ParallelRenderCommandEncoder { raw })
        }
    }

    pub fn blit_command_encoder(&self) -> Result<BlitCommandEncoder, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"blitCommandEncoder\0")));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal blit command encoder",
            ))
        } else {
            Ok(BlitCommandEncoder { raw })
        }
    }

    pub fn compute_command_encoder(&self) -> Result<ComputeCommandEncoder, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"computeCommandEncoder\0")));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal compute command encoder",
            ))
        } else {
            Ok(ComputeCommandEncoder { raw })
        }
    }

    pub fn compute_command_encoder_with_descriptor(
        &self,
        descriptor: &ComputePassDescriptor,
    ) -> Result<ComputeCommandEncoder, MetalError> {
        let selector = sel(b"computeCommandEncoderWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "computeCommandEncoderWithDescriptor: is not supported on this macOS version",
            ));
        }
        let raw = retain(msg_id_id(self.raw, selector, descriptor.raw));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal compute command encoder with descriptor",
            ))
        } else {
            Ok(ComputeCommandEncoder { raw })
        }
    }

    pub fn compute_command_encoder_with_dispatch_type(
        &self,
        dispatch_type: DispatchType,
    ) -> Result<ComputeCommandEncoder, MetalError> {
        let selector = sel(b"computeCommandEncoderWithDispatchType:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "computeCommandEncoderWithDispatchType: is not supported on this macOS version",
            ));
        }
        let raw = retain(msg_id_usize(self.raw, selector, dispatch_type as usize));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal compute command encoder with dispatch type",
            ))
        } else {
            Ok(ComputeCommandEncoder { raw })
        }
    }

    pub fn resource_state_command_encoder(
        &self,
    ) -> Result<ResourceStateCommandEncoder, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"resourceStateCommandEncoder\0")));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal resource state command encoder",
            ))
        } else {
            Ok(ResourceStateCommandEncoder { raw })
        }
    }

    pub fn resource_state_command_encoder_with_descriptor(
        &self,
        descriptor: &ResourceStatePassDescriptor,
    ) -> Result<ResourceStateCommandEncoder, MetalError> {
        let selector = sel(b"resourceStateCommandEncoderWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "resourceStateCommandEncoderWithDescriptor: is not supported on this macOS version",
            ));
        }
        let raw = retain(msg_id_id(self.raw, selector, descriptor.raw));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal resource state command encoder with descriptor",
            ))
        } else {
            Ok(ResourceStateCommandEncoder { raw })
        }
    }

    pub fn blit_command_encoder_with_descriptor(
        &self,
        descriptor: &BlitPassDescriptor,
    ) -> Result<BlitCommandEncoder, MetalError> {
        let selector = sel(b"blitCommandEncoderWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "blitCommandEncoderWithDescriptor: is not supported on this macOS version",
            ));
        }
        let raw = retain(msg_id_id(self.raw, selector, descriptor.raw));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create Metal blit command encoder with descriptor",
            ))
        } else {
            Ok(BlitCommandEncoder { raw })
        }
    }

    pub fn encode_signal_event(&self, event: &SharedEvent, value: u64) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, u64) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"encodeSignalEvent:value:\0"),
                event.raw,
                value,
            );
        }
    }

    pub fn encode_wait_for_event(&self, event: &SharedEvent, value: u64) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, u64) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"encodeWaitForEvent:value:\0"),
                event.raw,
                value,
            );
        }
    }

    pub fn present_drawable(&self, drawable: &Drawable) {
        msg_void_id(self.raw, sel(b"presentDrawable:\0"), drawable.raw);
    }

    pub fn present_drawable_at_time(&self, drawable: &Drawable, time: f64) {
        let selector = sel(b"presentDrawable:atTime:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, f64) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, drawable.raw, time);
            }
        }
    }

    pub fn present_drawable_after_minimum_duration(&self, drawable: &Drawable, duration: f64) {
        let selector = sel(b"presentDrawable:afterMinimumDuration:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, id, f64) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, drawable.raw, duration);
            }
        }
    }

    pub fn commit(&self) {
        msg_void(self.raw, sel(b"commit\0"));
    }

    pub fn enqueue(&self) {
        msg_void(self.raw, sel(b"enqueue\0"));
    }

    pub fn wait_until_scheduled(&self) {
        msg_void(self.raw, sel(b"waitUntilScheduled\0"));
    }

    pub fn wait_until_completed(&self) {
        msg_void(self.raw, sel(b"waitUntilCompleted\0"));
    }

    pub fn status(&self) -> CommandBufferStatus {
        let status = msg_usize(self.raw, sel(b"status\0"));
        match status {
            0 => CommandBufferStatus::NotEnqueued,
            1 => CommandBufferStatus::Enqueued,
            2 => CommandBufferStatus::Committed,
            3 => CommandBufferStatus::Scheduled,
            4 => CommandBufferStatus::Completed,
            5 => CommandBufferStatus::Error,
            _ => CommandBufferStatus::Error,
        }
    }

    pub fn error(&self) -> Option<MetalError> {
        let error = msg_id(self.raw, sel(b"error\0"));
        if error.is_null() {
            None
        } else {
            Some(MetalError::new(error_message(
                error,
                "Metal command buffer failed",
            )))
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn command_queue(&self) -> CommandQueue {
        let ptr = retain(msg_id(self.raw, sel(b"commandQueue\0")));
        CommandQueue { raw: ptr }
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn retained_references(&self) -> bool {
        msg_bool(self.raw, sel(b"retainedReferences\0")) != 0
    }

    pub fn error_options(&self) -> CommandBufferErrorOption {
        let selector = sel(b"errorOptions\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => CommandBufferErrorOption::EncoderExecutionStatus,
                _ => CommandBufferErrorOption::None,
            }
        } else {
            CommandBufferErrorOption::None
        }
    }

    pub fn kernel_start_time(&self) -> f64 {
        let selector = sel(b"kernelStartTime\0");
        if responds_to_selector(self.raw, selector) {
            msg_f64(self.raw, selector)
        } else {
            0.0
        }
    }

    pub fn kernel_end_time(&self) -> f64 {
        let selector = sel(b"kernelEndTime\0");
        if responds_to_selector(self.raw, selector) {
            msg_f64(self.raw, selector)
        } else {
            0.0
        }
    }

    pub fn gpu_start_time(&self) -> f64 {
        let selector = sel(b"GPUStartTime\0");
        if responds_to_selector(self.raw, selector) {
            msg_f64(self.raw, selector)
        } else {
            0.0
        }
    }

    pub fn gpu_end_time(&self) -> f64 {
        let selector = sel(b"GPUEndTime\0");
        if responds_to_selector(self.raw, selector) {
            msg_f64(self.raw, selector)
        } else {
            0.0
        }
    }

    pub fn logs(&self) -> Option<LogContainer> {
        let selector = sel(b"logs\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(LogContainer { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn use_residency_set(&self, residency_set: &ResidencySet) {
        let selector = sel(b"useResidencySet:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, residency_set.raw);
        }
    }

    pub fn use_residency_sets(&self, residency_sets: &[&ResidencySet]) {
        let selector = sel(b"useResidencySets:count:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_sets: Vec<id> = residency_sets.iter().map(|s| s.raw).collect();
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, raw_sets.as_ptr(), raw_sets.len());
            }
        }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct Library {
    pub raw: id,
}

impl Library {
    pub fn function(&self, name: &str) -> Result<Function, MetalError> {
        let ns_name = NSString::new(name);
        let raw = msg_id_id(self.raw, sel(b"newFunctionWithName:\0"), ns_name.raw());
        if raw.is_null() {
            Err(MetalError::new(format!(
                "failed to load Metal function '{}': not found in library",
                name
            )))
        } else {
            Ok(Function { raw })
        }
    }

    pub fn function_with_constants(
        &self,
        name: &str,
        constants: &FunctionConstantValues,
    ) -> Result<Function, MetalError> {
        unsafe {
            let ns_name = NSString::new(name);
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newFunctionWithName:constantValues:error:\0"),
                ns_name.raw(),
                constants.raw,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    &format!(
                        "failed to specialize Metal function '{}' with constants",
                        name
                    ),
                )))
            } else {
                Ok(Function { raw })
            }
        }
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn library_type(&self) -> Result<LibraryType, MetalError> {
        let selector = sel(b"type\0");
        if responds_to_selector(self.raw, selector) {
            let raw_type = msg_usize(self.raw, selector);
            LibraryType::from_raw(raw_type)
                .ok_or_else(|| MetalError::new(format!("unknown MTLLibraryType: {}", raw_type)))
        } else {
            Err(MetalError::new(
                "type is not supported on this macOS version",
            ))
        }
    }

    pub fn install_name(&self) -> Result<Option<String>, MetalError> {
        let selector = sel(b"installName\0");
        if responds_to_selector(self.raw, selector) {
            Ok(ns_string_to_string(msg_id(self.raw, selector)))
        } else {
            Err(MetalError::new(
                "installName is not supported on this macOS version",
            ))
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct Function {
    pub raw: id,
}

impl Function {
    pub fn name(&self) -> String {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
            .unwrap_or_else(|| "unknown".to_string())
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CompileOptions {
    pub raw: id,
}

impl CompileOptions {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLCompileOptions\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_library_type(&self, library_type: LibraryType) {
        let selector = sel(b"setLibraryType:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, library_type as usize);
        }
    }

    pub fn set_library_type_raw(&self, library_type: usize) {
        let selector = sel(b"setLibraryType:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, library_type);
        }
    }

    pub fn library_type(&self) -> Option<LibraryType> {
        let selector = sel(b"libraryType\0");
        if responds_to_selector(self.raw, selector) {
            let raw_type = msg_usize(self.raw, selector);
            LibraryType::from_raw(raw_type)
        } else {
            None
        }
    }

    pub fn install_name(&self) -> Option<String> {
        let selector = sel(b"installName\0");
        if responds_to_selector(self.raw, selector) {
            ns_string_to_string(msg_id(self.raw, selector))
        } else {
            None
        }
    }

    pub fn set_install_name(&self, name: &str) {
        let selector = sel(b"setInstallName:\0");
        if responds_to_selector(self.raw, selector) {
            let ns_name = NSString::new(name);
            msg_void_id(self.raw, selector, ns_name.raw());
        }
    }

    pub fn optimization_level(&self) -> Option<LibraryOptimizationLevel> {
        let selector = sel(b"optimizationLevel\0");
        if responds_to_selector(self.raw, selector) {
            let raw_val = msg_usize(self.raw, selector);
            LibraryOptimizationLevel::from_raw(raw_val as isize)
        } else {
            None
        }
    }

    pub fn set_optimization_level(&self, level: LibraryOptimizationLevel) {
        let selector = sel(b"setOptimizationLevel:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, level as usize);
        }
    }
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CompileOptions {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CommandQueueDescriptor {
    pub raw: id,
}

impl CommandQueueDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLCommandQueueDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new(
                "MTLCommandQueueDescriptor is not available",
            ));
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        let raw = msg_id(allocated, sel(b"init\0"));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to initialize MTLCommandQueueDescriptor",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn max_command_buffer_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxCommandBufferCount\0"))
    }

    pub fn set_max_command_buffer_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxCommandBufferCount:\0"), count);
    }

    pub fn log_state(&self) -> Option<LogState> {
        let selector = sel(b"logState\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(LogState { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn set_log_state(&self, state: &LogState) {
        let selector = sel(b"setLogState:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, state.raw);
        }
    }
}

impl Clone for CommandQueueDescriptor {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl Drop for CommandQueueDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CommandBufferDescriptor {
    pub raw: id,
}

impl CommandBufferDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLCommandBufferDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new(
                "MTLCommandBufferDescriptor is not available",
            ));
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        let raw = msg_id(allocated, sel(b"init\0"));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to initialize MTLCommandBufferDescriptor",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn retained_references(&self) -> bool {
        msg_bool(self.raw, sel(b"retainedReferences\0")) != 0
    }

    pub fn set_retained_references(&self, retained: bool) {
        msg_void_bool(self.raw, sel(b"setRetainedReferences:\0"), retained as BOOL);
    }

    pub fn error_options(&self) -> CommandBufferErrorOption {
        let selector = sel(b"errorOptions\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => CommandBufferErrorOption::EncoderExecutionStatus,
                _ => CommandBufferErrorOption::None,
            }
        } else {
            CommandBufferErrorOption::None
        }
    }

    pub fn set_error_options(&self, options: CommandBufferErrorOption) {
        let selector = sel(b"setErrorOptions:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, options as usize);
        }
    }

    pub fn log_state(&self) -> Option<LogState> {
        let selector = sel(b"logState\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(LogState { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn set_log_state(&self, state: &LogState) {
        let selector = sel(b"setLogState:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, state.raw);
        }
    }
}

impl Clone for CommandBufferDescriptor {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl Drop for CommandBufferDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct LogContainer {
    pub raw: id,
}

impl LogContainer {
    pub fn len(&self) -> usize {
        if self.raw.is_null() {
            0
        } else {
            msg_usize(self.raw, sel(b"count\0"))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn log_at(&self, index: usize) -> Option<FunctionLog> {
        if self.raw.is_null() || index >= self.len() {
            None
        } else {
            let item = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
            if item.is_null() {
                None
            } else {
                Some(FunctionLog { raw: retain(item) })
            }
        }
    }

    pub fn to_vec(&self) -> Vec<FunctionLog> {
        let count = self.len();
        let mut vec = Vec::with_capacity(count);
        for i in 0..count {
            if let Some(log) = self.log_at(i) {
                vec.push(log);
            }
        }
        vec
    }
}

impl Drop for LogContainer {
    fn drop(&mut self) {
        release(self.raw);
    }
}
