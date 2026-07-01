#![allow(non_camel_case_types, non_snake_case)]

mod device;
mod encoder;
mod ffi;
mod indirect;
mod layer;
mod pass;
mod pipeline;
mod resource;
mod types;
mod capture;
mod counters;
mod sparse;
mod raytracing;

pub use device::*;
pub use encoder::*;
pub use ffi::*;
pub use indirect::*;
pub use layer::*;
pub use pass::*;
pub use pipeline::*;
pub use resource::*;
pub use types::*;
pub use capture::*;
pub use counters::*;
pub use sparse::*;
pub use raytracing::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_shader_source() {
        let Some(device) = Device::system_default() else {
            return;
        };
        let result = device.new_library_with_source("invalid shader code here");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("failed to compile") || !err.to_string().is_empty());
    }

    #[test]
    fn test_missing_function_name() {
        let Some(device) = Device::system_default() else {
            return;
        };
        let library = device.new_library_with_source(r#"
            #include <metal_stdlib>
            using namespace metal;
            kernel void my_kernel() {}
        "#).unwrap();
        let result = library.function("non_existent_function");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("non_existent_function"));
    }

    #[test]
    fn test_invalid_pipeline_descriptor() {
        let Some(device) = Device::system_default() else {
            return;
        };
        let library = device.new_library_with_source(r#"
            #include <metal_stdlib>
            using namespace metal;
            vertex float4 vertex_main(uint vid [[vertex_id]]) { return float4(0.0); }
            fragment float4 fragment_main() { return float4(1.0); }
        "#).unwrap();
        let vertex = library.function("vertex_main").unwrap();
        let fragment = library.function("fragment_main").unwrap();
        let desc = RenderPipelineDescriptor::new();
        desc.set_vertex_function(&vertex);
        desc.set_fragment_function(&fragment);
        desc.set_sample_count(3); // Invalid sample count (must be 1, 2, 4, or 8)
        let result = device.new_render_pipeline_state(&desc);
        assert!(result.is_err());
    }

    #[test]
    fn test_nil_indirect_command_access() {
        let icb = IndirectCommandBuffer { raw: std::ptr::null_mut() };
        let result = icb.render_command(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_selector_availability_helper() {
        unsafe {
            let cls = class(b"NSString\0");
            assert!(responds_to_selector(cls, sel(b"alloc\0")));
            assert!(!responds_to_selector(cls, sel(b"someFakeSelectorThatDoesNotExist\0")));
        }
    }

    #[test]
    fn test_unsupported_api_paths_returning_metal_error() {
        let encoder = BlitCommandEncoder { raw: std::ptr::null_mut() };
        let sample_buf = CounterSampleBuffer { raw: std::ptr::null_mut() };
        let result = encoder.sample_counters_in_buffer(&sample_buf, 0, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not supported"));
    }

    #[test]
    fn test_nil_object_creation_paths() {
        let device = Device { raw: std::ptr::null_mut() };
        let result = device.counter_sets();
        assert!(result.is_err());
    }

    #[test]
    fn test_wrapper_drop_coverage() {
        let desc = CaptureDescriptor::new();
        drop(desc);
        let desc2 = CounterSampleBufferDescriptor::new();
        drop(desc2);
        let desc3 = PrimitiveAccelerationStructureDescriptor::new();
        drop(desc3);
        let desc4 = VisibleFunctionTableDescriptor::new();
        drop(desc4);
        let desc5 = IntersectionFunctionTableDescriptor::new();
        drop(desc5);
        let desc6 = FunctionDescriptor::new();
        drop(desc6);
        let desc7 = IntersectionFunctionDescriptor::new();
        drop(desc7);
        let desc8 = LinkedFunctions::new();
        drop(desc8);
    }
}

