use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void indirect_kernel(device uint* values [[buffer(0)]],
                            uint index [[thread_position_in_grid]]) {
    values[index] = index * 2 + 1;
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;
    let queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADER)?;
    let function = library.function("indirect_kernel")?;
    let pipeline_descriptor = ComputePipelineDescriptor::new();
    pipeline_descriptor.set_compute_function(&function);
    pipeline_descriptor.set_support_indirect_command_buffers(true);
    let pipeline = device.new_compute_pipeline_state(&pipeline_descriptor)?;
    let values = device.new_buffer(
        16 * std::mem::size_of::<u32>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    let descriptor = IndirectCommandBufferDescriptor::new();
    descriptor.set_command_types(IndirectCommandType::CONCURRENT_DISPATCH);
    descriptor.set_inherit_pipeline_state(false);
    descriptor.set_inherit_buffers(false);
    descriptor.set_max_kernel_buffer_bind_count(1);

    let indirect =
        device.new_indirect_command_buffer(&descriptor, 1, IndirectCommandBufferOptions::NONE)?;
    indirect.reset(Range::new(0, 1));

    let command = indirect.compute_command(0)?;
    command.set_compute_pipeline_state(&pipeline);
    command.set_kernel_buffer(0, &values, 0);
    command.dispatch_threadgroups(Size::new(1, 1, 1), Size::new(16, 1, 1));

    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.execute_commands_in_buffer(&indirect, Range::new(0, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(error) = command_buffer.error() {
        return Err(error.into());
    }

    let mut results = [0u32; 16];
    values.read_slice(&mut results);
    for (index, value) in results.iter().enumerate() {
        assert_eq!(*value, index as u32 * 2 + 1);
    }

    println!("indirect commands smoke test passed");
    Ok(())
}
