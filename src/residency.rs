use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

/// Common protocol handle for resources that can be added to a residency set.
#[derive(Debug, Clone, Copy)]
pub struct Allocation {
    pub raw: id,
}

impl Allocation {
    pub fn from_buffer(buffer: &Buffer) -> Self {
        Self { raw: buffer.raw }
    }

    pub fn from_texture(texture: &Texture) -> Self {
        Self { raw: texture.raw }
    }

    pub fn from_heap(heap: &Heap) -> Self {
        Self { raw: heap.raw }
    }

    pub fn from_acceleration_structure(structure: &AccelerationStructure) -> Self {
        Self { raw: structure.raw }
    }
}

#[derive(Debug)]
pub struct ResidencySetDescriptor {
    pub raw: id,
}

impl ResidencySetDescriptor {
    pub fn new() -> Result<Self, MetalError> {
        let class_ptr = class(b"MTLResidencySetDescriptor\0");
        if class_ptr.is_null() {
            return Err(MetalError::new("MTLResidencySetDescriptor is not available"));
        }
        let allocated = msg_id(class_ptr, sel(b"alloc\0"));
        let raw = msg_id(allocated, sel(b"init\0"));
        if raw.is_null() {
            Err(MetalError::new("failed to create MTLResidencySetDescriptor"))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn set_initial_capacity(&self, capacity: usize) {
        msg_void_usize(self.raw, sel(b"setInitialCapacity:\0"), capacity);
    }

    pub fn initial_capacity(&self) -> usize {
        msg_usize(self.raw, sel(b"initialCapacity\0"))
    }
}

impl Drop for ResidencySetDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct ResidencySet {
    pub raw: id,
}

impl ResidencySet {
    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn allocated_size(&self) -> u64 {
        msg_u64(self.raw, sel(b"allocatedSize\0"))
    }

    pub fn request_residency(&self) -> Result<(), MetalError> {
        let selector = sel(b"requestResidency\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("requestResidency is not supported"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn end_residency(&self) -> Result<(), MetalError> {
        let selector = sel(b"endResidency\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("endResidency is not supported"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn add_allocation(&self, allocation: &Allocation) -> Result<(), MetalError> {
        let selector = sel(b"addAllocation:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("addAllocation: is not supported"));
        }
        msg_void_id(self.raw, selector, allocation.raw);
        Ok(())
    }

    pub fn add_allocations(&self, allocations: &[Allocation]) -> Result<(), MetalError> {
        let selector = sel(b"addAllocations:count:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("addAllocations:count: is not supported"));
        }
        let raw_ptrs: Vec<id> = allocations.iter().map(|a| a.raw).collect();
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, raw_ptrs.as_ptr(), raw_ptrs.len());
        }
        Ok(())
    }

    pub fn remove_allocation(&self, allocation: &Allocation) -> Result<(), MetalError> {
        let selector = sel(b"removeAllocation:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("removeAllocation: is not supported"));
        }
        msg_void_id(self.raw, selector, allocation.raw);
        Ok(())
    }

    pub fn remove_allocations(&self, allocations: &[Allocation]) -> Result<(), MetalError> {
        let selector = sel(b"removeAllocations:count:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("removeAllocations:count: is not supported"));
        }
        let raw_ptrs: Vec<id> = allocations.iter().map(|a| a.raw).collect();
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, raw_ptrs.as_ptr(), raw_ptrs.len());
        }
        Ok(())
    }

    pub fn remove_all_allocations(&self) -> Result<(), MetalError> {
        let selector = sel(b"removeAllAllocations\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("removeAllAllocations is not supported"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }

    pub fn contains_allocation(&self, allocation: &Allocation) -> Result<bool, MetalError> {
        let selector = sel(b"containsAllocation:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("containsAllocation: is not supported"));
        }
        let result = unsafe {
            let f: unsafe extern "C" fn(id, SEL, id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, selector, allocation.raw)
        };
        Ok(result != NO)
    }

    pub fn allocation_count(&self) -> Result<usize, MetalError> {
        let selector = sel(b"allocationCount\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("allocationCount is not supported"));
        }
        Ok(msg_usize(self.raw, selector))
    }

    pub fn commit(&self) -> Result<(), MetalError> {
        let selector = sel(b"commit\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("commit is not supported on residency set"));
        }
        msg_void(self.raw, selector);
        Ok(())
    }
}

impl Drop for ResidencySet {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl Device {
    pub fn new_residency_set(
        &self,
        descriptor: &ResidencySetDescriptor,
    ) -> Result<ResidencySet, MetalError> {
        let selector = sel(b"newResidencySetWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newResidencySetWithDescriptor:error: is not supported",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create residency set",
            )))
        } else {
            Ok(ResidencySet { raw })
        }
    }
}
