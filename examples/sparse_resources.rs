use minmetal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping sparse resources example");
        return Ok(());
    };

    if !device.supports_sparse_textures() {
        println!("Sparse textures are not supported on this device, skipping example.");
        return Ok(());
    }

    let tile_size = device.sparse_tile_size(TextureType::D2, PixelFormat::Rgba8Unorm, 1);
    println!(
        "Sparse tile size for 2D Rgba8Unorm: width={}, height={}, depth={}",
        tile_size.width, tile_size.height, tile_size.depth
    );

    if tile_size.width == 0 || tile_size.height == 0 {
        println!("Zero tile size returned, skipping sparse texture creation.");
        return Ok(());
    }

    let heap_descriptor = HeapDescriptor::new();
    heap_descriptor.set_heap_type(HeapType::Sparse);
    heap_descriptor.set_size(1024 * 1024);
    heap_descriptor.set_storage_mode(StorageMode::Private);
    heap_descriptor.set_sparse_page_size(SparsePageSize::Size64);
    let heap = device.new_heap(&heap_descriptor)?;

    let texture_descriptor =
        TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 256, 256, false);
    texture_descriptor.set_storage_mode(StorageMode::Private);
    texture_descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = heap.new_texture(&texture_descriptor)?;

    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.resource_state_command_encoder()?;

    match encoder.update_texture_mapping(
        &texture,
        SparseTextureMappingMode::Map,
        Region::new_2d(0, 0, 256, 256),
        0,
        0,
    ) {
        Ok(_) => {
            println!("Encoded sparse texture mapping operation successfully");
        }
        Err(e) => {
            println!("Sparse texture mapping encoding returned: {}", e);
        }
    }

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    Ok(())
}
