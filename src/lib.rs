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

pub use device::*;
pub use encoder::*;
pub use ffi::*;
pub use indirect::*;
pub use layer::*;
pub use pass::*;
pub use pipeline::*;
pub use resource::*;
pub use types::*;

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
}

