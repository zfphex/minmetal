use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Args {
    device uint* values [[id(0)]];
};

kernel void argument_buffer_kernel(device Args& args [[buffer(0)]],
                                   uint index [[thread_position_in_grid]]) {
    args.values[index] = index + 11;
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;
    let queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADER)?;
    let function = library.function("argument_buffer_kernel")?;
    let pipeline = device.new_compute_pipeline_state_with_function(&function)?;

    let values = device.new_buffer(
        16 * std::mem::size_of::<u32>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    let value_argument = ArgumentDescriptor::new();
    value_argument.set_index(0);
    value_argument.set_data_type(DataType::Pointer);
    value_argument.set_access(ArgumentAccess::ReadWrite);

    let argument_encoder = device.new_argument_encoder(&[&value_argument])?;
    let argument_buffer = device.new_buffer(
        argument_encoder.encoded_length(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    argument_encoder.set_argument_buffer(&argument_buffer, 0);
    argument_encoder.set_buffer(0, &values, 0);

    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(0, &argument_buffer, 0);
    encoder.dispatch_threads(Size::new(16, 1, 1), Size::new(16, 1, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(error) = command_buffer.error() {
        return Err(error.into());
    }

    let mut results = [0u32; 16];
    values.read_slice(&mut results);
    for (index, value) in results.iter().enumerate() {
        assert_eq!(*value, index as u32 + 11);
    }

    println!("argument buffer smoke test passed");
    Ok(())
}
