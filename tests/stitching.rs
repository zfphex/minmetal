use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

[[stitchable]] float stitched_scale(float val) {
    return val;
}

kernel void compute_main(device float* out [[buffer(0)]],
                         device const float* in [[buffer(1)]],
                         uint index [[thread_position_in_grid]]) {
    out[index] = stitched_scale(in[index]);
}

[[stitchable]] float multiply_by_two(float val) {
    return val * 2.0;
}
"#;

#[test]
fn stitching_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cover StitchedLibraryOptions
    assert_eq!(StitchedLibraryOptions::NONE.as_raw(), 0);
    assert_eq!(
        StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS.as_raw(),
        1 << 0
    );
    assert_eq!(
        StitchedLibraryOptions::STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT.as_raw(),
        1 << 1
    );

    let combined = StitchedLibraryOptions(
        StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS.as_raw()
            | StitchedLibraryOptions::STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT.as_raw(),
    );
    assert_eq!(
        combined.as_raw(),
        StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS.as_raw()
            | StitchedLibraryOptions::STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT.as_raw()
    );

    // Check class availability first
    if class(b"MTLFunctionStitchingGraph\0").is_null() {
        println!("MTLFunctionStitchingGraph class not available, skipping stitching tests.");
        return Ok(());
    }

    // 2. Cover stitching object construction without requiring a device
    let always_inline = FunctionStitchingAttributeAlwaysInline::new();
    let attr = FunctionStitchingAttribute::from(always_inline);

    let input0 = FunctionStitchingInputNode::new(0);
    let input1 = FunctionStitchingInputNode::new(1);
    let input4 = FunctionStitchingInputNode::new(4);

    assert_eq!(input0.argument_index(), 0);
    assert_eq!(input1.argument_index(), 1);
    assert_eq!(input4.argument_index(), 4);

    input4.set_argument_index(5);
    assert_eq!(input4.argument_index(), 5);

    let node0 = FunctionStitchingNode::from(input0);
    let node1 = FunctionStitchingNode::from(input1);
    let node4 = FunctionStitchingNode::from(input4);

    let fn_empty = FunctionStitchingFunctionNode::new("empty", &[], &[]);
    let fn_one = FunctionStitchingFunctionNode::new("one", &[&node0], &[]);
    let fn_multi = FunctionStitchingFunctionNode::new("multi", &[&node0, &node1], &[]);
    let fn_dep = FunctionStitchingFunctionNode::new("dep", &[&node4], &[&fn_one]);

    assert_eq!(fn_empty.name(), "empty");
    fn_empty.set_name("empty_new");
    assert_eq!(fn_empty.name(), "empty_new");

    assert_eq!(fn_one.arguments().len(), 1);
    fn_one.set_arguments(&[&node0, &node1]);
    assert_eq!(fn_one.arguments().len(), 2);

    assert_eq!(fn_dep.control_dependencies().len(), 1);
    fn_dep.set_control_dependencies(&[&fn_one, &fn_multi]);
    assert_eq!(fn_dep.control_dependencies().len(), 2);

    let graph_none = FunctionStitchingGraph::new("g1", &[&fn_empty], None, &[]);
    let graph_some = FunctionStitchingGraph::new("g2", &[&fn_empty], Some(&fn_empty), &[&attr]);

    assert_eq!(graph_none.function_name(), "g1");
    graph_none.set_function_name("g1_new");
    assert_eq!(graph_none.function_name(), "g1_new");

    assert_eq!(graph_none.nodes().len(), 1);
    graph_none.set_nodes(&[&fn_empty, &fn_one]);
    assert_eq!(graph_none.nodes().len(), 2);

    assert!(graph_none.output_node().is_none());
    assert!(graph_some.output_node().is_some());

    graph_none.set_output_node(Some(&fn_empty));
    assert!(graph_none.output_node().is_some());
    graph_none.set_output_node(None);
    assert!(graph_none.output_node().is_none());

    assert_eq!(graph_none.attributes().len(), 0);
    assert_eq!(graph_some.attributes().len(), 1);
    graph_none.set_attributes(&[&attr]);
    assert_eq!(graph_none.attributes().len(), 1);

    // 3. Cover StitchedLibraryDescriptor
    let desc = StitchedLibraryDescriptor::new();
    assert_eq!(desc.function_graphs().len(), 0);
    assert_eq!(desc.functions().len(), 0);

    desc.set_function_graphs(&[]);
    assert_eq!(desc.function_graphs().len(), 0);
    desc.set_function_graphs(&[&graph_some]);
    assert_eq!(desc.function_graphs().len(), 1);
    desc.set_function_graphs(&[&graph_none, &graph_some]);
    assert_eq!(desc.function_graphs().len(), 2);

    // For options, set_options, binary_archives, and set_binary_archives, check selector availability explicitly
    if responds_to_selector(desc.raw, sel(b"options\0")) {
        desc.set_options(StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS)?;
        assert_eq!(
            desc.options()?.as_raw(),
            StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS.as_raw()
        );
        desc.set_options(StitchedLibraryOptions::NONE)?;
        assert_eq!(
            desc.options()?.as_raw(),
            StitchedLibraryOptions::NONE.as_raw()
        );
    } else {
        println!("options / setOptions: is unsupported on this platform");
    }

    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed stitching tests.");
        return Ok(());
    };

    if responds_to_selector(desc.raw, sel(b"binaryArchives\0")) {
        desc.set_binary_archives(&[])?;
        assert_eq!(desc.binary_archives()?.len(), 0);

        let archive_selector = sel(b"newBinaryArchiveWithDescriptor:error:\0");
        if responds_to_selector(device.raw, archive_selector) {
            let archive_desc = BinaryArchiveDescriptor::new();
            if let Ok(archive) = device.new_binary_archive(&archive_desc) {
                desc.set_binary_archives(&[&archive])?;
                assert_eq!(desc.binary_archives()?.len(), 1);
                desc.set_binary_archives(&[])?;
            }
        }
    } else {
        println!("binaryArchives / setBinaryArchives: is unsupported on this platform");
    }

    // 4. Cover device-backed stitched library behavior
    let stitched_selector = sel(b"newLibraryWithStitchedDescriptor:error:\0");
    if !responds_to_selector(device.raw, stitched_selector) {
        println!(
            "newLibraryWithStitchedDescriptor:error: is unsupported, skipping device-backed stitching."
        );
        return Ok(());
    }

    // Compile the base shader
    let library = device.new_library_with_source(SHADER)?;
    let base_func = library.function("multiply_by_two")?;
    let kernel_func = library.function("compute_main")?;

    desc.set_functions(&[&base_func]);
    assert_eq!(desc.functions().len(), 1);

    desc.set_functions(&[]);
    assert_eq!(desc.functions().len(), 0);

    desc.set_functions(&[&base_func]);

    // Re-create the graph to target the actual multiply_by_two function
    let input0_real = FunctionStitchingInputNode::new(0);
    let node_arg0_real = FunctionStitchingNode::from(input0_real);
    let func_node_real =
        FunctionStitchingFunctionNode::new("multiply_by_two", &[&node_arg0_real], &[]);
    let graph_real = FunctionStitchingGraph::new(
        "stitched_scale",
        &[&func_node_real],
        Some(&func_node_real),
        &[],
    );

    desc.set_function_graphs(&[&graph_real]);

    // Create a stitched library with Device::new_library_with_stitched_descriptor
    let stitched_lib = device.new_library_with_stitched_descriptor(&desc)?;
    let stitched_func = stitched_lib.function("stitched_scale")?;
    assert_eq!(stitched_func.name(), "stitched_scale");

    // Link and execute to verify correct calculation
    let pipeline_desc = ComputePipelineDescriptor::new();
    pipeline_desc.set_compute_function(&kernel_func);

    let linked_funcs = LinkedFunctions::new();
    linked_funcs.set_functions(&[&stitched_func]);
    pipeline_desc.set_linked_functions(&linked_funcs);

    let pipeline = device.new_compute_pipeline_state(&pipeline_desc)?;

    let count = 16;
    let size = count * std::mem::size_of::<f32>();
    let input_data: Vec<f32> = (0..count).map(|i| (i + 1) as f32).collect();
    let buffer_in =
        device.new_buffer_with_data(&input_data, ResourceOptions::STORAGE_MODE_SHARED)?;
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
    for (i, &val) in results.iter().enumerate() {
        let expected = (i + 1) as f32 * 2.0;
        assert!(
            (val - expected).abs() < 1e-5,
            "Expected {}, got {}",
            expected,
            val
        );
    }

    // Build an invalid graph using a non-existent stitchable function and assert it returns Err
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

    let compile_result = device.new_library_with_stitched_descriptor(&invalid_desc);
    assert!(
        compile_result.is_err(),
        "Expected compilation to fail for invalid stitching graph, but it succeeded."
    );
    let err_msg = compile_result.unwrap_err().to_string();
    assert!(
        !err_msg.is_empty(),
        "Expected non-empty compiler error message"
    );

    Ok(())
}
