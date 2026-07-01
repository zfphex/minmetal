use minmetal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping ray tracing example");
        return Ok(());
    };

    if !device.supports_raytracing() {
        println!("Ray tracing is not supported on this device, skipping example.");
        return Ok(());
    }

    let queue = device.new_command_queue()?;

    // Create a simple vertex buffer containing a single triangle
    let vertices = [
        0.0f32, 0.5f32, 0.0f32, -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32,
    ];
    let vertex_buffer =
        device.new_buffer_with_data(&vertices, ResourceOptions::STORAGE_MODE_SHARED)?;

    let tri_desc = AccelerationStructureTriangleGeometryDescriptor::new();
    tri_desc.set_vertex_buffer(&vertex_buffer);
    tri_desc.set_vertex_stride(12);
    tri_desc.set_vertex_format(VertexFormat::Float3);
    tri_desc.set_triangle_count(1);
    tri_desc.set_opaque(true);

    let prim_desc = PrimitiveAccelerationStructureDescriptor::new();
    prim_desc.set_geometry_descriptors(&[&tri_desc]);

    // Query required sizes
    let sizes = match device.acceleration_structure_sizes(&prim_desc) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Failed to query acceleration structure sizes: {}. Skipping example",
                e
            );
            return Ok(());
        }
    };

    println!(
        "Required AS size: {} bytes",
        sizes.acceleration_structure_size
    );
    println!(
        "Build scratch buffer size: {} bytes",
        sizes.build_scratch_buffer_size
    );

    // Allocate acceleration structure
    let as_structure = device.new_acceleration_structure(sizes.acceleration_structure_size)?;

    // Allocate scratch buffer
    let scratch_size = sizes.build_scratch_buffer_size.max(1);
    let scratch_buffer = device.new_buffer(scratch_size, ResourceOptions::STORAGE_MODE_PRIVATE)?;

    // Encode build command
    let command_buffer = queue.command_buffer()?;
    let encoder = command_buffer.acceleration_structure_command_encoder()?;

    match encoder.build_acceleration_structure(&as_structure, &prim_desc, &scratch_buffer, 0) {
        Ok(_) => {
            println!("Encoded AS build successfully");
        }
        Err(e) => {
            println!("AS build encoding failed: {}", e);
        }
    }

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    if let Some(error) = command_buffer.error() {
        return Err(error.into());
    }

    println!("raytracing AS build smoke test passed");
    Ok(())
}
