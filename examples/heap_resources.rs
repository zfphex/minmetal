use minmetal::*;

fn align_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 {
        value
    } else {
        value.div_ceil(alignment) * alignment
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::system_default().ok_or("no Metal device is available")?;

    let texture_descriptor = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 64, 64, false);
    texture_descriptor.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    texture_descriptor.set_storage_mode(StorageMode::Private);

    let buffer_size = 1024;
    let buffer_align =
        device.heap_buffer_size_and_align(buffer_size, ResourceOptions::STORAGE_MODE_PRIVATE);
    let texture_align = device.heap_texture_size_and_align(&texture_descriptor);
    let heap_size = align_up(buffer_align.size, buffer_align.align)
        + align_up(texture_align.size, texture_align.align);

    let heap_descriptor = HeapDescriptor::new();
    heap_descriptor.set_heap_type(HeapType::Automatic);
    heap_descriptor.set_storage_mode(StorageMode::Private);
    heap_descriptor.set_cpu_cache_mode(CpuCacheMode::DefaultCache);
    heap_descriptor.set_hazard_tracking_mode(HazardTrackingMode::Default);
    heap_descriptor.set_size(heap_size);

    let heap = device.new_heap(&heap_descriptor)?;
    let buffer = heap.new_buffer(buffer_size, ResourceOptions::STORAGE_MODE_PRIVATE)?;
    let texture = heap.new_texture(&texture_descriptor)?;

    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.compute_command_encoder()?;
    encoder.use_heap(&heap);
    encoder.use_buffer(&buffer, ResourceUsage::WRITE);
    encoder.use_texture(&texture, ResourceUsage::WRITE);
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(error) = command_buffer.error() {
        return Err(error.into());
    }

    println!("heap resources smoke test passed");
    println!("heap used size: {}", heap.used_size());
    println!("buffer length: {}", buffer.len());
    println!("texture size: {}x{}", texture.width(), texture.height());
    Ok(())
}
