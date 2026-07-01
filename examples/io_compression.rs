use minmetal::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;

    let selector = sel(b"newIOCommandQueueWithDescriptor:error:\0");
    if !responds_to_selector(device.raw, selector) {
        println!("MetalIO is not available on this system, skipping example");
        return Ok(());
    }

    let payload: [u8; 32] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
        0xF0, 0x01,
    ];

    let path = std::env::temp_dir().join("minmetal_io_compression_test.mtlio");
    let path_str = path.to_str().unwrap();
    let _ = fs::remove_file(&path);

    let chunk_size = io_compression_context_default_chunk_size();
    let context = IOCompressionContext::new(path_str, IOCompressionMethod::Lzfse, chunk_size)?;
    context.append_data(&payload);
    let status = context.flush_and_destroy()?;
    if status != IOCompressionStatus::Complete {
        return Err(MetalError::new("compression flush failed").into());
    }

    let file_handle =
        device.new_io_file_handle_compressed(path_str, IOCompressionMethod::Lzfse)?;

    let buffer = device.new_buffer(payload.len(), ResourceOptions::STORAGE_MODE_SHARED)?;

    let queue_descriptor = IOCommandQueueDescriptor::new();
    let io_queue = device.new_io_command_queue(&queue_descriptor)?;
    let io_command_buffer = io_queue.command_buffer()?;
    io_command_buffer.load_buffer(&buffer, 0, payload.len(), &file_handle, 0)?;
    io_command_buffer.commit()?;
    io_command_buffer.wait_until_completed()?;

    if let Some(err) = io_command_buffer.error() {
        return Err(err.into());
    }

    let mut readback = [0u8; 32];
    buffer.read_slice(&mut readback);
    assert_eq!(readback, payload);

    let _ = fs::remove_file(&path);
    println!("io_compression smoke test passed");
    Ok(())
}
