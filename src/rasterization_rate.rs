use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coordinate2D {
    pub x: f32,
    pub y: f32,
}

impl Coordinate2D {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct RasterizationRateSampleArray {
    pub raw: id,
}

impl Drop for RasterizationRateSampleArray {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl RasterizationRateSampleArray {
    pub fn object_at_indexed_subscript(&self, index: usize) -> f32 {
        let number = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        if number.is_null() {
            0.0
        } else {
            unsafe {
                let f: unsafe extern "C" fn(id, SEL) -> f32 =
                    transmute(objc_msgSend as *const c_void);
                f(number, sel(b"floatValue\0"))
            }
        }
    }

    pub fn set_object_at_indexed_subscript(&self, value: f32, index: usize) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, f32) -> id =
                transmute(objc_msgSend as *const c_void);
            let number = f(class(b"NSNumber\0"), sel(b"numberWithFloat:\0"), value);
            let set: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            set(
                self.raw,
                sel(b"setObject:atIndexedSubscript:\0"),
                number,
                index,
            );
        }
    }
}

#[derive(Debug)]
pub struct RasterizationRateLayerDescriptor {
    pub raw: id,
}

impl RasterizationRateLayerDescriptor {
    pub fn new(sample_count: Size) -> Self {
        unsafe {
            let allocated = msg_id(
                class(b"MTLRasterizationRateLayerDescriptor\0"),
                sel(b"alloc\0"),
            );
            let f: unsafe extern "C" fn(id, SEL, Size) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(allocated, sel(b"initWithSampleCount:\0"), sample_count);
            Self { raw }
        }
    }

    pub fn sample_count(&self) -> Size {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"sampleCount\0"))
        }
    }

    pub fn set_sample_count(&self, sample_count: Size) -> Result<(), MetalError> {
        let selector = sel(b"setSampleCount:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("setSampleCount: is not supported"));
        }
        msg_void_mtlsize(self.raw, selector, sample_count);
        Ok(())
    }

    pub fn max_sample_count(&self) -> Result<Size, MetalError> {
        let selector = sel(b"maxSampleCount\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("maxSampleCount is not supported"));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector))
        }
    }

    pub fn horizontal_sample_storage(&self) -> *mut f32 {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> *mut f32 =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"horizontalSampleStorage\0"))
        }
    }

    pub fn vertical_sample_storage(&self) -> *mut f32 {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> *mut f32 =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"verticalSampleStorage\0"))
        }
    }

    pub fn horizontal(&self) -> RasterizationRateSampleArray {
        RasterizationRateSampleArray {
            raw: retain(msg_id(self.raw, sel(b"horizontal\0"))),
        }
    }

    pub fn vertical(&self) -> RasterizationRateSampleArray {
        RasterizationRateSampleArray {
            raw: retain(msg_id(self.raw, sel(b"vertical\0"))),
        }
    }
}

impl Drop for RasterizationRateLayerDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct RasterizationRateLayerArray {
    pub raw: id,
}

impl Drop for RasterizationRateLayerArray {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl RasterizationRateLayerArray {
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> Option<RasterizationRateLayerDescriptor> {
        let layer = msg_id_usize(self.raw, sel(b"objectAtIndexedSubscript:\0"), index);
        if layer.is_null() {
            None
        } else {
            Some(RasterizationRateLayerDescriptor { raw: retain(layer) })
        }
    }

    pub fn set_object_at_indexed_subscript(
        &self,
        layer: Option<&RasterizationRateLayerDescriptor>,
        index: usize,
    ) {
        msg_void_id_usize(
            self.raw,
            sel(b"setObject:atIndexedSubscript:\0"),
            layer.map_or(NIL, |l| l.raw),
            index,
        );
    }
}

#[derive(Debug)]
pub struct RasterizationRateMapDescriptor {
    pub raw: id,
}

impl RasterizationRateMapDescriptor {
    pub fn with_screen_size(screen_size: Size) -> Self {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Size) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                class(b"MTLRasterizationRateMapDescriptor\0"),
                sel(b"rasterizationRateMapDescriptorWithScreenSize:\0"),
                screen_size,
            ));
            Self { raw }
        }
    }

    pub fn with_screen_size_and_layer(
        screen_size: Size,
        layer: &RasterizationRateLayerDescriptor,
    ) -> Self {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Size, id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                class(b"MTLRasterizationRateMapDescriptor\0"),
                sel(b"rasterizationRateMapDescriptorWithScreenSize:layer:\0"),
                screen_size,
                layer.raw,
            ));
            Self { raw }
        }
    }

    pub fn with_screen_size_and_layers(
        screen_size: Size,
        layers: &[&RasterizationRateLayerDescriptor],
    ) -> Self {
        let layer_ptrs: Vec<id> = layers.iter().map(|layer| layer.raw).collect();
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Size, usize, *const id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = retain(f(
                class(b"MTLRasterizationRateMapDescriptor\0"),
                sel(b"rasterizationRateMapDescriptorWithScreenSize:layerCount:layers:\0"),
                screen_size,
                layer_ptrs.len(),
                layer_ptrs.as_ptr(),
            ));
            Self { raw }
        }
    }

    pub fn layer_at_index(&self, index: usize) -> Option<RasterizationRateLayerDescriptor> {
        let layer = msg_id_usize(self.raw, sel(b"layerAtIndex:\0"), index);
        if layer.is_null() {
            None
        } else {
            Some(RasterizationRateLayerDescriptor { raw: retain(layer) })
        }
    }

    pub fn set_layer_at_index(
        &self,
        layer: Option<&RasterizationRateLayerDescriptor>,
        index: usize,
    ) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, id, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setLayer:atIndex:\0"),
                layer.map_or(NIL, |l| l.raw),
                index,
            );
        }
    }

    pub fn layers(&self) -> RasterizationRateLayerArray {
        RasterizationRateLayerArray {
            raw: retain(msg_id(self.raw, sel(b"layers\0"))),
        }
    }

    pub fn screen_size(&self) -> Size {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"screenSize\0"))
        }
    }

    pub fn set_screen_size(&self, screen_size: Size) {
        msg_void_mtlsize(self.raw, sel(b"setScreenSize:\0"), screen_size);
    }

    pub fn set_label(&self, label: &str) {
        let ns_label = NSString::new(label);
        msg_void_id(self.raw, sel(b"setLabel:\0"), ns_label.raw());
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn layer_count(&self) -> usize {
        msg_usize(self.raw, sel(b"layerCount\0"))
    }
}

impl Drop for RasterizationRateMapDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct RasterizationRateMap {
    pub raw: id,
}

impl RasterizationRateMap {
    pub fn device(&self) -> Device {
        Device {
            raw: retain(msg_id(self.raw, sel(b"device\0"))),
        }
    }

    pub fn label(&self) -> Option<String> {
        ns_string_to_string(msg_id(self.raw, sel(b"label\0")))
    }

    pub fn screen_size(&self) -> Size {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"screenSize\0"))
        }
    }

    pub fn physical_granularity(&self) -> Size {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> Size = transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"physicalGranularity\0"))
        }
    }

    pub fn layer_count(&self) -> usize {
        msg_usize(self.raw, sel(b"layerCount\0"))
    }

    pub fn parameter_buffer_size_and_align(&self) -> SizeAndAlign {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL) -> SizeAndAlign =
                transmute(objc_msgSend as *const c_void);
            f(self.raw, sel(b"parameterBufferSizeAndAlign\0"))
        }
    }

    pub fn copy_parameter_data_to_buffer(
        &self,
        buffer: &Buffer,
        offset: usize,
    ) -> Result<(), MetalError> {
        let selector = sel(b"copyParameterDataToBuffer:offset:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "copyParameterDataToBuffer:offset: is not supported",
            ));
        }
        msg_void_id_usize(self.raw, selector, buffer.raw, offset);
        Ok(())
    }

    pub fn physical_size_for_layer(&self, layer_index: usize) -> Result<Size, MetalError> {
        let selector = sel(b"physicalSizeForLayer:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new("physicalSizeForLayer: is not supported"));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> Size =
                transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector, layer_index))
        }
    }

    pub fn map_screen_to_physical_coordinates(
        &self,
        screen_coordinates: Coordinate2D,
        layer_index: usize,
    ) -> Result<Coordinate2D, MetalError> {
        let selector = sel(b"mapScreenToPhysicalCoordinates:forLayer:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "mapScreenToPhysicalCoordinates:forLayer: is not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Coordinate2D, usize) -> Coordinate2D =
                transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector, screen_coordinates, layer_index))
        }
    }

    pub fn map_physical_to_screen_coordinates(
        &self,
        physical_coordinates: Coordinate2D,
        layer_index: usize,
    ) -> Result<Coordinate2D, MetalError> {
        let selector = sel(b"mapPhysicalToScreenCoordinates:forLayer:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "mapPhysicalToScreenCoordinates:forLayer: is not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, Coordinate2D, usize) -> Coordinate2D =
                transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector, physical_coordinates, layer_index))
        }
    }
}

impl Drop for RasterizationRateMap {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl Device {
    pub fn supports_rasterization_rate_map_with_layer_count(
        &self,
        layer_count: usize,
    ) -> Result<bool, MetalError> {
        let selector = sel(b"supportsRasterizationRateMapWithLayerCount:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "supportsRasterizationRateMapWithLayerCount: is not supported",
            ));
        }
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            Ok(f(self.raw, selector, layer_count) != NO)
        }
    }

    pub fn new_rasterization_rate_map(
        &self,
        descriptor: &RasterizationRateMapDescriptor,
    ) -> Result<RasterizationRateMap, MetalError> {
        let selector = sel(b"newRasterizationRateMapWithDescriptor:\0");
        if !responds_to_selector(self.raw, selector) {
            return Err(MetalError::new(
                "newRasterizationRateMapWithDescriptor: is not supported",
            ));
        }
        let raw = msg_id_id(self.raw, selector, descriptor.raw);
        if raw.is_null() {
            Err(MetalError::new("failed to create rasterization rate map"))
        } else {
            Ok(RasterizationRateMap { raw })
        }
    }
}
