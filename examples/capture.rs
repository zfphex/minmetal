use minmetal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping capture example");
        return Ok(());
    };
    let queue = device.new_command_queue()?;

    let capture_manager = match CaptureManager::shared() {
        Ok(manager) => manager,
        Err(e) => {
            println!("CaptureManager not available: {}. Skipping example", e);
            return Ok(());
        }
    };

    if !capture_manager.supports_destination(CaptureDestination::DeveloperTools) {
        println!("DeveloperTools capture destination not supported, skipping capture example");
        return Ok(());
    }

    let descriptor = CaptureDescriptor::new();
    descriptor.set_capture_object(queue.raw);
    descriptor.set_destination(CaptureDestination::DeveloperTools);

    println!("Starting capture...");
    match capture_manager.start_capture(&descriptor) {
        Ok(_) => {
            let command_buffer = queue.command_buffer()?;
            let encoder = command_buffer.compute_command_encoder()?;
            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();

            capture_manager.stop_capture();
            println!("Capture finished successfully");
        }
        Err(e) => {
            println!("Could not start capture: {}. Skipping capture (this is normal if developer tools/profiling is not active)", e);
        }
    }

    Ok(())
}
