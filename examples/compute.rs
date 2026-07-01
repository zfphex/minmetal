use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void fill_values(device uint* values [[buffer(0)]],
                        uint index [[thread_position_in_grid]]) {
    values[index] = index * 3 + 7;
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;
    let queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADER)?;
    let function = library.function("fill_values")?;
    let pipeline = device.new_compute_pipeline_state_with_function(&function)?;
    let buffer = device.new_buffer(
        16 * std::mem::size_of::<u32>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(0, &buffer, 0);
    encoder.dispatch_threads(Size::new(16, 1, 1), Size::new(16, 1, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(error) = command_buffer.error() {
        return Err(error.into());
    }

    let mut results = [0u32; 16];
    buffer.read_slice(&mut results);
    for (index, value) in results.iter().enumerate() {
        assert_eq!(*value, index as u32 * 3 + 7);
    }

    println!("compute smoke test passed");
    Ok(())
}
