use minmetal::*;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;

    let selector = sel(b"newIOCommandQueueWithDescriptor:error:\0");
    if !responds_to_selector(device.raw, selector) {
        println!("MetalIO is not available on this system, skipping example");
        return Ok(());
    }

    let payload: [u8; 16] = [
        0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x02, 0x03, 0x04, 0xAA, 0xBB, 0xCC, 0xDD, 0x10, 0x20, 0x30,
        0x40,
    ];
    let path = std::env::temp_dir().join("minmetal_io_buffer_test.bin");
    {
        let mut file = fs::File::create(&path)?;
        file.write_all(&payload)?;
    }

    let file_handle = device.new_io_file_handle(path.to_str().unwrap())?;

    let queue_descriptor = IOCommandQueueDescriptor::new();
    queue_descriptor.set_type(IOCommandQueueType::Serial);
    let io_queue = device.new_io_command_queue(&queue_descriptor)?;

    let buffer = device.new_buffer(payload.len(), ResourceOptions::STORAGE_MODE_SHARED)?;

    let io_command_buffer = io_queue.command_buffer()?;
    io_command_buffer.load_buffer(&buffer, 0, payload.len(), &file_handle, 0)?;
    io_command_buffer.commit()?;
    io_command_buffer.wait_until_completed()?;

    if let Some(err) = io_command_buffer.error() {
        return Err(err.into());
    }
    assert_eq!(io_command_buffer.status()?, IOStatus::Complete);

    let mut readback = [0u8; 16];
    buffer.read_slice(&mut readback);
    assert_eq!(readback, payload);

    let _ = fs::remove_file(&path);
    println!("io_buffer smoke test passed");
    Ok(())
}
