use crate::*;
use std::cell::RefCell;
use std::ffi::c_void;
use std::mem::{size_of, transmute};
use std::ptr;

#[link(name = "Metal", kind = "framework")]
#[link(name = "System", kind = "framework")]
unsafe extern "C" {
    fn MTLCopyAllDevices() -> id;
    fn MTLCopyAllDevicesWithObserver(observer: *mut id, handler: id) -> id;
    fn MTLRemoveDeviceObserver(observer: id);
    static MTLDeviceWasAddedNotification: id;
    static MTLDeviceRemovalRequestedNotification: id;
    static MTLDeviceWasRemovedNotification: id;
    static _NSConcreteGlobalBlock: c_void;
    static _NSConcreteStackBlock: c_void;
    static _NSConcreteMallocBlock: c_void;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineOption(pub usize);

impl PipelineOption {
    pub const NONE: Self = Self(0);
    pub const BINDING_INFO: Self = Self(1 << 0);
    pub const BUFFER_TYPE_INFO: Self = Self(1 << 1);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PatchType {
    None = 0,
    Triangle = 1,
    Quad = 2,
}

impl PatchType {
    fn from_raw(raw: usize) -> Self {
        match raw {
            1 => Self::Triangle,
            2 => Self::Quad,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceNotificationName {
    WasAdded,
    RemovalRequested,
    WasRemoved,
}

fn device_notification_name_from_raw(raw: id) -> DeviceNotificationName {
    unsafe {
        if raw == MTLDeviceWasAddedNotification {
            DeviceNotificationName::WasAdded
        } else if raw == MTLDeviceRemovalRequestedNotification {
            DeviceNotificationName::RemovalRequested
        } else if raw == MTLDeviceWasRemovedNotification {
            DeviceNotificationName::WasRemoved
        } else {
            DeviceNotificationName::WasAdded
        }
    }
}

#[repr(C)]
struct BlockDescriptor {
    reserved: u64,
    size: u64,
}

#[repr(C)]
struct GlobalBlock {
    isa: *const c_void,
    flags: i32,
    reserved: i32,
    invoke: unsafe extern "C" fn(*mut c_void, id, id),
    descriptor: *const BlockDescriptor,
}

static DEVICE_NOTIFICATION_BLOCK_DESCRIPTOR: BlockDescriptor = BlockDescriptor {
    reserved: 0,
    size: size_of::<GlobalBlock>() as u64,
};

thread_local! {
    static DEVICE_NOTIFICATION_HANDLER: RefCell<Option<*const dyn Fn(Device, DeviceNotificationName)>> =
        const { RefCell::new(None) };
}

unsafe extern "C" fn device_notification_block_invoke(_block: *mut c_void, device: id, name: id) {
    DEVICE_NOTIFICATION_HANDLER.with(|slot| {
        if let Some(handler_ptr) = *slot.borrow() {
            let device = Device {
                raw: retain(device),
            };
            unsafe {
                (*handler_ptr)(device, device_notification_name_from_raw(name));
            }
        }
    });
}

fn device_notification_block() -> id {
    unsafe {
        static mut BLOCK: GlobalBlock = GlobalBlock {
            isa: ptr::null(),
            flags: 1 << 28,
            reserved: 0,
            invoke: device_notification_block_invoke,
            descriptor: &DEVICE_NOTIFICATION_BLOCK_DESCRIPTOR,
        };
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            BLOCK.isa = &_NSConcreteGlobalBlock as *const c_void;
        });
        id(&raw const BLOCK as *mut c_void)
    }
}

type CompletedHandlerFn = Box<dyn FnOnce(&CommandBuffer) + Send>;

#[repr(C)]
struct CompletedHandlerBlock {
    isa: *const c_void,
    flags: i32,
    reserved: i32,
    invoke: unsafe extern "C" fn(*mut CompletedHandlerBlock, id),
    descriptor: *const CompletedHandlerBlockDescriptor,
    closure: *mut CompletedHandlerFn,
}

#[repr(C)]
struct CompletedHandlerBlockDescriptor {
    reserved: u64,
    size: u64,
    copy_helper: unsafe extern "C" fn(*mut c_void, *mut c_void),
    dispose_helper: unsafe extern "C" fn(*mut c_void),
    signature: *const std::ffi::c_char,
}

unsafe impl Sync for CompletedHandlerBlockDescriptor {}

static COMPLETED_HANDLER_SIGNATURE: &[u8] = b"v16@?0@\"<MTLCommandBuffer>\"8\0";

static COMPLETED_HANDLER_BLOCK_DESCRIPTOR: CompletedHandlerBlockDescriptor =
    CompletedHandlerBlockDescriptor {
        reserved: 0,
        size: size_of::<CompletedHandlerBlock>() as u64,
        copy_helper: completed_handler_block_copy,
        dispose_helper: completed_handler_block_dispose,
        signature: COMPLETED_HANDLER_SIGNATURE.as_ptr() as *const std::ffi::c_char,
    };

// Pointer field is POD from ObjC's perspective, so copy is a plain memcpy —
// the block runtime already does that for the whole struct. We only need
// dispose to free the closure if it was never invoked.
unsafe extern "C" fn completed_handler_block_copy(_dst: *mut c_void, _src: *mut c_void) {}

unsafe extern "C" fn completed_handler_block_dispose(src: *mut c_void) {
    let block = src as *mut CompletedHandlerBlock;
    unsafe {
        let ptr = (*block).closure;
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    }
}

unsafe extern "C" fn completed_handler_block_invoke(block: *mut CompletedHandlerBlock, cb_raw: id) {
    unsafe {
        let ptr = (*block).closure;
        if !ptr.is_null() {
            (*block).closure = ptr::null_mut();
            let f = *Box::from_raw(ptr);
            let cb = CommandBuffer {
                raw: retain(cb_raw),
            };
            f(&cb);
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub raw: id,
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for Device {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
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

    pub fn name(&self) -> NSString {
        let ptr = msg_id(self.raw, sel(b"name\0"));
        NSString::from_raw(ptr)
    }

    pub fn registry_id(&self) -> u64 {
        msg_u64(self.raw, sel(b"registryID\0"))
    }

    pub fn is_low_power(&self) -> bool {
        msg_bool(self.raw, sel(b"isLowPower\0")) != 0
    }

    pub fn is_headless(&self) -> bool {
        msg_bool(self.raw, sel(b"isHeadless\0")) != 0
    }

    pub fn is_removable(&self) -> bool {
        msg_bool(self.raw, sel(b"isRemovable\0")) != 0
    }

    pub fn has_unified_memory(&self) -> bool {
        msg_bool(self.raw, sel(b"hasUnifiedMemory\0")) != 0
    }

    pub fn recommended_max_working_set_size(&self) -> u64 {
        msg_u64(self.raw, sel(b"recommendedMaxWorkingSetSize\0"))
    }

    pub fn location(&self) -> DeviceLocation {
        let val = msg_usize(self.raw, sel(b"location\0"));
        match val {
            0 => DeviceLocation::BuiltIn,
            1 => DeviceLocation::Slot,
            2 => DeviceLocation::External,
            _ => DeviceLocation::Unspecified,
        }
    }

    pub fn location_number(&self) -> usize {
        msg_usize(self.raw, sel(b"locationNumber\0"))
    }

    pub fn max_threads_per_threadgroup(&self) -> Size {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"maxThreadsPerThreadgroup\0"))
        }
    }

    pub fn max_transfer_rate(&self) -> u64 {
        msg_u64(self.raw, sel(b"maxTransferRate\0"))
    }

    pub fn max_threadgroup_memory_length(&self) -> usize {
        msg_usize(self.raw, sel(b"maxThreadgroupMemoryLength\0"))
    }

    pub fn max_argument_buffer_sampler_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxArgumentBufferSamplerCount\0"))
    }

    pub fn max_buffer_length(&self) -> usize {
        msg_usize(self.raw, sel(b"maxBufferLength\0"))
    }

    pub fn maximum_concurrent_compilation_task_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maximumConcurrentCompilationTaskCount\0"))
    }

    pub fn current_allocated_size(&self) -> usize {
        msg_usize(self.raw, sel(b"currentAllocatedSize\0"))
    }

    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier {
        let val = msg_usize(self.raw, sel(b"readWriteTextureSupport\0"));
        match val {
            1 => ReadWriteTextureTier::Tier1,
            2 => ReadWriteTextureTier::Tier2,
            _ => ReadWriteTextureTier::None,
        }
    }

    pub fn argument_buffers_support(&self) -> ArgumentBuffersTier {
        let val = msg_usize(self.raw, sel(b"argumentBuffersSupport\0"));
        match val {
            1 => ArgumentBuffersTier::Tier2,
            _ => ArgumentBuffersTier::Tier1,
        }
    }

    pub fn depth24_stencil8_pixel_format_supported(&self) -> bool {
        msg_bool(self.raw, sel(b"isDepth24Stencil8PixelFormatSupported\0")) != 0
    }

    pub fn supports_32bit_float_filtering(&self) -> bool {
        msg_bool(self.raw, sel(b"supports32BitFloatFiltering\0")) != 0
    }

    pub fn supports_32bit_msaa(&self) -> bool {
        msg_bool(self.raw, sel(b"supports32BitMSAA\0")) != 0
    }

    pub fn supports_query_texture_lod(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsQueryTextureLOD\0")) != 0
    }

    pub fn supports_bc_texture_compression(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsBCTextureCompression\0")) != 0
    }

    pub fn supports_pull_model_interpolation(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsPullModelInterpolation\0")) != 0
    }

    pub fn supports_shader_barycentric_coordinates(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsShaderBarycentricCoordinates\0")) != 0
    }

    pub fn supports_dynamic_libraries(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsDynamicLibraries\0")) != 0
    }

    pub fn supports_render_dynamic_libraries(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsRenderDynamicLibraries\0")) != 0
    }

    pub fn supports_function_pointers(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsFunctionPointers\0")) != 0
    }

    pub fn supports_function_pointers_from_render(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsFunctionPointersFromRender\0")) != 0
    }

    pub fn supports_raytracing_from_render(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsRaytracingFromRender\0")) != 0
    }

    pub fn supports_primitive_motion_blur(&self) -> bool {
        msg_bool(self.raw, sel(b"supportsPrimitiveMotionBlur\0")) != 0
    }

    pub fn supports_family(&self, family: GPUFamily) -> bool {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, isize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"supportsFamily:\0"), family as isize) != 0
        }
    }

    pub fn supports_feature_set(&self, feature_set: FeatureSet) -> bool {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"supportsFeatureSet:\0"),
                feature_set as usize,
            ) != 0
        }
    }

    pub fn supports_texture_sample_count(&self, sample_count: usize) -> bool {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"supportsTextureSampleCount:\0"),
                sample_count,
            ) != 0
        }
    }

    pub fn supports_vertex_amplification_count(&self, count: usize) -> bool {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"supportsVertexAmplificationCount:\0"), count) != 0
        }
    }

    pub fn peer_group_id(&self) -> u64 {
        msg_u64(self.raw, sel(b"peerGroupID\0"))
    }

    pub fn peer_index(&self) -> u32 {
        msg_u32(self.raw, sel(b"peerIndex\0"))
    }

    pub fn peer_count(&self) -> u32 {
        msg_u32(self.raw, sel(b"peerCount\0"))
    }

    pub fn new_buffer_with_bytes_no_copy(
        &self,
        pointer: *mut c_void,
        length: usize,
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *mut c_void, usize, usize, id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newBufferWithBytesNoCopy:length:options:deallocator:\0"),
                pointer,
                length,
                options.as_raw(),
                NIL,
            ));
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create buffer with bytes no copy",
                ))
            } else {
                Ok(Buffer { raw })
            }
        }
    }

    pub fn new_shared_texture(
        &self,
        descriptor: &TextureDescriptor,
    ) -> Result<Texture, MetalError> {
        let raw = retain(msg_id_id(
            self.raw,
            sel(b"newSharedTextureWithDescriptor:\0"),
            descriptor.raw,
        ));
        if raw.is_null() {
            Err(MetalError::new("failed to create new shared texture"))
        } else {
            Ok(Texture { raw })
        }
    }

    pub fn new_shared_texture_with_handle(
        &self,
        shared_handle: &SharedTextureHandle,
    ) -> Result<Texture, MetalError> {
        let raw = retain(msg_id_id(
            self.raw,
            sel(b"newSharedTextureWithHandle:\0"),
            shared_handle.raw,
        ));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create new shared texture with handle",
            ))
        } else {
            Ok(Texture { raw })
        }
    }

    pub fn new_texture_with_iosurface(
        &self,
        descriptor: &TextureDescriptor,
        iosurface: *mut c_void,
        plane: usize,
    ) -> Result<Texture, MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, *mut c_void, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newTextureWithDescriptor:iosurface:plane:\0"),
                descriptor.raw,
                iosurface,
                plane,
            ));
            if raw.is_null() {
                Err(MetalError::new("failed to create texture with iosurface"))
            } else {
                Ok(Texture { raw })
            }
        }
    }

    pub fn new_dynamic_library(&self, library: &Library) -> Result<DynamicLibrary, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newDynamicLibrary:error:\0"),
                library.raw,
                &mut error,
            ));
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create dynamic library",
                )))
            } else {
                Ok(DynamicLibrary { raw })
            }
        }
    }

    pub fn new_dynamic_library_with_url(
        &self,
        url_path: &str,
    ) -> Result<DynamicLibrary, MetalError> {
        unsafe {
            let url = ns_url_from_path(url_path);
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                self.raw,
                sel(b"newDynamicLibraryWithURL:error:\0"),
                url,
                &mut error,
            ));
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create dynamic library from URL",
                )))
            } else {
                Ok(DynamicLibrary { raw })
            }
        }
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
                "failed to load Metal library from URL",
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
                    "failed to load Metal library from file",
                )))
            } else {
                Ok(Library { raw })
            }
        }
    }

    pub fn new_library_with_data(&self, data: &[u8]) -> Result<Library, MetalError> {
        let ns_data = ns_data_from_bytes(data);
        if ns_data.is_null() {
            return Err(MetalError::new("failed to create NSData for Metal library"));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(
            self.raw,
            sel(b"newLibraryWithData:error:\0"),
            ns_data,
            &mut error,
        );
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to load Metal library from data",
            )))
        } else {
            Ok(Library { raw })
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

    pub fn new_default_library_from_main_bundle(&self) -> Result<Library, MetalError> {
        self.new_default_library_with_bundle(ns_bundle_main())
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

    // Async completionHandler variants (newRenderPipelineStateWithDescriptor:options:completionHandler:,
    // newComputePipelineStateWithFunction:options:completionHandler:,
    // newComputePipelineStateWithDescriptor:options:completionHandler:, etc.) are intentionally skipped.

    pub fn new_render_pipeline_state_with_options(
        &self,
        descriptor: &RenderPipelineDescriptor,
        options: PipelineOption,
        reflection: Option<&mut Option<RenderPipelineReflection>>,
    ) -> Result<RenderPipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let mut reflection_out = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newRenderPipelineStateWithDescriptor:options:reflection:error:\0"),
                descriptor.raw,
                options.0,
                &mut reflection_out,
                &mut error,
            );
            if let Some(out) = reflection {
                *out = (!reflection_out.is_null()).then_some(RenderPipelineReflection {
                    raw: retain(reflection_out),
                });
            }
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal render pipeline state with options",
                )))
            } else {
                Ok(RenderPipelineState { raw })
            }
        }
    }

    pub fn new_render_pipeline_state_with_mesh_descriptor(
        &self,
        descriptor: &MeshRenderPipelineDescriptor,
    ) -> Result<RenderPipelineState, MetalError> {
        let selector = sel(b"newRenderPipelineStateWithMeshDescriptor:options:reflection:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newRenderPipelineStateWithMeshDescriptor: is not supported on this device",
            ));
        }
        unsafe {
            let mut error = NIL;
            let mut reflection_out = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                selector,
                descriptor.raw,
                0,
                &mut reflection_out,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal mesh render pipeline state",
                )))
            } else {
                Ok(RenderPipelineState { raw })
            }
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

    pub fn new_compute_pipeline_state_with_function_options(
        &self,
        function: &Function,
        options: PipelineOption,
        reflection: Option<&mut Option<ComputePipelineReflection>>,
    ) -> Result<ComputePipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let mut reflection_out = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newComputePipelineStateWithFunction:options:reflection:error:\0"),
                function.raw,
                options.0,
                &mut reflection_out,
                &mut error,
            );
            if let Some(out) = reflection {
                *out = (!reflection_out.is_null()).then_some(ComputePipelineReflection {
                    raw: retain(reflection_out),
                });
            }
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal compute pipeline state with function and options",
                )))
            } else {
                Ok(ComputePipelineState { raw })
            }
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

    pub fn new_compute_pipeline_state_with_descriptor_options(
        &self,
        descriptor: &ComputePipelineDescriptor,
        options: PipelineOption,
        reflection: Option<&mut Option<ComputePipelineReflection>>,
    ) -> Result<ComputePipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let mut reflection_out = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newComputePipelineStateWithDescriptor:options:reflection:error:\0"),
                descriptor.raw,
                options.0,
                &mut reflection_out,
                &mut error,
            );
            if let Some(out) = reflection {
                *out = (!reflection_out.is_null()).then_some(ComputePipelineReflection {
                    raw: retain(reflection_out),
                });
            }
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal compute pipeline state with descriptor and options",
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

    pub fn new_shared_event_with_handle(
        &self,
        handle: &SharedEventHandle,
    ) -> Result<SharedEvent, MetalError> {
        let raw = retain(msg_id_id(
            self.raw,
            sel(b"newSharedEventWithHandle:\0"),
            handle.raw,
        ));
        if raw.is_null() {
            Err(MetalError::new("failed to create shared event with handle"))
        } else {
            Ok(SharedEvent { raw })
        }
    }

    pub fn new_event(&self) -> Result<Event, MetalError> {
        let raw = retain(msg_id(self.raw, sel(b"newEvent\0")));
        if raw.is_null() {
            Err(MetalError::new("failed to create Metal event"))
        } else {
            Ok(Event { raw })
        }
    }

    pub fn new_indirect_command_buffer(
        &self,
        descriptor: &IndirectCommandBufferDescriptor,
        max_command_count: usize,
        options: IndirectCommandBufferOptions,
    ) -> Result<IndirectCommandBuffer, MetalError> {
        descriptor.validate()?;
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
                Ok(IndirectCommandBuffer::with_config(
                    raw,
                    descriptor.snapshot_config(max_command_count),
                ))
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
        descriptors: &[ArgumentDescriptor],
    ) -> Result<ArgumentEncoder, MetalError> {
        let raw_ptrs = unsafe {
            std::slice::from_raw_parts(descriptors.as_ptr() as *const id, descriptors.len())
        };
        let array = ns_array_from_ids(raw_ptrs);
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

    pub fn copy_all_devices() -> NSArrayIterator<Device> {
        unsafe {
            let array = MTLCopyAllDevices();
            let iter = NSArrayIterator::new(array);
            release(array);
            iter
        }
    }

    pub fn copy_all_devices_with_observer<F>(
        handler: F,
    ) -> Result<(NSArrayIterator<Device>, DeviceObserver), MetalError>
    where
        F: Fn(Device, DeviceNotificationName) + 'static,
    {
        let handler = Box::new(handler);
        let handler_ptr: *const dyn Fn(Device, DeviceNotificationName) = &*handler;
        if DEVICE_NOTIFICATION_HANDLER.with(|slot| slot.borrow().is_some()) {
            return Err(MetalError::new(
                "only one device notification observer may be active per thread",
            ));
        }
        DEVICE_NOTIFICATION_HANDLER.with(|slot| {
            *slot.borrow_mut() = Some(handler_ptr);
        });

        unsafe {
            let mut observer = NIL;
            let array = MTLCopyAllDevicesWithObserver(&mut observer, device_notification_block());
            if array.is_null() {
                DEVICE_NOTIFICATION_HANDLER.with(|slot| {
                    *slot.borrow_mut() = None;
                });
                return Err(MetalError::new(
                    "MTLCopyAllDevicesWithObserver is not supported on this macOS version",
                ));
            }
            let iter = NSArrayIterator::new(array);
            release(array);
            Ok((
                iter,
                DeviceObserver {
                    raw: observer,
                    _handler: handler,
                },
            ))
        }
    }

    pub fn remove_device_observer(observer: &DeviceObserver) {
        if !observer.raw.is_null() {
            unsafe {
                MTLRemoveDeviceObserver(observer.raw);
            }
        }
    }

    pub fn new_tensor_with_descriptor(
        &self,
        descriptor: &TensorDescriptor,
    ) -> Result<Tensor, MetalError> {
        let selector = sel(b"newTensorWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newTensorWithDescriptor:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal tensor",
            )))
        } else {
            Ok(Tensor { raw })
        }
    }

    pub fn new_texture_view_pool_with_descriptor(
        &self,
        descriptor: &ResourceViewPoolDescriptor,
    ) -> Result<TextureViewPool, MetalError> {
        let selector = sel(b"newTextureViewPoolWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newTextureViewPoolWithDescriptor:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal texture view pool",
            )))
        } else {
            Ok(TextureViewPool { raw })
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

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
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

    pub fn add_residency_sets(&self, residency_sets: &[ResidencySet]) {
        let selector = sel(b"addResidencySets:count:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    residency_sets.as_ptr() as *const id,
                    residency_sets.len(),
                );
            }
        }
    }

    pub fn remove_residency_set(&self, residency_set: &ResidencySet) {
        let selector = sel(b"removeResidencySet:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, residency_set.raw);
        }
    }

    pub fn remove_residency_sets(&self, residency_sets: &[ResidencySet]) {
        let selector = sel(b"removeResidencySets:count:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    residency_sets.as_ptr() as *const id,
                    residency_sets.len(),
                );
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
            Ok(ParallelRenderCommandEncoder::new(raw))
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

    pub fn add_completed_handler<F>(&self, f: F)
    where
        F: FnOnce(&CommandBuffer) + Send + 'static,
    {
        let closure: CompletedHandlerFn = Box::new(f);
        let closure_ptr = Box::into_raw(Box::new(closure));
        let mut block = CompletedHandlerBlock {
            isa: unsafe { &_NSConcreteStackBlock as *const c_void },
            flags: (1 << 25) | (1 << 30),
            reserved: 0,
            invoke: completed_handler_block_invoke,
            descriptor: &COMPLETED_HANDLER_BLOCK_DESCRIPTOR,
            closure: closure_ptr,
        };
        let selector = sel(b"addCompletedHandler:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, id(&mut block as *mut _ as *mut c_void));
        } else {
            // selector not supported: avoid leaking the closure
            unsafe {
                drop(Box::from_raw(closure_ptr));
            }
        }
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

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
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

    pub fn use_residency_sets(&self, residency_sets: &[ResidencySet]) {
        let selector = sel(b"useResidencySets:count:\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                    transmute(objc_msgSend as *const c_void);
                f(
                    self.raw,
                    selector,
                    residency_sets.as_ptr() as *const id,
                    residency_sets.len(),
                );
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
                    "failed to specialize Metal function with constants",
                )))
            } else {
                Ok(Function { raw })
            }
        }
    }

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
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

    pub fn install_name(&self) -> Result<Option<NSString>, MetalError> {
        let selector = sel(b"installName\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                Ok(None)
            } else {
                Ok(Some(NSString::from_raw(ptr)))
            }
        } else {
            Err(MetalError::new(
                "installName is not supported on this macOS version",
            ))
        }
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel(b"device\0")));
        Device { raw: ptr }
    }

    pub fn function_names(&self) -> NSArrayIterator<NSString> {
        NSArrayIterator::new(msg_id(self.raw, sel(b"functionNames\0")))
    }

    pub fn new_function_with_descriptor(
        &self,
        descriptor: &FunctionDescriptor,
    ) -> Result<Function, MetalError> {
        let selector = sel(b"newFunctionWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newFunctionWithDescriptor:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal function with descriptor",
            )))
        } else {
            Ok(Function { raw })
        }
    }

    // Async completionHandler variants (newFunctionWithDescriptor:completionHandler:,
    // newIntersectionFunctionWithDescriptor:completionHandler:, etc.) are intentionally skipped.

    pub fn new_intersection_function_with_descriptor(
        &self,
        descriptor: &IntersectionFunctionDescriptor,
    ) -> Result<Function, MetalError> {
        let selector = sel(b"newIntersectionFunctionWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newIntersectionFunctionWithDescriptor:error: is not supported on this macOS version",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create Metal intersection function with descriptor",
            )))
        } else {
            Ok(Function { raw })
        }
    }

    pub fn reflection_for_function_with_name(
        &self,
        name: &str,
    ) -> Result<FunctionReflection, MetalError> {
        let selector = sel(b"reflectionForFunctionWithName:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "reflectionForFunctionWithName: is not supported on this macOS version",
            ));
        }
        let ns_name = NSString::new(name);
        let raw = msg_id_id(self.raw, selector, ns_name.raw());
        if raw.is_null() {
            Err(MetalError::new(format!(
                "failed to load reflection for Metal function '{}'",
                name
            )))
        } else {
            Ok(FunctionReflection::new_with_raw(raw))
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Function {
    pub raw: id,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for Function {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl Function {
    pub fn name(&self) -> NSString {
        let ptr = msg_id(self.raw, sel(b"name\0"));
        NSString::from_raw(ptr)
    }

    pub fn label(&self) -> Option<NSString> {
        let selector = sel(b"label\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(NSString::from_raw(ptr))
            }
        } else {
            None
        }
    }

    pub fn set_label(&self, label: &str) {
        let selector = sel(b"setLabel:\0");
        if responds_to_selector(self.raw, selector) {
            let ns_label = NSString::new(label);
            msg_void_id(self.raw, selector, ns_label.raw());
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

    pub fn patch_type(&self) -> Result<PatchType, MetalError> {
        let selector = sel(b"patchType\0");
        if responds_to_selector(self.raw, selector) {
            Ok(PatchType::from_raw(msg_usize(self.raw, selector)))
        } else {
            Err(MetalError::new(
                "patchType is not supported on this macOS version",
            ))
        }
    }

    pub fn patch_control_point_count(&self) -> Result<isize, MetalError> {
        let selector = sel(b"patchControlPointCount\0");
        if responds_to_selector(self.raw, selector) {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL) -> isize =
                    transmute(objc_msgSend as *const c_void);
                Ok(f(self.raw, selector))
            }
        } else {
            Err(MetalError::new(
                "patchControlPointCount is not supported on this macOS version",
            ))
        }
    }

    pub fn options(&self) -> Result<FunctionOptions, MetalError> {
        let selector = sel(b"options\0");
        if responds_to_selector(self.raw, selector) {
            Ok(FunctionOptions(msg_usize(self.raw, selector)))
        } else {
            Err(MetalError::new(
                "options is not supported on this macOS version",
            ))
        }
    }

    pub fn vertex_attributes(&self) -> NSArrayIterator<VertexAttribute> {
        let array = msg_id(self.raw, sel(b"vertexAttributes\0"));
        NSArrayIterator::new(array)
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct VertexAttribute {
    pub raw: id,
}

impl Clone for VertexAttribute {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for VertexAttribute {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl VertexAttribute {
    pub fn name(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel(b"name\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn attribute_index(&self) -> usize {
        msg_usize(self.raw, sel(b"attributeIndex\0"))
    }

    pub fn attribute_type(&self) -> DataType {
        let selector = sel(b"attributeType\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            unsafe { transmute(val) }
        } else {
            DataType::None
        }
    }

    pub fn is_active(&self) -> bool {
        let selector = sel(b"isActive\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }
}

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct RenderPipelineReflection {
    pub raw: id,
}

impl RenderPipelineReflection {
    pub fn vertex_bindings(&self) -> NSArrayIterator<Binding> {
        let selector = sel(b"vertexBindings\0");
        if responds_to_selector(self.raw, selector) {
            crate::reflection::bindings_from_array(msg_id(self.raw, selector))
        } else {
            NSArrayIterator::new(NIL)
        }
    }

    pub fn fragment_bindings(&self) -> NSArrayIterator<Binding> {
        let selector = sel(b"fragmentBindings\0");
        if responds_to_selector(self.raw, selector) {
            crate::reflection::bindings_from_array(msg_id(self.raw, selector))
        } else {
            NSArrayIterator::new(NIL)
        }
    }
}

impl Drop for RenderPipelineReflection {
    fn drop(&mut self) {
        release(self.raw);
    }
}

pub struct DeviceObserver {
    pub raw: id,
    _handler: Box<dyn Fn(Device, DeviceNotificationName)>,
}

impl DeviceObserver {
    pub fn remove(&self) {
        Device::remove_device_observer(self);
    }
}

impl std::fmt::Debug for DeviceObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceObserver")
            .field("raw", &self.raw)
            .finish_non_exhaustive()
    }
}

impl Drop for DeviceObserver {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                MTLRemoveDeviceObserver(self.raw);
            }
        }
        DEVICE_NOTIFICATION_HANDLER.with(|slot| {
            *slot.borrow_mut() = None;
        });
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

    pub fn install_name(&self) -> Option<NSString> {
        let selector = sel(b"installName\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(NSString::from_raw(ptr))
            }
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

    pub fn logs(&self) -> NSArrayIterator<FunctionLog> {
        NSArrayIterator::new(self.raw)
    }
}

impl Drop for LogContainer {
    fn drop(&mut self) {
        release(self.raw);
    }
}
