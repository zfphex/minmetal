use minmetal::*;

#[test]
fn resource_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Step 1: descriptors");
    // 1. TextureDescriptor non-device tests
    let desc2d = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 128, 64, true);
    desc2d.set_storage_mode(StorageMode::Shared);
    assert!(!desc2d.raw.is_null());

    let desc_arr = TextureDescriptor::texture_2d_array(PixelFormat::Bgra8Unorm, 64, 64, 4, false);
    assert!(!desc_arr.raw.is_null());

    let desc_cube = TextureDescriptor::texture_cube(PixelFormat::Rgba16Float, 32, 1, false);
    assert!(!desc_cube.raw.is_null());

    let desc_custom = TextureDescriptor::new();
    desc_custom.set_texture_type(TextureType::D3);
    desc_custom.set_pixel_format(PixelFormat::Rgba16Float);
    desc_custom.set_width(16);
    desc_custom.set_height(16);
    desc_custom.set_depth(16);
    desc_custom.set_mipmap_level_count(1);
    desc_custom.set_array_length(1);
    desc_custom.set_sample_count(1);
    desc_custom.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    desc_custom.set_storage_mode(StorageMode::Private);

    // 2. SamplerDescriptor non-device tests
    let sampler_desc = SamplerDescriptor::new();
    sampler_desc.set_min_filter(SamplerMinMagFilter::Linear);
    sampler_desc.set_mag_filter(SamplerMinMagFilter::Linear);
    sampler_desc.set_mip_filter(SamplerMipFilter::Linear);
    sampler_desc.set_s_address_mode(SamplerAddressMode::ClampToEdge);
    sampler_desc.set_t_address_mode(SamplerAddressMode::ClampToEdge);
    sampler_desc.set_r_address_mode(SamplerAddressMode::ClampToEdge);
    sampler_desc.set_compare_function(CompareFunction::Always);
    sampler_desc.set_lod_min_clamp(0.0);
    sampler_desc.set_lod_max_clamp(10.0);
    sampler_desc.set_max_anisotropy(8);

    // 3. HeapDescriptor non-device tests
    let heap_desc = HeapDescriptor::new();
    heap_desc.set_size(1024 * 1024);
    heap_desc.set_storage_mode(StorageMode::Private);
    heap_desc.set_cpu_cache_mode(CpuCacheMode::DefaultCache);
    heap_desc.set_hazard_tracking_mode(HazardTrackingMode::Tracked);
    heap_desc.set_heap_type(HeapType::Automatic);

    // 4. ArgumentDescriptor non-device tests
    let arg_desc = ArgumentDescriptor::new();
    arg_desc.set_index(1);
    arg_desc.set_data_type(DataType::Texture);
    arg_desc.set_access(ArgumentAccess::ReadOnly);
    arg_desc.set_texture_type(TextureType::D2);
    arg_desc.set_array_length(5);

    println!("Step 2: Device-backed tests start");
    // 5. Device-backed tests
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed resource tests.");
        return Ok(());
    };

    println!("Step 3: SharedEvent");
    // SharedEvent value round trips
    let shared_event = device.new_shared_event()?;
    shared_event.set_signaled_value(42);
    assert_eq!(shared_event.signaled_value(), 42);

    println!("Step 4: Buffer");
    // Buffer tests
    let buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
    assert_eq!(buffer.len(), 1024);
    assert!(!buffer.is_empty());
    assert!(!buffer.contents().is_null());

    buffer.set_label("test-buffer");
    assert_eq!(buffer.label().as_deref(), Some("test-buffer"));

    buffer.write(&12345u32);
    let mut out_val = 0u32;
    buffer.read_slice(std::slice::from_mut(&mut out_val));
    assert_eq!(out_val, 12345);

    buffer.write_slice(&[1u32, 2, 3, 4]);
    let mut out_slice = [0u32; 4];
    buffer.read_slice(&mut out_slice);
    assert_eq!(out_slice, [1u32, 2, 3, 4]);

    buffer.did_modify_range(Range::new(0, 16));

    let gpu_addr_sel = sel(b"gpuAddress\0");
    if responds_to_selector(buffer.raw, gpu_addr_sel) {
        let gpu_addr = buffer.gpu_address()?;
        assert!(gpu_addr > 0);
    }

    println!("Step 5: Texture");
    // Texture tests
    let texture = device.new_texture(&desc2d)?;
    assert_eq!(texture.width(), 128);
    assert_eq!(texture.height(), 64);
    assert_eq!(texture.pixel_format(), PixelFormat::Rgba8Unorm as usize);

    texture.set_label("test-texture");
    assert_eq!(texture.label().as_deref(), Some("test-texture"));

    let gpu_res_sel = sel(b"gpuResourceID\0");
    if responds_to_selector(texture.raw, gpu_res_sel) {
        let gpu_id = texture.gpu_resource_id()?;
        assert!(gpu_id.impl_ > 0);
    }

    println!("Step 6: replace_region & get_bytes");
    // Write and read back patterned non-zero bytes
    let pixels: Vec<u8> = (0..(128 * 64 * 4))
        .map(|i| ((i * 3 + 7) % 256) as u8)
        .collect();
    texture.replace_region(Region::new_2d(0, 0, 128, 64), 0, &pixels, 128 * 4);
    let mut readback_pixels = vec![0u8; 128 * 64 * 4];
    texture.get_bytes(
        Region::new_2d(0, 0, 128, 64),
        0,
        &mut readback_pixels,
        128 * 4,
    );
    assert_eq!(readback_pixels, pixels);

    println!("Step 7: view");
    let view = texture.new_texture_view(PixelFormat::Rgba8UnormSrgb)?;
    assert_eq!(view.pixel_format(), PixelFormat::Rgba8UnormSrgb as usize);

    println!("Step 8: Sampler state");
    // Sampler State tests
    let sampler_state = device.new_sampler_state(&sampler_desc)?;
    assert!(!sampler_state.raw.is_null());
    if responds_to_selector(sampler_state.raw, sel(b"setLabel:\0")) {
        sampler_state.set_label("test-sampler-state");
        if responds_to_selector(sampler_state.raw, sel(b"label\0")) {
            assert_eq!(sampler_state.label().as_deref(), Some("test-sampler-state"));
        }
    }
    if responds_to_selector(sampler_state.raw, gpu_res_sel) {
        let sampler_gpu_id = sampler_state.gpu_resource_id()?;
        assert!(sampler_gpu_id.impl_ > 0);
    }

    println!("Step 9: Heap sizes");
    // Heap tests
    let size_align = device.heap_texture_size_and_align(&desc2d);
    let buf_size_align =
        device.heap_buffer_size_and_align(1024, ResourceOptions::STORAGE_MODE_SHARED);
    assert!(size_align.size > 0);
    assert!(buf_size_align.size > 0);

    println!("Step 10: Automatic Heap");
    let auto_heap_desc = HeapDescriptor::new();
    auto_heap_desc.set_size(size_align.size + buf_size_align.size + 131072);
    auto_heap_desc.set_storage_mode(StorageMode::Shared);
    auto_heap_desc.set_heap_type(HeapType::Automatic);
    let auto_heap = device.new_heap(&auto_heap_desc)?;
    assert_eq!(auto_heap.label(), None);
    auto_heap.set_label("test-auto-heap");
    assert_eq!(auto_heap.label().as_deref(), Some("test-auto-heap"));

    let initial_used = auto_heap.used_size();
    let heap_buf = auto_heap.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
    assert_eq!(heap_buf.len(), 1024);
    let after_alloc_used = auto_heap.used_size();
    assert!(
        after_alloc_used > initial_used,
        "Expected used heap size to increase after allocation"
    );

    let heap_tex = auto_heap.new_texture(&desc2d)?;
    assert_eq!(heap_tex.width(), 128);

    println!("Step 11: Placement Heap");
    let placement_heap_desc = HeapDescriptor::new();
    placement_heap_desc.set_size(size_align.size + buf_size_align.size + 131072);
    placement_heap_desc.set_storage_mode(StorageMode::Shared);
    placement_heap_desc.set_heap_type(HeapType::Placement);
    let placement_heap = device.new_heap(&placement_heap_desc)?;

    let offset_buf =
        placement_heap.new_buffer_at_offset(512, ResourceOptions::STORAGE_MODE_SHARED, 0)?;
    assert_eq!(offset_buf.len(), 512);

    let offset_tex = placement_heap.new_texture_at_offset(&desc2d, buf_size_align.size)?;
    assert_eq!(offset_tex.width(), 128);

    println!("Step 12: Heap AS");
    // Heap acceleration structures
    let heap_as_sel = sel(b"newAccelerationStructureWithSize:\0");
    if responds_to_selector(placement_heap.raw, heap_as_sel) {
        if let Ok(heap_as) = placement_heap.new_acceleration_structure(1024) {
            assert!(heap_as.size() >= 1024);
        }
        if let Ok(heap_as_at) = placement_heap.new_acceleration_structure_at_offset(1024, 1024) {
            assert!(heap_as_at.size() >= 1024);
        }
    }

    println!("Step 13: Argument Encoder");
    // Create argument descriptors mapping to range offsets
    let buf_arg0 = ArgumentDescriptor::new();
    buf_arg0.set_index(0);
    buf_arg0.set_data_type(DataType::Pointer);

    let buf_arg1 = ArgumentDescriptor::new();
    buf_arg1.set_index(1);
    buf_arg1.set_data_type(DataType::Pointer);

    let tex_arg0 = ArgumentDescriptor::new();
    tex_arg0.set_index(2);
    tex_arg0.set_data_type(DataType::Texture);

    let tex_arg1 = ArgumentDescriptor::new();
    tex_arg1.set_index(3);
    tex_arg1.set_data_type(DataType::Texture);

    let samp_arg0 = ArgumentDescriptor::new();
    samp_arg0.set_index(4);
    samp_arg0.set_data_type(DataType::Sampler);

    let samp_arg1 = ArgumentDescriptor::new();
    samp_arg1.set_index(5);
    samp_arg1.set_data_type(DataType::Sampler);

    let byte_arg = ArgumentDescriptor::new();
    byte_arg.set_index(6);
    byte_arg.set_data_type(DataType::Pointer);
    byte_arg.set_access(ArgumentAccess::ReadOnly);

    let arg_encoder = device.new_argument_encoder(&[
        &buf_arg0, &buf_arg1, &tex_arg0, &tex_arg1, &samp_arg0, &samp_arg1, &byte_arg,
    ])?;
    assert!(arg_encoder.encoded_length() > 0);
    assert!(arg_encoder.alignment() > 0);

    arg_encoder.set_label("test-arg-encoder");
    assert_eq!(arg_encoder.label().as_deref(), Some("test-arg-encoder"));

    println!("Step 14: Argument buffer / encoder ranges");
    let arg_buf = device.new_buffer(
        arg_encoder.encoded_length(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    println!("Step 14.1: set_argument_buffer");
    arg_encoder.set_argument_buffer(&arg_buf, 0);

    println!("Step 14.2: set_buffer");
    arg_encoder.set_buffer(0, &buffer, 0);

    println!("Step 14.3: set_texture");
    arg_encoder.set_texture(2, &texture);

    println!("Step 14.4: set_sampler_state");
    arg_encoder.set_sampler_state(4, &sampler_state);

    println!("Step 14.5: set_buffers range");
    arg_encoder.set_buffers(&[Some(&buffer), None], &[0, 0], Range::new(0, 2));

    println!("Step 14.6: set_textures range");
    arg_encoder.set_textures(&[Some(&texture), None], Range::new(2, 2));

    println!("Step 14.7: set_sampler_states range");
    arg_encoder.set_sampler_states(&[Some(&sampler_state), None], Range::new(4, 2));

    println!("Step 14.8: set_bytes");
    println!(
        "Skipping ArgumentEncoder::set_bytes: descriptor-created argument encoders raise an Objective-C exception for this slot shape."
    );

    println!("Step 15: Table / AS setters");
    // Table / AS setters if supported
    let table_desc = VisibleFunctionTableDescriptor::new();
    table_desc.set_function_count(4);
    let lib_source = r#"
        #include <metal_stdlib>
        using namespace metal;
        kernel void compute_main() {}
    "#;
    let lib = device.new_library_with_source(lib_source)?;
    let func = lib.function("compute_main")?;
    let pipeline_desc = ComputePipelineDescriptor::new();
    pipeline_desc.set_compute_function(&func);
    if let Ok(state) = device.new_compute_pipeline_state(&pipeline_desc) {
        if let Ok(v_table) = state.new_visible_function_table(&table_desc) {
            arg_encoder.set_visible_function_table(Some(&v_table), 0)?;
            arg_encoder.set_visible_function_table(None, 0)?;
            arg_encoder.set_visible_function_tables(&[Some(&v_table), None], Range::new(0, 2))?;
        }
        let isect_desc = IntersectionFunctionTableDescriptor::new();
        isect_desc.set_function_count(4);
        if let Ok(i_table) = state.new_intersection_function_table(&isect_desc) {
            arg_encoder.set_intersection_function_table(Some(&i_table), 0)?;
            arg_encoder.set_intersection_function_table(None, 0)?;
            arg_encoder
                .set_intersection_function_tables(&[Some(&i_table), None], Range::new(0, 2))?;
        }
    }

    if device.supports_raytracing() {
        let vertices = [0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let vertex_buffer =
            device.new_buffer_with_data(&vertices, ResourceOptions::STORAGE_MODE_SHARED)?;
        let tri_desc = AccelerationStructureTriangleGeometryDescriptor::new();
        tri_desc.set_vertex_buffer(&vertex_buffer);
        tri_desc.set_vertex_stride(3 * std::mem::size_of::<f32>());
        tri_desc.set_vertex_format(VertexFormat::Float3);
        tri_desc.set_triangle_count(1);
        let primitive_desc = PrimitiveAccelerationStructureDescriptor::new();
        primitive_desc.set_geometry_descriptors(&[&tri_desc]);
        let sizes = device.acceleration_structure_sizes(&primitive_desc)?;
        if let Ok(accel) = device.new_acceleration_structure(sizes.acceleration_structure_size) {
            arg_encoder.set_acceleration_structure(Some(&accel), 0)?;
            arg_encoder.set_acceleration_structure(None, 0)?;
        }
    }

    println!("Done!");
    Ok(())
}
