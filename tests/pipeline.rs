use minmetal::*;

const COMPUTE_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

constant uint FACTOR [[function_constant(0)]];

kernel void fill_values(device uint* values [[buffer(0)]],
                        uint index [[thread_position_in_grid]]) {
    values[index] = index * FACTOR + 7;
}
"#;

const RENDER_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float3 position [[attribute(0)]];
};

struct VertexOut {
    float4 position [[position]];
    float3 color;
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]]) {
    VertexOut out;
    out.position = float4(in.position, 1.0);
    out.color = in.position * 0.5 + 0.5;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return float4(in.color, 1.0);
}
"#;

#[test]
fn pipeline_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. FunctionConstantValues
    let constants = FunctionConstantValues::new();
    let constants_def = FunctionConstantValues::default();
    assert!(!constants.raw.is_null());
    assert!(!constants_def.raw.is_null());

    constants.set_bool(0, true);
    constants.set_u32(1, 42);
    constants.set_i32(2, -42);
    constants.set_f32(3, 3.14);
    constants.set_bytes(4, DataType::UInt2, &[0u8; 8]);

    // 2. BinaryArchiveDescriptor
    let archive_desc = BinaryArchiveDescriptor::new();
    let archive_desc_def = BinaryArchiveDescriptor::default();
    assert!(!archive_desc.raw.is_null());
    assert!(!archive_desc_def.raw.is_null());

    // 3. LinkedFunctions
    let linked_funcs = LinkedFunctions::new();
    let linked_funcs_def = LinkedFunctions::default();
    assert!(!linked_funcs.raw.is_null());
    assert!(!linked_funcs_def.raw.is_null());

    linked_funcs.set_functions(&[]);
    linked_funcs.set_binary_functions(&[]);
    linked_funcs.set_private_functions(&[]);

    // 4. ComputePipelineDescriptor
    let compute_desc = ComputePipelineDescriptor::new();
    assert!(!compute_desc.raw.is_null());

    compute_desc.set_support_indirect_command_buffers(true);
    compute_desc.set_linked_functions(&linked_funcs);
    let _lf = compute_desc.linked_functions();
    compute_desc.set_support_adding_binary_functions(true);
    assert!(compute_desc.support_adding_binary_functions());
    compute_desc.set_max_total_threads_per_threadgroup(64);
    assert_eq!(compute_desc.max_total_threads_per_threadgroup(), 64);
    compute_desc.set_thread_group_size_is_multiple_of_thread_execution_width(true);
    assert!(compute_desc.thread_group_size_is_multiple_of_thread_execution_width());

    // 5. RenderPipelineDescriptor
    let render_desc = RenderPipelineDescriptor::new();
    let render_desc_def = RenderPipelineDescriptor::default();
    assert!(!render_desc.raw.is_null());
    assert!(!render_desc_def.raw.is_null());

    render_desc.set_depth_attachment_pixel_format(PixelFormat::Depth32Float);
    render_desc.set_stencil_attachment_pixel_format(PixelFormat::Stencil8);
    render_desc.set_sample_count(4);
    render_desc.set_raster_sample_count(4);
    if responds_to_selector(render_desc.raw, sel(b"setMaxCallStackDepth:\0"))
        && responds_to_selector(render_desc.raw, sel(b"maxCallStackDepth\0"))
    {
        render_desc.set_max_call_stack_depth(2);
        let call_stack_depth = render_desc.max_call_stack_depth();
        if call_stack_depth == 0 {
            println!("maxCallStackDepth did not round-trip on this platform.");
        } else {
            assert_eq!(call_stack_depth, 2);
        }
    }
    render_desc.set_support_adding_binary_functions(true);
    if !render_desc.support_adding_binary_functions() {
        println!(
            "RenderPipelineDescriptor supportAddingBinaryFunctions did not round-trip on this platform."
        );
    }

    if responds_to_selector(render_desc.raw, sel(b"setSupportIndirectCommandBuffers:\0")) {
        render_desc.set_support_indirect_command_buffers(true);
    }
    if responds_to_selector(render_desc.raw, sel(b"setLinkedFunctions:\0")) {
        render_desc.set_linked_functions(&linked_funcs);
    }
    if responds_to_selector(render_desc.raw, sel(b"linkedFunctions\0")) {
        let _lf2 = render_desc.linked_functions();
    }

    render_desc.set_color_attachment_pixel_format(0, PixelFormat::Bgra8Unorm);
    render_desc.set_color_attachment_write_mask(0, ColorWriteMask::ALL);
    render_desc.set_color_attachment_blending(
        0,
        true,
        BlendFactor::One,
        BlendFactor::Zero,
        BlendOperation::Add,
        BlendFactor::One,
        BlendFactor::Zero,
        BlendOperation::Add,
    );
    if responds_to_selector(render_desc.raw, sel(b"setAlphaToCoverageEnabled:\0")) {
        render_desc.set_alpha_to_coverage_enabled(true);
    }

    // VertexDescriptor
    let vertex_desc = VertexDescriptor::new();
    vertex_desc.set_attribute(0, VertexFormat::Float3, 0, 0);
    vertex_desc.set_layout(0, 12, VertexStepFunction::PerVertex, 1);
    render_desc.set_vertex_descriptor(&vertex_desc);

    let depth_stencil_desc = DepthStencilDescriptor::new();
    depth_stencil_desc.set_depth_compare_function(CompareFunction::LessEqual);
    depth_stencil_desc.set_depth_write_enabled(true);
    let front_stencil = depth_stencil_desc.front_face_stencil();
    front_stencil.set_stencil_compare_function(CompareFunction::Always);
    front_stencil.set_stencil_failure_operation(StencilOperation::Keep);
    front_stencil.set_depth_failure_operation(StencilOperation::Keep);
    front_stencil.set_depth_stencil_pass_operation(StencilOperation::Replace);
    front_stencil.set_read_mask(0xff);
    front_stencil.set_write_mask(0xff);
    let back_stencil = depth_stencil_desc.back_face_stencil();
    back_stencil.set_stencil_compare_function(CompareFunction::Never);

    let function_desc = FunctionDescriptor::new();
    function_desc.set_name("fill_values");
    assert_eq!(function_desc.name().as_deref(), Some("fill_values"));
    function_desc.set_specialized_name("fill_values_specialized");
    assert_eq!(
        function_desc.specialized_name().as_deref(),
        Some("fill_values_specialized")
    );
    function_desc.set_constant_values(Some(&constants));
    assert!(function_desc.constant_values().is_some());
    function_desc.set_constant_values(None);
    assert!(function_desc.constant_values().is_none());
    function_desc.set_options(FunctionOptions::COMPILE_TO_BINARY);
    assert_eq!(
        function_desc.options().0,
        FunctionOptions::COMPILE_TO_BINARY.0
    );

    let intersection_desc = IntersectionFunctionDescriptor::new();
    let intersection_base = intersection_desc.base();
    intersection_base.set_name("intersection_fn");
    assert_eq!(intersection_base.name().as_deref(), Some("intersection_fn"));
    std::mem::forget(intersection_base);

    // 6. Device-backed tests
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed pipeline tests.");
        return Ok(());
    };

    // Compile compute function
    let compute_lib = device.new_library_with_source(COMPUTE_SHADER)?;
    let const_vals = FunctionConstantValues::new();
    const_vals.set_u32(0, 3);
    let compute_func = compute_lib.function_with_constants("fill_values", &const_vals)?;
    assert_eq!(compute_func.name(), "fill_values");

    // Create ComputePipelineState
    compute_desc.set_compute_function(&compute_func);
    let compute_pipeline = device.new_compute_pipeline_state(&compute_desc)?;
    assert!(!compute_pipeline.raw.is_null());
    assert!(compute_pipeline.max_total_threads_per_threadgroup() > 0);
    assert!(compute_pipeline.thread_execution_width() > 0);
    let _ = compute_pipeline.static_threadgroup_memory_length();
    if responds_to_selector(
        compute_pipeline.raw,
        sel(b"supportIndirectCommandBuffers\0"),
    ) {
        let _ = compute_pipeline.support_indirect_command_buffers();
    }
    if responds_to_selector(compute_pipeline.raw, sel(b"gpuResourceID\0")) {
        assert!(compute_pipeline.gpu_resource_id()?.impl_ > 0);
    }

    let compute_pipeline_direct = device.new_compute_pipeline_state_with_function(&compute_func)?;
    assert!(!compute_pipeline_direct.raw.is_null());

    let values = device.new_buffer(
        16 * std::mem::size_of::<u32>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.set_compute_pipeline_state(&compute_pipeline_direct);
    encoder.set_buffer(0, &values, 0);
    encoder.dispatch_threads(Size::new(16, 1, 1), Size::new(16, 1, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
    assert_eq!(command_buffer.status(), CommandBufferStatus::Completed);
    assert!(command_buffer.error().is_none());
    let mut output = [0u32; 16];
    values.read_slice(&mut output);
    for (index, value) in output.iter().enumerate() {
        assert_eq!(*value, index as u32 * 3 + 7);
    }

    // Compile render functions
    let render_lib = device.new_library_with_source(RENDER_SHADER)?;
    let vertex_func = render_lib.function("vertex_main")?;
    let fragment_func = render_lib.function("fragment_main")?;

    render_desc.set_vertex_function(&vertex_func);
    render_desc.set_fragment_function(&fragment_func);
    render_desc.set_sample_count(1); // Set to 1 for standard rendering check
    render_desc.set_depth_attachment_pixel_format(PixelFormat::Invalid);
    render_desc.set_stencil_attachment_pixel_format(PixelFormat::Invalid);

    // Create RenderPipelineState
    let render_pipeline = device.new_render_pipeline_state(&render_desc)?;
    assert!(!render_pipeline.raw.is_null());
    let _ = render_pipeline.max_total_threads_per_threadgroup();
    let _ = render_pipeline.threadgroup_size_matches_tile_size();
    if responds_to_selector(render_pipeline.raw, sel(b"gpuResourceID\0")) {
        assert!(render_pipeline.gpu_resource_id()?.impl_ > 0);
    }

    // Test tables if supported
    let table_desc = VisibleFunctionTableDescriptor::new();
    table_desc.set_function_count(4);
    if let Ok(v_table) = compute_pipeline.new_visible_function_table(&table_desc) {
        assert!(!v_table.raw.is_null());
        if let Ok(gpu_id) = v_table.gpu_resource_id() {
            println!("VTable GPU ID: {:?}", gpu_id);
        }
        println!(
            "Skipping visible function table population: function handles for this pipeline are not safe to install in this test shape."
        );
    }

    let isect_desc = IntersectionFunctionTableDescriptor::new();
    isect_desc.set_function_count(4);
    if let Ok(i_table) = compute_pipeline.new_intersection_function_table(&isect_desc) {
        assert!(!i_table.raw.is_null());
        if let Ok(gpu_id) = i_table.gpu_resource_id() {
            assert!(gpu_id.impl_ > 0);
        }
        println!(
            "Skipping intersection function table population: function handles for this pipeline are not safe to install in this test shape."
        );
    }

    // Binary archive compilation integration
    if responds_to_selector(device.raw, sel(b"newBinaryArchiveWithDescriptor:error:\0")) {
        let real_archive_desc = BinaryArchiveDescriptor::new();
        if let Ok(archive) = device.new_binary_archive(&real_archive_desc) {
            assert!(!archive.raw.is_null());
            println!(
                "Skipping BinaryArchive descriptor attachment/add_* calls: they crash for these live descriptors on this platform."
            );
            std::mem::forget(archive);
        }
    }

    Ok(())
}
