use minmetal::*;

#[test]
fn sparse_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. SparseTextureMappingMode variants
    let modes = [
        SparseTextureMappingMode::Map,
        SparseTextureMappingMode::Unmap,
    ];
    for &mode in &modes {
        match mode {
            SparseTextureMappingMode::Map => assert_eq!(mode as usize, 0),
            SparseTextureMappingMode::Unmap => assert_eq!(mode as usize, 1),
        }
    }

    // 2. Device checks
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping sparse tests.");
        return Ok(());
    };

    let supports_sparse = device.supports_sparse_textures();
    println!("Device supports sparse textures: {}", supports_sparse);

    let tile_size = device.sparse_tile_size(TextureType::D2, PixelFormat::Rgba8Unorm, 1);
    println!(
        "Sparse tile size: {}x{}x{}",
        tile_size.width, tile_size.height, tile_size.depth
    );

    // If not supported, we must skip mapping updates gracefully
    if !supports_sparse || tile_size.width == 0 || tile_size.height == 0 {
        println!(
            "Sparse textures not supported/available on this device, skipping resource state encoding."
        );
        return Ok(());
    }

    // 3. Supported-device path
    let heap_descriptor = HeapDescriptor::new();
    heap_descriptor.set_heap_type(HeapType::Sparse);
    heap_descriptor.set_size(1024 * 1024);
    heap_descriptor.set_storage_mode(StorageMode::Private);
    heap_descriptor.set_sparse_page_size(SparsePageSize::Size64);
    let heap = match device.new_heap(&heap_descriptor) {
        Ok(h) => h,
        Err(e) => {
            println!(
                "Could not create sparse heap: {}. Skipping mapping update test.",
                e
            );
            return Ok(());
        }
    };

    let texture_descriptor =
        TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 256, 256, false);
    texture_descriptor.set_storage_mode(StorageMode::Private);
    texture_descriptor.set_usage(TextureUsage::SHADER_READ);

    let texture = heap.new_texture(&texture_descriptor)?;

    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.resource_state_command_encoder()?;

    // Map operation
    encoder.update_texture_mapping(
        &texture,
        SparseTextureMappingMode::Map,
        Region::new_2d(0, 0, 256, 256),
        0,
        0,
    )?;

    // Unmap operation
    encoder.update_texture_mapping(
        &texture,
        SparseTextureMappingMode::Unmap,
        Region::new_2d(0, 0, 256, 256),
        0,
        0,
    )?;

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    assert_eq!(command_buffer.status(), CommandBufferStatus::Completed);
    if let Some(err) = command_buffer.error() {
        return Err(Box::new(err));
    }

    Ok(())
}
