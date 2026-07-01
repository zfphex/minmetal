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
        let allocated = msg_id(class(b"MTLCaptureDescriptor\0"), sel(b"alloc\0"));
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_capture_object(&self, object: id) {
        msg_void_id(self.raw, sel(b"setCaptureObject:\0"), object);
    }

    pub fn set_destination(&self, destination: CaptureDestination) {
        msg_void_usize(self.raw, sel(b"setDestination:\0"), destination as usize);
    }

    pub fn set_output_url(&self, path: &str) {
        let ns_path = NSString::new(path);
        let ns_url = msg_id_id(class(b"NSURL\0"), sel(b"fileURLWithPath:\0"), ns_path.raw());
        msg_void_id(self.raw, sel(b"setOutputURL:\0"), ns_url);
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
        let selector = sel(b"sharedCaptureManager\0");
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
            let selector = sel(b"supportsDestination:\0");
            if !responds_to_selector(self.raw, selector) {
                return false;
            }
            let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, destination as usize) != NO
        }
    }

    pub fn start_capture(&self, descriptor: &CaptureDescriptor) -> Result<(), MetalError> {
        let selector = sel(b"startCaptureWithDescriptor:error:\0");
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

    pub fn stop_capture(&self) {
        let selector = sel(b"stopCapture\0");
        if responds_to_selector(self.raw, selector) {
            msg_void(self.raw, selector);
        }
    }

    pub fn is_capturing(&self) -> bool {
        let selector = sel(b"isCapturing\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != NO
        } else {
            false
        }
    }
}

impl Drop for CaptureManager {
    fn drop(&mut self) {
        release(self.raw);
    }
}
