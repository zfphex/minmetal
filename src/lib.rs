#![allow(non_camel_case_types, non_snake_case)]

mod capture;
mod counters;
mod device;
mod encoder;
mod ffi;
mod indirect;
mod io;
mod layer;
mod pass;
mod pipeline;
mod rasterization_rate;
mod raytracing;
mod residency;
mod resource;
mod sparse;
mod stitching;
mod types;

pub use capture::*;
pub use counters::*;
pub use device::*;
pub use encoder::*;
pub(crate) use ffi::*;
pub use ffi::{
    AutoreleasePool, BOOL, Class, NIL, NO, NSString, SEL, YES, class, id, ns_string_to_string,
    ns_url_from_path, release, responds_to_selector, retain, sel,
};
pub use indirect::*;
pub use io::*;
pub use layer::*;
pub use pass::*;
pub use pipeline::*;
pub use rasterization_rate::*;
pub use raytracing::*;
pub use residency::*;
pub use resource::*;
pub use sparse::*;
pub use stitching::*;
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
        let library = device
            .new_library_with_source(
                r#"
            #include <metal_stdlib>
            using namespace metal;
            kernel void my_kernel() {}
        "#,
            )
            .unwrap();
        let result = library.function("non_existent_function");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("non_existent_function")
        );
    }

    #[test]
    fn test_invalid_pipeline_descriptor() {
        let Some(device) = Device::system_default() else {
            return;
        };
        let library = device
            .new_library_with_source(
                r#"
            #include <metal_stdlib>
            using namespace metal;
            vertex float4 vertex_main(uint vid [[vertex_id]]) { return float4(0.0); }
            fragment float4 fragment_main() { return float4(1.0); }
        "#,
            )
            .unwrap();
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
        let icb = IndirectCommandBuffer {
            raw: std::ptr::null_mut(),
        };
        let result = icb.render_command(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_selector_availability_helper() {
        let cls = class(b"NSString\0");
        assert!(responds_to_selector(cls, sel(b"alloc\0")));
        assert!(!responds_to_selector(
            cls,
            sel(b"someFakeSelectorThatDoesNotExist\0")
        ));
    }

    #[test]
    fn test_unsupported_api_paths_returning_metal_error() {
        let encoder = BlitCommandEncoder {
            raw: std::ptr::null_mut(),
        };
        let sample_buf = CounterSampleBuffer {
            raw: std::ptr::null_mut(),
        };
        let result = encoder.sample_counters_in_buffer(&sample_buf, 0, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not supported"));
    }

    #[test]
    fn test_nil_object_creation_paths() {
        let device = Device {
            raw: std::ptr::null_mut(),
        };
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
        let desc9 = IOCommandQueueDescriptor::new();
        drop(desc9);
        if let Ok(ctx) = IOCompressionContext::new("/dev/null", IOCompressionMethod::Lzfse, 4096) {
            drop(ctx);
        }
        if let Ok(desc10) = ResidencySetDescriptor::new() {
            drop(desc10);
        }
        let layer = RasterizationRateLayerDescriptor::new(Size::new(2, 2, 0));
        drop(layer);
        let map_desc = RasterizationRateMapDescriptor::with_screen_size(Size::new(64, 64, 0));
        drop(map_desc);
    }

    #[test]
    fn test_load_non_existent_metallib() {
        let Some(device) = Device::system_default() else {
            return;
        };
        let result = device.new_library_with_file("/tmp/non_existent_file_xyz_123.metallib");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        println!("Actual error message: {}", err_msg);
        assert!(!err_msg.is_empty());
    }

    #[test]
    fn test_enum_discriminants() {
        assert_eq!(PixelFormat::Invalid as usize, 0);
        assert_eq!(PixelFormat::Rgba8Unorm as usize, 70);
        assert_eq!(StorageMode::Shared as usize, 0);
        assert_eq!(StorageMode::Managed as usize, 1);
        assert_eq!(StorageMode::Private as usize, 2);
        assert_eq!(LoadAction::DontCare as usize, 0);
        assert_eq!(LoadAction::Load as usize, 1);
        assert_eq!(LoadAction::Clear as usize, 2);
        assert_eq!(StoreAction::DontCare as usize, 0);
        assert_eq!(StoreAction::Store as usize, 1);
        assert_eq!(LogLevel::Undefined as isize, -1);
        assert_eq!(LogLevel::Debug as isize, 0);
        assert_eq!(DispatchType::Serial as usize, 0);
        assert_eq!(DispatchType::Concurrent as usize, 1);
        assert_eq!(CommandBufferStatus::NotEnqueued as usize, 0);
        assert_eq!(CommandBufferStatus::Completed as usize, 4);
        assert_eq!(LibraryType::Executable as usize, 0);
        assert_eq!(LibraryType::Dynamic as usize, 1);
        assert_eq!(LibraryOptimizationLevel::Default as isize, 0);
        assert_eq!(LibraryOptimizationLevel::Size as isize, 1);
        assert_eq!(LibraryError::Unsupported as usize, 1);
        assert_eq!(LibraryError::FileNotFound as usize, 6);
    }
}
