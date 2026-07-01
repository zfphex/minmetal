use minmetal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping counters example");
        return Ok(());
    };

    let counter_sets = match device.counter_sets() {
        Ok(sets) => sets,
        Err(e) => {
            println!("Counter sets query not supported: {}. Skipping example", e);
            return Ok(());
        }
    };

    println!("Available counter sets (count={}):", counter_sets.len());
    for set in &counter_sets {
        println!(" - Counter Set: {}", set.name());
        for counter in set.counters() {
            println!("    * Counter: {}", counter.name());
        }
    }

    if counter_sets.is_empty() {
        println!("No counter sets available on this device, skipping counter sampling.");
        return Ok(());
    }

    let descriptor = CounterSampleBufferDescriptor::new();
    descriptor.set_counter_set(&counter_sets[0]);
    descriptor.set_storage_mode(StorageMode::Shared);
    descriptor.set_sample_count(4);

    let sample_buffer = match device.new_counter_sample_buffer(&descriptor) {
        Ok(buf) => buf,
        Err(e) => {
            println!(
                "Failed to create counter sample buffer: {}. Skipping sampling.",
                e
            );
            return Ok(());
        }
    };

    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;

    if device.supports_counter_sampling(CounterSamplingPoint::AtBlitBoundary) {
        println!("AtBlitBoundary counter sampling supported, encoding blit sample...");
        let encoder = command_buffer.blit_command_encoder()?;
        let _ = encoder.sample_counters_in_buffer(&sample_buffer, 0, true);
        let dest_buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
        let _ = encoder.resolve_counters(&sample_buffer, Range::new(0, 1), &dest_buffer, 0);
        encoder.end_encoding();
    } else if device.supports_counter_sampling(CounterSamplingPoint::AtDispatchBoundary) {
        println!("AtDispatchBoundary counter sampling supported, encoding compute sample...");
        let encoder = command_buffer.compute_command_encoder()?;
        let _ = encoder.sample_counters_in_buffer(&sample_buffer, 0, true);
        encoder.end_encoding();

        let blit = command_buffer.blit_command_encoder()?;
        let dest_buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
        let _ = blit.resolve_counters(&sample_buffer, Range::new(0, 1), &dest_buffer, 0);
        blit.end_encoding();
    } else {
        println!("No counter sampling points supported on this device. Discovery-only success.");
    }

    command_buffer.commit();
    command_buffer.wait_until_completed();

    Ok(())
}
