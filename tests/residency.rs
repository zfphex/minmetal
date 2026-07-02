use minmetal::*;

#[test]
fn residency_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. ResidencySetDescriptor::new
    let descriptor = match ResidencySetDescriptor::new() {
        Ok(desc) => desc,
        Err(e) => {
            println!(
                "Residency sets not supported (descriptor allocation failed): {}. Skipping tests.",
                e
            );
            return Ok(());
        }
    };

    // 2. label empty/nonempty
    descriptor.set_label("");
    assert_eq!(descriptor.label().as_deref(), Some(""));

    descriptor.set_label("test-residency-set");
    assert_eq!(descriptor.label().as_deref(), Some("test-residency-set"));

    // 3. initial capacity 0, 1, multiple
    descriptor.set_initial_capacity(0);
    assert_eq!(descriptor.initial_capacity(), 0);

    descriptor.set_initial_capacity(1);
    assert_eq!(descriptor.initial_capacity(), 1);

    descriptor.set_initial_capacity(128);
    assert_eq!(descriptor.initial_capacity(), 128);

    // 4. Device checks
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping remaining residency tests.");
        return Ok(());
    };

    // 5. Device::new_residency_set
    let residency_set = match device.new_residency_set(&descriptor) {
        Ok(rs) => rs,
        Err(e) => {
            println!(
                "Residency sets are not supported on this device/OS version: {}. Skipping remaining tests.",
                e
            );
            return Ok(());
        }
    };

    if let Some(lbl) = residency_set.label() {
        assert_eq!(lbl, "test-residency-set");
    }
    assert_eq!(residency_set.allocated_size(), 0);

    // Test ResidencySet::device()
    let device_from_set = residency_set.device();
    assert!(!device_from_set.raw.is_null());

    // 6. Allocation types
    let buffer = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;
    let alloc_buf = Allocation::from_buffer(&buffer);

    let texture_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 8, 8, false);
    texture_desc.set_storage_mode(StorageMode::Shared);
    let texture = device.new_texture(&texture_desc)?;
    let alloc_tex = Allocation::from_texture(&texture);

    let heap_desc = HeapDescriptor::new();
    heap_desc.set_heap_type(HeapType::Automatic);
    heap_desc.set_storage_mode(StorageMode::Private);
    heap_desc.set_size(1024 * 1024);
    let heap = device.new_heap(&heap_desc)?;
    let alloc_heap = Allocation::from_heap(&heap);

    // 7. add/remove single allocation
    residency_set.add_allocation(&alloc_buf)?;
    assert!(residency_set.contains_allocation(&alloc_buf)?);
    assert_eq!(residency_set.allocation_count()?, 1);

    residency_set.remove_allocation(&alloc_buf)?;
    assert!(!residency_set.contains_allocation(&alloc_buf)?);
    assert_eq!(residency_set.allocation_count()?, 0);

    // Test Allocation::from_acceleration_structure
    if device.supports_raytracing() {
        if let Ok(as_struct) = device.new_acceleration_structure(256) {
            let alloc_as = Allocation::from_acceleration_structure(&as_struct);
            residency_set.add_allocation(&alloc_as)?;
            assert!(residency_set.contains_allocation(&alloc_as)?);
            assert_eq!(residency_set.allocation_count()?, 1);
            residency_set.remove_allocation(&alloc_as)?;
            assert!(!residency_set.contains_allocation(&alloc_as)?);
            assert_eq!(residency_set.allocation_count()?, 0);
        }
    }

    // 8. add/remove allocation slices
    let allocations = [alloc_buf, alloc_tex, alloc_heap];
    residency_set.add_allocations(&allocations)?;
    assert!(residency_set.contains_allocation(&alloc_buf)?);
    assert!(residency_set.contains_allocation(&alloc_tex)?);
    assert!(residency_set.contains_allocation(&alloc_heap)?);
    assert_eq!(residency_set.allocation_count()?, 3);

    residency_set.remove_allocations(&allocations)?;
    assert_eq!(residency_set.allocation_count()?, 0);

    // 9. commit
    residency_set.add_allocations(&allocations)?;
    residency_set.commit()?;

    // 10. request_residency / end_residency
    residency_set.request_residency()?;
    residency_set.end_residency()?;

    // Clean up
    residency_set.remove_all_allocations()?;
    residency_set.commit()?;
    assert_eq!(residency_set.allocation_count()?, 0);

    Ok(())
}
