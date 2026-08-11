use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Mutability {
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}

impl Mutability {
    fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            1 => Some(Self::Mutable),
            2 => Some(Self::Immutable),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum ShaderValidation {
    Default = 0,
    Enabled = 1,
    Disabled = 2,
}

impl ShaderValidation {
    fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            1 => Some(Self::Enabled),
            2 => Some(Self::Disabled),
            _ => None,
        }
    }
}

fn functions_from_array(array: id) -> NSArrayIterator<Function> {
    NSArrayIterator::new(array)
}

fn pipeline_label(raw: id) -> Option<NSString> {
    if raw.is_null() {
        return None;
    }
    let ptr = msg_id(raw, sel(b"label\0"));
    if ptr.is_null() {
        None
    } else {
        Some(NSString::from_raw(ptr))
    }
}

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
        if let Some(expected_size) = data_type.size() {
            assert!(
                bytes.len() >= expected_size,
                "set_bytes: byte slice length ({}) is less than the expected size for {:?} ({})",
                bytes.len(),
                data_type,
                expected_size
            );
        }
        self.set_raw(index, data_type, bytes.as_ptr() as *const c_void);
    }

    pub fn set_constant_values_in_range(&self, data_type: DataType, bytes: &[u8], range: Range) {
        if let Some(expected_size) = data_type.size() {
            let total_expected_size = expected_size * range.length;
            assert!(
                bytes.len() >= total_expected_size,
                "set_constant_values_in_range: byte slice length ({}) is less than the expected size for {} elements of {:?} ({})",
                bytes.len(),
                range.length,
                data_type,
                total_expected_size
            );
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setConstantValues:type:withRange:\0"),
                bytes.as_ptr() as *const c_void,
                data_type as usize,
                range,
            );
        }
    }

    pub fn set_constant_value_with_name(&self, name: &str, data_type: DataType, bytes: &[u8]) {
        if let Some(expected_size) = data_type.size() {
            assert!(
                bytes.len() >= expected_size,
                "set_constant_value_with_name: byte slice length ({}) is less than the expected size for {:?} ({})",
                bytes.len(),
                data_type,
                expected_size
            );
        }
        let ns_name = NSString::new(name);
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, id) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setConstantValue:type:withName:\0"),
                bytes.as_ptr() as *const c_void,
                data_type as usize,
                ns_name.raw(),
            );
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
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

    pub fn url(&self) -> Option<NSString> {
        let url = msg_id(self.raw, sel(b"url\0"));
        ns_url_to_path(url)
    }

    pub fn set_url(&self, path: Option<&str>) {
        let url = path.map(ns_url_from_path).unwrap_or(NIL);
        msg_void_id(self.raw, sel(b"setUrl:\0"), url);
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
#[repr(transparent)]
pub struct BinaryArchive {
    pub raw: id,
}

impl Clone for BinaryArchive {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for BinaryArchive {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl BinaryArchive {
    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
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
                    "failed to serialize Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }

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

    pub fn add_tile_render_pipeline_functions(
        &self,
        descriptor: &TileRenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let selector = sel(b"addTileRenderPipelineFunctionsWithDescriptor:error:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "addTileRenderPipelineFunctionsWithDescriptor:error: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(self.raw, selector, descriptor.raw, &mut error);
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add tile render pipeline functions to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_mesh_render_pipeline_functions(
        &self,
        descriptor: &MeshRenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let selector = sel(b"addMeshRenderPipelineFunctionsWithDescriptor:error:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "addMeshRenderPipelineFunctionsWithDescriptor:error: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(self.raw, selector, descriptor.raw, &mut error);
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add mesh render pipeline functions to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_library_with_descriptor(
        &self,
        descriptor: &StitchedLibraryDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let selector = sel(b"addLibraryWithDescriptor:error:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "addLibraryWithDescriptor:error: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(self.raw, selector, descriptor.raw, &mut error);
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add stitched library to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_function_with_descriptor(
        &self,
        descriptor: &FunctionDescriptor,
        library: &Library,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let selector = sel(b"addFunctionWithDescriptor:library:error:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "addFunctionWithDescriptor:library:error: not supported",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(self.raw, selector, descriptor.raw, library.raw, &mut error);
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add function to Metal binary archive",
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
pub struct PipelineBufferDescriptor {
    pub raw: id,
}

impl PipelineBufferDescriptor {
    fn borrowed(raw: id) -> Self {
        Self { raw }
    }

    pub fn mutability(&self) -> Mutability {
        Mutability::from_raw(msg_usize(self.raw, sel(b"mutability\0")))
            .unwrap_or(Mutability::Default)
    }

    pub fn set_mutability(&self, mutability: Mutability) {
        msg_void_usize(self.raw, sel(b"setMutability:\0"), mutability as usize);
    }
}

#[derive(Debug)]
pub struct PipelineBufferDescriptorArray {
    pub raw: id,
}

impl PipelineBufferDescriptorArray {
    pub fn object_at_index(&self, index: usize) -> PipelineBufferDescriptor {
        let ptr = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        PipelineBufferDescriptor::borrowed(ptr)
    }
}

#[derive(Debug)]
pub struct ComputePipelineReflection {
    pub raw: id,
}

impl ComputePipelineReflection {
    pub fn bindings(&self) -> NSArrayIterator<Binding> {
        let array = msg_id(self.raw, sel(b"bindings\0"));
        NSArrayIterator::new(array)
    }
}

impl Drop for ComputePipelineReflection {
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

    pub fn set_binary_archives(&self, archives: &[BinaryArchive]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(archives.as_ptr() as *const id, archives.len()) };
        let array = ns_array_from_ids(raw_ptrs);
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

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn compute_function(&self) -> Option<Function> {
        let f = msg_id(self.raw, sel(b"computeFunction\0"));
        (!f.is_null()).then(|| Function { raw: retain(f) })
    }

    pub fn reset(&self) {
        let selector = sel(b"reset\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
        }
    }

    pub fn stage_input_output_descriptor(&self) -> Option<StageInputOutputDescriptor> {
        let selector = sel(b"stageInputOutputDescriptor\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            (!ptr.is_null()).then(|| StageInputOutputDescriptor { raw: retain(ptr) })
        } else {
            None
        }
    }

    pub fn set_stage_input_output_descriptor(
        &self,
        descriptor: Option<&StageInputOutputDescriptor>,
    ) {
        let selector = sel(b"setStageInputOutputDescriptor:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(
                self.raw,
                selector,
                descriptor.map_or(NIL, |descriptor| descriptor.raw),
            );
        }
    }

    pub fn buffers(&self) -> PipelineBufferDescriptorArray {
        PipelineBufferDescriptorArray {
            raw: msg_id(self.raw, sel(b"buffers\0")),
        }
    }

    pub fn preloaded_libraries(&self) -> Result<NSArrayIterator<DynamicLibrary>, MetalError> {
        let selector = sel(b"preloadedLibraries\0");
        if responds_to_selector(self.raw, selector) {
            let array = msg_id(self.raw, selector);
            Ok(NSArrayIterator::new(array))
        } else {
            Err(MetalError::new(
                "preloadedLibraries not supported on ComputePipelineDescriptor",
            ))
        }
    }

    pub fn set_preloaded_libraries(&self, libraries: &[DynamicLibrary]) -> Result<(), MetalError> {
        let selector = sel(b"setPreloadedLibraries:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_ptrs = unsafe {
                std::slice::from_raw_parts(libraries.as_ptr() as *const id, libraries.len())
            };
            let array = ns_array_from_ids(raw_ptrs);
            msg_void_id(self.raw, selector, array);
            Ok(())
        } else {
            Err(MetalError::new(
                "setPreloadedLibraries: not supported on ComputePipelineDescriptor",
            ))
        }
    }

    pub fn binary_archives(&self) -> Result<NSArrayIterator<BinaryArchive>, MetalError> {
        let selector = sel(b"binaryArchives\0");
        if responds_to_selector(self.raw, selector) {
            let array = msg_id(self.raw, selector);
            Ok(NSArrayIterator::new(array))
        } else {
            Err(MetalError::new(
                "binaryArchives not supported on ComputePipelineDescriptor",
            ))
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

    pub fn shader_validation(&self) -> ShaderValidation {
        let selector = sel(b"shaderValidation\0");
        if responds_to_selector(self.raw, selector) {
            ShaderValidation::from_raw(msg_usize(self.raw, selector) as isize)
                .unwrap_or(ShaderValidation::Default)
        } else {
            ShaderValidation::Default
        }
    }

    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        let selector = sel(b"setShaderValidation:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, validation as usize);
        }
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
#[repr(transparent)]
pub struct ComputePipelineState {
    pub raw: id,
}

impl Clone for ComputePipelineState {
    fn clone(&self) -> Self {
        Self { raw: retain(self.raw) }
    }
}

impl Drop for ComputePipelineState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl ComputePipelineState {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn imageblock_memory_length_for_dimensions(&self, dimensions: Size) -> usize {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Size) -> usize =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"imageblockMemoryLengthForDimensions:\0"),
                dimensions,
            )
        }
    }

    pub fn reflection(&self) -> Option<ComputePipelineReflection> {
        let selector = sel(b"reflection\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            (!ptr.is_null()).then(|| ComputePipelineReflection { raw: retain(ptr) })
        } else {
            None
        }
    }

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
pub struct VertexAttributeDescriptor {
    pub raw: id,
}

impl VertexAttributeDescriptor {
    fn borrowed(raw: id) -> Self {
        Self { raw }
    }

    pub fn format(&self) -> VertexFormat {
        let val = msg_usize(self.raw, sel(b"format\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn offset(&self) -> usize {
        msg_usize(self.raw, sel(b"offset\0"))
    }

    pub fn buffer_index(&self) -> usize {
        msg_usize(self.raw, sel(b"bufferIndex\0"))
    }
}

#[derive(Debug)]
pub struct VertexAttributeDescriptorArray {
    pub raw: id,
}

impl VertexAttributeDescriptorArray {
    pub fn object_at_index(&self, index: usize) -> VertexAttributeDescriptor {
        let ptr = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        VertexAttributeDescriptor::borrowed(ptr)
    }
}

#[derive(Debug)]
pub struct VertexBufferLayoutDescriptor {
    pub raw: id,
}

impl VertexBufferLayoutDescriptor {
    fn borrowed(raw: id) -> Self {
        Self { raw }
    }

    pub fn stride(&self) -> usize {
        msg_usize(self.raw, sel(b"stride\0"))
    }

    pub fn step_function(&self) -> VertexStepFunction {
        let val = msg_usize(self.raw, sel(b"stepFunction\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn step_rate(&self) -> usize {
        msg_usize(self.raw, sel(b"stepRate\0"))
    }
}

#[derive(Debug)]
pub struct VertexBufferLayoutDescriptorArray {
    pub raw: id,
}

impl VertexBufferLayoutDescriptorArray {
    pub fn object_at_index(&self, index: usize) -> VertexBufferLayoutDescriptor {
        let ptr = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        VertexBufferLayoutDescriptor::borrowed(ptr)
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

    pub fn attributes(&self) -> VertexAttributeDescriptorArray {
        VertexAttributeDescriptorArray {
            raw: msg_id(self.raw, sel(b"attributes\0")),
        }
    }

    pub fn layouts(&self) -> VertexBufferLayoutDescriptorArray {
        VertexBufferLayoutDescriptorArray {
            raw: msg_id(self.raw, sel(b"layouts\0")),
        }
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
pub struct StageInputOutputDescriptor {
    pub raw: id,
}

impl StageInputOutputDescriptor {
    pub fn new() -> Self {
        let raw = retain(msg_id(
            class(b"MTLStageInputOutputDescriptor\0"),
            sel(b"stageInputOutputDescriptor\0"),
        ));
        Self { raw }
    }

    pub fn set_attribute(
        &self,
        index: usize,
        format: AttributeFormat,
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
        step_function: StepFunction,
        step_rate: usize,
    ) {
        let layouts = msg_id(self.raw, sel(b"layouts\0"));
        let layout = msg_id_usize(layouts, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(layout, sel(b"setStride:\0"), stride);
        msg_void_usize(layout, sel(b"setStepFunction:\0"), step_function as usize);
        msg_void_usize(layout, sel(b"setStepRate:\0"), step_rate);
    }

    pub fn index_type(&self) -> IndexType {
        let val = msg_usize(self.raw, sel(b"indexType\0"));
        match val {
            1 => IndexType::UInt32,
            _ => IndexType::UInt16,
        }
    }

    pub fn set_index_type(&self, index_type: IndexType) {
        msg_void_usize(self.raw, sel(b"setIndexType:\0"), index_type as usize);
    }

    pub fn index_buffer_index(&self) -> usize {
        msg_usize(self.raw, sel(b"indexBufferIndex\0"))
    }

    pub fn set_index_buffer_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setIndexBufferIndex:\0"), index);
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for StageInputOutputDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StageInputOutputDescriptor {
    fn clone(&self) -> Self {
        Self {
            raw: retain(msg_id(self.raw, sel(b"copy\0"))),
        }
    }
}

impl Drop for StageInputOutputDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct RenderPipelineColorAttachmentDescriptor {
    pub raw: id,
}

impl RenderPipelineColorAttachmentDescriptor {
    fn borrowed(raw: id) -> Self {
        Self { raw }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_raw(msg_usize(self.raw, sel(b"pixelFormat\0")))
    }

    pub fn blending_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isBlendingEnabled\0")) != NO
    }

    pub fn source_rgb_blend_factor(&self) -> BlendFactor {
        let val = msg_usize(self.raw, sel(b"sourceRGBBlendFactor\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn destination_rgb_blend_factor(&self) -> BlendFactor {
        let val = msg_usize(self.raw, sel(b"destinationRGBBlendFactor\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn rgb_blend_operation(&self) -> BlendOperation {
        let val = msg_usize(self.raw, sel(b"rgbBlendOperation\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn source_alpha_blend_factor(&self) -> BlendFactor {
        let val = msg_usize(self.raw, sel(b"sourceAlphaBlendFactor\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn destination_alpha_blend_factor(&self) -> BlendFactor {
        let val = msg_usize(self.raw, sel(b"destinationAlphaBlendFactor\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn alpha_blend_operation(&self) -> BlendOperation {
        let val = msg_usize(self.raw, sel(b"alphaBlendOperation\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn write_mask(&self) -> ColorWriteMask {
        let raw = msg_usize(self.raw, sel(b"writeMask\0"));
        unsafe { std::mem::transmute(raw) }
    }
}

#[derive(Debug)]
pub struct RenderPipelineColorAttachmentDescriptorArray {
    pub raw: id,
}

impl RenderPipelineColorAttachmentDescriptorArray {
    pub fn object_at_index(&self, index: usize) -> RenderPipelineColorAttachmentDescriptor {
        let ptr = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        RenderPipelineColorAttachmentDescriptor::borrowed(ptr)
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

    pub fn set_binary_archives(&self, archives: &[BinaryArchive]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(archives.as_ptr() as *const id, archives.len()) };
        let array = ns_array_from_ids(raw_ptrs);
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

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn vertex_function(&self) -> Option<Function> {
        let f = msg_id(self.raw, sel(b"vertexFunction\0"));
        (!f.is_null()).then(|| Function { raw: retain(f) })
    }

    pub fn fragment_function(&self) -> Option<Function> {
        let f = msg_id(self.raw, sel(b"fragmentFunction\0"));
        (!f.is_null()).then(|| Function { raw: retain(f) })
    }

    pub fn vertex_descriptor(&self) -> Option<VertexDescriptor> {
        let ptr = msg_id(self.raw, sel(b"vertexDescriptor\0"));
        (!ptr.is_null()).then(|| VertexDescriptor { raw: retain(ptr) })
    }

    pub fn sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"sampleCount\0"))
    }

    pub fn raster_sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"rasterSampleCount\0"))
    }

    pub fn alpha_to_coverage_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isAlphaToCoverageEnabled\0")) != NO
    }

    pub fn alpha_to_one_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isAlphaToOneEnabled\0")) != NO
    }

    pub fn rasterization_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isRasterizationEnabled\0")) != NO
    }

    pub fn support_indirect_command_buffers(&self) -> bool {
        let selector = sel(b"supportIndirectCommandBuffers\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn reset(&self) {
        let selector = sel(b"reset\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
        }
    }

    pub fn color_attachments(&self) -> RenderPipelineColorAttachmentDescriptorArray {
        RenderPipelineColorAttachmentDescriptorArray {
            raw: msg_id(self.raw, sel(b"colorAttachments\0")),
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
pub struct TileRenderPipelineDescriptor {
    pub raw: id,
}

impl TileRenderPipelineDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLTileRenderPipelineDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_tile_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setTileFunction:\0"), function.raw);
    }
}

impl Default for TileRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TileRenderPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct MeshRenderPipelineDescriptor {
    pub raw: id,
}

impl MeshRenderPipelineDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLMeshRenderPipelineDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_mesh_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setMeshFunction:\0"), function.raw);
    }

    pub fn set_object_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setObjectFunction:\0"), function.raw);
    }

    pub fn set_fragment_function(&self, function: &Function) {
        msg_void_id(self.raw, sel(b"setFragmentFunction:\0"), function.raw);
    }

    pub fn set_color_attachment_pixel_format(&self, index: usize, pixel_format: PixelFormat) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(attachment, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
    }

    pub fn set_color_attachment_write_mask(&self, index: usize, mask: ColorWriteMask) {
        let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
        let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
        msg_void_usize(attachment, sel(b"setWriteMask:\0"), mask.as_raw());
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

    pub fn set_raster_sample_count(&self, sample_count: usize) {
        msg_void_usize(self.raw, sel(b"setRasterSampleCount:\0"), sample_count);
    }

    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setAlphaToCoverageEnabled:\0"),
            if enabled { YES } else { NO },
        );
    }

    pub fn set_max_total_threads_per_object_threadgroup(&self, threads: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMaxTotalThreadsPerObjectThreadgroup:\0"),
            threads,
        );
    }

    pub fn set_max_total_threads_per_mesh_threadgroup(&self, threads: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMaxTotalThreadsPerMeshThreadgroup:\0"),
            threads,
        );
    }
}

impl Default for MeshRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MeshRenderPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RenderPipelineState {
    pub raw: id,
}

impl Clone for RenderPipelineState {
    fn clone(&self) -> Self {
        Self { raw: retain(self.raw) }
    }
}

impl Drop for RenderPipelineState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl RenderPipelineState {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn imageblock_sample_length(&self) -> usize {
        msg_usize(self.raw, sel(b"imageblockSampleLength\0"))
    }

    pub fn support_indirect_command_buffers(&self) -> bool {
        let selector = sel(b"supportIndirectCommandBuffers\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

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
    owned: bool,
}

impl StencilDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLStencilDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
            owned: true,
        }
    }

    fn borrowed(raw: id) -> Self {
        Self { raw, owned: false }
    }

    pub fn stencil_compare_function(&self) -> CompareFunction {
        let val = msg_usize(self.raw, sel(b"stencilCompareFunction\0"));
        match val {
            0 => CompareFunction::Never,
            1 => CompareFunction::Less,
            2 => CompareFunction::Equal,
            3 => CompareFunction::LessEqual,
            4 => CompareFunction::Greater,
            5 => CompareFunction::NotEqual,
            6 => CompareFunction::GreaterEqual,
            7 => CompareFunction::Always,
            _ => CompareFunction::Always,
        }
    }

    pub fn set_stencil_compare_function(&self, compare_function: CompareFunction) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilCompareFunction:\0"),
            compare_function as usize,
        );
    }

    pub fn stencil_failure_operation(&self) -> StencilOperation {
        let val = msg_usize(self.raw, sel(b"stencilFailureOperation\0"));
        match val {
            0 => StencilOperation::Keep,
            1 => StencilOperation::Zero,
            2 => StencilOperation::Replace,
            3 => StencilOperation::IncrementClamp,
            4 => StencilOperation::DecrementClamp,
            5 => StencilOperation::Invert,
            6 => StencilOperation::IncrementWrap,
            7 => StencilOperation::DecrementWrap,
            _ => StencilOperation::Keep,
        }
    }

    pub fn set_stencil_failure_operation(&self, operation: StencilOperation) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilFailureOperation:\0"),
            operation as usize,
        );
    }

    pub fn depth_failure_operation(&self) -> StencilOperation {
        let val = msg_usize(self.raw, sel(b"depthFailureOperation\0"));
        match val {
            0 => StencilOperation::Keep,
            1 => StencilOperation::Zero,
            2 => StencilOperation::Replace,
            3 => StencilOperation::IncrementClamp,
            4 => StencilOperation::DecrementClamp,
            5 => StencilOperation::Invert,
            6 => StencilOperation::IncrementWrap,
            7 => StencilOperation::DecrementWrap,
            _ => StencilOperation::Keep,
        }
    }

    pub fn set_depth_failure_operation(&self, operation: StencilOperation) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthFailureOperation:\0"),
            operation as usize,
        );
    }

    pub fn depth_stencil_pass_operation(&self) -> StencilOperation {
        let val = msg_usize(self.raw, sel(b"depthStencilPassOperation\0"));
        match val {
            0 => StencilOperation::Keep,
            1 => StencilOperation::Zero,
            2 => StencilOperation::Replace,
            3 => StencilOperation::IncrementClamp,
            4 => StencilOperation::DecrementClamp,
            5 => StencilOperation::Invert,
            6 => StencilOperation::IncrementWrap,
            7 => StencilOperation::DecrementWrap,
            _ => StencilOperation::Keep,
        }
    }

    pub fn set_depth_stencil_pass_operation(&self, operation: StencilOperation) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthStencilPassOperation:\0"),
            operation as usize,
        );
    }

    pub fn read_mask(&self) -> u32 {
        msg_usize(self.raw, sel(b"readMask\0")) as u32
    }

    pub fn set_read_mask(&self, mask: u32) {
        msg_void_usize(self.raw, sel(b"setReadMask:\0"), mask as usize);
    }

    pub fn write_mask(&self) -> u32 {
        msg_usize(self.raw, sel(b"writeMask\0")) as u32
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

    pub fn depth_compare_function(&self) -> CompareFunction {
        let val = msg_usize(self.raw, sel(b"depthCompareFunction\0"));
        match val {
            0 => CompareFunction::Never,
            1 => CompareFunction::Less,
            2 => CompareFunction::Equal,
            3 => CompareFunction::LessEqual,
            4 => CompareFunction::Greater,
            5 => CompareFunction::NotEqual,
            6 => CompareFunction::GreaterEqual,
            7 => CompareFunction::Always,
            _ => CompareFunction::Always,
        }
    }

    pub fn set_depth_compare_function(&self, compare_function: CompareFunction) {
        msg_void_usize(
            self.raw,
            sel(b"setDepthCompareFunction:\0"),
            compare_function as usize,
        );
    }

    pub fn is_depth_write_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isDepthWriteEnabled\0")) == YES
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

    pub fn set_front_face_stencil(&self, stencil: &StencilDescriptor) {
        msg_void_id(self.raw, sel(b"setFrontFaceStencil:\0"), stencil.raw);
    }

    pub fn set_back_face_stencil(&self, stencil: &StencilDescriptor) {
        msg_void_id(self.raw, sel(b"setBackFaceStencil:\0"), stencil.raw);
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for StencilDescriptor {
    fn drop(&mut self) {
        if self.owned {
            release(self.raw);
        }
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

impl DepthStencilState {
    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on DepthStencilState",
            ))
        }
    }
}

impl Clone for DepthStencilState {
    fn clone(&self) -> Self {
        Self { raw: retain(self.raw) }
    }
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

    pub fn min_filter(&self) -> SamplerMinMagFilter {
        let val = msg_usize(self.raw, sel(b"minFilter\0"));
        match val {
            1 => SamplerMinMagFilter::Linear,
            _ => SamplerMinMagFilter::Nearest,
        }
    }

    pub fn set_min_filter(&self, filter: SamplerMinMagFilter) {
        msg_void_usize(self.raw, sel(b"setMinFilter:\0"), filter as usize);
    }

    pub fn mag_filter(&self) -> SamplerMinMagFilter {
        let val = msg_usize(self.raw, sel(b"magFilter\0"));
        match val {
            1 => SamplerMinMagFilter::Linear,
            _ => SamplerMinMagFilter::Nearest,
        }
    }

    pub fn set_mag_filter(&self, filter: SamplerMinMagFilter) {
        msg_void_usize(self.raw, sel(b"setMagFilter:\0"), filter as usize);
    }

    pub fn mip_filter(&self) -> SamplerMipFilter {
        let val = msg_usize(self.raw, sel(b"mipFilter\0"));
        match val {
            1 => SamplerMipFilter::Nearest,
            2 => SamplerMipFilter::Linear,
            _ => SamplerMipFilter::NotMipmapped,
        }
    }

    pub fn set_mip_filter(&self, filter: SamplerMipFilter) {
        msg_void_usize(self.raw, sel(b"setMipFilter:\0"), filter as usize);
    }

    pub fn set_address_mode(&self, mode: SamplerAddressMode) {
        self.set_s_address_mode(mode);
        self.set_t_address_mode(mode);
        self.set_r_address_mode(mode);
    }

    pub fn s_address_mode(&self) -> SamplerAddressMode {
        let val = msg_usize(self.raw, sel(b"sAddressMode\0"));
        match val {
            0 => SamplerAddressMode::ClampToEdge,
            1 => SamplerAddressMode::MirrorClampToEdge,
            2 => SamplerAddressMode::Repeat,
            3 => SamplerAddressMode::MirrorRepeat,
            4 => SamplerAddressMode::ClampToZero,
            5 => SamplerAddressMode::ClampToBorderColor,
            _ => SamplerAddressMode::ClampToEdge,
        }
    }

    pub fn set_s_address_mode(&self, mode: SamplerAddressMode) {
        msg_void_usize(self.raw, sel(b"setSAddressMode:\0"), mode as usize);
    }

    pub fn t_address_mode(&self) -> SamplerAddressMode {
        let val = msg_usize(self.raw, sel(b"tAddressMode\0"));
        match val {
            0 => SamplerAddressMode::ClampToEdge,
            1 => SamplerAddressMode::MirrorClampToEdge,
            2 => SamplerAddressMode::Repeat,
            3 => SamplerAddressMode::MirrorRepeat,
            4 => SamplerAddressMode::ClampToZero,
            5 => SamplerAddressMode::ClampToBorderColor,
            _ => SamplerAddressMode::ClampToEdge,
        }
    }

    pub fn set_t_address_mode(&self, mode: SamplerAddressMode) {
        msg_void_usize(self.raw, sel(b"setTAddressMode:\0"), mode as usize);
    }

    pub fn r_address_mode(&self) -> SamplerAddressMode {
        let val = msg_usize(self.raw, sel(b"rAddressMode\0"));
        match val {
            0 => SamplerAddressMode::ClampToEdge,
            1 => SamplerAddressMode::MirrorClampToEdge,
            2 => SamplerAddressMode::Repeat,
            3 => SamplerAddressMode::MirrorRepeat,
            4 => SamplerAddressMode::ClampToZero,
            5 => SamplerAddressMode::ClampToBorderColor,
            _ => SamplerAddressMode::ClampToEdge,
        }
    }

    pub fn set_r_address_mode(&self, mode: SamplerAddressMode) {
        msg_void_usize(self.raw, sel(b"setRAddressMode:\0"), mode as usize);
    }

    pub fn border_color(&self) -> SamplerBorderColor {
        let selector = sel(b"borderColor\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => SamplerBorderColor::OpaqueBlack,
                2 => SamplerBorderColor::OpaqueWhite,
                _ => SamplerBorderColor::TransparentBlack,
            }
        } else {
            SamplerBorderColor::TransparentBlack
        }
    }

    pub fn set_border_color(&self, border_color: SamplerBorderColor) {
        let selector = sel(b"setBorderColor:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, border_color as usize);
        }
    }

    pub fn reduction_mode(&self) -> SamplerReductionMode {
        let selector = sel(b"reductionMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => SamplerReductionMode::Minimum,
                2 => SamplerReductionMode::Maximum,
                _ => SamplerReductionMode::WeightedAverage,
            }
        } else {
            SamplerReductionMode::WeightedAverage
        }
    }

    pub fn set_reduction_mode(&self, reduction_mode: SamplerReductionMode) {
        let selector = sel(b"setReductionMode:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, reduction_mode as usize);
        }
    }

    pub fn normalized_coordinates(&self) -> bool {
        msg_bool(self.raw, sel(b"normalizedCoordinates\0")) != 0
    }

    pub fn set_normalized_coordinates(&self, normalized_coordinates: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setNormalizedCoordinates:\0"),
            if normalized_coordinates { YES } else { NO },
        );
    }

    pub fn lod_min_clamp(&self) -> f32 {
        msg_f32(self.raw, sel(b"lodMinClamp\0"))
    }

    pub fn set_lod_min_clamp(&self, value: f32) {
        msg_void_f32(self.raw, sel(b"setLodMinClamp:\0"), value);
    }

    pub fn lod_max_clamp(&self) -> f32 {
        msg_f32(self.raw, sel(b"lodMaxClamp\0"))
    }

    pub fn set_lod_max_clamp(&self, value: f32) {
        msg_void_f32(self.raw, sel(b"setLodMaxClamp:\0"), value);
    }

    pub fn lod_average(&self) -> bool {
        let selector = sel(b"lodAverage\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn set_lod_average(&self, lod_average: bool) {
        let selector = sel(b"setLodAverage:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if lod_average { YES } else { NO });
        }
    }

    pub fn lod_bias(&self) -> f32 {
        let selector = sel(b"lodBias\0");
        if responds_to_selector(self.raw, selector) {
            msg_f32(self.raw, selector)
        } else {
            0.0
        }
    }

    pub fn set_lod_bias(&self, bias: f32) {
        let selector = sel(b"setLodBias:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_f32(self.raw, selector, bias);
        }
    }

    pub fn compare_function(&self) -> CompareFunction {
        let val = msg_usize(self.raw, sel(b"compareFunction\0"));
        match val {
            0 => CompareFunction::Never,
            1 => CompareFunction::Less,
            2 => CompareFunction::Equal,
            3 => CompareFunction::LessEqual,
            4 => CompareFunction::Greater,
            5 => CompareFunction::NotEqual,
            6 => CompareFunction::GreaterEqual,
            7 => CompareFunction::Always,
            _ => CompareFunction::Never,
        }
    }

    pub fn set_compare_function(&self, compare_function: CompareFunction) {
        msg_void_usize(
            self.raw,
            sel(b"setCompareFunction:\0"),
            compare_function as usize,
        );
    }

    pub fn max_anisotropy(&self) -> usize {
        msg_usize(self.raw, sel(b"maxAnisotropy\0"))
    }

    pub fn set_max_anisotropy(&self, max_anisotropy: usize) {
        msg_void_usize(self.raw, sel(b"setMaxAnisotropy:\0"), max_anisotropy);
    }

    pub fn support_argument_buffers(&self) -> bool {
        let selector = sel(b"supportArgumentBuffers\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn set_support_argument_buffers(&self, support: bool) {
        let selector = sel(b"setSupportArgumentBuffers:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_bool(self.raw, selector, if support { YES } else { NO });
        }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
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
#[repr(transparent)]
pub struct SamplerState {
    pub raw: id,
}

impl Drop for SamplerState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl SamplerState {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
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

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }
}

#[derive(Debug)]
pub struct Fence {
    pub raw: id,
}

impl Fence {
    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
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

    pub fn function_descriptor() -> Self {
        Self {
            raw: retain(msg_id(
                class(b"MTLFunctionDescriptor\0"),
                sel(b"functionDescriptor\0"),
            )),
        }
    }

    pub fn name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"name\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn set_name(&self, name: &str) {
        let ns_name = NSString::new(name);
        msg_void_id(self.raw, sel(b"setName:\0"), ns_name.raw());
    }

    pub fn specialized_name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"specializedName\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
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

    pub fn set_binary_archives(&self, archives: &[BinaryArchive]) -> Result<(), MetalError> {
        let selector = sel(b"setBinaryArchives:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_ptrs = unsafe {
                std::slice::from_raw_parts(archives.as_ptr() as *const id, archives.len())
            };
            let array = ns_array_from_ids(raw_ptrs);
            msg_void_id(self.raw, selector, array);
            Ok(())
        } else {
            Err(MetalError::new(
                "setBinaryArchives: not supported on FunctionDescriptor",
            ))
        }
    }

    pub fn binary_archives(&self) -> Result<NSArrayIterator<BinaryArchive>, MetalError> {
        let selector = sel(b"binaryArchives\0");
        if responds_to_selector(self.raw, selector) {
            let array = msg_id(self.raw, selector);
            Ok(NSArrayIterator::new(array))
        } else {
            Err(MetalError::new(
                "binaryArchives not supported on FunctionDescriptor",
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

    pub fn set_functions(&self, functions: &[Function]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(functions.as_ptr() as *const id, functions.len()) };
        let array = ns_array_from_ids(raw_ptrs);
        msg_void_id(self.raw, sel(b"setFunctions:\0"), array);
    }

    pub fn set_binary_functions(&self, functions: &[Function]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(functions.as_ptr() as *const id, functions.len()) };
        let array = ns_array_from_ids(raw_ptrs);
        msg_void_id(self.raw, sel(b"setBinaryFunctions:\0"), array);
    }

    pub fn set_private_functions(&self, functions: &[Function]) {
        let selector = sel(b"setPrivateFunctions:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_ptrs = unsafe {
                std::slice::from_raw_parts(functions.as_ptr() as *const id, functions.len())
            };
            let array = ns_array_from_ids(raw_ptrs);
            msg_void_id(self.raw, selector, array);
        }
    }

    pub fn functions(&self) -> NSArrayIterator<Function> {
        functions_from_array(msg_id(self.raw, sel(b"functions\0")))
    }

    pub fn private_functions(&self) -> NSArrayIterator<Function> {
        let selector = sel(b"privateFunctions\0");
        if responds_to_selector(self.raw, selector) {
            functions_from_array(msg_id(self.raw, selector))
        } else {
            NSArrayIterator::new(NIL)
        }
    }

    pub fn groups(&self) -> DynamicLibraryGroupIterator {
        let dict = msg_id(self.raw, sel(b"groups\0"));
        let keys = if dict.is_null() {
            NIL
        } else {
            msg_id(dict, sel(b"allKeys\0"))
        };
        DynamicLibraryGroupIterator {
            dict: retain(dict),
            keys_iter: NSArrayIterator::new(keys),
        }
    }
}

pub struct DynamicLibraryGroupIterator {
    dict: id,
    keys_iter: NSArrayIterator<NSString>,
}

impl Drop for DynamicLibraryGroupIterator {
    fn drop(&mut self) {
        release(self.dict);
    }
}

impl Iterator for DynamicLibraryGroupIterator {
    type Item = (NSString, NSArrayIterator<Function>);

    fn next(&mut self) -> Option<Self::Item> {
        self.keys_iter.next().map(|name| {
            let array = msg_id_id(self.dict, sel(b"objectForKey:\0"), name.raw());
            (name, NSArrayIterator::new(array))
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.keys_iter.size_hint()
    }
}

impl ExactSizeIterator for DynamicLibraryGroupIterator {}

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
#[repr(transparent)]
pub struct DynamicLibrary {
    pub raw: id,
}

impl Clone for DynamicLibrary {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for DynamicLibrary {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl DynamicLibrary {
    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn install_name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"installName\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
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
#[repr(transparent)]
pub struct FunctionHandle {
    pub raw: id,
}

impl FunctionHandle {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    pub fn name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"name\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn function_type(&self) -> FunctionType {
        let val = msg_usize(self.raw, sel(b"functionType\0"));
        match val {
            1 => FunctionType::Vertex,
            2 => FunctionType::Fragment,
            3 => FunctionType::Kernel,
            5 => FunctionType::Visible,
            6 => FunctionType::Intersection,
            7 => FunctionType::Mesh,
            8 => FunctionType::Object,
            _ => FunctionType::Vertex,
        }
    }

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on FunctionHandle",
            ))
        }
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
    pub fn function_name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"functionName\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn url_path(&self) -> Option<NSString> {
        let url = msg_id(self.raw, sel(b"URL\0"));
        if url.is_null() {
            None
        } else {
            let ptr = msg_id(url, sel(b"path\0"));
            if ptr.is_null() {
                None
            } else {
                Some(NSString::from_raw(ptr))
            }
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

impl Clone for FunctionLog {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for FunctionLog {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl FunctionLog {
    pub fn log_type(&self) -> FunctionLogType {
        let val = msg_usize(self.raw, sel(b"type\0"));
        match val {
            0 => FunctionLogType::Validation,
            _ => FunctionLogType::Validation,
        }
    }

    pub fn encoder_label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"encoderLabel\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
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

    pub fn level(&self) -> Result<LogLevel, MetalError> {
        let raw = msg_usize(self.raw, sel(b"level\0"));
        LogLevel::from_raw(raw as isize).ok_or_else(|| {
            MetalError::new(format!("invalid MTLLogLevel value from Metal: {}", raw))
        })
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

impl LogState {
    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
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
#[repr(transparent)]
pub struct VisibleFunctionTable {
    pub raw: id,
}

impl VisibleFunctionTable {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
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

    pub fn set_functions(&self, functions: &[FunctionHandle], range: Range) {
        msg_void_ptr_range(
            self.raw,
            sel(b"setFunctions:withRange:\0"),
            functions.as_ptr() as *const id,
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
    pub fn intersection_function_table_descriptor() -> Self {
        let raw = retain(msg_id(
            class(b"MTLIntersectionFunctionTableDescriptor\0"),
            sel(b"intersectionFunctionTableDescriptor\0"),
        ));
        Self { raw }
    }

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

    pub fn copy(&self) -> Self {
        Self {
            raw: retain(objc_copy(self.raw)),
        }
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
#[repr(transparent)]
pub struct IntersectionFunctionTable {
    pub raw: id,
}

impl IntersectionFunctionTable {
    #[inline]
    pub const fn null() -> Self {
        Self {
            raw: NIL,
        }
    }

    fn validate_range(&self, range: Range, count: usize) -> Result<(), MetalError> {
        if count != range.length {
            return Err(MetalError::new(format!(
                "array length {count} does not match range length {}",
                range.length
            )));
        }
        Ok(())
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn label(&self) -> Option<NSString> {
        pipeline_label(self.raw)
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
            Err(MetalError::new("gpuResourceID not supported"))
        }
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        let val = msg_usize(self.raw, sel(b"cpuCacheMode\0"));
        match val {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn storage_mode(&self) -> StorageMode {
        let val = msg_usize(self.raw, sel(b"storageMode\0"));
        match val {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        let selector = sel(b"hazardTrackingMode\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            match val {
                1 => HazardTrackingMode::Untracked,
                2 => HazardTrackingMode::Tracked,
                _ => HazardTrackingMode::Default,
            }
        } else {
            HazardTrackingMode::Default
        }
    }

    pub fn resource_options(&self) -> ResourceOptions {
        let selector = sel(b"resourceOptions\0");
        if responds_to_selector(self.raw, selector) {
            ResourceOptions::from_raw(msg_usize(self.raw, selector))
        } else {
            ResourceOptions::from_raw(0)
        }
    }

    pub fn heap(&self) -> Option<Heap> {
        let selector = sel(b"heap\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(Heap { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn heap_offset(&self) -> usize {
        let selector = sel(b"heapOffset\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn allocated_size(&self) -> usize {
        let selector = sel(b"allocatedSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState {
        let val = msg_usize_usize(self.raw, sel(b"setPurgeableState:\0"), state as usize);
        match val {
            1 => PurgeableState::KeepCurrent,
            2 => PurgeableState::NonVolatile,
            3 => PurgeableState::Volatile,
            4 => PurgeableState::Empty,
            _ => PurgeableState::KeepCurrent,
        }
    }

    pub fn set_function(
        &self,
        function: Option<&FunctionHandle>,
        index: usize,
    ) -> Result<(), MetalError> {
        msg_void_id_usize(
            self.raw,
            sel(b"setFunction:atIndex:\0"),
            function.map_or(NIL, |f| f.raw),
            index,
        );
        Ok(())
    }

    pub fn set_functions(
        &self,
        functions: &[FunctionHandle],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, functions.len())?;
        msg_void_ptr_range(
            self.raw,
            sel(b"setFunctions:withRange:\0"),
            functions.as_ptr() as *const id,
            range,
        );
        Ok(())
    }

    pub fn set_buffer(
        &self,
        buffer: Option<&Buffer>,
        offset: usize,
        index: usize,
    ) -> Result<(), MetalError> {
        msg_void_id_usize_usize(
            self.raw,
            sel(b"setBuffer:offset:atIndex:\0"),
            buffer.map_or(NIL, |b| b.raw),
            offset,
            index,
        );
        Ok(())
    }

    pub fn set_buffers(
        &self,
        buffers: &[Buffer],
        offsets: &[usize],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, buffers.len())?;
        if offsets.len() != buffers.len() {
            return Err(MetalError::new(format!(
                "offsets length {} does not match buffers length {}",
                offsets.len(),
                buffers.len()
            )));
        }
        msg_void_ptr_ptr_range(
            self.raw,
            sel(b"setBuffers:offsets:withRange:\0"),
            buffers.as_ptr() as *const id,
            offsets.as_ptr(),
            range,
        );
        Ok(())
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
    ) -> Result<(), MetalError> {
        if range.length == 0 {
            return Err(MetalError::new(
                "opaque triangle intersection function range length must be greater than zero",
            ));
        }
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
        Ok(())
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
    ) -> Result<(), MetalError> {
        if range.length == 0 {
            return Err(MetalError::new(
                "opaque curve intersection function range length must be greater than zero",
            ));
        }
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
        Ok(())
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
        tables: &[VisibleFunctionTable],
        range: Range,
    ) -> Result<(), MetalError> {
        self.validate_range(range, tables.len())?;
        msg_void_ptr_range(
            self.raw,
            sel(b"setVisibleFunctionTables:withBufferRange:\0"),
            tables.as_ptr() as *const id,
            range,
        );
        Ok(())
    }
}

impl Drop for IntersectionFunctionTable {
    fn drop(&mut self) {
        release(self.raw);
    }
}
