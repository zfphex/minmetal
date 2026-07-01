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

    const WIDTH: usize = 4;
    const HEIGHT: usize = 4;
    const BYTES_PER_PIXEL: usize = 4;
    const BYTES_PER_ROW: usize = WIDTH * BYTES_PER_PIXEL;
    let pixel_count = WIDTH * HEIGHT;
    let mut pixels = vec![0u8; pixel_count * BYTES_PER_PIXEL];
    for (i, chunk) in pixels.chunks_mut(BYTES_PER_PIXEL).enumerate() {
        chunk[0] = (i * 17) as u8;
        chunk[1] = (i * 31) as u8;
        chunk[2] = (i * 47) as u8;
        chunk[3] = 255;
    }

    let path = std::env::temp_dir().join("minmetal_io_texture_test.rgba");
    {
        let mut file = fs::File::create(&path)?;
        file.write_all(&pixels)?;
    }

    let file_handle = device.new_io_file_handle(path.to_str().unwrap())?;

    let texture_descriptor =
        TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, WIDTH, HEIGHT, false);
    texture_descriptor.set_usage(TextureUsage::SHADER_READ);
    texture_descriptor.set_storage_mode(StorageMode::Shared);
    let texture = device.new_texture(&texture_descriptor)?;

    let queue_descriptor = IOCommandQueueDescriptor::new();
    let io_queue = device.new_io_command_queue(&queue_descriptor)?;
    let io_command_buffer = io_queue.command_buffer()?;

    io_command_buffer.load_texture(
        &texture,
        0,
        0,
        Size::new(WIDTH, HEIGHT, 1),
        BYTES_PER_ROW,
        pixels.len(),
        Origin::new(0, 0, 0),
        &file_handle,
        0,
    )?;
    io_command_buffer.commit()?;
    io_command_buffer.wait_until_completed()?;

    if let Some(err) = io_command_buffer.error() {
        return Err(err.into());
    }

    let mut readback = vec![0u8; pixels.len()];
    texture.get_bytes(
        Region::new_2d(0, 0, WIDTH, HEIGHT),
        0,
        &mut readback,
        BYTES_PER_ROW,
    );
    assert_eq!(readback, pixels);

    let _ = fs::remove_file(&path);
    println!("io_texture smoke test passed");
    Ok(())
}
