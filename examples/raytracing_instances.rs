use minmetal::*;

fn main() {
    println!("Running raytracing_instances example...");
    let Some(device) = Device::system_default() else {
        println!("No Metal device available.");
        return;
    };

    if !device.supports_raytracing() {
        println!("Ray tracing is not supported on this device.");
        return;
    }

    println!("Raytracing is supported!");

    // Create a bounding box descriptor
    let bbox_desc = AccelerationStructureBoundingBoxGeometryDescriptor::new();
    let bbox_buffer = device
        .new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)
        .unwrap();
    bbox_desc.set_bounding_box_buffer(&bbox_buffer);
    bbox_desc.set_bounding_box_buffer_offset(0);
    bbox_desc.set_bounding_box_stride(24);
    bbox_desc.set_bounding_box_count(1);

    // Create primitive acceleration structure descriptor
    let primitive_desc = PrimitiveAccelerationStructureDescriptor::new();
    primitive_desc.set_bounding_box_geometry_descriptors(&[&bbox_desc]);

    let sizes = device
        .acceleration_structure_sizes(&primitive_desc)
        .unwrap();
    println!(
        "Acceleration structure size: {} bytes",
        sizes.acceleration_structure_size
    );

    let structure = device
        .new_acceleration_structure(sizes.acceleration_structure_size)
        .unwrap();
    println!(
        "Successfully created AccelerationStructure with size: {}",
        structure.size()
    );
}
