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

    pub fn commit(&self) {
        msg_void(self.raw, sel(b"commit\0"));
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
            match raw_type {
                0 => Ok(LibraryType::Executable),
                1 => Ok(LibraryType::Dynamic),
                _ => Err(MetalError::new(format!(
                    "unknown MTLLibraryType: {}",
                    raw_type
                ))),
            }
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
            match raw_type {
                0 => Some(LibraryType::Executable),
                1 => Some(LibraryType::Dynamic),
                _ => None,
            }
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
            match raw_val as isize {
                0 => Some(LibraryOptimizationLevel::Default),
                1 => Some(LibraryOptimizationLevel::Size),
                _ => None,
            }
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
