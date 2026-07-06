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

pub(crate) fn msg_id(obj: id, selector: SEL) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_id_id(obj: id, selector: SEL, arg: id) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) -> id = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub(crate) fn msg_void(obj: id, selector: SEL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector);
    }
}

pub(crate) fn msg_void_id(obj: id, selector: SEL, arg: id) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_bool(obj: id, selector: SEL, arg: BOOL) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, BOOL) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_usize(obj: id, selector: SEL, arg: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_u64(obj: id, selector: SEL, arg: u64) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, u64) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_f64(obj: id, selector: SEL, arg: f64) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, f64) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_f32(obj: id, selector: SEL, arg: f32) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, f32) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_usize(obj: id, selector: SEL) -> usize {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> usize = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_usize_usize(obj: id, selector: SEL, arg: usize) -> usize {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) -> usize =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub(crate) fn msg_u64(obj: id, selector: SEL) -> u64 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> u64 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_u32(obj: id, selector: SEL) -> u32 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> u32 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

#[allow(dead_code)]
pub(crate) fn msg_bool(obj: id, selector: SEL) -> BOOL {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> BOOL = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_f64(obj: id, selector: SEL) -> f64 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> f64 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_f32(obj: id, selector: SEL) -> f32 {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> f32 = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_cgsize(obj: id, selector: SEL) -> CGSize {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> CGSize = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_void_size(obj: id, selector: SEL, arg: CGSize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, CGSize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn objc_copy(obj: id) -> id {
    if obj.is_null() {
        obj
    } else {
        msg_id(obj, sel(b"copy\0"))
    }
}

pub(crate) fn msg_void_clear_color(obj: id, selector: SEL, arg: ClearColor) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ClearColor) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_viewport(obj: id, selector: SEL, arg: Viewport) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Viewport) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_scissor_rect(obj: id, selector: SEL, arg: ScissorRect) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, ScissorRect) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_id_usize(obj: id, selector: SEL, arg: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg)
    }
}

pub(crate) fn msg_id_usize_usize(obj: id, selector: SEL, arg1: usize, arg2: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, usize, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2)
    }
}

pub(crate) fn msg_id_id_usize(obj: id, selector: SEL, arg1: id, arg2: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2)
    }
}

pub(crate) fn msg_id_ptr_usize(obj: id, selector: SEL, arg1: *const c_void, arg2: usize) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const c_void, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2)
    }
}

pub(crate) fn msg_id_ptr_usize_usize(
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

/// Marker for `#[repr(transparent)]` wrappers over `id`.
pub(crate) trait TransparentId {}

#[inline]
pub(crate) fn transparent_id_slice<T: TransparentId>(slice: &[T]) -> &[id] {
    debug_assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<id>());
    debug_assert_eq!(std::mem::align_of::<T>(), std::mem::align_of::<id>());
    unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) }
}

pub(crate) fn ns_array_from_ids(objects: &[id]) -> id {
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

pub(crate) fn ns_array_count(array: id) -> usize {
    if array.is_null() {
        0
    } else {
        msg_usize(array, sel(b"count\0"))
    }
}

pub(crate) fn ns_array_object_at_index(array: id, index: usize) -> id {
    if array.is_null() {
        ptr::null_mut()
    } else {
        msg_id_usize(array, sel(b"objectAtIndex:\0"), index)
    }
}

pub trait FromRawId {
    fn from_raw_id(raw: id) -> Self;
}

impl FromRawId for id {
    fn from_raw_id(raw: id) -> Self {
        raw
    }
}

impl FromRawId for String {
    fn from_raw_id(raw: id) -> Self {
        ns_string_to_string(raw).unwrap_or_default()
    }
}

pub struct NSArrayIterator<T> {
    array: id,
    index: usize,
    count: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<T> NSArrayIterator<T> {
    pub fn new(array: id) -> Self {
        let count = ns_array_count(array);
        Self {
            array: retain(array),
            index: 0,
            count,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for NSArrayIterator<T> {
    fn clone(&self) -> Self {
        Self {
            array: retain(self.array),
            index: self.index,
            count: self.count,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for NSArrayIterator<T> {
    fn drop(&mut self) {
        release(self.array);
    }
}

impl<T: FromRawId> Iterator for NSArrayIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let raw_item = ns_array_object_at_index(self.array, self.index);
            self.index += 1;
            Some(T::from_raw_id(raw_item))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.count - self.index;
        (remaining, Some(remaining))
    }
}

impl<T: FromRawId> ExactSizeIterator for NSArrayIterator<T> {}

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

#[derive(Debug)]
pub struct NSString {
    raw: id,
}

impl Clone for NSString {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for NSString {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
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

    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn raw(&self) -> id {
        self.raw
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.raw.is_null() {
            return None;
        }
        unsafe {
            let utf8_ptr = msg_id(self.raw, sel(b"UTF8String\0")) as *const std::ffi::c_char;
            if utf8_ptr.is_null() {
                None
            } else {
                let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
                c_str.to_str().ok()
            }
        }
    }
}

impl std::ops::Deref for NSString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str().unwrap_or("")
    }
}

impl PartialEq for NSString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for NSString {}

impl PartialEq<str> for NSString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == Some(other)
    }
}

impl PartialEq<&str> for NSString {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == Some(*other)
    }
}

impl std::fmt::Display for NSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{}", s)
        } else {
            Ok(())
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct NSData {
    raw: id,
}

impl Clone for NSData {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for NSData {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl NSData {
    pub fn from_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn raw(&self) -> id {
        self.raw
    }

    pub fn bytes(&self) -> &[u8] {
        if self.raw.is_null() {
            return &[];
        }
        unsafe {
            let bytes_ptr = msg_id(self.raw, sel(b"bytes\0")) as *const u8;
            let length = msg_usize(self.raw, sel(b"length\0"));
            if bytes_ptr.is_null() || length == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(bytes_ptr, length)
            }
        }
    }
}

impl std::ops::Deref for NSData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes()
    }
}

impl Drop for NSData {
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

#[derive(Clone, Copy)]
pub struct LazyErrorMessage {
    pub(crate) error: id,
    pub(crate) fallback: &'static str,
}

impl LazyErrorMessage {
    pub fn to_string(&self) -> String {
        format_error_message(self.error, self.fallback)
    }
}

pub(crate) fn error_message(error: id, fallback: &'static str) -> LazyErrorMessage {
    LazyErrorMessage { error, fallback }
}

pub(crate) fn format_error_message(error: id, fallback: &str) -> String {
    if error.is_null() {
        return fallback.to_string();
    }
    let description = msg_id(error, sel(b"localizedDescription\0"));
    if let Some(msg) = ns_string_to_string(description) {
        if let Some(domain) = error_domain(error) {
            return format!("{} (domain: {}, code: {})", msg, domain, error_code(error));
        }
        return msg;
    }
    let user_info = error_user_info(error);
    if !user_info.is_null() {
        let key = NSString::new("NSLocalizedDescriptionKey");
        let value = ns_dictionary_object_for_key(user_info, key.raw());
        if let Some(msg) = ns_string_to_string(value) {
            return msg;
        }
    }
    fallback.to_string()
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

pub(crate) fn msg_id_id_err(obj: id, selector: SEL, arg: id, err: *mut id) -> id {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg, err)
    }
}

pub(crate) fn msg_bool_id_err(obj: id, selector: SEL, arg: id, err: *mut id) -> BOOL {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg, err)
    }
}

pub(crate) fn msg_void_id_usize_usize(obj: id, selector: SEL, arg1: id, arg2: usize, arg3: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, usize, usize) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2, arg3);
    }
}

pub(crate) fn msg_void_id_usize(obj: id, selector: SEL, arg1: id, arg2: usize) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, usize) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub(crate) fn msg_void_ptr_usize_usize(
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

pub(crate) fn msg_void_id_range(obj: id, selector: SEL, arg1: id, arg2: Range) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, id, Range) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub(crate) fn msg_void_range(obj: id, selector: SEL, arg: Range) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Range) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_range(obj: id, selector: SEL) -> Range {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> Range = transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_void_size_size(obj: id, selector: SEL, arg1: Size, arg2: Size) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Size, Size) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub(crate) fn msg_void_id_u64(obj: id, selector: SEL, arg1: id, arg2: u64) {
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

pub(crate) fn msg_resource_id(obj: id, selector: SEL) -> ResourceID {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL) -> ResourceID =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector)
    }
}

pub(crate) fn msg_void_mtlsize(obj: id, selector: SEL, arg: Size) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, Size) = transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg);
    }
}

pub(crate) fn msg_void_ptr_ptr_range(
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

pub(crate) fn msg_void_ptr_range(obj: id, selector: SEL, arg1: *const id, arg2: Range) {
    unsafe {
        let f: unsafe extern "C" fn(id, SEL, *const id, Range) =
            transmute(objc_msgSend as *const c_void);
        f(obj, selector, arg1, arg2);
    }
}

pub fn ns_url_from_path(path: &str) -> id {
    let ns_path = NSString::new(path);
    msg_id_id(class(b"NSURL\0"), sel(b"fileURLWithPath:\0"), ns_path.raw())
}

pub fn ns_url_to_path(url: id) -> Option<NSString> {
    if url.is_null() {
        return None;
    }
    let ptr = msg_id(url, sel(b"path\0"));
    if ptr.is_null() {
        None
    } else {
        Some(NSString::from_raw(ptr))
    }
}

pub(crate) fn ns_data_from_bytes(bytes: &[u8]) -> id {
    msg_id_ptr_usize(
        class(b"NSData\0"),
        sel(b"dataWithBytes:length:\0"),
        bytes.as_ptr() as *const c_void,
        bytes.len(),
    )
}

pub(crate) fn ns_dictionary_object_for_key(dictionary: id, key: id) -> id {
    if dictionary.is_null() {
        ptr::null_mut()
    } else {
        msg_id_id(dictionary, sel(b"objectForKey:\0"), key)
    }
}

pub(crate) fn ns_dictionary_all_keys(dictionary: id) -> id {
    if dictionary.is_null() {
        ptr::null_mut()
    } else {
        msg_id(dictionary, sel(b"allKeys\0"))
    }
}

pub fn ns_bundle_main() -> id {
    msg_id(class(b"NSBundle\0"), sel(b"mainBundle\0"))
}

pub(crate) fn error_domain(error: id) -> Option<String> {
    if error.is_null() {
        None
    } else {
        ns_string_to_string(msg_id(error, sel(b"domain\0")))
    }
}

pub(crate) fn error_code(error: id) -> isize {
    if error.is_null() {
        0
    } else {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> isize =
                transmute(objc_msgSend as *const c_void);
            f(error, sel(b"code\0"))
        }
    }
}

pub(crate) fn error_user_info(error: id) -> id {
    if error.is_null() {
        NIL
    } else {
        msg_id(error, sel(b"userInfo\0"))
    }
}
