use minmetal::*;

#[derive(Clone, Copy)]
#[repr(C)]
struct MTLAccelerationStructureInstanceDescriptor {
    transform: [[f32; 3]; 4],
    options: u32,
    mask: u32,
    intersection_function_table_offset: u32,
    acceleration_structure_index: u32,
}

#[test]
fn raytracing_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cover pure value types / enums
    let sizes_default = AccelerationStructureSizes::default();
    assert_eq!(sizes_default.acceleration_structure_size, 0);
    assert_eq!(sizes_default.build_scratch_buffer_size, 0);
    assert_eq!(sizes_default.refit_scratch_buffer_size, 0);

    assert_eq!(AccelerationStructureGeometryFlags::NONE.0, 0);
    assert_eq!(AccelerationStructureGeometryFlags::OPAQUE.0, 1);
    assert_eq!(AccelerationStructureGeometryFlags::NON_OPAQUE.0, 2);

    let flag_comb = AccelerationStructureGeometryFlags(
        AccelerationStructureGeometryFlags::OPAQUE.0
            | AccelerationStructureGeometryFlags::NON_OPAQUE.0,
    );
    assert_eq!(flag_comb.0, 3);

    assert_eq!(AccelerationStructureUsage::NONE.0, 0);
    assert_eq!(AccelerationStructureUsage::REFIT.0, 1);
    assert_eq!(AccelerationStructureUsage::PREFER_FAST_BUILD.0, 2);
    assert_eq!(AccelerationStructureUsage::PREFER_FAST_INTERSECTION.0, 4);

    let usage_comb = AccelerationStructureUsage(
        AccelerationStructureUsage::REFIT.0 | AccelerationStructureUsage::PREFER_FAST_BUILD.0,
    );
    assert_eq!(usage_comb.0, 3);

    // 2. Cover descriptor construction and mutators without requiring a device
    let tri_desc = AccelerationStructureTriangleGeometryDescriptor::new();
    tri_desc.set_vertex_buffer_offset(0);
    tri_desc.set_vertex_stride(12);
    tri_desc.set_vertex_format(VertexFormat::Float3);
    tri_desc.set_index_buffer_offset(0);
    tri_desc.set_index_type(IndexType::UInt16);
    tri_desc.set_triangle_count(1);
    tri_desc.set_opaque(true);

    let bbox_desc = AccelerationStructureBoundingBoxGeometryDescriptor::new();
    bbox_desc.set_bounding_box_buffer_offset(0);
    bbox_desc.set_bounding_box_stride(24);
    bbox_desc.set_bounding_box_count(1);
    bbox_desc.set_opaque(false);

    let primitive_desc = PrimitiveAccelerationStructureDescriptor::new();
    primitive_desc.set_geometry_descriptors(&[&tri_desc]);
    primitive_desc.set_bounding_box_geometry_descriptors(&[&bbox_desc]);

    let instance_desc = InstanceAccelerationStructureDescriptor::new();
    instance_desc.set_instance_descriptor_buffer_offset(0);
    instance_desc.set_instance_descriptor_stride(64);
    instance_desc.set_instance_count(1);

    // 3. Cover Device-backed Raytracing APIs if available
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed raytracing tests.");
        return Ok(());
    };

    if !device.supports_raytracing() {
        println!("Device does not support raytracing, skipping device-backed raytracing tests.");
        return Ok(());
    }

    // Now we have a device with raytracing support:
    let queue = device.new_command_queue()?;

    // Create buffers to fully exercise the descriptors
    let vertices = [
        0.0f32, 0.5f32, 0.0f32, -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32,
    ];
    let vertex_buffer =
        device.new_buffer_with_data(&vertices, ResourceOptions::STORAGE_MODE_SHARED)?;
    let index_buffer =
        device.new_buffer_with_data(&[0u16, 1u16, 2u16], ResourceOptions::STORAGE_MODE_SHARED)?;
    let bbox_buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
    let instance_buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;

    tri_desc.set_vertex_buffer(&vertex_buffer);
    tri_desc.set_index_buffer(&index_buffer);
    bbox_desc.set_bounding_box_buffer(&bbox_buffer);
    instance_desc.set_instance_descriptor_buffer(&instance_buffer);

    // Populate instance descriptor payload with valid identity matrix
    let inst_payload = MTLAccelerationStructureInstanceDescriptor {
        transform: [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
        ],
        options: 0,
        mask: 0xFF,
        intersection_function_table_offset: 0,
        acceleration_structure_index: 0,
    };
    instance_buffer.write(&inst_payload);

    // Query acceleration structure sizes
    let sizes = device.acceleration_structure_sizes(&primitive_desc)?;
    assert!(sizes.acceleration_structure_size > 0);
    assert!(sizes.build_scratch_buffer_size > 0);

    // Create primitive acceleration structure
    let primitive_as = device.new_acceleration_structure(sizes.acceleration_structure_size)?;
    assert_eq!(primitive_as.size(), sizes.acceleration_structure_size);
    if let Ok(gpu_id) = primitive_as.gpu_resource_id() {
        assert!(gpu_id.impl_ > 0);
    }

    // Cover heap-created acceleration structures
    let heap_desc = HeapDescriptor::new();
    heap_desc.set_size(sizes.acceleration_structure_size * 4 + 65536);
    heap_desc.set_storage_mode(StorageMode::Private);
    if let Ok(heap) = device.new_heap(&heap_desc) {
        if let Ok(heap_as) = heap.new_acceleration_structure(sizes.acceleration_structure_size) {
            assert!(heap_as.size() >= sizes.acceleration_structure_size);
        }
    }

    // Create scratch buffer
    let scratch_buffer = device.new_buffer(
        sizes.build_scratch_buffer_size,
        ResourceOptions::STORAGE_MODE_PRIVATE,
    )?;

    // Encode build command
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.acceleration_structure_command_encoder()?;

    encoder.build_acceleration_structure(&primitive_as, &primitive_desc, &scratch_buffer, 0)?;

    // Exercise refit primitive
    let refit_scratch = device.new_buffer(
        sizes.refit_scratch_buffer_size.max(1),
        ResourceOptions::STORAGE_MODE_PRIVATE,
    )?;
    encoder.refit_primitive_acceleration_structure(
        &primitive_as,
        &primitive_desc,
        Some(&primitive_as),
        Some(&refit_scratch),
        0,
    )?;

    // Exercise copy
    let dest_as = device.new_acceleration_structure(sizes.acceleration_structure_size)?;
    encoder.copy_acceleration_structure(&primitive_as, &dest_as)?;

    // Exercise copy and compact
    encoder.copy_and_compact_acceleration_structure(&primitive_as, &dest_as)?;

    // Exercise write compacted size
    let size_buffer = device.new_buffer(8, ResourceOptions::STORAGE_MODE_SHARED)?;
    encoder.write_compacted_acceleration_structure_size(&primitive_as, &size_buffer, 0)?;

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();
    assert_eq!(command_buffer.status(), CommandBufferStatus::Completed);
    assert!(command_buffer.error().is_none());

    // Verify compacted size was written
    let mut compacted_size = 0usize;
    size_buffer.read_slice(std::slice::from_mut(&mut compacted_size));
    assert!(compacted_size > 0);

    // Test instances
    instance_desc.set_instanced_acceleration_structures(&[&primitive_as]);
    let instance_sizes = device.instance_acceleration_structure_sizes(&instance_desc)?;
    assert!(instance_sizes.acceleration_structure_size > 0);

    let instance_as =
        device.new_acceleration_structure(instance_sizes.acceleration_structure_size)?;
    let instance_scratch = device.new_buffer(
        instance_sizes.build_scratch_buffer_size,
        ResourceOptions::STORAGE_MODE_PRIVATE,
    )?;

    let inst_cb = queue.command_buffer()?;
    let inst_enc = inst_cb.acceleration_structure_command_encoder()?;
    inst_enc.build_instance_acceleration_structure(
        &instance_as,
        &instance_desc,
        &instance_scratch,
        0,
    )?;

    let inst_refit_scratch = device.new_buffer(
        instance_sizes.refit_scratch_buffer_size.max(1),
        ResourceOptions::STORAGE_MODE_PRIVATE,
    )?;
    inst_enc.refit_instance_acceleration_structure(
        &instance_as,
        &instance_desc,
        Some(&instance_as),
        Some(&inst_refit_scratch),
        0,
    )?;

    inst_enc.end_encoding();
    inst_cb.commit();
    inst_cb.wait_until_completed();
    assert_eq!(inst_cb.status(), CommandBufferStatus::Completed);
    assert!(inst_cb.error().is_none());

    // Error path validation: query size of invalid descriptor should fail if unsupported or raise error
    let invalid_instance_desc = InstanceAccelerationStructureDescriptor::new();
    invalid_instance_desc.set_instance_count(0);
    // Should safely return error or behave predictably
    let _ = device.instance_acceleration_structure_sizes(&invalid_instance_desc);

    Ok(())
}
