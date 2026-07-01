use minmetal::*;

fn align_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 {
        value
    } else {
        value.div_ceil(alignment) * alignment
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;

    let selector = sel(b"newResidencySetWithDescriptor:error:\0");
    if !responds_to_selector(device.raw, selector) {
        println!("Residency sets are not available on this system, skipping example");
        return Ok(());
    }

    let descriptor = ResidencySetDescriptor::new()?;
    descriptor.set_label("minmetal residency smoke test");
    descriptor.set_initial_capacity(4);
    let residency_set = device.new_residency_set(&descriptor)?;

    let buffer = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;

    let texture_descriptor = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 8, 8, false);
    texture_descriptor.set_storage_mode(StorageMode::Shared);
    let texture = device.new_texture(&texture_descriptor)?;

    let buffer_size = 512;
    let buffer_align =
        device.heap_buffer_size_and_align(buffer_size, ResourceOptions::STORAGE_MODE_PRIVATE);
    let heap_descriptor = HeapDescriptor::new();
    heap_descriptor.set_heap_type(HeapType::Automatic);
    heap_descriptor.set_storage_mode(StorageMode::Private);
    heap_descriptor.set_size(align_up(buffer_align.size, buffer_align.align));
    let heap = device.new_heap(&heap_descriptor)?;

    residency_set.add_allocation(&Allocation::from_buffer(&buffer))?;
    residency_set.add_allocation(&Allocation::from_texture(&texture))?;
    residency_set.add_allocation(&Allocation::from_heap(&heap))?;
    residency_set.commit()?;

    assert_eq!(residency_set.allocation_count()?, 3);
    assert!(residency_set.contains_allocation(&Allocation::from_buffer(&buffer))?);
    assert!(residency_set.contains_allocation(&Allocation::from_texture(&texture))?);
    assert!(residency_set.contains_allocation(&Allocation::from_heap(&heap))?);

    residency_set.request_residency()?;
    residency_set.end_residency()?;

    println!("residency_set smoke test passed");
    println!("allocation count: {}", residency_set.allocation_count()?);
    println!("allocated size: {}", residency_set.allocated_size());
    Ok(())
}
