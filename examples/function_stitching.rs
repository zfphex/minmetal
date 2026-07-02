use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

// Dummy fallback implementation of the stitched function to satisfy compiling the base library.
// The stitched version will override this when we link it at pipeline creation time.
[[stitchable]] float stitched_scale(float val) {
    return val;
}

// The main compute kernel that calls the stitched function
kernel void compute_main(device float* out [[buffer(0)]],
                         device const float* in [[buffer(1)]],
                         uint index [[thread_position_in_grid]]) {
    out[index] = stitched_scale(in[index]);
}

// The stitchable building block function
[[stitchable]] float multiply_by_two(float val) {
    return val * 2.0;
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _pool = AutoreleasePool::new();

    let Some(device) = Device::system_default() else {
        println!("No Metal device found, skipping example.");
        return Ok(());
    };

    println!("Using Metal device: {}", device.name());

    // 1. Compile base library containing the stitchable helper and the main kernel
    let library = match device.new_library_with_source(SHADER) {
        Ok(lib) => lib,
        Err(e) => {
            println!("Failed to compile base library: {}", e);
            return Ok(());
        }
    };

    let base_func = match library.function("multiply_by_two") {
        Ok(func) => func,
        Err(e) => {
            println!("Failed to get multiply_by_two function: {}", e);
            return Ok(());
        }
    };

    let kernel_func = match library.function("compute_main") {
        Ok(func) => func,
        Err(e) => {
            println!("Failed to get compute_main function: {}", e);
            return Ok(());
        }
    };

    // 2. Define inputs and function node for stitched_scale(val)
    let input0 = FunctionStitchingInputNode::new(0); // float val
    let node_arg0 = FunctionStitchingNode::from(input0);

    let func_node = FunctionStitchingFunctionNode::new("multiply_by_two", &[&node_arg0], &[]);

    // 3. Create Function Stitching Graph
    let graph = FunctionStitchingGraph::new("stitched_scale", &[&func_node], Some(&func_node), &[]);

    // 4. Set up descriptor
    let desc = StitchedLibraryDescriptor::new();
    desc.set_functions(&[&base_func]);
    desc.set_function_graphs(&[&graph]);

    // Test macOS 15+ options/binary archives if available
    let _ = desc.options();
    let _ = desc.set_options(StitchedLibraryOptions::NONE);
    let _ = desc.binary_archives();
    let _ = desc.set_binary_archives(&[]);

    // 5. Compile stitched library
    println!("Stitching and compiling library...");
    let stitched_lib = match device.new_library_with_stitched_descriptor(&desc) {
        Ok(lib) => lib,
        Err(e) => {
            println!("Failed to compile stitched library: {}", e);
            return Ok(());
        }
    };

    // 6. Retrieve stitched function
    let stitched_func = stitched_lib.function("stitched_scale")?;
    println!(
        "Successfully retrieved stitched function: {}",
        stitched_func.name()
    );

    // 7. Create compute pipeline state linking the stitched function
    let pipeline_desc = ComputePipelineDescriptor::new();
    pipeline_desc.set_compute_function(&kernel_func);

    let linked_funcs = LinkedFunctions::new();
    linked_funcs.set_functions(&[&stitched_func]);
    pipeline_desc.set_linked_functions(&linked_funcs);

    let pipeline = device.new_compute_pipeline_state(&pipeline_desc)?;

    // 8. Run compute pipeline
    let count = 16;
    let size = count * std::mem::size_of::<f32>();
    let buffer_in = device.new_buffer_with_data(
        &[
            1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0,
        ],
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    let buffer_out = device.new_buffer(size, ResourceOptions::STORAGE_MODE_SHARED)?;

    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(0, &buffer_out, 0);
    encoder.set_buffer(1, &buffer_in, 0);
    encoder.dispatch_threads(Size::new(count, 1, 1), Size::new(count, 1, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(err) = command_buffer.error() {
        return Err(err.into());
    }

    let mut results = vec![0.0f32; count];
    buffer_out.read_slice(&mut results);
    println!("Stitched output results: {:?}", results);
    for (i, val) in results.iter().enumerate() {
        let expected = (i + 1) as f32 * 2.0;
        assert!(
            (val - expected).abs() < 1e-5,
            "Expected {}, got {}",
            expected,
            val
        );
    }

    println!("Stitched compute pipeline execution validation: PASSED!");

    // 9. Verify graph construction/compilation failure returns a readable MetalError
    // Re-create nodes/args for a new graph to test validation failure path
    let input0_err = FunctionStitchingInputNode::new(0);
    let node_arg0_err = FunctionStitchingNode::from(input0_err);

    let invalid_func_node =
        FunctionStitchingFunctionNode::new("non_existent_function", &[&node_arg0_err], &[]);
    let invalid_graph = FunctionStitchingGraph::new(
        "invalid_stitched_name",
        &[&invalid_func_node],
        Some(&invalid_func_node),
        &[],
    );
    let invalid_desc = StitchedLibraryDescriptor::new();
    invalid_desc.set_functions(&[&base_func]);
    invalid_desc.set_function_graphs(&[&invalid_graph]);

    match device.new_library_with_stitched_descriptor(&invalid_desc) {
        Ok(_) => {
            println!("WARNING: Unexpectedly compiled invalid stitched library successfully");
        }
        Err(e) => {
            println!("Got expected compilation error on invalid graph:\n{}", e);
            assert!(!e.to_string().is_empty());
        }
    }

    println!("All function stitching tests PASSED!");
    Ok(())
}
