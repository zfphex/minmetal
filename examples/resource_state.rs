use minmetal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;
    let queue = device.new_command_queue()?;
    let fence = device.new_fence()?;

    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.resource_state_command_encoder()?;
    encoder.update_fence(&fence);
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(error) = command_buffer.error() {
        return Err(error.into());
    }

    println!("resource state smoke test passed");
    Ok(())
}
