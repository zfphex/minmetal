use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CaptureDestination {
    DeveloperTools = 1,
    GpuTraceDocument = 2,
}

#[derive(Debug)]
pub struct CaptureDescriptor {
    pub raw: id,
}

impl CaptureDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(class(b"MTLCaptureDescriptor\0"), sel!(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel!(b"init\0")),
        }
    }

    pub fn capture_object(&self) -> id {
        msg_id(self.raw, sel!(b"captureObject\0"))
    }

    pub fn set_capture_object(&self, object: id) {
        msg_void_id(self.raw, sel!(b"setCaptureObject:\0"), object);
    }

    pub fn destination(&self) -> CaptureDestination {
        let val = msg_usize(self.raw, sel!(b"destination\0"));
        match val {
            2 => CaptureDestination::GpuTraceDocument,
            _ => CaptureDestination::DeveloperTools,
        }
    }

    pub fn set_destination(&self, destination: CaptureDestination) {
        msg_void_usize(self.raw, sel!(b"setDestination:\0"), destination as usize);
    }

    pub fn output_url(&self) -> Option<NSString> {
        let url = msg_id(self.raw, sel!(b"outputURL\0"));
        if url.is_null() {
            None
        } else {
            let path_ptr = msg_id(url, sel!(b"path\0"));
            if path_ptr.is_null() {
                None
            } else {
                Some(NSString::from_raw(path_ptr))
            }
        }
    }

    pub fn set_output_url(&self, path: &str) {
        let ns_url = ns_url_from_path(path);
        msg_void_id(self.raw, sel!(b"setOutputURL:\0"), ns_url);
    }

    pub fn copy(&self) -> Self {
        Self {
            raw: retain(objc_copy(self.raw)),
        }
    }
}

impl Default for CaptureDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CaptureDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CaptureManager {
    pub raw: id,
}

impl CaptureManager {
    pub fn shared() -> Result<Self, MetalError> {
        let cls = class(b"MTLCaptureManager\0");
        if cls.is_null() {
            return Err(MetalError::new("MTLCaptureManager class not found"));
        }
        let selector = sel!(b"sharedCaptureManager\0");
        if !responds_to_selector(cls, selector) {
            return Err(MetalError::new(
                "MTLCaptureManager does not respond to sharedCaptureManager",
            ));
        }
        let raw = retain(msg_id(cls, selector));
        if raw.is_null() {
            Err(MetalError::new(
                "MTLCaptureManager.sharedCaptureManager returned nil",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn supports_destination(&self, destination: CaptureDestination) -> bool {
        unsafe {
            let selector = sel!(b"supportsDestination:\0");
            if !responds_to_selector(self.raw, selector) {
                return false;
            }
            let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, destination as usize) != NO
        }
    }

    pub fn start_capture(&self, descriptor: &CaptureDescriptor) -> Result<(), MetalError> {
        let selector = sel!(b"startCaptureWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "startCaptureWithDescriptor:error: is not supported",
            ));
        }
        let mut error = NIL;
        let ok = msg_bool_id_err(self.raw, selector, descriptor.raw, &mut error);
        if ok == NO {
            Err(MetalError::new(error_message(
                error,
                "failed to start Metal capture",
            )))
        } else {
            Ok(())
        }
    }

    pub fn start_capture_with_device(&self, device: &Device) {
        let selector = sel!(b"startCaptureWithDevice:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, device.raw);
        }
    }

    pub fn start_capture_with_command_queue(&self, queue: &CommandQueue) {
        let selector = sel!(b"startCaptureWithCommandQueue:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, queue.raw);
        }
    }

    pub fn start_capture_with_scope(&self, scope: &CaptureScope) {
        let selector = sel!(b"startCaptureWithScope:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_id(self.raw, selector, scope.raw);
        }
    }

    pub fn stop_capture(&self) {
        let selector = sel!(b"stopCapture\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
        }
    }

    pub fn is_capturing(&self) -> bool {
        let selector = sel!(b"isCapturing\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }

    pub fn new_capture_scope_with_device(
        &self,
        device: &Device,
    ) -> Result<CaptureScope, MetalError> {
        let selector = sel!(b"newCaptureScopeWithDevice:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id(self.raw, selector, device.raw);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create capture scope with device",
                ))
            } else {
                Ok(CaptureScope { raw })
            }
        } else {
            Err(MetalError::new("newCaptureScopeWithDevice: not supported"))
        }
    }

    pub fn new_capture_scope_with_command_queue(
        &self,
        queue: &CommandQueue,
    ) -> Result<CaptureScope, MetalError> {
        let selector = sel!(b"newCaptureScopeWithCommandQueue:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = msg_id_id(self.raw, selector, queue.raw);
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create capture scope with command queue",
                ))
            } else {
                Ok(CaptureScope { raw })
            }
        } else {
            Err(MetalError::new(
                "newCaptureScopeWithCommandQueue: not supported",
            ))
        }
    }

    pub fn default_capture_scope(&self) -> Option<CaptureScope> {
        let selector = sel!(b"defaultCaptureScope\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if ptr.is_null() {
                None
            } else {
                Some(CaptureScope { raw: retain(ptr) })
            }
        } else {
            None
        }
    }

    pub fn set_default_capture_scope(&self, scope: Option<&CaptureScope>) {
        let selector = sel!(b"setDefaultCaptureScope:\0");
        if responds_to_selector(self.raw, selector) {
            let raw = scope.map_or(NIL, |s| s.raw);
            msg_void_id(self.raw, selector, raw);
        }
    }
}

impl Drop for CaptureManager {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CaptureScope {
    pub raw: id,
}

impl CaptureScope {
    pub fn begin_scope(&self) {
        msg_void(self.raw, sel!(b"beginScope\0"));
    }

    pub fn end_scope(&self) {
        msg_void(self.raw, sel!(b"endScope\0"));
    }

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel!(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel!(b"setLabel:\0"), ns_label.raw());
    }

    pub fn device(&self) -> Device {
        let ptr = retain(msg_id(self.raw, sel!(b"device\0")));
        Device { raw: ptr }
    }

    pub fn command_queue(&self) -> Option<CommandQueue> {
        let ptr = msg_id(self.raw, sel!(b"commandQueue\0"));
        if ptr.is_null() {
            None
        } else {
            Some(CommandQueue { raw: retain(ptr) })
        }
    }
}

impl Drop for CaptureScope {
    fn drop(&mut self) {
        release(self.raw);
    }
}
