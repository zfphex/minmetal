use crate::*;
use std::ffi::{c_char, c_void};
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum IOPriority {
    High = 0,
    Normal = 1,
    Low = 2,
}

impl IOPriority {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::High),
            1 => Some(Self::Normal),
            2 => Some(Self::Low),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum IOCommandQueueType {
    Concurrent = 0,
    Serial = 1,
}

impl IOCommandQueueType {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::Concurrent),
            1 => Some(Self::Serial),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum IOStatus {
    Pending = 0,
    Cancelled = 1,
    Error = 2,
    Complete = 3,
}

impl IOStatus {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            0 => Some(Self::Pending),
            1 => Some(Self::Cancelled),
            2 => Some(Self::Error),
            3 => Some(Self::Complete),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum IOError {
    UrlInvalid = 1,
    Internal = 2,
}

impl IOError {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            1 => Some(Self::UrlInvalid),
            2 => Some(Self::Internal),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum IOCompressionStatus {
    Complete = 0,
    Error = 1,
}

impl IOCompressionStatus {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            0 => Some(Self::Complete),
            1 => Some(Self::Error),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum IOCompressionMethod {
    Zlib = 0,
    Lzfse = 1,
    Lz4 = 2,
    Lzma = 3,
    LzBitmap = 4,
}

impl IOCompressionMethod {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            0 => Some(Self::Zlib),
            1 => Some(Self::Lzfse),
            2 => Some(Self::Lz4),
            3 => Some(Self::Lzma),
            4 => Some(Self::LzBitmap),
            _ => None,
        }
    }
}

type RawIOCompressionContext = *mut c_void;

#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    fn MTLIOCompressionContextDefaultChunkSize() -> usize;
    fn MTLIOCreateCompressionContext(
        path: *const c_char,
        compression_method: isize,
        chunk_size: usize,
    ) -> RawIOCompressionContext;
    fn MTLIOCompressionContextAppendData(
        context: RawIOCompressionContext,
        data: *const c_void,
        size: usize,
    );
    fn MTLIOFlushAndDestroyCompressionContext(
        context: RawIOCompressionContext,
    ) -> isize;
}

pub fn io_compression_context_default_chunk_size() -> usize {
    unsafe { MTLIOCompressionContextDefaultChunkSize() }
}

#[derive(Debug)]
pub struct IOCompressionContext {
    raw: Option<RawIOCompressionContext>,
}

impl IOCompressionContext {
    pub fn new(
        path: &str,
        method: IOCompressionMethod,
        chunk_size: usize,
    ) -> Result<Self, MetalError> {
        let c_path = std::ffi::CString::new(path)
            .map_err(|_| MetalError::new("compression context path contains interior nul bytes"))?;
        let raw = unsafe {
            MTLIOCreateCompressionContext(c_path.as_ptr(), method as isize, chunk_size)
        };
        if raw.is_null() {
            Err(MetalError::new("failed to create IO compression context"))
        } else {
            Ok(Self { raw: Some(raw) })
        }
    }

    pub fn append_data(&self, data: &[u8]) {
        let raw = self
            .raw
            .expect("IOCompressionContext::append_data called after flush_and_destroy");
        unsafe {
            MTLIOCompressionContextAppendData(raw, data.as_ptr() as *const c_void, data.len());
        }
    }

    pub fn flush_and_destroy(mut self) -> Result<IOCompressionStatus, MetalError> {
        let raw = self.raw.take().ok_or_else(|| {
            MetalError::new("IOCompressionContext::flush_and_destroy called more than once")
        })?;
        let status = unsafe { MTLIOFlushAndDestroyCompressionContext(raw) };
        IOCompressionStatus::from_raw(status).ok_or_else(|| {
            MetalError::new(format!("invalid IOCompressionStatus value from Metal: {status}"))
        })
    }
}

impl Drop for IOCompressionContext {
    fn drop(&mut self) {
        if let Some(raw) = self.raw.take() {
            unsafe {
                MTLIOFlushAndDestroyCompressionContext(raw);
            }
        }
    }
}

fn file_url(path: &str) -> id {
    let ns_path = NSString::new(path);
    msg_id_id(class(b"NSURL\0"), sel(b"fileURLWithPath:\0"), ns_path.raw())
}

#[derive(Debug)]
pub struct IOCommandQueueDescriptor {
    pub raw: id,
}

impl IOCommandQueueDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLIOCommandQueueDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_max_command_buffer_count(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxCommandBufferCount:\0"), count);
    }

    pub fn max_command_buffer_count(&self) -> usize {
        msg_usize(self.raw, sel(b"maxCommandBufferCount\0"))
    }

    pub fn set_priority(&self, priority: IOPriority) {
        msg_void_usize(self.raw, sel(b"setPriority:\0"), priority as usize);
    }

    pub fn priority(&self) -> Result<IOPriority, MetalError> {
        IOPriority::from_raw(msg_usize(self.raw, sel(b"priority\0")))
            .ok_or_else(|| MetalError::new("invalid IOPriority value from Metal"))
    }

    pub fn set_type(&self, queue_type: IOCommandQueueType) {
        msg_void_usize(self.raw, sel(b"setType:\0"), queue_type as usize);
    }

    pub fn queue_type(&self) -> Result<IOCommandQueueType, MetalError> {
        IOCommandQueueType::from_raw(msg_usize(self.raw, sel(b"type\0")))
            .ok_or_else(|| MetalError::new("invalid IOCommandQueueType value from Metal"))
    }

    pub fn set_max_commands_in_flight(&self, count: usize) {
        msg_void_usize(self.raw, sel(b"setMaxCommandsInFlight:\0"), count);
    }

    pub fn max_commands_in_flight(&self) -> usize {
        msg_usize(self.raw, sel(b"maxCommandsInFlight\0"))
    }
}

impl Default for IOCommandQueueDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IOCommandQueueDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IOFileHandle {
    pub raw: id,
}

impl IOFileHandle {
    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for IOFileHandle {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IOCommandQueue {
    pub raw: id,
}

impl IOCommandQueue {
    pub fn command_buffer(&self) -> Result<IOCommandBuffer, MetalError> {
        let selector = sel(b"commandBuffer\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("commandBuffer is not supported"));
        }
        let raw = retain(msg_id(self.raw, selector));
        if raw.is_null() {
            Err(MetalError::new("failed to create IO command buffer"))
        } else {
            Ok(IOCommandBuffer { raw })
        }
    }

    pub fn command_buffer_with_unretained_references(&self) -> Result<IOCommandBuffer, MetalError> {
        let selector = sel(b"commandBufferWithUnretainedReferences\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "commandBufferWithUnretainedReferences is not supported",
            ));
        }
        let raw = retain(msg_id(self.raw, selector));
        if raw.is_null() {
            Err(MetalError::new(
                "failed to create IO command buffer with unretained references",
            ))
        } else {
            Ok(IOCommandBuffer { raw })
        }
    }

    pub fn enqueue_barrier(&self) -> Result<(), MetalError> {
        let selector = sel(b"enqueueBarrier\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("enqueueBarrier is not supported"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for IOCommandQueue {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct IOCommandBuffer {
    pub raw: id,
}

impl IOCommandBuffer {
    pub fn load_bytes(
        &self,
        pointer: *mut c_void,
        size: usize,
        source_handle: &IOFileHandle,
        source_handle_offset: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"loadBytes:size:sourceHandle:sourceHandleOffset:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("loadBytes:size:sourceHandle:sourceHandleOffset: is not supported"));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *mut c_void, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                pointer,
                size,
                source_handle.raw,
                source_handle_offset,
            );
        }
        Ok(())
    }

    pub fn load_buffer(
        &self,
        buffer: &Buffer,
        offset: usize,
        size: usize,
        source_handle: &IOFileHandle,
        source_handle_offset: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"loadBuffer:offset:size:sourceHandle:sourceHandleOffset:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "loadBuffer:offset:size:sourceHandle:sourceHandleOffset: is not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize, usize, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                buffer.raw,
                offset,
                size,
                source_handle.raw,
                source_handle_offset,
            );
        }
        Ok(())
    }

    pub fn load_texture(
        &self,
        texture: &Texture,
        slice: usize,
        level: usize,
        size: Size,
        source_bytes_per_row: usize,
        source_bytes_per_image: usize,
        destination_origin: Origin,
        source_handle: &IOFileHandle,
        source_handle_offset: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(
            b"loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:\0",
        );
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "loadTexture:slice:level:size:... is not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(
                id,
                SEL,
                id,
                usize,
                usize,
                Size,
                usize,
                usize,
                Origin,
                id,
                usize,
            ) = transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                texture.raw,
                slice,
                level,
                size,
                source_bytes_per_row,
                source_bytes_per_image,
                destination_origin,
                source_handle.raw,
                source_handle_offset,
            );
        }
        Ok(())
    }

    pub fn copy_status_to_buffer(&self, buffer: &Buffer, offset: usize) -> Result<(), MetalError> {
        let selector = sel(b"copyStatusToBuffer:offset:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("copyStatusToBuffer:offset: is not supported"));
        }
        msg_void_id_usize(self.raw, selector, buffer.raw, offset);
        Ok(())
    }

    pub fn commit(&self) -> Result<(), MetalError> {
        let selector = sel(b"commit\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("commit is not supported on IO command buffer"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn wait_until_completed(&self) -> Result<(), MetalError> {
        let selector = sel(b"waitUntilCompleted\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "waitUntilCompleted is not supported on IO command buffer",
            ));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn try_cancel(&self) -> Result<(), MetalError> {
        let selector = sel(b"tryCancel\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("tryCancel is not supported on IO command buffer"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn add_barrier(&self) -> Result<(), MetalError> {
        let selector = sel(b"addBarrier\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("addBarrier is not supported on IO command buffer"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn enqueue(&self) -> Result<(), MetalError> {
        let selector = sel(b"enqueue\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("enqueue is not supported on IO command buffer"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn wait_for_event(&self, event: &SharedEvent, value: u64) -> Result<(), MetalError> {
        let selector = sel(b"waitForEvent:value:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("waitForEvent:value: is not supported"));
        }
        msg_void_id_u64(self.raw, selector, event.raw, value);
        Ok(())
    }

    pub fn signal_event(&self, event: &SharedEvent, value: u64) -> Result<(), MetalError> {
        let selector = sel(b"signalEvent:value:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("signalEvent:value: is not supported"));
        }
        msg_void_id_u64(self.raw, selector, event.raw, value);
        Ok(())
    }

    pub fn status(&self) -> Result<IOStatus, MetalError> {
        let selector = sel(b"status\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("status is not supported on IO command buffer"));
        }
        let raw = msg_usize(self.raw, selector) as isize;
        IOStatus::from_raw(raw)
            .ok_or_else(|| MetalError::new(format!("invalid IOStatus value from Metal: {raw}")))
    }

    pub fn error(&self) -> Option<MetalError> {
        let selector = sel(b"error\0");
        if !responds_to_selector(self.raw, selector) {
            return None;
        }
        let error = msg_id(self.raw, selector);
        if error.is_null() {
            None
        } else {
            Some(MetalError::new(error_message(
                error,
                "IO command buffer failed",
            )))
        }
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }
}

impl Drop for IOCommandBuffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl Device {
    pub fn new_io_command_queue(
        &self,
        descriptor: &IOCommandQueueDescriptor,
    ) -> Result<IOCommandQueue, MetalError> {
        let selector = sel(b"newIOCommandQueueWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newIOCommandQueueWithDescriptor:error: is not supported",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create IO command queue",
            )))
        } else {
            Ok(IOCommandQueue { raw })
        }
    }

    pub fn new_io_file_handle(&self, path: &str) -> Result<IOFileHandle, MetalError> {
        let url = file_url(path);
        let new_selector = sel(b"newIOFileHandleWithURL:error:\0");
        let legacy_selector = sel(b"newIOHandleWithURL:error:\0");
        let selector = if responds_to_selector(self.raw, new_selector) {
            new_selector
        } else if responds_to_selector(self.raw, legacy_selector) {
            legacy_selector
        } else {
            return Err(MetalError::new(
                "newIOFileHandleWithURL:error: is not supported",
            ));
        };
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, url, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create IO file handle",
            )))
        } else {
            Ok(IOFileHandle { raw })
        }
    }

    pub fn new_io_file_handle_compressed(
        &self,
        path: &str,
        compression_method: IOCompressionMethod,
    ) -> Result<IOFileHandle, MetalError> {
        let url = file_url(path);
        let new_selector = sel(b"newIOFileHandleWithURL:compressionMethod:error:\0");
        let legacy_selector = sel(b"newIOHandleWithURL:compressionMethod:error:\0");
        let selector = if responds_to_selector(self.raw, new_selector) {
            new_selector
        } else if responds_to_selector(self.raw, legacy_selector) {
            legacy_selector
        } else {
            return Err(MetalError::new(
                "newIOFileHandleWithURL:compressionMethod:error: is not supported",
            ));
        };
        let mut error = NIL;
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, isize, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                selector,
                url,
                compression_method as isize,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create compressed IO file handle",
                )))
            } else {
                Ok(IOFileHandle { raw })
            }
        }
    }
}
