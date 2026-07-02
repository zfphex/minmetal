use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void indirect_kernel(device uint* values [[buffer(0)]],
                            uint index [[thread_position_in_grid]]) {
    values[index] = index * 3 + 2;
}
"#;

#[test]
fn indirect_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. IndirectCommandType flags and combinations
    let cmd_draw = IndirectCommandType::DRAW;
    let cmd_draw_indexed = IndirectCommandType::DRAW_INDEXED;
    let cmd_dispatch = IndirectCommandType::CONCURRENT_DISPATCH;
    let cmd_dispatch_threads = IndirectCommandType::CONCURRENT_DISPATCH_THREADS;

    assert_eq!(cmd_draw.as_raw(), 1);
    assert_eq!(cmd_draw_indexed.as_raw(), 2);
    assert_eq!(cmd_dispatch.as_raw(), 32);
    assert_eq!(cmd_dispatch_threads.as_raw(), 64);

    let draw_combo = cmd_draw | cmd_draw_indexed;
    assert_eq!(draw_combo.as_raw(), 3);

    let dispatch_combo = cmd_dispatch | cmd_dispatch_threads;
    assert_eq!(dispatch_combo.as_raw(), 96);

    let render_compute_combo = cmd_draw | cmd_dispatch;
    assert_eq!(render_compute_combo.as_raw(), 33);

    // 2. IndirectCommandBufferDescriptor tests
    let desc = IndirectCommandBufferDescriptor::new();
    let desc_default = IndirectCommandBufferDescriptor::default();

    desc.set_command_types(cmd_dispatch);
    desc.set_inherit_pipeline_state(true);
    desc.set_inherit_pipeline_state(false);
    desc.set_inherit_buffers(true);
    desc.set_inherit_buffers(false);

    // vertex counts
    desc.set_max_vertex_buffer_bind_count(0);
    desc.set_max_vertex_buffer_bind_count(1);
    desc.set_max_vertex_buffer_bind_count(8);

    // fragment counts
    desc.set_max_fragment_buffer_bind_count(0);
    desc.set_max_fragment_buffer_bind_count(1);
    desc.set_max_fragment_buffer_bind_count(8);

    // kernel counts
    desc.set_max_kernel_buffer_bind_count(0);
    desc.set_max_kernel_buffer_bind_count(1);
    desc.set_max_kernel_buffer_bind_count(8);

    drop(desc_default);

    // 3. Nil-safe / Out-of-bounds error behavior
    let nil_icb = IndirectCommandBuffer { raw: std::ptr::null_mut() };
    assert!(nil_icb.render_command(0).is_err());
    assert!(nil_icb.compute_command(0).is_err());

    // 4. Device path
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping remaining indirect tests.");
        return Ok(());
    };

    // Verify support/creation with descriptor
    let desc_run = IndirectCommandBufferDescriptor::new();
    desc_run.set_command_types(IndirectCommandType::CONCURRENT_DISPATCH);
    desc_run.set_inherit_pipeline_state(false);
    desc_run.set_inherit_buffers(false);
    desc_run.set_max_kernel_buffer_bind_count(1);

    let icb = match device.new_indirect_command_buffer(
        &desc_run,
        2,
        IndirectCommandBufferOptions::NONE,
    ) {
        Ok(buf) => buf,
        Err(e) => {
            println!("Indirect command buffers not supported/created on this device: {}. Skipping execution test.", e);
            return Ok(());
        }
    };

    // Reset buffer
    icb.reset(Range::new(0, 2));

    // Get compute commands
    let comp_cmd0 = icb.compute_command(0)?;
    let comp_cmd1 = icb.compute_command(1)?;

    // Reset commands
    comp_cmd0.reset();
    comp_cmd1.reset();

    // 5. Test compile & execute compute commands
    let queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADER)?;
    let function = library.function("indirect_kernel")?;
    let pipeline_desc = ComputePipelineDescriptor::new();
    pipeline_desc.set_compute_function(&function);
    pipeline_desc.set_support_indirect_command_buffers(true);
    let pipeline = device.new_compute_pipeline_state(&pipeline_desc)?;

    let values = device.new_buffer(
        8 * std::mem::size_of::<u32>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    // Configure dispatch commands
    comp_cmd0.set_compute_pipeline_state(&pipeline);
    comp_cmd0.set_kernel_buffer(0, &values, 0);
    comp_cmd0.dispatch_threadgroups(Size::new(1, 1, 1), Size::new(8, 1, 1));

    // Exercise dispatch_threads path
    comp_cmd1.set_compute_pipeline_state(&pipeline);
    comp_cmd1.set_kernel_buffer(0, &values, 0);
    comp_cmd1.dispatch_threads(Size::new(8, 1, 1), Size::new(8, 1, 1));

    // Execution
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.execute_commands_in_buffer(&icb, Range::new(0, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    assert_eq!(command_buffer.status(), CommandBufferStatus::Completed);
    if let Some(err) = command_buffer.error() {
        return Err(Box::new(err));
    }

    // Verify outcomes
    let mut results = [0u32; 8];
    values.read_slice(&mut results);
    for (index, &val) in results.iter().enumerate() {
        assert_eq!(val, index as u32 * 3 + 2);
    }

    // 6. Test IndirectRenderCommand APIs in isolation (since we don't fully execute render in V11 Batch 2)
    // Separate descriptor to test inheritance options coverage without calling setters on its commands
    let inherit_test_desc = IndirectCommandBufferDescriptor::new();
    inherit_test_desc.set_command_types(IndirectCommandType::DRAW);
    inherit_test_desc.set_inherit_pipeline_state(true);
    inherit_test_desc.set_inherit_buffers(true);
    let _ = device.new_indirect_command_buffer(&inherit_test_desc, 1, IndirectCommandBufferOptions::NONE);

    // Descriptor for testing command-level setters (must not inherit pipeline state or buffers)
    let render_desc = IndirectCommandBufferDescriptor::new();
    render_desc.set_command_types(IndirectCommandType::DRAW | IndirectCommandType::DRAW_INDEXED);
    render_desc.set_inherit_pipeline_state(false);
    render_desc.set_inherit_buffers(false);
    if let Ok(render_icb) = device.new_indirect_command_buffer(&render_desc, 1, IndirectCommandBufferOptions::NONE) {
        render_icb.reset(Range::new(0, 1));
        if let Ok(render_cmd) = render_icb.render_command(0) {
            let vertex_lib = device.new_library_with_source(
                r#"
                #include <metal_stdlib>
                using namespace metal;
                vertex float4 vs(uint vid [[vertex_id]]) { return float4(0.0); }
                fragment float4 fs() { return float4(1.0); }
                "#
            )?;
            let vs_func = vertex_lib.function("vs")?;
            let fs_func = vertex_lib.function("fs")?;
            let pipe_desc = RenderPipelineDescriptor::new();
            pipe_desc.set_vertex_function(&vs_func);
            pipe_desc.set_fragment_function(&fs_func);
            pipe_desc.set_support_indirect_command_buffers(true);
            if let Ok(render_pipeline) = device.new_render_pipeline_state(&pipe_desc) {
                render_cmd.set_render_pipeline_state(&render_pipeline);
                let dummy_buf = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;
                render_cmd.set_vertex_buffer(0, &dummy_buf, 0);
                render_cmd.draw_primitives(PrimitiveType::Triangle, 0, 3, 1, 0);
                render_cmd.draw_indexed_primitives(
                    PrimitiveType::Triangle,
                    3,
                    IndexType::UInt16,
                    &dummy_buf,
                    0,
                    1,
                    0,
                    0,
                );
                render_cmd.reset();
            }
        }
    }

    Ok(())
}
