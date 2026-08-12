use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

pub const M4_BUFFER_RANGE_WHOLE: u64 = u64::MAX;

// ---------------------------------------------------------------------------
// MTL4BufferRange.h
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct M4BufferRange {
    pub buffer_address: u64,
    pub length: u64,
}

impl M4BufferRange {
    pub const fn new(buffer_address: u64, length: u64) -> Self {
        Self {
            buffer_address,
            length,
        }
    }

    pub const fn from_address(buffer_address: u64) -> Self {
        Self {
            buffer_address,
            length: M4_BUFFER_RANGE_WHOLE,
        }
    }

    pub fn make(buffer_address: u64, length: u64) -> Self {
        Self::new(buffer_address, length)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct M4TimestampHeapEntry {
    pub timestamp: u64,
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn m4_alloc_init(class_name: &[u8]) -> id {
    let allocated = msg_id(class(class_name), sel(b"alloc\0"));
    msg_id(allocated, sel(b"init\0"))
}

fn m4_label(raw: id) -> Option<NSString> {
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

fn m4_set_label(raw: id, label: &str) {
    let ns = NSString::new(label);
    msg_void_id(raw, sel(b"setLabel:\0"), ns.raw());
}

fn m4_optional_id(raw: id, getter: SEL) -> Option<id> {
    let value = msg_id(raw, getter);
    (!value.is_null()).then_some(retain(value))
}

fn m4_set_optional_id(raw: id, setter: SEL, value: Option<id>) {
    msg_void_id(raw, setter, value.unwrap_or(NIL));
}

fn m4_buffer_range(raw: id, getter: SEL) -> M4BufferRange {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> M4BufferRange =
            transmute(objc_msgSend as *const c_void);
        f(raw, getter)
    }
}

fn m4_set_buffer_range(raw: id, setter: SEL, range: M4BufferRange) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, M4BufferRange) =
            transmute(objc_msgSend as *const c_void);
        f(raw, setter, range);
    }
}

fn m4_function_descriptors_from_array(array: id) -> NSArrayIterator<M4FunctionDescriptor> {
    NSArrayIterator::new(array)
}

fn m4_binary_functions_from_array(array: id) -> NSArrayIterator<M4BinaryFunction> {
    NSArrayIterator::new(array)
}

fn m4_archives_from_array(array: id) -> NSArrayIterator<M4Archive> {
    NSArrayIterator::new(array)
}

fn m4_geometry_base_get(raw: id) -> M4AccelerationStructureGeometryDescriptorBase {
    M4AccelerationStructureGeometryDescriptorBase {
        intersection_function_table_offset: msg_usize(
            raw,
            sel(b"intersectionFunctionTableOffset\0"),
        ),
        opaque: msg_bool(raw, sel(b"isOpaque\0")) != NO,
        allow_duplicate_intersection_function_invocation: msg_bool(
            raw,
            sel(b"allowDuplicateIntersectionFunctionInvocation\0"),
        ) != NO,
        label: m4_label(raw),
        primitive_data_buffer: m4_buffer_range(raw, sel(b"primitiveDataBuffer\0")),
        primitive_data_stride: msg_usize(raw, sel(b"primitiveDataStride\0")),
        primitive_data_element_size: msg_usize(raw, sel(b"primitiveDataElementSize\0")),
    }
}

fn m4_geometry_base_set(raw: id, base: &M4AccelerationStructureGeometryDescriptorBase) {
    msg_void_usize(
        raw,
        sel(b"setIntersectionFunctionTableOffset:\0"),
        base.intersection_function_table_offset,
    );
    msg_void_bool(
        raw,
        sel(b"setOpaque:\0"),
        if base.opaque { YES } else { NO },
    );
    msg_void_bool(
        raw,
        sel(b"setAllowDuplicateIntersectionFunctionInvocation:\0"),
        if base.allow_duplicate_intersection_function_invocation {
            YES
        } else {
            NO
        },
    );
    if let Some(label) = &base.label {
        msg_void_id(raw, sel(b"setLabel:\0"), label.raw());
    }
    m4_set_buffer_range(
        raw,
        sel(b"setPrimitiveDataBuffer:\0"),
        base.primitive_data_buffer,
    );
    msg_void_usize(
        raw,
        sel(b"setPrimitiveDataStride:\0"),
        base.primitive_data_stride,
    );
    msg_void_usize(
        raw,
        sel(b"setPrimitiveDataElementSize:\0"),
        base.primitive_data_element_size,
    );
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct M4AccelerationStructureGeometryDescriptorBase {
    pub intersection_function_table_offset: usize,
    pub opaque: bool,
    pub allow_duplicate_intersection_function_invocation: bool,
    pub label: Option<NSString>,
    pub primitive_data_buffer: M4BufferRange,
    pub primitive_data_stride: usize,
    pub primitive_data_element_size: usize,
}

// ---------------------------------------------------------------------------
// MTL4PipelineState.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4ShaderReflection(pub usize);

impl M4ShaderReflection {
    pub const NONE: Self = Self(0);
    pub const BINDING_INFO: Self = Self(1 << 0);
    pub const BUFFER_TYPE_INFO: Self = Self(1 << 1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4AlphaToOneState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4AlphaToCoverageState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4BlendState {
    Disabled = 0,
    Enabled = 1,
    Unspecialized = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4IndirectCommandBufferSupportState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug)]
pub struct M4PipelineOptions {
    pub raw: id,
}

impl M4PipelineOptions {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4PipelineOptions\0"),
        }
    }

    pub fn shader_validation(&self) -> ShaderValidation {
        match msg_usize(self.raw, sel(b"shaderValidation\0")) as isize {
            1 => ShaderValidation::Enabled,
            2 => ShaderValidation::Disabled,
            _ => ShaderValidation::Default,
        }
    }

    pub fn set_shader_validation(&self, validation: ShaderValidation) {
        msg_void_usize(
            self.raw,
            sel(b"setShaderValidation:\0"),
            validation as usize,
        );
    }

    pub fn shader_reflection(&self) -> M4ShaderReflection {
        M4ShaderReflection(msg_usize(self.raw, sel(b"shaderReflection\0")))
    }

    pub fn set_shader_reflection(&self, reflection: M4ShaderReflection) {
        msg_void_usize(self.raw, sel(b"setShaderReflection:\0"), reflection.0);
    }
}

impl Default for M4PipelineOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4PipelineOptions {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4PipelineDescriptor {
    pub raw: id,
}

impl M4PipelineDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4PipelineDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn options(&self) -> Option<M4PipelineOptions> {
        m4_optional_id(self.raw, sel(b"options\0")).map(|raw| M4PipelineOptions { raw })
    }

    pub fn set_options(&self, options: Option<&M4PipelineOptions>) {
        m4_set_optional_id(self.raw, sel(b"setOptions:\0"), options.map(|o| o.raw));
    }
}

impl Default for M4PipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4PipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4FunctionDescriptor.h and derivatives
// ---------------------------------------------------------------------------

#[derive(Debug)]
#[repr(transparent)]
pub struct M4FunctionDescriptor {
    pub raw: id,
}

impl Clone for M4FunctionDescriptor {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl M4FunctionDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4FunctionDescriptor\0"),
        }
    }
}

impl Default for M4FunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4FunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4LibraryDescriptor {
    pub raw: id,
}

impl M4LibraryDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4LibraryDescriptor\0"),
        }
    }

    pub fn source(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"source\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn set_source(&self, source: &str) {
        let ns = NSString::new(source);
        msg_void_id(self.raw, sel(b"setSource:\0"), ns.raw());
    }

    pub fn options(&self) -> Option<CompileOptions> {
        m4_optional_id(self.raw, sel(b"options\0")).map(|raw| CompileOptions { raw })
    }

    pub fn set_options(&self, options: Option<&CompileOptions>) {
        m4_set_optional_id(self.raw, sel(b"setOptions:\0"), options.map(|o| o.raw));
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
        let ns = NSString::new(name);
        msg_void_id(self.raw, sel(b"setName:\0"), ns.raw());
    }
}

impl Default for M4LibraryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4LibraryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4LibraryFunctionDescriptor {
    pub raw: id,
}

impl M4LibraryFunctionDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4LibraryFunctionDescriptor\0"),
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
        let ns = NSString::new(name);
        msg_void_id(self.raw, sel(b"setName:\0"), ns.raw());
    }

    pub fn library(&self) -> Option<Library> {
        m4_optional_id(self.raw, sel(b"library\0")).map(|raw| Library { raw })
    }

    pub fn as_function_descriptor(&self) -> M4FunctionDescriptor {
        M4FunctionDescriptor {
            raw: retain(self.raw),
        }
    }

    pub fn set_library(&self, library: Option<&Library>) {
        m4_set_optional_id(self.raw, sel(b"setLibrary:\0"), library.map(|l| l.raw));
    }
}

impl Default for M4LibraryFunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4LibraryFunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4BinaryFunctionOptions(pub usize);

impl M4BinaryFunctionOptions {
    pub const NONE: Self = Self(0);
    pub const PIPELINE_INDEPENDENT: Self = Self(1 << 1);
}

#[derive(Debug)]
pub struct M4BinaryFunctionDescriptor {
    pub raw: id,
}

impl M4BinaryFunctionDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4BinaryFunctionDescriptor\0"),
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
        let ns = NSString::new(name);
        msg_void_id(self.raw, sel(b"setName:\0"), ns.raw());
    }

    pub fn function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"functionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn options(&self) -> M4BinaryFunctionOptions {
        M4BinaryFunctionOptions(msg_usize(self.raw, sel(b"options\0")))
    }

    pub fn set_options(&self, options: M4BinaryFunctionOptions) {
        msg_void_usize(self.raw, sel(b"setOptions:\0"), options.0);
    }
}

impl Default for M4BinaryFunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4BinaryFunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4SpecializedFunctionDescriptor {
    pub raw: id,
}

impl M4SpecializedFunctionDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4SpecializedFunctionDescriptor\0"),
        }
    }

    pub fn function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"functionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
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
        let ns = NSString::new(name);
        msg_void_id(self.raw, sel(b"setSpecializedName:\0"), ns.raw());
    }

    pub fn constant_values(&self) -> Option<FunctionConstantValues> {
        m4_optional_id(self.raw, sel(b"constantValues\0")).map(|raw| FunctionConstantValues { raw })
    }

    pub fn set_constant_values(&self, values: Option<&FunctionConstantValues>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setConstantValues:\0"),
            values.map(|v| v.raw),
        );
    }
}

impl Default for M4SpecializedFunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4SpecializedFunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4StitchedFunctionDescriptor {
    pub raw: id,
}

impl M4StitchedFunctionDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4StitchedFunctionDescriptor\0"),
        }
    }

    pub fn function_graph(&self) -> Option<FunctionStitchingGraph> {
        m4_optional_id(self.raw, sel(b"functionGraph\0")).map(|raw| FunctionStitchingGraph { raw })
    }

    pub fn set_function_graph(&self, graph: Option<&FunctionStitchingGraph>) {
        m4_set_optional_id(self.raw, sel(b"setFunctionGraph:\0"), graph.map(|g| g.raw));
    }

    pub fn function_descriptors(&self) -> NSArrayIterator<M4FunctionDescriptor> {
        m4_function_descriptors_from_array(msg_id(self.raw, sel(b"functionDescriptors\0")))
    }

    pub fn set_function_descriptors(&self, descriptors: &[M4FunctionDescriptor]) {
        let raw_ptrs = unsafe {
            std::slice::from_raw_parts(descriptors.as_ptr() as *const id, descriptors.len())
        };
        msg_void_id(
            self.raw,
            sel(b"setFunctionDescriptors:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }
}

impl Default for M4StitchedFunctionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4StitchedFunctionDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4BinaryFunction.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
#[repr(transparent)]
pub struct M4BinaryFunction {
    pub raw: id,
}

impl Clone for M4BinaryFunction {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl M4BinaryFunction {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"name\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn function_type(&self) -> FunctionType {
        match msg_usize(self.raw, sel(b"functionType\0")) {
            2 => FunctionType::Fragment,
            3 => FunctionType::Kernel,
            5 => FunctionType::Visible,
            6 => FunctionType::Intersection,
            7 => FunctionType::Mesh,
            8 => FunctionType::Object,
            _ => FunctionType::Vertex,
        }
    }
}

impl Drop for M4BinaryFunction {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4LinkingDescriptor.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4StaticLinkingDescriptor {
    pub raw: id,
}

impl M4StaticLinkingDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4StaticLinkingDescriptor\0"),
        }
    }

    pub fn function_descriptors(&self) -> NSArrayIterator<M4FunctionDescriptor> {
        m4_function_descriptors_from_array(msg_id(self.raw, sel(b"functionDescriptors\0")))
    }

    pub fn set_function_descriptors(&self, descriptors: &[M4FunctionDescriptor]) {
        let raw_ptrs = unsafe {
            std::slice::from_raw_parts(descriptors.as_ptr() as *const id, descriptors.len())
        };
        msg_void_id(
            self.raw,
            sel(b"setFunctionDescriptors:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }

    pub fn private_function_descriptors(&self) -> NSArrayIterator<M4FunctionDescriptor> {
        m4_function_descriptors_from_array(msg_id(self.raw, sel(b"privateFunctionDescriptors\0")))
    }

    pub fn set_private_function_descriptors(&self, descriptors: &[M4FunctionDescriptor]) {
        let raw_ptrs = unsafe {
            std::slice::from_raw_parts(descriptors.as_ptr() as *const id, descriptors.len())
        };
        msg_void_id(
            self.raw,
            sel(b"setPrivateFunctionDescriptors:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }
}

impl Default for M4StaticLinkingDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4StaticLinkingDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4PipelineStageDynamicLinkingDescriptor {
    pub raw: id,
}

impl M4PipelineStageDynamicLinkingDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4PipelineStageDynamicLinkingDescriptor\0"),
        }
    }

    pub fn max_call_stack_depth(&self) -> usize {
        msg_usize(self.raw, sel(b"maxCallStackDepth\0"))
    }

    pub fn set_max_call_stack_depth(&self, depth: usize) {
        msg_void_usize(self.raw, sel(b"setMaxCallStackDepth:\0"), depth);
    }

    pub fn binary_linked_functions(&self) -> NSArrayIterator<M4BinaryFunction> {
        m4_binary_functions_from_array(msg_id(self.raw, sel(b"binaryLinkedFunctions\0")))
    }

    pub fn set_binary_linked_functions(&self, functions: &[M4BinaryFunction]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(functions.as_ptr() as *const id, functions.len()) };
        msg_void_id(
            self.raw,
            sel(b"setBinaryLinkedFunctions:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }

    pub fn preloaded_libraries(&self) -> NSArrayIterator<DynamicLibrary> {
        NSArrayIterator::new(msg_id(self.raw, sel(b"preloadedLibraries\0")))
    }

    pub fn set_preloaded_libraries(&self, libraries: &[DynamicLibrary]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(libraries.as_ptr() as *const id, libraries.len()) };
        msg_void_id(
            self.raw,
            sel(b"setPreloadedLibraries:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }
}

impl Default for M4PipelineStageDynamicLinkingDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4PipelineStageDynamicLinkingDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4RenderPipelineDynamicLinkingDescriptor {
    pub raw: id,
}

impl M4RenderPipelineDynamicLinkingDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4RenderPipelineDynamicLinkingDescriptor\0"),
        }
    }

    pub fn vertex_linking_descriptor(&self) -> M4PipelineStageDynamicLinkingDescriptor {
        M4PipelineStageDynamicLinkingDescriptor {
            raw: retain(msg_id(self.raw, sel(b"vertexLinkingDescriptor\0"))),
        }
    }

    pub fn fragment_linking_descriptor(&self) -> M4PipelineStageDynamicLinkingDescriptor {
        M4PipelineStageDynamicLinkingDescriptor {
            raw: retain(msg_id(self.raw, sel(b"fragmentLinkingDescriptor\0"))),
        }
    }

    pub fn tile_linking_descriptor(&self) -> M4PipelineStageDynamicLinkingDescriptor {
        M4PipelineStageDynamicLinkingDescriptor {
            raw: retain(msg_id(self.raw, sel(b"tileLinkingDescriptor\0"))),
        }
    }

    pub fn object_linking_descriptor(&self) -> M4PipelineStageDynamicLinkingDescriptor {
        M4PipelineStageDynamicLinkingDescriptor {
            raw: retain(msg_id(self.raw, sel(b"objectLinkingDescriptor\0"))),
        }
    }

    pub fn mesh_linking_descriptor(&self) -> M4PipelineStageDynamicLinkingDescriptor {
        M4PipelineStageDynamicLinkingDescriptor {
            raw: retain(msg_id(self.raw, sel(b"meshLinkingDescriptor\0"))),
        }
    }
}

impl Default for M4RenderPipelineDynamicLinkingDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4RenderPipelineDynamicLinkingDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4RenderPipeline.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4LogicalToPhysicalColorAttachmentMappingState {
    Identity = 0,
    Inherited = 1,
}

#[derive(Debug)]
pub struct M4RenderPipelineColorAttachmentDescriptor {
    pub raw: id,
}

impl M4RenderPipelineColorAttachmentDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4RenderPipelineColorAttachmentDescriptor\0"),
        }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_raw(msg_usize(self.raw, sel(b"pixelFormat\0")))
    }

    pub fn set_pixel_format(&self, format: PixelFormat) {
        msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), format.as_raw());
    }

    pub fn blending_state(&self) -> M4BlendState {
        match msg_usize(self.raw, sel(b"blendingState\0")) {
            1 => M4BlendState::Enabled,
            2 => M4BlendState::Unspecialized,
            _ => M4BlendState::Disabled,
        }
    }

    pub fn set_blending_state(&self, state: M4BlendState) {
        msg_void_usize(self.raw, sel(b"setBlendingState:\0"), state as usize);
    }

    pub fn set_blend_factors(
        &self,
        source_rgb: BlendFactor,
        destination_rgb: BlendFactor,
        rgb_operation: BlendOperation,
        source_alpha: BlendFactor,
        destination_alpha: BlendFactor,
        alpha_operation: BlendOperation,
    ) {
        msg_void_usize(
            self.raw,
            sel(b"setSourceRGBBlendFactor:\0"),
            source_rgb as usize,
        );
        msg_void_usize(
            self.raw,
            sel(b"setDestinationRGBBlendFactor:\0"),
            destination_rgb as usize,
        );
        msg_void_usize(
            self.raw,
            sel(b"setRgbBlendOperation:\0"),
            rgb_operation as usize,
        );
        msg_void_usize(
            self.raw,
            sel(b"setSourceAlphaBlendFactor:\0"),
            source_alpha as usize,
        );
        msg_void_usize(
            self.raw,
            sel(b"setDestinationAlphaBlendFactor:\0"),
            destination_alpha as usize,
        );
        msg_void_usize(
            self.raw,
            sel(b"setAlphaBlendOperation:\0"),
            alpha_operation as usize,
        );
    }

    pub fn write_mask(&self) -> ColorWriteMask {
        let raw = msg_usize(self.raw, sel(b"writeMask\0"));
        unsafe { std::mem::transmute(raw) }
    }

    pub fn set_write_mask(&self, mask: ColorWriteMask) {
        msg_void_usize(self.raw, sel(b"setWriteMask:\0"), mask.as_raw());
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4RenderPipelineColorAttachmentDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4RenderPipelineColorAttachmentDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4RenderPipelineColorAttachmentDescriptorArray {
    pub raw: id,
}

impl M4RenderPipelineColorAttachmentDescriptorArray {
    pub fn object_at(&self, index: usize) -> M4RenderPipelineColorAttachmentDescriptor {
        M4RenderPipelineColorAttachmentDescriptor {
            raw: retain(msg_id_usize(
                self.raw,
                sel(b"objectAtIndexedSubscript:\0"),
                index,
            )),
        }
    }

    pub fn set_object(
        &self,
        attachment: Option<&M4RenderPipelineColorAttachmentDescriptor>,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setObject:atIndexedSubscript:\0"),
                attachment.map_or(NIL, |a| a.raw),
                index,
            );
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Drop for M4RenderPipelineColorAttachmentDescriptorArray {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4RenderPipelineBinaryFunctionsDescriptor {
    pub raw: id,
}

impl M4RenderPipelineBinaryFunctionsDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4RenderPipelineBinaryFunctionsDescriptor\0"),
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4RenderPipelineBinaryFunctionsDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4RenderPipelineBinaryFunctionsDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4RenderPipelineDescriptor {
    pub raw: id,
}

impl M4RenderPipelineDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4RenderPipelineDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn as_pipeline_descriptor(&self) -> M4PipelineDescriptor {
        M4PipelineDescriptor {
            raw: retain(self.raw),
        }
    }

    pub fn vertex_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"vertexFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_vertex_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setVertexFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn fragment_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"fragmentFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_fragment_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setFragmentFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn vertex_descriptor(&self) -> Option<VertexDescriptor> {
        m4_optional_id(self.raw, sel(b"vertexDescriptor\0")).map(|raw| VertexDescriptor { raw })
    }

    pub fn set_vertex_descriptor(&self, descriptor: Option<&VertexDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setVertexDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn raster_sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"rasterSampleCount\0"))
    }

    pub fn set_raster_sample_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setRasterSampleCount:\0"), count);
    }

    pub fn alpha_to_coverage_state(&self) -> M4AlphaToCoverageState {
        match msg_usize(self.raw, sel(b"alphaToCoverageState\0")) {
            1 => M4AlphaToCoverageState::Enabled,
            _ => M4AlphaToCoverageState::Disabled,
        }
    }

    pub fn set_alpha_to_coverage_state(&self, state: M4AlphaToCoverageState) {
        msg_void_usize(self.raw, sel(b"setAlphaToCoverageState:\0"), state as usize);
    }

    pub fn input_primitive_topology(&self) -> PrimitiveTopologyClass {
        match msg_usize(self.raw, sel(b"inputPrimitiveTopology\0")) {
            1 => PrimitiveTopologyClass::Point,
            2 => PrimitiveTopologyClass::Line,
            3 => PrimitiveTopologyClass::Triangle,
            _ => PrimitiveTopologyClass::Unspecified,
        }
    }

    pub fn set_input_primitive_topology(&self, topology: PrimitiveTopologyClass) {
        msg_void_usize(
            self.raw,
            sel(b"setInputPrimitiveTopology:\0"),
            topology as usize,
        );
    }

    pub fn alpha_to_one_state(&self) -> M4AlphaToOneState {
        match msg_usize(self.raw, sel(b"alphaToOneState\0")) {
            1 => M4AlphaToOneState::Enabled,
            _ => M4AlphaToOneState::Disabled,
        }
    }

    pub fn set_alpha_to_one_state(&self, state: M4AlphaToOneState) {
        msg_void_usize(self.raw, sel(b"setAlphaToOneState:\0"), state as usize);
    }

    pub fn is_rasterization_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isRasterizationEnabled\0")) != NO
    }

    pub fn set_rasterization_enabled(&self, enabled: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setRasterizationEnabled:\0"),
            if enabled { YES } else { NO },
        );
    }

    pub fn color_attachments(&self) -> M4RenderPipelineColorAttachmentDescriptorArray {
        M4RenderPipelineColorAttachmentDescriptorArray {
            raw: retain(msg_id(self.raw, sel(b"colorAttachments\0"))),
        }
    }

    pub fn support_indirect_command_buffers(&self) -> M4IndirectCommandBufferSupportState {
        match msg_usize(self.raw, sel(b"supportIndirectCommandBuffers\0")) {
            1 => M4IndirectCommandBufferSupportState::Enabled,
            _ => M4IndirectCommandBufferSupportState::Disabled,
        }
    }

    pub fn set_support_indirect_command_buffers(&self, state: M4IndirectCommandBufferSupportState) {
        msg_void_usize(
            self.raw,
            sel(b"setSupportIndirectCommandBuffers:\0"),
            state as usize,
        );
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4RenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4RenderPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4ComputePipeline.h / MTL4TileRenderPipeline.h / MTL4MeshRenderPipeline.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4ComputePipelineDescriptor {
    pub raw: id,
}

impl M4ComputePipelineDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4ComputePipelineDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn compute_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"computeFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_compute_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setComputeFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        msg_bool(
            self.raw,
            sel(b"isThreadGroupSizeIsMultipleOfThreadExecutionWidth\0"),
        ) != NO
    }

    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, value: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setThreadGroupSizeIsMultipleOfThreadExecutionWidth:\0"),
            if value { YES } else { NO },
        );
    }

    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTotalThreadsPerThreadgroup\0"))
    }

    pub fn set_max_total_threads_per_threadgroup(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxTotalThreadsPerThreadgroup:\0"), count);
    }

    pub fn required_threads_per_threadgroup(&self) -> Size {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"requiredThreadsPerThreadgroup\0"))
        }
    }

    pub fn set_required_threads_per_threadgroup(&self, size: Size) {
        msg_void_mtlsize(self.raw, sel(b"setRequiredThreadsPerThreadgroup:\0"), size);
    }

    pub fn support_binary_linking(&self) -> bool {
        msg_bool(self.raw, sel(b"supportBinaryLinking\0")) != NO
    }

    pub fn set_support_binary_linking(&self, value: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setSupportBinaryLinking:\0"),
            if value { YES } else { NO },
        );
    }

    pub fn static_linking_descriptor(&self) -> Option<M4StaticLinkingDescriptor> {
        m4_optional_id(self.raw, sel(b"staticLinkingDescriptor\0"))
            .map(|raw| M4StaticLinkingDescriptor { raw })
    }

    pub fn set_static_linking_descriptor(&self, descriptor: Option<&M4StaticLinkingDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setStaticLinkingDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn support_indirect_command_buffers(&self) -> M4IndirectCommandBufferSupportState {
        match msg_usize(self.raw, sel(b"supportIndirectCommandBuffers\0")) {
            1 => M4IndirectCommandBufferSupportState::Enabled,
            _ => M4IndirectCommandBufferSupportState::Disabled,
        }
    }

    pub fn set_support_indirect_command_buffers(&self, state: M4IndirectCommandBufferSupportState) {
        msg_void_usize(
            self.raw,
            sel(b"setSupportIndirectCommandBuffers:\0"),
            state as usize,
        );
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4ComputePipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4ComputePipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct TileRenderPipelineColorAttachmentDescriptor {
    pub raw: id,
}

impl TileRenderPipelineColorAttachmentDescriptor {
    pub fn pixel_format(&self) -> PixelFormat {
        PixelFormat::from_raw(msg_usize(self.raw, sel(b"pixelFormat\0")))
    }

    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), pixel_format as usize);
    }
}

#[derive(Debug)]
pub struct TileRenderPipelineColorAttachmentDescriptorArray {
    pub raw: id,
}

impl TileRenderPipelineColorAttachmentDescriptorArray {
    pub fn object_at(&self, index: usize) -> TileRenderPipelineColorAttachmentDescriptor {
        TileRenderPipelineColorAttachmentDescriptor {
            raw: msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index),
        }
    }
}

impl Drop for TileRenderPipelineColorAttachmentDescriptorArray {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4TileRenderPipelineDescriptor {
    pub raw: id,
}

impl M4TileRenderPipelineDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4TileRenderPipelineDescriptor\0"),
        }
    }

    pub fn as_pipeline_descriptor(&self) -> M4PipelineDescriptor {
        M4PipelineDescriptor {
            raw: retain(self.raw),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn tile_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"tileFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_tile_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setTileFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn raster_sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"rasterSampleCount\0"))
    }

    pub fn set_raster_sample_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setRasterSampleCount:\0"), count);
    }

    pub fn color_attachments(&self) -> TileRenderPipelineColorAttachmentDescriptorArray {
        TileRenderPipelineColorAttachmentDescriptorArray {
            raw: retain(msg_id(self.raw, sel(b"colorAttachments\0"))),
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4TileRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4TileRenderPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4MeshRenderPipelineDescriptor {
    pub raw: id,
}

impl M4MeshRenderPipelineDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4MeshRenderPipelineDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn as_pipeline_descriptor(&self) -> M4PipelineDescriptor {
        M4PipelineDescriptor {
            raw: retain(self.raw),
        }
    }

    pub fn object_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"objectFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_object_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setObjectFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn mesh_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"meshFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_mesh_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setMeshFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn fragment_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"fragmentFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_fragment_function_descriptor(&self, descriptor: Option<&M4FunctionDescriptor>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setFragmentFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn max_total_threads_per_object_threadgroup(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTotalThreadsPerObjectThreadgroup\0"))
    }

    pub fn set_max_total_threads_per_object_threadgroup(&self, count: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMaxTotalThreadsPerObjectThreadgroup:\0"),
            count,
        );
    }

    pub fn max_total_threads_per_mesh_threadgroup(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTotalThreadsPerMeshThreadgroup\0"))
    }

    pub fn set_max_total_threads_per_mesh_threadgroup(&self, count: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMaxTotalThreadsPerMeshThreadgroup:\0"),
            count,
        );
    }

    pub fn payload_memory_length(&self) -> usize {
        msg_usize(self.raw, sel(b"payloadMemoryLength\0"))
    }

    pub fn set_payload_memory_length(&self, length: usize) {
        msg_void_usize(self.raw, sel(b"setPayloadMemoryLength:\0"), length);
    }

    pub fn max_total_threadgroups_per_mesh_grid(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTotalThreadgroupsPerMeshGrid\0"))
    }

    pub fn set_max_total_threadgroups_per_mesh_grid(&self, count: usize) {
        msg_void_usize(
            self.raw,
            sel(b"setMaxTotalThreadgroupsPerMeshGrid:\0"),
            count,
        );
    }

    pub fn raster_sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"rasterSampleCount\0"))
    }

    pub fn set_raster_sample_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setRasterSampleCount:\0"), count);
    }

    pub fn alpha_to_coverage_state(&self) -> M4AlphaToCoverageState {
        match msg_usize(self.raw, sel(b"alphaToCoverageState\0")) {
            1 => M4AlphaToCoverageState::Enabled,
            _ => M4AlphaToCoverageState::Disabled,
        }
    }

    pub fn set_alpha_to_coverage_state(&self, state: M4AlphaToCoverageState) {
        msg_void_usize(
            self.raw,
            sel(b"setAlphaToCoverageState:\0"),
            state as usize,
        );
    }

    pub fn is_rasterization_enabled(&self) -> bool {
        msg_bool(self.raw, sel(b"isRasterizationEnabled\0")) != NO
    }

    pub fn set_rasterization_enabled(&self, enabled: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setRasterizationEnabled:\0"),
            if enabled { YES } else { NO },
        );
    }

    pub fn support_indirect_command_buffers(&self) -> M4IndirectCommandBufferSupportState {
        match msg_usize(self.raw, sel(b"supportIndirectCommandBuffers\0")) {
            1 => M4IndirectCommandBufferSupportState::Enabled,
            _ => M4IndirectCommandBufferSupportState::Disabled,
        }
    }

    pub fn set_support_indirect_command_buffers(&self, state: M4IndirectCommandBufferSupportState) {
        msg_void_usize(
            self.raw,
            sel(b"setSupportIndirectCommandBuffers:\0"),
            state as usize,
        );
    }

    pub fn color_attachments(&self) -> M4RenderPipelineColorAttachmentDescriptorArray {
        M4RenderPipelineColorAttachmentDescriptorArray {
            raw: retain(msg_id(self.raw, sel(b"colorAttachments\0"))),
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4MeshRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4MeshRenderPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4MachineLearningPipeline.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4MachineLearningPipelineDescriptor {
    pub raw: id,
}

impl M4MachineLearningPipelineDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4MachineLearningPipelineDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn machine_learning_function_descriptor(&self) -> Option<M4FunctionDescriptor> {
        m4_optional_id(self.raw, sel(b"machineLearningFunctionDescriptor\0"))
            .map(|raw| M4FunctionDescriptor { raw })
    }

    pub fn set_machine_learning_function_descriptor(
        &self,
        descriptor: Option<&M4FunctionDescriptor>,
    ) {
        m4_set_optional_id(
            self.raw,
            sel(b"setMachineLearningFunctionDescriptor:\0"),
            descriptor.map(|d| d.raw),
        );
    }

    pub fn set_input_dimensions(&self, dimensions: id, buffer_index: isize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, isize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setInputDimensions:atBufferIndex:\0"),
                dimensions,
                buffer_index,
            );
        }
    }

    pub fn input_dimensions_at_buffer_index(&self, buffer_index: isize) -> id {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, isize) -> id =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"inputDimensionsAtBufferIndex:\0"),
                buffer_index,
            )
        }
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Default for M4MachineLearningPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4MachineLearningPipelineDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4MachineLearningPipelineReflection {
    pub raw: id,
}

impl M4MachineLearningPipelineReflection {
    pub fn bindings(&self) -> NSArrayIterator<Binding> {
        NSArrayIterator::new(msg_id(self.raw, sel(b"bindings\0")))
    }
}

impl Drop for M4MachineLearningPipelineReflection {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4MachineLearningPipelineState {
    pub raw: id,
}

impl M4MachineLearningPipelineState {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn reflection(&self) -> Option<M4MachineLearningPipelineReflection> {
        m4_optional_id(self.raw, sel(b"reflection\0"))
            .map(|raw| M4MachineLearningPipelineReflection { raw })
    }

    pub fn intermediates_heap_size(&self) -> usize {
        msg_usize(self.raw, sel(b"intermediatesHeapSize\0"))
    }
}

impl Drop for M4MachineLearningPipelineState {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4Counters.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4CounterHeapType {
    Invalid = 0,
    Timestamp = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4TimestampGranularity {
    Relaxed = 0,
    Precise = 1,
}

#[derive(Debug)]
pub struct M4CounterHeapDescriptor {
    pub raw: id,
}

impl M4CounterHeapDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CounterHeapDescriptor\0"),
        }
    }

    pub fn heap_type(&self) -> M4CounterHeapType {
        match msg_usize(self.raw, sel(b"type\0")) {
            1 => M4CounterHeapType::Timestamp,
            _ => M4CounterHeapType::Invalid,
        }
    }

    pub fn set_type(&self, heap_type: M4CounterHeapType) {
        msg_void_usize(self.raw, sel(b"setType:\0"), heap_type as usize);
    }

    pub fn count(&self) -> usize {
        msg_usize(self.raw, sel(b"count\0"))
    }

    pub fn set_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setCount:\0"), count);
    }
}

impl Default for M4CounterHeapDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CounterHeapDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4CounterHeap {
    pub raw: id,
}

impl M4CounterHeap {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn count(&self) -> usize {
        msg_usize(self.raw, sel(b"count\0"))
    }

    pub fn heap_type(&self) -> M4CounterHeapType {
        match msg_usize(self.raw, sel(b"type\0")) {
            1 => M4CounterHeapType::Timestamp,
            _ => M4CounterHeapType::Invalid,
        }
    }

    pub fn resolve_counter_range(&self, range: Range) -> Option<NSData> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Range) -> id =
                transmute(objc_msgSend as *const c_void);
            let data = f(self.raw, sel(b"resolveCounterRange:\0"), range);
            if data.is_null() {
                return None;
            }
            Some(NSData::from_raw(data))
        }
    }

    pub fn invalidate_counter_range(&self, range: Range) {
        msg_void_range(self.raw, sel(b"invalidateCounterRange:\0"), range);
    }
}

impl Drop for M4CounterHeap {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4ArgumentTable.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4ArgumentTableDescriptor {
    pub raw: id,
}

impl M4ArgumentTableDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4ArgumentTableDescriptor\0"),
        }
    }

    pub fn max_buffer_bind_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxBufferBindCount\0"))
    }

    pub fn set_max_buffer_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxBufferBindCount:\0"), count);
    }

    pub fn max_texture_bind_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxTextureBindCount\0"))
    }

    pub fn set_max_texture_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxTextureBindCount:\0"), count);
    }

    pub fn max_sampler_state_bind_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxSamplerStateBindCount\0"))
    }

    pub fn set_max_sampler_state_bind_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxSamplerStateBindCount:\0"), count);
    }

    pub fn initialize_bindings(&self) -> bool {
        msg_bool(self.raw, sel(b"initializeBindings\0")) != NO
    }

    pub fn set_initialize_bindings(&self, value: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setInitializeBindings:\0"),
            if value { YES } else { NO },
        );
    }

    pub fn support_attribute_strides(&self) -> bool {
        msg_bool(self.raw, sel(b"supportAttributeStrides\0")) != NO
    }

    pub fn set_support_attribute_strides(&self, value: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setSupportAttributeStrides:\0"),
            if value { YES } else { NO },
        );
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }
}

impl Default for M4ArgumentTableDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4ArgumentTableDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4ArgumentTable {
    pub raw: id,
}

impl M4ArgumentTable {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn set_address(&self, gpu_address: u64, binding_index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u64, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setAddress:atIndex:\0"),
                gpu_address,
                binding_index,
            );
        }
    }

    pub fn set_address_attribute_stride(
        &self,
        gpu_address: u64,
        stride: usize,
        binding_index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u64, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setAddress:attributeStride:atIndex:\0"),
                gpu_address,
                stride,
                binding_index,
            );
        }
    }

    pub fn set_resource(&self, resource_id: ResourceID, buffer_index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, ResourceID, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setResource:atBufferIndex:\0"),
                resource_id,
                buffer_index,
            );
        }
    }

    pub fn set_texture(&self, resource_id: ResourceID, binding_index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, ResourceID, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setTexture:atIndex:\0"),
                resource_id,
                binding_index,
            );
        }
    }

    pub fn set_sampler_state(&self, resource_id: ResourceID, binding_index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, ResourceID, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setSamplerState:atIndex:\0"),
                resource_id,
                binding_index,
            );
        }
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }
}

impl Drop for M4ArgumentTable {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4Archive.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
#[repr(transparent)]
pub struct M4Archive {
    pub raw: id,
}

impl Clone for M4Archive {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl M4Archive {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn new_compute_pipeline_state(
        &self,
        descriptor: &M4ComputePipelineDescriptor,
    ) -> Result<ComputePipelineState, MetalError> {
        let mut error = NIL;
        let raw = retain(msg_id_id_err(
            self.raw,
            sel(b"newComputePipelineStateWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newComputePipelineStateWithDescriptor failed",
            )))
        } else {
            Ok(ComputePipelineState { raw })
        }
    }

    pub fn new_render_pipeline_state(
        &self,
        descriptor: &M4PipelineDescriptor,
    ) -> Result<RenderPipelineState, MetalError> {
        let mut error = NIL;
        let raw = retain(msg_id_id_err(
            self.raw,
            sel(b"newRenderPipelineStateWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newRenderPipelineStateWithDescriptor failed",
            )))
        } else {
            Ok(RenderPipelineState { raw })
        }
    }

    pub fn new_binary_function(
        &self,
        descriptor: &M4BinaryFunctionDescriptor,
    ) -> Result<M4BinaryFunction, MetalError> {
        let mut error = NIL;
        let raw = retain(msg_id_id_err(
            self.raw,
            sel(b"newBinaryFunctionWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newBinaryFunctionWithDescriptor failed",
            )))
        } else {
            Ok(M4BinaryFunction { raw })
        }
    }
}

impl Drop for M4Archive {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4CommandAllocator.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4CommandAllocatorDescriptor {
    pub raw: id,
}

impl M4CommandAllocatorDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CommandAllocatorDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }
}

impl Default for M4CommandAllocatorDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CommandAllocatorDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4CommandAllocator {
    pub raw: id,
}

impl M4CommandAllocator {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn allocated_size(&self) -> u64 {
        msg_u64(self.raw, sel(b"allocatedSize\0"))
    }

    pub fn reset(&self) {
        msg_void(self.raw, sel(b"reset\0"));
    }
}

impl Drop for M4CommandAllocator {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4CommandBuffer.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4CommandBufferOptions {
    pub raw: id,
}

impl M4CommandBufferOptions {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CommandBufferOptions\0"),
        }
    }

    pub fn log_state(&self) -> Option<LogState> {
        m4_optional_id(self.raw, sel(b"logState\0")).map(|raw| LogState { raw })
    }

    pub fn set_log_state(&self, log_state: Option<&LogState>) {
        m4_set_optional_id(self.raw, sel(b"setLogState:\0"), log_state.map(|s| s.raw));
    }
}

impl Default for M4CommandBufferOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CommandBufferOptions {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct M4CommandBuffer {
    pub raw: id,
}

impl Clone for M4CommandBuffer {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl M4CommandBuffer {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn begin_command_buffer_with_allocator(&self, allocator: &M4CommandAllocator) {
        msg_void_id(
            self.raw,
            sel(b"beginCommandBufferWithAllocator:\0"),
            allocator.raw,
        );
    }

    pub fn begin_command_buffer_with_allocator_options(
        &self,
        allocator: &M4CommandAllocator,
        options: &M4CommandBufferOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, id) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"beginCommandBufferWithAllocator:options:\0"),
                allocator.raw,
                options.raw,
            );
        }
    }

    pub fn end_command_buffer(&self) {
        msg_void(self.raw, sel(b"endCommandBuffer\0"));
    }

    pub fn render_command_encoder_with_descriptor(
        &self,
        descriptor: &M4RenderPassDescriptor,
    ) -> Option<M4RenderCommandEncoder> {
        let raw = msg_id_id(
            self.raw,
            sel(b"renderCommandEncoderWithDescriptor:\0"),
            descriptor.raw,
        );
        (!raw.is_null()).then(|| M4RenderCommandEncoder { raw: retain(raw) })
    }

    pub fn compute_command_encoder(&self) -> Option<M4ComputeCommandEncoder> {
        let raw = msg_id(self.raw, sel(b"computeCommandEncoder\0"));
        (!raw.is_null()).then(|| M4ComputeCommandEncoder { raw: retain(raw) })
    }

    pub fn machine_learning_command_encoder(&self) -> Option<M4MachineLearningCommandEncoder> {
        let raw = msg_id(self.raw, sel(b"machineLearningCommandEncoder\0"));
        (!raw.is_null()).then(|| M4MachineLearningCommandEncoder { raw: retain(raw) })
    }

    pub fn use_residency_set(&self, residency_set: &ResidencySet) {
        msg_void_id(self.raw, sel(b"useResidencySet:\0"), residency_set.raw);
    }

    pub fn write_timestamp_into_heap(&self, counter_heap: &M4CounterHeap, index: usize) {
        msg_void_id_usize(
            self.raw,
            sel(b"writeTimestampIntoHeap:atIndex:\0"),
            counter_heap.raw,
            index,
        );
    }

    pub fn resolve_counter_heap(
        &self,
        counter_heap: &M4CounterHeap,
        range: Range,
        buffer_range: M4BufferRange,
        wait_fence: Option<&Fence>,
        update_fence: Option<&Fence>,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range, M4BufferRange, id, id) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"resolveCounterHeap:withRange:intoBuffer:waitFence:updateFence:\0"),
                counter_heap.raw,
                range,
                buffer_range,
                wait_fence.map_or(NIL, |f| f.raw),
                update_fence.map_or(NIL, |f| f.raw),
            );
        }
    }
}

impl Drop for M4CommandBuffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4CommandEncoder.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4VisibilityOptions(pub usize);

impl M4VisibilityOptions {
    pub const NONE: Self = Self(0);
    pub const DEVICE: Self = Self(1 << 0);
    pub const RESOURCE_ALIAS: Self = Self(1 << 1);
}

impl std::ops::BitOr for M4VisibilityOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stages(pub usize);

impl Stages {
    pub const NONE: Self = Self(0);
    pub const VERTEX: Self = Self(1 << 0);
    pub const FRAGMENT: Self = Self(1 << 1);
    pub const TILE: Self = Self(1 << 2);
    pub const OBJECT: Self = Self(1 << 3);
    pub const MESH: Self = Self(1 << 4);
    pub const RESOURCE_STATE: Self = Self(1 << 26);
    pub const DISPATCH: Self = Self(1 << 27);
    pub const BLIT: Self = Self(1 << 28);
    pub const ACCELERATION_STRUCTURE: Self = Self(1 << 29);
    pub const MACHINE_LEARNING: Self = Self(1 << 30);
    pub const ALL: Self = Self(isize::MAX as usize);

    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl std::ops::BitOr for Stages {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}


#[derive(Debug)]
pub struct M4CommandEncoder {
    pub raw: id,
}

impl M4CommandEncoder {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn command_buffer(&self) -> Option<M4CommandBuffer> {
        m4_optional_id(self.raw, sel(b"commandBuffer\0")).map(|raw| M4CommandBuffer { raw })
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }

    pub fn insert_debug_signpost(&self, string: &str) {
        let ns = NSString::new(string);
        msg_void_id(self.raw, sel(b"insertDebugSignpost:\0"), ns.raw());
    }

    pub fn push_debug_group(&self, string: &str) {
        let ns = NSString::new(string);
        msg_void_id(self.raw, sel(b"pushDebugGroup:\0"), ns.raw());
    }

    pub fn pop_debug_group(&self) {
        msg_void(self.raw, sel(b"popDebugGroup\0"));
    }

    pub fn barrier_after_queue_stages(
        &self,
        after_queue_stages: Stages,
        before_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterQueueStages:beforeStages:visibilityOptions:\0"),
                after_queue_stages.0,
                before_stages.0,
                visibility.0,
            );
        }
    }

    pub fn barrier_before_queue_stages(
        &self,
        after_stages: Stages,
        before_queue_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterStages:beforeQueueStages:visibilityOptions:\0"),
                after_stages.0,
                before_queue_stages.0,
                visibility.0,
            );
        }
    }

    pub fn barrier(
        &self,
        after_encoder_stages: Stages,
        before_encoder_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterEncoderStages:beforeEncoderStages:visibilityOptions:\0"),
                after_encoder_stages.0,
                before_encoder_stages.0,
                visibility.0,
            );
        }
    }

    pub fn update_fence(&self, fence: &Fence, after_encoder_stages: Stages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"updateFence:afterEncoderStages:\0"),
                fence.raw,
                after_encoder_stages.0,
            );
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence, before_encoder_stages: Stages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"waitForFence:beforeEncoderStages:\0"),
                fence.raw,
                before_encoder_stages.0,
            );
        }
    }
}

impl Drop for M4CommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4CommandQueue.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4CommandQueueError {
    None = 0,
    Timeout = 1,
    NotPermitted = 2,
    OutOfMemory = 3,
    DeviceRemoved = 4,
    AccessRevoked = 5,
    Internal = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M4UpdateSparseTextureMappingOperation {
    pub mode: SparseTextureMappingMode,
    pub texture_region: Region,
    pub texture_level: usize,
    pub texture_slice: usize,
    pub heap_offset: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M4CopySparseTextureMappingOperation {
    pub source_region: Region,
    pub source_level: usize,
    pub source_slice: usize,
    pub destination_origin: Origin,
    pub destination_level: usize,
    pub destination_slice: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M4UpdateSparseBufferMappingOperation {
    pub mode: SparseTextureMappingMode,
    pub buffer_range: Range,
    pub heap_offset: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M4CopySparseBufferMappingOperation {
    pub source_range: Range,
    pub destination_offset: usize,
}

#[derive(Debug)]
pub struct M4CommitOptions {
    pub raw: id,
}

impl M4CommitOptions {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CommitOptions\0"),
        }
    }
}

impl Default for M4CommitOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CommitOptions {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4CommandQueueDescriptor {
    pub raw: id,
}

impl M4CommandQueueDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CommandQueueDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }
}

impl Default for M4CommandQueueDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CommandQueueDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4CommandQueue {
    pub raw: id,
}

impl M4CommandQueue {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn commit(&self, command_buffers: &[M4CommandBuffer]) {
        if command_buffers.is_empty() {
            return;
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"commit:count:\0"),
                command_buffers.as_ptr() as *const id,
                command_buffers.len(),
            );
        }
    }

    pub fn commit_with_options(
        &self,
        command_buffers: &[M4CommandBuffer],
        options: &M4CommitOptions,
    ) {
        if command_buffers.is_empty() {
            return;
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize, id) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"commit:count:options:\0"),
                command_buffers.as_ptr() as *const id,
                command_buffers.len(),
                options.raw,
            );
        }
    }

    pub fn signal_event(&self, event: &Event, value: u64) {
        msg_void_id_u64(self.raw, sel(b"signalEvent:value:\0"), event.raw, value);
    }

    pub fn wait_for_event(&self, event: &Event, value: u64) {
        msg_void_id_u64(self.raw, sel(b"waitForEvent:value:\0"), event.raw, value);
    }

    pub fn signal_drawable(&self, drawable: &Drawable) {
        msg_void_id(self.raw, sel(b"signalDrawable:\0"), drawable.raw);
    }

    pub fn wait_for_drawable(&self, drawable: &Drawable) {
        msg_void_id(self.raw, sel(b"waitForDrawable:\0"), drawable.raw);
    }

    pub fn add_residency_set(&self, residency_set: &ResidencySet) {
        msg_void_id(self.raw, sel(b"addResidencySet:\0"), residency_set.raw);
    }

    pub fn remove_residency_set(&self, residency_set: &ResidencySet) {
        msg_void_id(self.raw, sel(b"removeResidencySet:\0"), residency_set.raw);
    }
}

impl Drop for M4CommandQueue {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4CommitFeedback.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4CommitFeedback {
    pub raw: id,
}

impl M4CommitFeedback {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn error_description(&self) -> Option<NSString> {
        let error = msg_id(self.raw, sel(b"error\0"));
        if error.is_null() {
            None
        } else {
            let description = msg_id(error, sel(b"localizedDescription\0"));
            if description.is_null() {
                None
            } else {
                Some(NSString::from_raw(description))
            }
        }
    }

    pub fn gpu_start_time(&self) -> f64 {
        msg_f64(self.raw, sel(b"GPUStartTime\0"))
    }

    pub fn gpu_end_time(&self) -> f64 {
        msg_f64(self.raw, sel(b"GPUEndTime\0"))
    }
}

impl Drop for M4CommitFeedback {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4Compiler.h / MTL4CompilerTask.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4CompilerDescriptor {
    pub raw: id,
}

impl M4CompilerDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CompilerDescriptor\0"),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn set_label(&self, label: &str) {
        m4_set_label(self.raw, label);
    }

    pub fn pipeline_data_set_serializer(&self) -> Option<M4PipelineDataSetSerializer> {
        m4_optional_id(self.raw, sel(b"pipelineDataSetSerializer\0"))
            .map(|raw| M4PipelineDataSetSerializer { raw })
    }

    pub fn set_pipeline_data_set_serializer(
        &self,
        serializer: Option<&M4PipelineDataSetSerializer>,
    ) {
        m4_set_optional_id(
            self.raw,
            sel(b"setPipelineDataSetSerializer:\0"),
            serializer.map(|s| s.raw),
        );
    }
}

impl Default for M4CompilerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CompilerDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4CompilerTaskOptions {
    pub raw: id,
}

impl M4CompilerTaskOptions {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4CompilerTaskOptions\0"),
        }
    }

    pub fn lookup_archives(&self) -> NSArrayIterator<M4Archive> {
        m4_archives_from_array(msg_id(self.raw, sel(b"lookupArchives\0")))
    }

    pub fn set_lookup_archives(&self, archives: &[M4Archive]) {
        let raw_ptrs =
            unsafe { std::slice::from_raw_parts(archives.as_ptr() as *const id, archives.len()) };
        msg_void_id(
            self.raw,
            sel(b"setLookupArchives:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }
}

impl Default for M4CompilerTaskOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4CompilerTaskOptions {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum M4CompilerTaskStatus {
    None = 0,
    Scheduled = 1,
    Compiling = 2,
    Finished = 3,
}

#[derive(Debug)]
pub struct M4CompilerTask {
    pub raw: id,
}

impl M4CompilerTask {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn compiler(&self) -> M4Compiler {
        M4Compiler {
            raw: retain(msg_id(self.raw, sel(b"compiler\0"))),
        }
    }

    pub fn status(&self) -> M4CompilerTaskStatus {
        match msg_usize(self.raw, sel(b"status\0")) {
            1 => M4CompilerTaskStatus::Scheduled,
            2 => M4CompilerTaskStatus::Compiling,
            3 => M4CompilerTaskStatus::Finished,
            _ => M4CompilerTaskStatus::None,
        }
    }

    pub fn wait_until_completed(&self) {
        msg_void(self.raw, sel(b"waitUntilCompleted\0"));
    }
}

impl Drop for M4CompilerTask {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4Compiler {
    pub raw: id,
}

impl M4Compiler {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<NSString> {
        m4_label(self.raw)
    }

    pub fn pipeline_data_set_serializer(&self) -> Option<M4PipelineDataSetSerializer> {
        m4_optional_id(self.raw, sel(b"pipelineDataSetSerializer\0"))
            .map(|raw| M4PipelineDataSetSerializer { raw })
    }

    pub fn new_library_with_descriptor(
        &self,
        descriptor: &M4LibraryDescriptor,
    ) -> Result<Library, MetalError> {
        let mut error = NIL;
        let raw = retain(msg_id_id_err(
            self.raw,
            sel(b"newLibraryWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newLibraryWithDescriptor failed",
            )))
        } else {
            Ok(Library { raw })
        }
    }

    pub fn new_compute_pipeline_state(
        &self,
        descriptor: &M4ComputePipelineDescriptor,
        task_options: Option<&M4CompilerTaskOptions>,
    ) -> Result<ComputePipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newComputePipelineStateWithDescriptor:compilerTaskOptions:error:\0"),
                descriptor.raw,
                task_options.map_or(NIL, |o| o.raw),
                &mut error,
            ));
            if raw.is_null() {
                Err(MetalError::new(&error_message(
                    error,
                    "newComputePipelineStateWithDescriptor failed",
                )))
            } else {
                Ok(ComputePipelineState { raw })
            }
        }
    }

    pub fn new_render_pipeline_state(
        &self,
        descriptor: &M4PipelineDescriptor,
        task_options: Option<&M4CompilerTaskOptions>,
    ) -> Result<RenderPipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newRenderPipelineStateWithDescriptor:compilerTaskOptions:error:\0"),
                descriptor.raw,
                task_options.map_or(NIL, |o| o.raw),
                &mut error,
            ));
            if raw.is_null() {
                Err(MetalError::new(&error_message(
                    error,
                    "newRenderPipelineStateWithDescriptor failed",
                )))
            } else {
                Ok(RenderPipelineState { raw })
            }
        }
    }

    pub fn new_binary_function(
        &self,
        descriptor: &M4BinaryFunctionDescriptor,
        task_options: Option<&M4CompilerTaskOptions>,
    ) -> Result<M4BinaryFunction, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newBinaryFunctionWithDescriptor:compilerTaskOptions:error:\0"),
                descriptor.raw,
                task_options.map_or(NIL, |o| o.raw),
                &mut error,
            ));
            if raw.is_null() {
                Err(MetalError::new(&error_message(
                    error,
                    "newBinaryFunctionWithDescriptor failed",
                )))
            } else {
                Ok(M4BinaryFunction { raw })
            }
        }
    }

    pub fn new_machine_learning_pipeline_state(
        &self,
        descriptor: &M4MachineLearningPipelineDescriptor,
    ) -> Result<M4MachineLearningPipelineState, MetalError> {
        let mut error = NIL;
        let raw = retain(msg_id_id_err(
            self.raw,
            sel(b"newMachineLearningPipelineStateWithDescriptor:error:\0"),
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newMachineLearningPipelineStateWithDescriptor failed",
            )))
        } else {
            Ok(M4MachineLearningPipelineState { raw })
        }
    }
}

impl Drop for M4Compiler {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4PipelineDataSetSerializer.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4PipelineDataSetSerializerConfiguration(pub usize);

impl M4PipelineDataSetSerializerConfiguration {
    pub const CAPTURE_DESCRIPTORS: Self = Self(1 << 0);
    pub const CAPTURE_BINARIES: Self = Self(1 << 1);
}

#[derive(Debug)]
pub struct M4PipelineDataSetSerializerDescriptor {
    pub raw: id,
}

impl M4PipelineDataSetSerializerDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4PipelineDataSetSerializerDescriptor\0"),
        }
    }

    pub fn configuration(&self) -> M4PipelineDataSetSerializerConfiguration {
        M4PipelineDataSetSerializerConfiguration(msg_usize(self.raw, sel(b"configuration\0")))
    }

    pub fn set_configuration(&self, configuration: M4PipelineDataSetSerializerConfiguration) {
        msg_void_usize(self.raw, sel(b"setConfiguration:\0"), configuration.0);
    }
}

impl Default for M4PipelineDataSetSerializerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4PipelineDataSetSerializerDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4PipelineDataSetSerializer {
    pub raw: id,
}

impl M4PipelineDataSetSerializer {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn serialize_as_archive_and_flush_to_url(&self, url_path: &str) -> Result<(), MetalError> {
        let mut error = NIL;
        let url = ns_url_from_path(url_path);
        let ok = msg_bool_id_err(
            self.raw,
            sel(b"serializeAsArchiveAndFlushToURL:error:\0"),
            url,
            &mut error,
        );
        if ok == NO {
            Err(MetalError::new(&error_message(
                error,
                "serializeAsArchiveAndFlushToURL failed",
            )))
        } else {
            Ok(())
        }
    }

    pub fn serialize_as_pipelines_script(&self) -> Result<NSData, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let data = f(
                self.raw,
                sel(b"serializeAsPipelinesScriptWithError:\0"),
                &mut error,
            );
            if data.is_null() {
                return Err(MetalError::new(&error_message(
                    error,
                    "serializeAsPipelinesScriptWithError failed",
                )));
            }
            Ok(NSData::from_raw(data))
        }
    }
}

impl Drop for M4PipelineDataSetSerializer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4RenderPass.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4RenderPassDescriptor {
    pub raw: id,
}

impl M4RenderPassDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4RenderPassDescriptor\0"),
        }
    }

    pub fn color_attachments(&self) -> RenderPassColorAttachmentDescriptorArray {
        RenderPassColorAttachmentDescriptorArray {
            raw: retain(msg_id(self.raw, sel(b"colorAttachments\0"))),
        }
    }

    pub fn depth_attachment(&self) -> RenderPassDepthAttachmentDescriptor {
        RenderPassDepthAttachmentDescriptor {
            raw: retain(msg_id(self.raw, sel(b"depthAttachment\0"))),
        }
    }

    pub fn stencil_attachment(&self) -> RenderPassStencilAttachmentDescriptor {
        RenderPassStencilAttachmentDescriptor {
            raw: retain(msg_id(self.raw, sel(b"stencilAttachment\0"))),
        }
    }

    pub fn render_target_array_length(&self) -> usize {
        msg_usize(self.raw, sel(b"renderTargetArrayLength\0"))
    }

    pub fn set_render_target_array_length(&self, length: usize) {
        msg_void_usize(self.raw, sel(b"setRenderTargetArrayLength:\0"), length);
    }

    pub fn render_target_width(&self) -> usize {
        msg_usize(self.raw, sel(b"renderTargetWidth\0"))
    }

    pub fn set_render_target_width(&self, width: usize) {
        msg_void_usize(self.raw, sel(b"setRenderTargetWidth:\0"), width);
    }

    pub fn render_target_height(&self) -> usize {
        msg_usize(self.raw, sel(b"renderTargetHeight\0"))
    }

    pub fn set_render_target_height(&self, height: usize) {
        msg_void_usize(self.raw, sel(b"setRenderTargetHeight:\0"), height);
    }

    pub fn default_raster_sample_count(&self) -> usize {
        msg_usize(self.raw, sel(b"defaultRasterSampleCount\0"))
    }

    pub fn set_default_raster_sample_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setDefaultRasterSampleCount:\0"), count);
    }

    pub fn imageblock_sample_length(&self) -> usize {
        msg_usize(self.raw, sel(b"imageblockSampleLength\0"))
    }

    pub fn set_imageblock_sample_length(&self, length: usize) {
        msg_void_usize(self.raw, sel(b"setImageblockSampleLength:\0"), length);
    }

    pub fn threadgroup_memory_length(&self) -> usize {
        msg_usize(self.raw, sel(b"threadgroupMemoryLength\0"))
    }

    pub fn set_threadgroup_memory_length(&self, length: usize) {
        msg_void_usize(self.raw, sel(b"setThreadgroupMemoryLength:\0"), length);
    }

    pub fn visibility_result_buffer(&self) -> Option<Buffer> {
        m4_optional_id(self.raw, sel(b"visibilityResultBuffer\0")).map(|raw| Buffer { raw })
    }

    pub fn set_visibility_result_buffer(&self, buffer: Option<&Buffer>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setVisibilityResultBuffer:\0"),
            buffer.map(|b| b.raw),
        );
    }

    pub fn visibility_result_type(&self) -> VisibilityResultType {
        match msg_usize(self.raw, sel(b"visibilityResultType\0")) {
            1 => VisibilityResultType::Accumulate,
            _ => VisibilityResultType::Reset,
        }
    }

    pub fn set_visibility_result_type(&self, kind: VisibilityResultType) {
        msg_void_usize(self.raw, sel(b"setVisibilityResultType:\0"), kind as usize);
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

    pub fn support_color_attachment_mapping(&self) -> bool {
        msg_bool(self.raw, sel(b"supportColorAttachmentMapping\0")) != NO
    }

    pub fn set_support_color_attachment_mapping(&self, value: bool) {
        msg_void_bool(
            self.raw,
            sel(b"setSupportColorAttachmentMapping:\0"),
            if value { YES } else { NO },
        );
    }
}

impl Default for M4RenderPassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4RenderPassDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4ComputeCommandEncoder.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4ComputeCommandEncoder {
    pub raw: id,
}

impl M4ComputeCommandEncoder {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn stages(&self) -> Stages {
        Stages(msg_usize(self.raw, sel(b"stages\0")))
    }

    pub fn barrier_after_queue_stages(
        &self,
        after_queue_stages: Stages,
        before_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterQueueStages:beforeStages:visibilityOptions:\0"),
                after_queue_stages.0,
                before_stages.0,
                visibility.0,
            );
        }
    }

    pub fn barrier_before_queue_stages(
        &self,
        after_stages: Stages,
        before_queue_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterStages:beforeQueueStages:visibilityOptions:\0"),
                after_stages.0,
                before_queue_stages.0,
                visibility.0,
            );
        }
    }

    pub fn barrier(
        &self,
        after_encoder_stages: Stages,
        before_encoder_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterEncoderStages:beforeEncoderStages:visibilityOptions:\0"),
                after_encoder_stages.0,
                before_encoder_stages.0,
                visibility.0,
            );
        }
    }

    pub fn update_fence(&self, fence: &Fence, after_encoder_stages: Stages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"updateFence:afterEncoderStages:\0"),
                fence.raw,
                after_encoder_stages.0,
            );
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence, before_encoder_stages: Stages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"waitForFence:beforeEncoderStages:\0"),
                fence.raw,
                before_encoder_stages.0,
            );
        }
    }

    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineState) {
        msg_void_id(self.raw, sel(b"setComputePipelineState:\0"), state.raw);
    }

    pub fn set_argument_table(&self, argument_table: Option<&M4ArgumentTable>) {
        m4_set_optional_id(
            self.raw,
            sel(b"setArgumentTable:\0"),
            argument_table.map(|t| t.raw),
        );
    }

    pub fn dispatch_threads(&self, threads_per_grid: Size, threads_per_threadgroup: Size) {
        msg_void_size_size(
            self.raw,
            sel(b"dispatchThreads:threadsPerThreadgroup:\0"),
            threads_per_grid,
            threads_per_threadgroup,
        );
    }

    pub fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: Size,
        threads_per_threadgroup: Size,
    ) {
        msg_void_size_size(
            self.raw,
            sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
            threadgroups_per_grid,
            threads_per_threadgroup,
        );
    }

    pub fn dispatch_threadgroups_with_indirect_buffer(
        &self,
        indirect_buffer: u64,
        threads_per_threadgroup: Size,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u64, Size) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"dispatchThreadgroupsWithIndirectBuffer:threadsPerThreadgroup:\0"),
                indirect_buffer,
                threads_per_threadgroup,
            );
        }
    }

    pub fn dispatch_threads_with_indirect_buffer(&self, indirect_buffer: u64) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u64) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"dispatchThreadsWithIndirectBuffer:\0"),
                indirect_buffer,
            );
        }
    }

    pub fn execute_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"executeCommandsInBuffer:withRange:\0"),
                buffer.raw,
                range,
            );
        }
    }

    pub fn execute_commands_in_buffer_with_indirect_buffer(
        &self,
        buffer: &IndirectCommandBuffer,
        indirect_range_buffer: u64,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, u64) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"executeCommandsInBuffer:indirectBuffer:\0"),
                buffer.raw,
                indirect_range_buffer,
            );
        }
    }

    pub fn reset_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"resetCommandsInBuffer:withRange:\0"),
                buffer.raw,
                range,
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

    pub fn copy_textures(&self, source: &Texture, destination: &Texture) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, id) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyFromTexture:toTexture:\0"),
                source.raw,
                destination.raw,
            );
        }
    }

    pub fn copy_texture_to_texture(
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
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize, id, usize, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:\0"),
                source.raw,
                source_slice,
                source_level,
                destination.raw,
                destination_slice,
                destination_level,
                slice_count,
                level_count,
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

    pub fn optimize_contents_for_gpu_access(&self, texture: &Texture) {
        msg_void_id(
            self.raw,
            sel(b"optimizeContentsForGPUAccess:\0"),
            texture.raw,
        );
    }

    pub fn optimize_contents_for_cpu_access(&self, texture: &Texture) {
        msg_void_id(
            self.raw,
            sel(b"optimizeContentsForCPUAccess:\0"),
            texture.raw,
        );
    }

    pub fn write_timestamp_with_granularity(
        &self,
        granularity: M4TimestampGranularity,
        counter_heap: &M4CounterHeap,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, isize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"writeTimestampWithGranularity:intoHeap:atIndex:\0"),
                granularity as isize,
                counter_heap.raw,
                index,
            );
        }
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for M4ComputeCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4RenderCommandEncoder.h
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4RenderEncoderOptions(pub usize);

impl M4RenderEncoderOptions {
    pub const NONE: Self = Self(0);
    pub const SUSPENDING: Self = Self(1 << 0);
    pub const RESUMING: Self = Self(1 << 1);
}

#[derive(Debug)]
pub struct M4RenderCommandEncoder {
    pub raw: id,
}

impl M4RenderCommandEncoder {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn barrier_after_queue_stages(
        &self,
        after_queue_stages: Stages,
        before_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterQueueStages:beforeStages:visibilityOptions:\0"),
                after_queue_stages.0,
                before_stages.0,
                visibility.0,
            );
        }
    }

    pub fn barrier_before_queue_stages(
        &self,
        after_stages: Stages,
        before_queue_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterStages:beforeQueueStages:visibilityOptions:\0"),
                after_stages.0,
                before_queue_stages.0,
                visibility.0,
            );
        }
    }

    pub fn barrier(
        &self,
        after_encoder_stages: Stages,
        before_encoder_stages: Stages,
        visibility: M4VisibilityOptions,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"barrierAfterEncoderStages:beforeEncoderStages:visibilityOptions:\0"),
                after_encoder_stages.0,
                before_encoder_stages.0,
                visibility.0,
            );
        }
    }

    pub fn update_fence(&self, fence: &Fence, after_encoder_stages: Stages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"updateFence:afterEncoderStages:\0"),
                fence.raw,
                after_encoder_stages.0,
            );
        }
    }

    pub fn wait_for_fence(&self, fence: &Fence, before_encoder_stages: Stages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"waitForFence:beforeEncoderStages:\0"),
                fence.raw,
                before_encoder_stages.0,
            );
        }
    }

    pub fn tile_width(&self) -> usize {
        msg_usize(self.raw, sel(b"tileWidth\0"))
    }

    pub fn tile_height(&self) -> usize {
        msg_usize(self.raw, sel(b"tileHeight\0"))
    }

    pub fn set_render_pipeline_state(&self, pipeline_state: &RenderPipelineState) {
        msg_void_id(
            self.raw,
            sel(b"setRenderPipelineState:\0"),
            pipeline_state.raw,
        );
    }

    pub fn set_argument_table(&self, argument_table: &M4ArgumentTable, stages: RenderStages) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setArgumentTable:atStages:\0"),
                argument_table.raw,
                stages.0,
            );
        }
    }

    pub fn set_viewport(&self, viewport: Viewport) {
        msg_void_viewport(self.raw, sel(b"setViewport:\0"), viewport);
    }

    pub fn set_scissor_rect(&self, rect: ScissorRect) {
        msg_void_scissor_rect(self.raw, sel(b"setScissorRect:\0"), rect);
    }

    pub fn set_viewports(&self, viewports: &[Viewport]) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const Viewport, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setViewports:count:\0"),
                viewports.as_ptr(),
                viewports.len(),
            );
        }
    }

    pub fn set_scissor_rects(&self, rects: &[ScissorRect]) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const ScissorRect, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setScissorRects:count:\0"),
                rects.as_ptr(),
                rects.len(),
            );
        }
    }

    pub fn set_depth_stencil_state(&self, state: &DepthStencilState) {
        msg_void_id(self.raw, sel(b"setDepthStencilState:\0"), state.raw);
    }

    pub fn set_stencil_reference_value(&self, value: u32) {
        msg_void_usize(
            self.raw,
            sel(b"setStencilReferenceValue:\0"),
            value as usize,
        );
    }

    pub fn set_stencil_reference_values(&self, front: u32, back: u32) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u32, u32) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setStencilFrontReferenceValue:backReferenceValue:\0"),
                front,
                back,
            );
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

    pub fn set_cull_mode(&self, mode: CullMode) {
        msg_void_usize(self.raw, sel(b"setCullMode:\0"), mode as usize);
    }

    pub fn set_front_facing_winding(&self, winding: Winding) {
        msg_void_usize(self.raw, sel(b"setFrontFacingWinding:\0"), winding as usize);
    }

    pub fn set_triangle_fill_mode(&self, mode: TriangleFillMode) {
        msg_void_usize(self.raw, sel(b"setTriangleFillMode:\0"), mode as usize);
    }

    pub fn set_depth_clip_mode(&self, mode: DepthClipMode) {
        msg_void_usize(self.raw, sel(b"setDepthClipMode:\0"), mode as usize);
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

    pub fn draw_indexed_primitives(
        &self,
        primitive_type: PrimitiveType,
        index_count: usize,
        index_type: IndexType,
        index_buffer: u64,
        index_buffer_length: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, u64, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferLength:\0"),
                primitive_type as usize,
                index_count,
                index_type as usize,
                index_buffer,
                index_buffer_length,
            );
        }
    }

    pub fn draw_primitives_with_indirect_buffer(
        &self,
        primitive_type: PrimitiveType,
        indirect_buffer: u64,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, u64) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawPrimitives:indirectBuffer:\0"),
                primitive_type as usize,
                indirect_buffer,
            );
        }
    }

    pub fn draw_indexed_primitives_with_indirect_buffer(
        &self,
        primitive_type: PrimitiveType,
        index_type: IndexType,
        index_buffer: u64,
        index_buffer_length: usize,
        indirect_buffer: u64,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, u64, usize, u64) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawIndexedPrimitives:indexType:indexBuffer:indexBufferLength:indirectBuffer:\0"),
                primitive_type as usize,
                index_type as usize,
                index_buffer,
                index_buffer_length,
                indirect_buffer,
            );
        }
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

    pub fn draw_mesh_threadgroups_with_indirect_buffer(
        &self,
        indirect_buffer: u64,
        threads_per_object_threadgroup: Size,
        threads_per_mesh_threadgroup: Size,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, u64, Size, Size) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"drawMeshThreadgroupsWithIndirectBuffer:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:\0"),
                indirect_buffer,
                threads_per_object_threadgroup,
                threads_per_mesh_threadgroup,
            );
        }
    }

    pub fn execute_commands_in_buffer(&self, buffer: &IndirectCommandBuffer, range: Range) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, Range) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"executeCommandsInBuffer:withRange:\0"),
                buffer.raw,
                range,
            );
        }
    }

    pub fn execute_commands_in_buffer_with_indirect_buffer(
        &self,
        buffer: &IndirectCommandBuffer,
        indirect_range_buffer: u64,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, u64) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"executeCommandsInBuffer:indirectBuffer:\0"),
                buffer.raw,
                indirect_range_buffer,
            );
        }
    }

    pub fn write_timestamp_with_granularity(
        &self,
        granularity: M4TimestampGranularity,
        after_stage: RenderStages,
        counter_heap: &M4CounterHeap,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, isize, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"writeTimestampWithGranularity:afterStage:intoHeap:atIndex:\0"),
                granularity as isize,
                after_stage.0,
                counter_heap.raw,
                index,
            );
        }
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for M4RenderCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4MachineLearningCommandEncoder.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4MachineLearningCommandEncoder {
    pub raw: id,
}

impl M4MachineLearningCommandEncoder {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn set_pipeline_state(&self, pipeline_state: &M4MachineLearningPipelineState) {
        msg_void_id(self.raw, sel(b"setPipelineState:\0"), pipeline_state.raw);
    }

    pub fn set_argument_table(&self, argument_table: &M4ArgumentTable) {
        msg_void_id(self.raw, sel(b"setArgumentTable:\0"), argument_table.raw);
    }

    pub fn dispatch_network_with_intermediates_heap(&self, heap: &Heap) {
        msg_void_id(
            self.raw,
            sel(b"dispatchNetworkWithIntermediatesHeap:\0"),
            heap.raw,
        );
    }

    pub fn end_encoding(&self) {
        msg_void(self.raw, sel(b"endEncoding\0"));
    }
}

impl Drop for M4MachineLearningCommandEncoder {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// MTL4AccelerationStructure.h
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct M4AccelerationStructureDescriptor {
    pub raw: id,
}

impl M4AccelerationStructureDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4AccelerationStructureDescriptor\0"),
        }
    }
}

impl Default for M4AccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4AccelerationStructureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct M4AccelerationStructureGeometryDescriptor {
    pub raw: id,
}

impl Clone for M4AccelerationStructureGeometryDescriptor {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl M4AccelerationStructureGeometryDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4AccelerationStructureGeometryDescriptor\0"),
        }
    }

    pub fn base(&self) -> M4AccelerationStructureGeometryDescriptorBase {
        m4_geometry_base_get(self.raw)
    }

    pub fn set_base(&self, base: &M4AccelerationStructureGeometryDescriptorBase) {
        m4_geometry_base_set(self.raw, base);
    }
}

impl Default for M4AccelerationStructureGeometryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4AccelerationStructureGeometryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4PrimitiveAccelerationStructureDescriptor {
    pub raw: id,
}

impl M4PrimitiveAccelerationStructureDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4PrimitiveAccelerationStructureDescriptor\0"),
        }
    }

    pub fn geometry_descriptors(
        &self,
    ) -> NSArrayIterator<M4AccelerationStructureGeometryDescriptor> {
        NSArrayIterator::new(msg_id(self.raw, sel(b"geometryDescriptors\0")))
    }

    pub fn set_geometry_descriptors(
        &self,
        descriptors: &[M4AccelerationStructureGeometryDescriptor],
    ) {
        let raw_ptrs = unsafe {
            std::slice::from_raw_parts(descriptors.as_ptr() as *const id, descriptors.len())
        };
        msg_void_id(
            self.raw,
            sel(b"setGeometryDescriptors:\0"),
            ns_array_from_ids(raw_ptrs),
        );
    }

    pub fn motion_keyframe_count(&self) -> usize {
        msg_usize(self.raw, sel(b"motionKeyframeCount\0"))
    }

    pub fn set_motion_keyframe_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMotionKeyframeCount:\0"), count);
    }
}

impl Default for M4PrimitiveAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4PrimitiveAccelerationStructureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

macro_rules! m4_as_geometry_desc {
    ($name:ident, $class:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub raw: id,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    raw: m4_alloc_init($class),
                }
            }

            pub fn base(&self) -> M4AccelerationStructureGeometryDescriptorBase {
                m4_geometry_base_get(self.raw)
            }

            pub fn set_base(&self, base: &M4AccelerationStructureGeometryDescriptorBase) {
                m4_geometry_base_set(self.raw, base);
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                release(self.raw);
            }
        }
    };
}

m4_as_geometry_desc!(
    M4AccelerationStructureTriangleGeometryDescriptor,
    b"MTL4AccelerationStructureTriangleGeometryDescriptor\0"
);
m4_as_geometry_desc!(
    M4AccelerationStructureBoundingBoxGeometryDescriptor,
    b"MTL4AccelerationStructureBoundingBoxGeometryDescriptor\0"
);
m4_as_geometry_desc!(
    M4AccelerationStructureMotionTriangleGeometryDescriptor,
    b"MTL4AccelerationStructureMotionTriangleGeometryDescriptor\0"
);
m4_as_geometry_desc!(
    M4AccelerationStructureMotionBoundingBoxGeometryDescriptor,
    b"MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor\0"
);
m4_as_geometry_desc!(
    M4AccelerationStructureCurveGeometryDescriptor,
    b"MTL4AccelerationStructureCurveGeometryDescriptor\0"
);
m4_as_geometry_desc!(
    M4AccelerationStructureMotionCurveGeometryDescriptor,
    b"MTL4AccelerationStructureMotionCurveGeometryDescriptor\0"
);

impl M4AccelerationStructureTriangleGeometryDescriptor {
    pub fn vertex_buffer(&self) -> M4BufferRange {
        m4_buffer_range(self.raw, sel(b"vertexBuffer\0"))
    }

    pub fn set_vertex_buffer(&self, range: M4BufferRange) {
        m4_set_buffer_range(self.raw, sel(b"setVertexBuffer:\0"), range);
    }

    pub fn triangle_count(&self) -> usize {
        msg_usize(self.raw, sel(b"triangleCount\0"))
    }

    pub fn set_triangle_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setTriangleCount:\0"), count);
    }
}

impl M4AccelerationStructureBoundingBoxGeometryDescriptor {
    pub fn bounding_box_buffer(&self) -> M4BufferRange {
        m4_buffer_range(self.raw, sel(b"boundingBoxBuffer\0"))
    }

    pub fn set_bounding_box_buffer(&self, range: M4BufferRange) {
        m4_set_buffer_range(self.raw, sel(b"setBoundingBoxBuffer:\0"), range);
    }

    pub fn bounding_box_count(&self) -> usize {
        msg_usize(self.raw, sel(b"boundingBoxCount\0"))
    }

    pub fn set_bounding_box_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setBoundingBoxCount:\0"), count);
    }
}

#[derive(Debug)]
pub struct M4InstanceAccelerationStructureDescriptor {
    pub raw: id,
}

impl M4InstanceAccelerationStructureDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4InstanceAccelerationStructureDescriptor\0"),
        }
    }

    pub fn instance_descriptor_buffer(&self) -> M4BufferRange {
        m4_buffer_range(self.raw, sel(b"instanceDescriptorBuffer\0"))
    }

    pub fn set_instance_descriptor_buffer(&self, range: M4BufferRange) {
        m4_set_buffer_range(self.raw, sel(b"setInstanceDescriptorBuffer:\0"), range);
    }

    pub fn instance_count(&self) -> usize {
        msg_usize(self.raw, sel(b"instanceCount\0"))
    }

    pub fn set_instance_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setInstanceCount:\0"), count);
    }

    pub fn instance_descriptor_type(&self) -> AccelerationStructureInstanceDescriptorType {
        match msg_usize(self.raw, sel(b"instanceDescriptorType\0")) {
            4 => AccelerationStructureInstanceDescriptorType::IndirectMotion,
            3 => AccelerationStructureInstanceDescriptorType::Indirect,
            _ => AccelerationStructureInstanceDescriptorType::Default,
        }
    }

    pub fn set_instance_descriptor_type(
        &self,
        descriptor_type: AccelerationStructureInstanceDescriptorType,
    ) {
        msg_void_usize(
            self.raw,
            sel(b"setInstanceDescriptorType:\0"),
            descriptor_type as usize,
        );
    }
}

impl Default for M4InstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4InstanceAccelerationStructureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct M4IndirectInstanceAccelerationStructureDescriptor {
    pub raw: id,
}

impl M4IndirectInstanceAccelerationStructureDescriptor {
    pub fn new() -> Self {
        Self {
            raw: m4_alloc_init(b"MTL4IndirectInstanceAccelerationStructureDescriptor\0"),
        }
    }

    pub fn instance_descriptor_buffer(&self) -> M4BufferRange {
        m4_buffer_range(self.raw, sel(b"instanceDescriptorBuffer\0"))
    }

    pub fn set_instance_descriptor_buffer(&self, range: M4BufferRange) {
        m4_set_buffer_range(self.raw, sel(b"setInstanceDescriptorBuffer:\0"), range);
    }

    pub fn max_instance_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxInstanceCount\0"))
    }

    pub fn set_max_instance_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxInstanceCount:\0"), count);
    }

    pub fn instance_count_buffer(&self) -> M4BufferRange {
        m4_buffer_range(self.raw, sel(b"instanceCountBuffer\0"))
    }

    pub fn set_instance_count_buffer(&self, range: M4BufferRange) {
        m4_set_buffer_range(self.raw, sel(b"setInstanceCountBuffer:\0"), range);
    }
}

impl Default for M4IndirectInstanceAccelerationStructureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for M4IndirectInstanceAccelerationStructureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// ---------------------------------------------------------------------------
// Device creation APIs (MTLDevice.h)
// ---------------------------------------------------------------------------

impl Device {
    pub fn new_m4_command_allocator(&self) -> Option<M4CommandAllocator> {
        let selector = sel(b"newCommandAllocator\0");
        if !responds_to_selector(self.raw, selector) {
            return None;
        }
        let raw = msg_id(self.raw, selector);
        (!raw.is_null()).then(|| M4CommandAllocator { raw: retain(raw) })
    }

    pub fn new_m4_command_allocator_with_descriptor(
        &self,
        descriptor: &M4CommandAllocatorDescriptor,
    ) -> Result<M4CommandAllocator, MetalError> {
        let mut error = NIL;
        let selector = sel(b"newCommandAllocatorWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newCommandAllocatorWithDescriptor:error: not supported",
            ));
        }
        let raw = retain(msg_id_id_err(
            self.raw,
            selector,
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newCommandAllocatorWithDescriptor failed",
            )))
        } else {
            Ok(M4CommandAllocator { raw })
        }
    }

    pub fn new_m4_command_queue(&self) -> Option<M4CommandQueue> {
        let selector = sel(b"newMTL4CommandQueue\0");
        if !responds_to_selector(self.raw, selector) {
            return None;
        }
        let raw = msg_id(self.raw, selector);
        (!raw.is_null()).then(|| M4CommandQueue { raw: retain(raw) })
    }

    pub fn new_m4_command_queue_with_descriptor(
        &self,
        descriptor: &M4CommandQueueDescriptor,
    ) -> Result<M4CommandQueue, MetalError> {
        let mut error = NIL;
        let selector = sel(b"newMTL4CommandQueueWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newMTL4CommandQueueWithDescriptor:error: not supported",
            ));
        }
        let raw = retain(msg_id_id_err(
            self.raw,
            selector,
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newMTL4CommandQueueWithDescriptor failed",
            )))
        } else {
            Ok(M4CommandQueue { raw })
        }
    }

    pub fn new_m4_command_buffer(&self) -> Option<M4CommandBuffer> {
        let selector = sel(b"newCommandBuffer\0");
        if !responds_to_selector(self.raw, selector) {
            return None;
        }
        let raw = msg_id(self.raw, selector);
        (!raw.is_null()).then(|| M4CommandBuffer { raw: retain(raw) })
    }

    pub fn new_m4_argument_table_with_descriptor(
        &self,
        descriptor: &M4ArgumentTableDescriptor,
    ) -> Result<M4ArgumentTable, MetalError> {
        let mut error = NIL;
        let selector = sel(b"newArgumentTableWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newArgumentTableWithDescriptor:error: not supported",
            ));
        }
        let raw = retain(msg_id_id_err(
            self.raw,
            selector,
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newArgumentTableWithDescriptor failed",
            )))
        } else {
            Ok(M4ArgumentTable { raw })
        }
    }

    pub fn new_m4_compiler_with_descriptor(
        &self,
        descriptor: &M4CompilerDescriptor,
    ) -> Result<M4Compiler, MetalError> {
        let mut error = NIL;
        let selector = sel(b"newCompilerWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newCompilerWithDescriptor:error: not supported",
            ));
        }
        let raw = retain(msg_id_id_err(
            self.raw,
            selector,
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newCompilerWithDescriptor failed",
            )))
        } else {
            Ok(M4Compiler { raw })
        }
    }

    pub fn new_m4_archive_with_url(&self, url_path: &str) -> Result<M4Archive, MetalError> {
        let mut error = NIL;
        let selector = sel(b"newArchiveWithURL:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("newArchiveWithURL:error: not supported"));
        }
        let url = ns_url_from_path(url_path);
        let raw = retain(msg_id_id_err(self.raw, selector, url, &mut error));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newArchiveWithURL failed",
            )))
        } else {
            Ok(M4Archive { raw })
        }
    }

    pub fn new_m4_pipeline_data_set_serializer_with_descriptor(
        &self,
        descriptor: &M4PipelineDataSetSerializerDescriptor,
    ) -> Result<M4PipelineDataSetSerializer, MetalError> {
        let selector = sel(b"newPipelineDataSetSerializerWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newPipelineDataSetSerializerWithDescriptor: not supported",
            ));
        }
        let raw = retain(msg_id_id(self.raw, selector, descriptor.raw));
        if raw.is_null() {
            Err(MetalError::new(
                "newPipelineDataSetSerializerWithDescriptor failed",
            ))
        } else {
            Ok(M4PipelineDataSetSerializer { raw })
        }
    }

    pub fn new_m4_counter_heap_with_descriptor(
        &self,
        descriptor: &M4CounterHeapDescriptor,
    ) -> Result<M4CounterHeap, MetalError> {
        let mut error = NIL;
        let selector = sel(b"newCounterHeapWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newCounterHeapWithDescriptor:error: not supported",
            ));
        }
        let raw = retain(msg_id_id_err(
            self.raw,
            selector,
            descriptor.raw,
            &mut error,
        ));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "newCounterHeapWithDescriptor failed",
            )))
        } else {
            Ok(M4CounterHeap { raw })
        }
    }

    pub fn size_of_m4_counter_heap_entry(&self, heap_type: M4CounterHeapType) -> usize {
        let selector = sel(b"sizeOfCounterHeapEntry:\0");
        if !responds_to_selector(self.raw, selector) {
            return 0;
        }
        msg_usize_usize(self.raw, selector, heap_type as usize)
    }

    pub fn function_handle_with_m4_binary_function(
        &self,
        function: &M4BinaryFunction,
    ) -> Result<FunctionHandle, MetalError> {
        let mut error = NIL;
        let selector = sel(b"functionHandleWithBinaryFunction:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "functionHandleWithBinaryFunction:error: not supported",
            ));
        }
        let raw = retain(msg_id_id_err(self.raw, selector, function.raw, &mut error));
        if raw.is_null() {
            Err(MetalError::new(&error_message(
                error,
                "functionHandleWithBinaryFunction failed",
            )))
        } else {
            Ok(FunctionHandle { raw })
        }
    }
}
