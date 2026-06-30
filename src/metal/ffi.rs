use super::types::{CGSize, ClearColor, ScissorRect, Viewport};
use std::ffi::{CStr, c_char, c_void};
use std::mem::transmute;
use std::ptr;

pub(crate) type id = *mut c_void;
pub(crate) type Class = *mut c_void;
pub(crate) type SEL = *mut c_void;
pub(crate) type BOOL = i8;

pub(crate) const YES: BOOL = 1;
pub(crate) const NO: BOOL = 0;
pub(crate) const NIL: id = ptr::null_mut();

#[link(name = "objc")]
#[link(name = "Foundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    pub(crate) fn objc_getClass(name: *const c_char) -> Class;
    pub(crate) fn sel_registerName(name: *const c_char) -> SEL;
    pub(crate) fn objc_msgSend();
    pub(crate) fn MTLCreateSystemDefaultDevice() -> id;
}

pub(crate) unsafe fn class(name: &[u8]) -> Class {
    unsafe { objc_getClass(name.as_ptr() as *const c_char) }
}

pub(crate) unsafe fn sel(name: &[u8]) -> SEL {
    unsafe { sel_registerName(name.as_ptr() as *const c_char) }
}

pub(crate) unsafe fn msg_id(obj: id, selector: SEL) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) unsafe fn msg_id_id(obj: id, selector: SEL, arg: id) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub(crate) unsafe fn msg_void(obj: id, selector: SEL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector);
    }
}

pub(crate) unsafe fn msg_void_id(obj: id, selector: SEL, arg: id) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_void_bool(obj: id, selector: SEL, arg: BOOL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, BOOL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_void_usize(obj: id, selector: SEL, arg: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_void_f64(obj: id, selector: SEL, arg: f64) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, f64) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_usize(obj: id, selector: SEL) -> usize {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> usize = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
pub(crate) unsafe fn msg_bool(obj: id, selector: SEL) -> BOOL {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> BOOL = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
pub(crate) unsafe fn msg_f64(obj: id, selector: SEL) -> f64 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> f64 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) unsafe fn msg_void_size(obj: id, selector: SEL, arg: CGSize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, CGSize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_void_clear_color(obj: id, selector: SEL, arg: ClearColor) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ClearColor) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_void_viewport(obj: id, selector: SEL, arg: Viewport) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Viewport) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_void_scissor_rect(obj: id, selector: SEL, arg: ScissorRect) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ScissorRect) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) unsafe fn msg_id_usize(obj: id, selector: SEL, arg: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub(crate) unsafe fn retain(obj: id) -> id {
    if !obj.is_null() {
        unsafe { msg_id(obj, sel(b"retain\0")) }
    } else {
        obj
    }
}

pub(crate) unsafe fn release(obj: id) {
    if !obj.is_null() {
        unsafe { msg_void(obj, sel(b"release\0")) };
    }
}

pub(crate) struct NSString {
    raw: id,
}

impl NSString {
    pub(crate) fn new(value: &str) -> Self {
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

    pub(crate) fn raw(&self) -> id {
        self.raw
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

pub(crate) unsafe fn ns_string_to_string(raw: id) -> Option<String> {
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

pub(crate) unsafe fn error_message(error: id, fallback: &str) -> String {
    if error.is_null() {
        return fallback.to_string();
    }
    unsafe {
        let description = msg_id(error, sel(b"localizedDescription\0"));
        ns_string_to_string(description).unwrap_or_else(|| fallback.to_string())
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
