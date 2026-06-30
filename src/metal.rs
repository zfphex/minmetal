#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::{CStr, c_char, c_void};
use std::mem::transmute;
use std::ptr;

type id = *mut c_void;
type Class = *mut c_void;
type SEL = *mut c_void;
type BOOL = i8;

const YES: BOOL = 1;
const NO: BOOL = 0;
const NIL: id = ptr::null_mut();

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Origin {
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Region {
    pub origin: Origin,
    pub size: Size,
}

impl Region {
    pub const fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: Origin::new(x, y, 0),
            size: Size::new(width, height, 1),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl ClearColor {
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct CGSize {
    width: f64,
    height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PixelFormat {
    Invalid = 0,
    Rgba8Unorm = 70,
    Bgra8Unorm = 80,
    Bgra8UnormSrgb = 81,
}

impl PixelFormat {
    const fn as_raw(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StorageMode {
    Shared = 0,
    Managed = 1,
    Private = 2,
}

impl StorageMode {
    const fn as_resource_bits(self) -> usize {
        (self as usize) << 4
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceOptions(usize);

impl ResourceOptions {
    pub const CPU_CACHE_MODE_DEFAULT: Self = Self(0);
    pub const STORAGE_MODE_SHARED: Self = Self(StorageMode::Shared.as_resource_bits());
    pub const STORAGE_MODE_MANAGED: Self = Self(StorageMode::Managed.as_resource_bits());
    pub const STORAGE_MODE_PRIVATE: Self = Self(StorageMode::Private.as_resource_bits());

    const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureUsage(usize);

impl TextureUsage {
    pub const UNKNOWN: Self = Self(0);
    pub const SHADER_READ: Self = Self(1);
    pub const SHADER_WRITE: Self = Self(1 << 1);
    pub const RENDER_TARGET: Self = Self(1 << 2);

    const fn as_raw(self) -> usize {
        self.0
    }
}

impl std::ops::BitOr for TextureUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

#[derive(Debug, Clone)]
pub struct MetalError {
    message: String,
}

impl MetalError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MetalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MetalError {}

#[link(name = "objc")]
#[link(name = "Foundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> Class;
    fn sel_registerName(name: *const c_char) -> SEL;
    fn objc_msgSend();
    fn MTLCreateSystemDefaultDevice() -> id;
}

unsafe fn class(name: &[u8]) -> Class {
    unsafe { objc_getClass(name.as_ptr() as *const c_char) }
}

unsafe fn sel(name: &[u8]) -> SEL {
    unsafe { sel_registerName(name.as_ptr() as *const c_char) }
}

unsafe fn msg_id(obj: id, selector: SEL) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

unsafe fn msg_id_id(obj: id, selector: SEL, arg: id) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

unsafe fn msg_void(obj: id, selector: SEL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector);
    }
}

unsafe fn msg_void_id(obj: id, selector: SEL, arg: id) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

unsafe fn msg_void_bool(obj: id, selector: SEL, arg: BOOL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, BOOL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

unsafe fn msg_void_usize(obj: id, selector: SEL, arg: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

unsafe fn msg_usize(obj: id, selector: SEL) -> usize {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> usize = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
unsafe fn msg_bool(obj: id, selector: SEL) -> BOOL {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> BOOL = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
unsafe fn msg_f64(obj: id, selector: SEL) -> f64 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> f64 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

unsafe fn msg_void_size(obj: id, selector: SEL, arg: CGSize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, CGSize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

unsafe fn msg_void_clear_color(obj: id, selector: SEL, arg: ClearColor) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ClearColor) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

unsafe fn msg_id_usize(obj: id, selector: SEL, arg: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

unsafe fn retain(obj: id) -> id {
    if !obj.is_null() {
        unsafe { msg_id(obj, sel(b"retain\0")) }
    } else {
        obj
    }
}

unsafe fn release(obj: id) {
    if !obj.is_null() {
        unsafe { msg_void(obj, sel(b"release\0")) };
    }
}

struct NSString {
    raw: id,
}

impl NSString {
    fn new(value: &str) -> Self {
        unsafe {
            let allocated = msg_id(class(b"NSString\0"), sel(b"alloc\0"));
            let init: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = init(
                allocated,
                sel(b"initWithBytes:length:encoding:\0"),
                value.as_ptr() as *const c_void,
                value.len(),
                4,
            );
            Self { raw }
        }
    }

    fn raw(&self) -> id {
        self.raw
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

unsafe fn ns_string_to_string(raw: id) -> Option<String> {
    if raw.is_null() {
        return None;
    }
    unsafe {
        let utf8: unsafe extern "C" fn(id, SEL) -> *const c_char =
            transmute(objc_msgSend as *const c_void);
        let ptr = utf8(raw, sel(b"UTF8String\0"));
        if ptr.is_null() {
            None
        } else {
            CStr::from_ptr(ptr).to_str().ok().map(ToOwned::to_owned)
        }
    }
}

unsafe fn error_message(error: id, fallback: &str) -> String {
    if error.is_null() {
        return fallback.to_string();
    }
    unsafe {
        let description = msg_id(error, sel(b"localizedDescription\0"));
        ns_string_to_string(description).unwrap_or_else(|| fallback.to_string())
    }
}

#[derive(Debug)]
pub struct Device {
    raw: id,
}

impl Device {
    pub fn system_default() -> Option<Self> {
        unsafe {
            let raw = MTLCreateSystemDefaultDevice();
            (!raw.is_null()).then_some(Self { raw })
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
                .unwrap_or_else(|| "Unknown Metal Device".to_string())
        }
    }

    pub fn new_command_queue(&self) -> Result<CommandQueue, MetalError> {
        unsafe {
            let raw = msg_id(self.raw, sel(b"newCommandQueue\0"));
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal command queue"))
            } else {
                Ok(CommandQueue { raw })
            }
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

    pub fn new_render_pipeline_state(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<RenderPipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
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
    }

    pub fn new_buffer(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
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
    }

    pub fn new_buffer_with_data<T>(
        &self,
        data: &[T],
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        unsafe {
            let length = std::mem::size_of_val(data);
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
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
    }

    pub fn new_texture(&self, descriptor: &TextureDescriptor) -> Result<Texture, MetalError> {
        unsafe {
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
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct CommandQueue {
    raw: id,
}

impl CommandQueue {
    pub fn command_buffer(&self) -> Result<CommandBuffer, MetalError> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"commandBuffer\0")));
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal command buffer"))
            } else {
                Ok(CommandBuffer { raw })
            }
        }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    raw: id,
}

impl CommandBuffer {
    pub fn render_command_encoder(
        &self,
        descriptor: &RenderPassDescriptor,
    ) -> Result<RenderCommandEncoder, MetalError> {
        unsafe {
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
    }

    pub fn blit_command_encoder(&self) -> Result<BlitCommandEncoder, MetalError> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"blitCommandEncoder\0")));
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal blit command encoder",
                ))
            } else {
                Ok(BlitCommandEncoder { raw })
            }
        }
    }

    pub fn present_drawable(&self, drawable: &Drawable) {
        unsafe {
            msg_void_id(self.raw, sel(b"presentDrawable:\0"), drawable.raw);
        }
    }

    pub fn commit(&self) {
        unsafe { msg_void(self.raw, sel(b"commit\0")) };
    }

    pub fn wait_until_completed(&self) {
        unsafe { msg_void(self.raw, sel(b"waitUntilCompleted\0")) };
    }

    pub fn status(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"status\0")) }
    }

    pub fn error(&self) -> Option<String> {
        unsafe {
            let error = msg_id(self.raw, sel(b"error\0"));
            (!error.is_null()).then(|| error_message(error, "Metal command buffer failed"))
        }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Library {
    raw: id,
}

impl Library {
    pub fn function(&self, name: &str) -> Result<Function, MetalError> {
        unsafe {
            let name = NSString::new(name);
            let raw = msg_id_id(self.raw, sel(b"newFunctionWithName:\0"), name.raw());
            if raw.is_null() {
                Err(MetalError::new("failed to load Metal function"))
            } else {
                Ok(Function { raw })
            }
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Function {
    raw: id,
}

impl Function {
    pub fn name(&self) -> String {
        unsafe {
            ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
                .unwrap_or_else(|| "unknown".to_string())
        }
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct RenderPipelineDescriptor {
    raw: id,
}

impl RenderPipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLRenderPipelineDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_vertex_function(&self, function: &Function) {
        unsafe {
            msg_void_id(self.raw, sel(b"setVertexFunction:\0"), function.raw);
        }
    }

    pub fn set_fragment_function(&self, function: &Function) {
        unsafe {
            msg_void_id(self.raw, sel(b"setFragmentFunction:\0"), function.raw);
        }
    }

    pub fn set_color_attachment_pixel_format(&self, index: usize, pixel_format: PixelFormat) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_usize(attachment, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
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
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct RenderPipelineState {
    raw: id,
}

impl Drop for RenderPipelineState {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct RenderPassDescriptor {
    raw: id,
}

impl RenderPassDescriptor {
    pub fn new() -> Self {
        unsafe {
            let raw = retain(msg_id(
                class(b"MTLRenderPassDescriptor\0"),
                sel(b"renderPassDescriptor\0"),
            ));
            Self { raw }
        }
    }

    pub fn set_color_attachment(
        &self,
        index: usize,
        texture: &Texture,
        load_action: LoadAction,
        store_action: StoreAction,
        clear_color: ClearColor,
    ) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_id(attachment, sel(b"setTexture:\0"), texture.raw);
            msg_void_usize(attachment, sel(b"setLoadAction:\0"), load_action as usize);
            msg_void_usize(attachment, sel(b"setStoreAction:\0"), store_action as usize);
            msg_void_clear_color(attachment, sel(b"setClearColor:\0"), clear_color);
        }
    }
}

impl Default for RenderPassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RenderPassDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct RenderCommandEncoder {
    raw: id,
}

impl RenderCommandEncoder {
    pub fn set_render_pipeline_state(&self, state: &RenderPipelineState) {
        unsafe {
            msg_void_id(self.raw, sel(b"setRenderPipelineState:\0"), state.raw);
        }
    }

    pub fn set_vertex_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setVertexBuffer:offset:atIndex:\0"),
                buffer.raw,
                offset,
                index,
            );
        }
    }

    pub fn set_fragment_buffer(&self, index: usize, buffer: &Buffer, offset: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
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
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setFragmentTexture:atIndex:\0"),
                texture.raw,
                index,
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
pub struct BlitCommandEncoder {
    raw: id,
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

    pub fn end_encoding(&self) {
        unsafe { msg_void(self.raw, sel(b"endEncoding\0")) };
    }
}

impl Drop for BlitCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Buffer {
    raw: id,
}

impl Buffer {
    pub fn len(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"length\0")) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contents(&self) -> *mut c_void {
        unsafe { msg_id(self.raw, sel(b"contents\0")) }
    }

    pub fn write<T: Copy>(&self, value: &T) {
        let size = std::mem::size_of::<T>();
        assert!(size <= self.len());
        unsafe {
            ptr::copy_nonoverlapping(
                value as *const T as *const u8,
                self.contents() as *mut u8,
                size,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct TextureDescriptor {
    raw: id,
}

impl TextureDescriptor {
    pub fn texture_2d(
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> Self {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize, usize, BOOL) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                class(b"MTLTextureDescriptor\0"),
                sel(b"texture2DDescriptorWithPixelFormat:width:height:mipmapped:\0"),
                pixel_format.as_raw(),
                width,
                height,
                if mipmapped { YES } else { NO },
            ));
            Self { raw }
        }
    }

    pub fn set_usage(&self, usage: TextureUsage) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setUsage:\0"), usage.as_raw());
        }
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setStorageMode:\0"), storage_mode as usize);
        }
    }
}

impl Drop for TextureDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Texture {
    raw: id,
}

impl Texture {
    pub fn replace_region(
        &self,
        region: Region,
        mipmap_level: usize,
        bytes: &[u8],
        bytes_per_row: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Region, usize, *const c_void, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"replaceRegion:mipmapLevel:withBytes:bytesPerRow:\0"),
                region,
                mipmap_level,
                bytes.as_ptr() as *const c_void,
                bytes_per_row,
            );
        }
    }

    pub fn width(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"width\0")) }
    }

    pub fn height(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"height\0")) }
    }

    pub fn pixel_format(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"pixelFormat\0")) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct MetalLayer {
    raw: id,
}

impl MetalLayer {
    pub unsafe fn attach_to_view(
        ns_view: *mut c_void,
        device: &Device,
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<Self, MetalError> {
        unsafe {
            let raw = retain(msg_id(class(b"CAMetalLayer\0"), sel(b"layer\0")));
            if raw.is_null() {
                return Err(MetalError::new("failed to create CAMetalLayer"));
            }

            let layer = Self { raw };
            layer.set_device(device);
            layer.set_pixel_format(pixel_format);
            layer.set_framebuffer_only(true);
            layer.set_presents_with_transaction(false);
            layer.set_contents_scale(scale);
            layer.set_drawable_size(width, height);

            msg_void_bool(ns_view, sel(b"setWantsLayer:\0"), YES);
            msg_void_id(ns_view, sel(b"setLayer:\0"), layer.raw);

            Ok(layer)
        }
    }

    pub fn set_device(&self, device: &Device) {
        unsafe {
            msg_void_id(self.raw, sel(b"setDevice:\0"), device.raw);
        }
    }

    pub fn set_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
        }
    }

    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setFramebufferOnly:\0"),
                if framebuffer_only { YES } else { NO },
            );
        }
    }

    pub fn set_presents_with_transaction(&self, presents_with_transaction: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setPresentsWithTransaction:\0"),
                if presents_with_transaction { YES } else { NO },
            );
        }
    }

    pub fn set_contents_scale(&self, scale: f64) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, f64) = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"setContentsScale:\0"), scale);
        }
    }

    pub fn set_drawable_size(&self, width: usize, height: usize) {
        unsafe {
            msg_void_size(
                self.raw,
                sel(b"setDrawableSize:\0"),
                CGSize {
                    width: width as f64,
                    height: height as f64,
                },
            );
        }
    }

    pub fn next_drawable(&self) -> Option<Drawable> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"nextDrawable\0")));
            (!raw.is_null()).then_some(Drawable { raw })
        }
    }
}

impl Drop for MetalLayer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Drawable {
    raw: id,
}

impl Drawable {
    pub fn texture(&self) -> Texture {
        unsafe {
            Texture {
                raw: retain(msg_id(self.raw, sel(b"texture\0"))),
            }
        }
    }

    pub fn present(&self) {
        unsafe { msg_void(self.raw, sel(b"present\0")) };
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

pub struct AutoreleasePool {
    raw: id,
}

impl AutoreleasePool {
    pub fn new() -> Self {
        unsafe {
            let pool = msg_id(
                msg_id(class(b"NSAutoreleasePool\0"), sel(b"alloc\0")),
                sel(b"init\0"),
            );
            Self { raw: pool }
        }
    }
}

impl Default for AutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe { msg_void(self.raw, sel(b"drain\0")) };
    }
}
