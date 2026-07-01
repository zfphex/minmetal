use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CounterSamplingPoint {
    AtStageBoundary = 0,
    AtDrawBoundary = 1,
    AtDispatchBoundary = 2,
    AtTileBoundary = 3,
    AtBlitBoundary = 4,
}

#[derive(Debug)]
pub struct Counter {
    pub raw: id,
}

impl Counter {
    pub fn name(&self) -> String {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
            .unwrap_or_else(|| "Unknown Counter".to_string())
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CounterSet {
    pub raw: id,
}

impl CounterSet {
    pub fn name(&self) -> String {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
            .unwrap_or_else(|| "Unknown CounterSet".to_string())
    }

    pub fn counters(&self) -> Vec<Counter> {
        let array = msg_id(self.raw, sel(b"counters\0"));
        if array.is_null() {
            return Vec::new();
        }
        let count = msg_usize(array, sel(b"count\0"));
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let item = retain(msg_id_usize(array, sel(b"objectAtIndex:\0"), i));
            if !item.is_null() {
                result.push(Counter { raw: item });
            }
        }
        result
    }
}

impl Drop for CounterSet {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CounterSampleBufferDescriptor {
    pub raw: id,
}

impl CounterSampleBufferDescriptor {
    pub fn new() -> Self {
        let allocated = msg_id(
            class(b"MTLCounterSampleBufferDescriptor\0"),
            sel(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel(b"init\0")),
        }
    }

    pub fn set_counter_set(&self, counter_set: &CounterSet) {
        msg_void_id(self.raw, sel(b"setCounterSet:\0"), counter_set.raw);
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        msg_void_usize(self.raw, sel(b"setStorageMode:\0"), storage_mode as usize);
    }

    pub fn set_sample_count(&self, sample_count: usize) {
        msg_void_usize(self.raw, sel(b"setSampleCount:\0"), sample_count);
    }
}

impl Default for CounterSampleBufferDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CounterSampleBufferDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct CounterSampleBuffer {
    pub raw: id,
}

impl Drop for CounterSampleBuffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// Add Device methods for counters
impl Device {
    pub fn supports_counter_sampling(&self, sampling_point: CounterSamplingPoint) -> bool {
        unsafe {
            let selector = sel(b"supportsCounterSampling:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, sampling_point as usize) != NO
            } else {
                false
            }
        }
    }

    pub fn counter_sets(&self) -> Result<Vec<CounterSet>, MetalError> {
        let selector = sel(b"counterSets\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("MTLDevice does not respond to counterSets"));
        }
        let array = msg_id(self.raw, selector);
        if array.is_null() {
            return Ok(Vec::new());
        }
        let count = msg_usize(array, sel(b"count\0"));
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let item = retain(msg_id_usize(array, sel(b"objectAtIndex:\0"), i));
            if !item.is_null() {
                result.push(CounterSet { raw: item });
            }
        }
        Ok(result)
    }

    pub fn new_counter_sample_buffer(
        &self,
        descriptor: &CounterSampleBufferDescriptor,
    ) -> Result<CounterSampleBuffer, MetalError> {
        let selector = sel(b"newCounterSampleBufferWithDescriptor:error:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newCounterSampleBufferWithDescriptor:error: is not supported",
            ));
        }
        let mut error = NIL;
        let raw = msg_id_id_err(self.raw, selector, descriptor.raw, &mut error);
        if raw.is_null() {
            Err(MetalError::new(error_message(
                error,
                "failed to create counter sample buffer",
            )))
        } else {
            Ok(CounterSampleBuffer { raw })
        }
    }
}

// Counter sampling encoder implementations
impl RenderCommandEncoder {
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "sampleCountersInBuffer:atSampleIndex:withBarrier: not supported on this RenderCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, BOOL) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                sample_buffer.raw,
                sample_index,
                if barrier { YES } else { NO },
            );
            Ok(())
        }
    }
}

impl ComputeCommandEncoder {
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "sampleCountersInBuffer:atSampleIndex:withBarrier: not supported on this ComputeCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, BOOL) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                sample_buffer.raw,
                sample_index,
                if barrier { YES } else { NO },
            );
            Ok(())
        }
    }
}

impl BlitCommandEncoder {
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "sampleCountersInBuffer:atSampleIndex:withBarrier: not supported on this BlitCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, usize, BOOL) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                sample_buffer.raw,
                sample_index,
                if barrier { YES } else { NO },
            );
            Ok(())
        }
    }

    pub fn resolve_counters(
        &self,
        sample_buffer: &CounterSampleBuffer,
        range: Range,
        destination_buffer: &Buffer,
        destination_offset: usize,
    ) -> Result<(), MetalError> {
        unsafe {
            let selector = sel(b"resolveCounters:inRange:destinationBuffer:destinationOffset:\0");
            if !responds_to_selector(self.raw, selector) {
                return Err(MetalError::new(
                    "resolveCounters:inRange:destinationBuffer:destinationOffset: not supported on this BlitCommandEncoder",
                ));
            }
            let f: unsafe extern "C" fn(id, SEL, id, Range, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                selector,
                sample_buffer.raw,
                range,
                destination_buffer.raw,
                destination_offset,
            );
            Ok(())
        }
    }
}
