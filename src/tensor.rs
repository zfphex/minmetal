use crate::*;
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr;

pub const TENSOR_MAX_RANK: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum TensorDataType {
    None = 0,
    Float32 = 1,
    Float16 = 2,
    BFloat16 = 3,
    Int8 = 4,
    UInt8 = 5,
    Int16 = 6,
    UInt16 = 7,
    Int32 = 8,
    UInt32 = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TensorUsage(pub usize);

impl TensorUsage {
    pub const COMPUTE: Self = Self(1 << 0);
    pub const RENDER: Self = Self(1 << 1);
    pub const MACHINE_LEARNING: Self = Self(1 << 2);

    pub const fn as_raw(self) -> usize {
        self.0
    }

    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }
}

impl std::ops::BitOr for TensorUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum TensorError {
    None = 0,
    InternalError = 1,
    InvalidDescriptor = 2,
}

#[derive(Debug)]
pub struct TensorExtents {
    pub raw: id,
}

impl TensorExtents {
    pub fn new(rank: usize, values: Option<&[isize]>) -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLTensorExtents\0");
        if class_ptr.is_null() {
            return Err(MetalError::new("MTLTensorExtents is not available"));
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        let values_ptr = values.map_or(ptr::null(), |v| v.as_ptr());
        let raw = unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, *const isize) -> id =
                transmute(objc_msgSend as *const c_void);
            f(allocated, sel(b"initWithRank:values:\0"), rank, values_ptr)
        };
        if raw.is_null() {
            Err(MetalError::new("failed to create MTLTensorExtents"))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn rank(&self) -> usize {
        msg_usize(self.raw, sel(b"rank\0"))
    }

    pub fn extent_at_dimension_index(&self, dimension_index: usize) -> isize {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> isize =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"extentAtDimensionIndex:\0"), dimension_index)
        }
    }
}

impl Drop for TensorExtents {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct TensorDescriptor {
    pub raw: id,
}

impl TensorDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLTensorDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new("MTLTensorDescriptor is not available"));
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        let raw = msg_id(allocated, sel(b"init\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create MTLTensorDescriptor"))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn dimensions(&self) -> TensorExtents {
        TensorExtents {
            raw: retain(msg_id(self.raw, sel(b"dimensions\0"))),
        }
    }

    pub fn set_dimensions(&self, dimensions: &TensorExtents) {
        msg_void_id(self.raw, sel(b"setDimensions:\0"), dimensions.raw);
    }

    pub fn strides(&self) -> Option<TensorExtents> {
        let raw = msg_id(self.raw, sel(b"strides\0"));
        if raw.is_null() {
            None
        } else {
            Some(TensorExtents { raw: retain(raw) })
        }
    }

    pub fn set_strides(&self, strides: Option<&TensorExtents>) {
        msg_void_id(
            self.raw,
            sel(b"setStrides:\0"),
            strides.map_or(NIL, |s| s.raw),
        );
    }

    pub fn data_type(&self) -> TensorDataType {
        match msg_usize(self.raw, sel(b"dataType\0")) {
            1 => TensorDataType::Float32,
            2 => TensorDataType::Float16,
            3 => TensorDataType::BFloat16,
            4 => TensorDataType::Int8,
            5 => TensorDataType::UInt8,
            6 => TensorDataType::Int16,
            7 => TensorDataType::UInt16,
            8 => TensorDataType::Int32,
            9 => TensorDataType::UInt32,
            _ => TensorDataType::None,
        }
    }

    pub fn set_data_type(&self, data_type: TensorDataType) {
        msg_void_usize(self.raw, sel(b"setDataType:\0"), data_type as usize);
    }

    pub fn usage(&self) -> TensorUsage {
        TensorUsage::from_raw(msg_usize(self.raw, sel(b"usage\0")))
    }

    pub fn set_usage(&self, usage: TensorUsage) {
        msg_void_usize(self.raw, sel(b"setUsage:\0"), usage.as_raw());
    }

    pub fn resource_options(&self) -> ResourceOptions {
        ResourceOptions::from_raw(msg_usize(self.raw, sel(b"resourceOptions\0")))
    }

    pub fn set_resource_options(&self, resource_options: ResourceOptions) {
        msg_void_usize(
            self.raw,
            sel(b"setResourceOptions:\0"),
            resource_options.as_raw(),
        );
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        match msg_usize(self.raw, sel(b"cpuCacheMode\0")) {
            1 => CpuCacheMode::WriteCombined,
            _ => CpuCacheMode::DefaultCache,
        }
    }

    pub fn set_cpu_cache_mode(&self, mode: CpuCacheMode) {
        msg_void_usize(self.raw, sel(b"setCpuCacheMode:\0"), mode as usize);
    }

    pub fn storage_mode(&self) -> StorageMode {
        match msg_usize(self.raw, sel(b"storageMode\0")) {
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn set_storage_mode(&self, mode: StorageMode) {
        msg_void_usize(self.raw, sel(b"setStorageMode:\0"), mode as usize);
    }

    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        match msg_usize(self.raw, sel(b"hazardTrackingMode\0")) {
            1 => HazardTrackingMode::Untracked,
            2 => HazardTrackingMode::Tracked,
            _ => HazardTrackingMode::Default,
        }
    }

    pub fn set_hazard_tracking_mode(&self, mode: HazardTrackingMode) {
        msg_void_usize(self.raw, sel(b"setHazardTrackingMode:\0"), mode as usize);
    }
}

impl Drop for TensorDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct Tensor {
    pub raw: id,
}

impl Tensor {
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

    pub fn gpu_resource_id(&self) -> Result<ResourceID, MetalError> {
        let selector = sel(b"gpuResourceID\0");
        if responds_to_selector(self.raw, selector) {
            Ok(msg_resource_id(self.raw, selector))
        } else {
            Err(MetalError::new(
                "gpuResourceID not supported on this Tensor",
            ))
        }
    }

    pub fn buffer(&self) -> Option<Buffer> {
        let raw = msg_id(self.raw, sel(b"buffer\0"));
        if raw.is_null() {
            None
        } else {
            Some(Buffer { raw: retain(raw) })
        }
    }

    pub fn buffer_offset(&self) -> usize {
        msg_usize(self.raw, sel(b"bufferOffset\0"))
    }

    pub fn strides(&self) -> Option<TensorExtents> {
        let raw = msg_id(self.raw, sel(b"strides\0"));
        if raw.is_null() {
            None
        } else {
            Some(TensorExtents { raw: retain(raw) })
        }
    }

    pub fn dimensions(&self) -> TensorExtents {
        TensorExtents {
            raw: retain(msg_id(self.raw, sel(b"dimensions\0"))),
        }
    }

    pub fn data_type(&self) -> TensorDataType {
        match msg_usize(self.raw, sel(b"dataType\0")) {
            1 => TensorDataType::Float32,
            2 => TensorDataType::Float16,
            3 => TensorDataType::BFloat16,
            4 => TensorDataType::Int8,
            5 => TensorDataType::UInt8,
            6 => TensorDataType::Int16,
            7 => TensorDataType::UInt16,
            8 => TensorDataType::Int32,
            9 => TensorDataType::UInt32,
            _ => TensorDataType::None,
        }
    }

    pub fn usage(&self) -> TensorUsage {
        TensorUsage::from_raw(msg_usize(self.raw, sel(b"usage\0")))
    }

    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn replace_slice(
        &self,
        slice_origin: &TensorExtents,
        slice_dimensions: &TensorExtents,
        bytes: &[u8],
        strides: &TensorExtents,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, id, *const c_void, id) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"replaceSliceOrigin:sliceDimensions:withBytes:strides:\0"),
                slice_origin.raw,
                slice_dimensions.raw,
                bytes.as_ptr() as *const c_void,
                strides.raw,
            );
        }
    }

    pub fn get_bytes(
        &self,
        out: &mut [u8],
        strides: &TensorExtents,
        slice_origin: &TensorExtents,
        slice_dimensions: &TensorExtents,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *mut c_void, id, id, id) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"getBytes:strides:fromSliceOrigin:sliceDimensions:\0"),
                out.as_mut_ptr() as *mut c_void,
                strides.raw,
                slice_origin.raw,
                slice_dimensions.raw,
            );
        }
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        release(self.raw);
    }
}
