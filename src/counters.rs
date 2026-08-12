use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

pub const COUNTER_ERROR_VALUE: u64 = u64::MAX;
pub const COUNTER_DONT_SAMPLE: usize = usize::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CounterSamplingPoint {
    AtStageBoundary = 0,
    AtDrawBoundary = 1,
    AtDispatchBoundary = 2,
    AtTileDispatchBoundary = 3,
    AtBlitBoundary = 4,
}

#[derive(Debug)]
pub struct Counter {
    pub raw: id,
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for Counter {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl Counter {
    pub fn name(&self) -> NSString {
        let ptr = msg_id(self.raw, sel!(b"name\0"));
        NSString::from_raw(ptr)
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

impl Clone for CounterSet {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl FromRawId for CounterSet {
    fn from_raw_id(raw: id) -> Self {
        Self { raw: retain(raw) }
    }
}

impl CounterSet {
    pub fn name(&self) -> NSString {
        let ptr = msg_id(self.raw, sel!(b"name\0"));
        NSString::from_raw(ptr)
    }

    pub fn counters(&self) -> NSArrayIterator<Counter> {
        NSArrayIterator::new(msg_id(self.raw, sel!(b"counters\0")))
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
            sel!(b"alloc\0"),
        );
        Self {
            raw: msg_id(allocated, sel!(b"init\0")),
        }
    }

    pub fn counter_set(&self) -> Option<CounterSet> {
        let raw = msg_id(self.raw, sel!(b"counterSet\0"));
        if raw.is_null() {
            None
        } else {
            Some(CounterSet { raw: retain(raw) })
        }
    }

    pub fn set_counter_set(&self, counter_set: &CounterSet) {
        msg_void_id(self.raw, sel!(b"setCounterSet:\0"), counter_set.raw);
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

    pub fn storage_mode(&self) -> StorageMode {
        match msg_usize(self.raw, sel!(b"storageMode\0")) {
            0 => StorageMode::Shared,
            1 => StorageMode::Managed,
            2 => StorageMode::Private,
            3 => StorageMode::Memoryless,
            _ => StorageMode::Shared,
        }
    }

    pub fn set_storage_mode(&self, storage_mode: StorageMode) {
        msg_void_usize(self.raw, sel!(b"setStorageMode:\0"), storage_mode as usize);
    }

    pub fn sample_count(&self) -> usize {
        msg_usize(self.raw, sel!(b"sampleCount\0"))
    }

    pub fn set_sample_count(&self, sample_count: usize) {
        msg_void_usize(self.raw, sel!(b"setSampleCount:\0"), sample_count);
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

impl CounterSampleBuffer {
    pub fn sample_count(&self) -> Result<usize, MetalError> {
        if self.raw.is_null() {
            return Err(MetalError::new("counter sample buffer is null"));
        }
        Ok(msg_usize(self.raw, sel!(b"sampleCount\0")))
    }

    pub fn label(&self) -> Option<NSString> {
        let ptr = msg_id(self.raw, sel!(b"label\0"));
        if ptr.is_null() {
            None
        } else {
            Some(NSString::from_raw(ptr))
        }
    }

    pub fn device(&self) -> Result<Device, MetalError> {
        if self.raw.is_null() {
            return Err(MetalError::new("counter sample buffer is null"));
        }
        let ptr = retain(msg_id(self.raw, sel!(b"device\0")));
        if ptr.is_null() {
            Err(MetalError::new("counter sample buffer device is null"))
        } else {
            Ok(Device { raw: ptr })
        }
    }

    pub fn storage_mode(&self) -> Result<StorageMode, MetalError> {
        if self.raw.is_null() {
            return Err(MetalError::new("counter sample buffer is null"));
        }
        let selector = sel!(b"storageMode\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "MTLCounterSampleBuffer does not respond to storageMode",
            ));
        }
        let val = msg_usize(self.raw, selector);
        match val {
            0 => Ok(StorageMode::Shared),
            1 => Ok(StorageMode::Managed),
            2 => Ok(StorageMode::Private),
            3 => Ok(StorageMode::Memoryless),
            _ => Err(MetalError::new(format!(
                "invalid counter sample buffer storage mode: {}",
                val
            ))),
        }
    }

    pub fn validate_sample_index(&self, sample_index: usize) -> Result<(), MetalError> {
        if sample_index == COUNTER_DONT_SAMPLE {
            return Ok(());
        }
        let sample_count = self.sample_count()?;
        if sample_index >= sample_count {
            return Err(MetalError::new(format!(
                "counter sample index {} is out of bounds for sample count {}",
                sample_index, sample_count
            )));
        }
        Ok(())
    }

    pub fn validate_resolve_range(&self, range: Range) -> Result<(), MetalError> {
        let sample_count = self.sample_count()?;
        let end = range.location.saturating_add(range.length);
        if end > sample_count {
            return Err(MetalError::new(format!(
                "counter resolve range [{}, {}) exceeds sample count {}",
                range.location, end, sample_count
            )));
        }
        Ok(())
    }

    pub fn resolve_counter_range(&self, range: Range) -> Result<NSData, MetalError> {
        if self.raw.is_null() {
            return Err(MetalError::new("counter sample buffer is null"));
        }
        self.validate_resolve_range(range)?;
        // storageMode is only implemented on the descriptor, not on every backing
        // MTLCounterSampleBuffer, so enforce the requirement only when it can be read.
        if matches!(self.storage_mode(), Ok(mode) if mode != StorageMode::Shared) {
            return Err(MetalError::new(
                "resolveCounterRange: requires MTLStorageModeShared counter sample buffer",
            ));
        }
        let selector = sel!(b"resolveCounterRange:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "resolveCounterRange: is not supported on this counter sample buffer",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Range) -> id =
                transmute(objc_msgSend as *const c_void);
            let data = f(self.raw, selector, range);
            if data.is_null() {
                return Err(MetalError::new("resolveCounterRange: returned null data"));
            }
            Ok(NSData::from_raw(data))
        }
    }
}

fn validate_counter_sampling(
    device: &Device,
    sample_buffer: &CounterSampleBuffer,
    sample_index: usize,
    sampling_point: CounterSamplingPoint,
) -> Result<(), MetalError> {
    device.validate_counter_sampling_point(sampling_point)?;
    sample_buffer.validate_sample_index(sample_index)
}

impl Drop for CounterSampleBuffer {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl Device {
    pub fn supports_counter_sampling(&self, sampling_point: CounterSamplingPoint) -> bool {
        unsafe {
            let selector = sel!(b"supportsCounterSampling:\0");
            if responds_to_selector(self.raw, selector) {
                let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                    transmute(objc_msgSend as *const c_void);
                f(self.raw, selector, sampling_point as usize) != NO
            } else {
                false
            }
        }
    }

    pub fn validate_counter_sampling_point(
        &self,
        sampling_point: CounterSamplingPoint,
    ) -> Result<(), MetalError> {
        if !self.supports_counter_sampling(sampling_point) {
            return Err(MetalError::new(format!(
                "device does not support counter sampling at sampling point {}",
                sampling_point as usize
            )));
        }
        Ok(())
    }

    pub fn counter_sets(&self) -> Result<NSArrayIterator<CounterSet>, MetalError> {
        let selector = sel!(b"counterSets\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("MTLDevice does not respond to counterSets"));
        }
        let array = msg_id(self.raw, selector);
        Ok(NSArrayIterator::new(array))
    }

    pub fn new_counter_sample_buffer(
        &self,
        descriptor: &CounterSampleBufferDescriptor,
    ) -> Result<CounterSampleBuffer, MetalError> {
        let selector = sel!(b"newCounterSampleBufferWithDescriptor:error:\0");
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

impl RenderCommandEncoder {
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) -> Result<(), MetalError> {
        let device = self.device();
        validate_counter_sampling(
            &device,
            sample_buffer,
            sample_index,
            CounterSamplingPoint::AtDrawBoundary,
        )?;
        unsafe {
            let selector = sel!(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
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
        let device = self.device();
        validate_counter_sampling(
            &device,
            sample_buffer,
            sample_index,
            CounterSamplingPoint::AtDispatchBoundary,
        )?;
        unsafe {
            let selector = sel!(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
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
        if self.raw.is_null() {
            return Err(MetalError::new(
                "sampleCountersInBuffer:atSampleIndex:withBarrier: not supported on this BlitCommandEncoder",
            ));
        }
        let device = self.device();
        validate_counter_sampling(
            &device,
            sample_buffer,
            sample_index,
            CounterSamplingPoint::AtBlitBoundary,
        )?;
        unsafe {
            let selector = sel!(b"sampleCountersInBuffer:atSampleIndex:withBarrier:\0");
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
        sample_buffer.validate_resolve_range(range)?;
        unsafe {
            let selector = sel!(b"resolveCounters:inRange:destinationBuffer:destinationOffset:\0");
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
