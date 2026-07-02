use crate::*;
use std::ffi::{CStr, c_char, c_void};
use std::mem::transmute;
use std::ptr;

pub type id = *mut c_void;
pub type Class = *mut c_void;
pub type SEL = *mut c_void;
pub type BOOL = i8;

pub const YES: BOOL = 1;
pub const NO: BOOL = 0;
pub const NIL: id = ptr::null_mut();

#[link(name = "objc")]
#[link(name = "Foundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    pub fn objc_getClass(name: *const c_char) -> Class;
    pub fn sel_registerName(name: *const c_char) -> SEL;
    pub fn objc_msgSend();
    pub fn MTLCreateSystemDefaultDevice() -> id;
}

pub fn class(name: &[u8]) -> Class {
    unsafe { objc_getClass(name.as_ptr() as *const c_char) }
}

pub fn sel(name: &[u8]) -> SEL {
    unsafe { sel_registerName(name.as_ptr() as *const c_char) }
}

pub fn msg_id(obj: id, selector: SEL) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub fn msg_id_id(obj: id, selector: SEL, arg: id) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub fn msg_void(obj: id, selector: SEL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector);
    }
}

pub fn msg_void_id(obj: id, selector: SEL, arg: id) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_bool(obj: id, selector: SEL, arg: BOOL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, BOOL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_usize(obj: id, selector: SEL, arg: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_u64(obj: id, selector: SEL, arg: u64) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, u64) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_f64(obj: id, selector: SEL, arg: f64) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, f64) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_usize(obj: id, selector: SEL) -> usize {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> usize = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub fn msg_u64(obj: id, selector: SEL) -> u64 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> u64 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
pub fn msg_bool(obj: id, selector: SEL) -> BOOL {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> BOOL = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
pub fn msg_f64(obj: id, selector: SEL) -> f64 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> f64 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub fn msg_void_size(obj: id, selector: SEL, arg: CGSize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, CGSize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_clear_color(obj: id, selector: SEL, arg: ClearColor) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ClearColor) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_viewport(obj: id, selector: SEL, arg: Viewport) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Viewport) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_scissor_rect(obj: id, selector: SEL, arg: ScissorRect) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ScissorRect) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_id_usize(obj: id, selector: SEL, arg: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub fn msg_id_usize_usize(obj: id, selector: SEL, arg1: usize, arg2: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2)
    }
}

pub fn msg_id_id_usize(obj: id, selector: SEL, arg1: id, arg2: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2)
    }
}

pub fn msg_id_ptr_usize_usize(
    obj: id,
    selector: SEL,
    arg1: *const c_void,
    arg2: usize,
    arg3: usize,
) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2, arg3)
    }
}

pub fn ns_array_from_ids(objects: &[id]) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const id, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(
            class(b"NSArray\0"),
            sel(b"arrayWithObjects:count:\0"),
            objects.as_ptr(),
            objects.len(),
        )
    }
}

pub fn ns_array_count(array: id) -> usize {
    if array.is_null() {
        0
    } else {
        msg_usize(array, sel(b"count\0"))
    }
}

pub fn ns_array_object_at_index(array: id, index: usize) -> id {
    if array.is_null() {
        ptr::null_mut()
    } else {
        msg_id_usize(array, sel(b"objectAtIndex:\0"), index)
    }
}

pub fn ns_array_to_vec(array: id) -> Vec<id> {
    let count = ns_array_count(array);
    let mut vec = Vec::with_capacity(count);
    for i in 0..count {
        vec.push(ns_array_object_at_index(array, i));
    }
    vec
}

pub fn retain(obj: id) -> id {
    if !obj.is_null() {
        msg_id(obj, sel(b"retain\0"))
    } else {
        obj
    }
}

pub fn release(obj: id) {
    if !obj.is_null() {
        msg_void(obj, sel(b"release\0"));
    }
}

pub struct NSString {
    raw: id,
}

impl NSString {
    pub fn new(value: &str) -> Self {
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

    pub fn raw(&self) -> id {
        self.raw
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        release(self.raw);
    }
}

pub fn ns_string_to_string(raw: id) -> Option<String> {
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

pub fn error_message(error: id, fallback: &str) -> String {
    if error.is_null() {
        return fallback.to_string();
    }
    let description = msg_id(error, sel(b"localizedDescription\0"));
    ns_string_to_string(description).unwrap_or_else(|| fallback.to_string())
}

pub struct AutoreleasePool {
    raw: id,
}

impl AutoreleasePool {
    pub fn new() -> Self {
        let pool = msg_id(
            msg_id(class(b"NSAutoreleasePool\0"), sel(b"alloc\0")),
            sel(b"init\0"),
        );
        Self { raw: pool }
    }
}

impl Default for AutoreleasePool {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        msg_void(self.raw, sel(b"drain\0"));
    }
}

pub fn msg_id_id_err(obj: id, selector: SEL, arg: id, err: *mut id) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg, err)
    }
}

pub fn msg_bool_id_err(obj: id, selector: SEL, arg: id, err: *mut id) -> BOOL {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg, err)
    }
}

pub fn msg_void_id_usize_usize(obj: id, selector: SEL, arg1: id, arg2: usize, arg3: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2, arg3);
    }
}

pub fn msg_void_id_usize(obj: id, selector: SEL, arg1: id, arg2: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, usize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub fn msg_void_ptr_usize_usize(
    obj: id,
    selector: SEL,
    arg1: *const c_void,
    arg2: usize,
    arg3: usize,
) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2, arg3);
    }
}

pub fn msg_void_id_range(obj: id, selector: SEL, arg1: id, arg2: Range) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, Range) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub fn msg_void_range(obj: id, selector: SEL, arg: Range) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Range) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_size_size(obj: id, selector: SEL, arg1: Size, arg2: Size) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Size, Size) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub fn msg_void_id_u64(obj: id, selector: SEL, arg1: id, arg2: u64) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, u64) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub fn responds_to_selector(obj: id, selector: SEL) -> bool {
    if obj.is_null() {
        return false;
    }
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, SEL) -> BOOL =
            transmute(objc_msgSend as *const c_void);
        f(obj, sel(b"respondsToSelector:\0"), selector) != NO
    }
}

pub fn msg_resource_id(obj: id, selector: SEL) -> ResourceID {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> ResourceID =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub fn msg_void_mtlsize(obj: id, selector: SEL, arg: Size) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Size) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub fn msg_void_ptr_ptr_range(
    obj: id,
    selector: SEL,
    arg1: *const id,
    arg2: *const usize,
    arg3: Range,
) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const id, *const usize, Range) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2, arg3);
    }
}

pub fn msg_void_ptr_range(obj: id, selector: SEL, arg1: *const id, arg2: Range) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const id, Range) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}
