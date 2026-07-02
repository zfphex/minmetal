use minmetal::*;

#[test]
fn pass_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. RenderPassDescriptor construction and basics
    let render_pass_desc = RenderPassDescriptor::new();
    let render_pass_default = RenderPassDescriptor::default();
    assert!(!render_pass_desc.raw.is_null());
    assert!(!render_pass_default.raw.is_null());

    // Color Attachment permutations
    let color_attachment = render_pass_desc.color_attachment(0);
    let base = color_attachment.base();

    // Verify all LoadActions
    for action in [LoadAction::DontCare, LoadAction::Load, LoadAction::Clear] {
        base.set_load_action(action);
        assert_eq!(base.load_action()?, action);
    }

    // Verify all StoreActions
    for action in [
        StoreAction::DontCare,
        StoreAction::Store,
        StoreAction::MultisampleResolve,
        StoreAction::StoreAndMultisampleResolve,
        StoreAction::Unknown,
        StoreAction::CustomSampleDepthStore,
    ] {
        base.set_store_action(action);
        assert_eq!(base.store_action()?, action);
    }

    // StoreActionOptions
    if responds_to_selector(base.raw, sel(b"storeActionOptions\0")) {
        base.set_store_action_options(StoreActionOptions::NONE)?;
        assert_eq!(base.store_action_options()?.0, StoreActionOptions::NONE.0);
        base.set_store_action_options(StoreActionOptions::CUSTOM_SAMPLE_POSITIONS)?;
        assert_eq!(
            base.store_action_options()?.0,
            StoreActionOptions::CUSTOM_SAMPLE_POSITIONS.0
        );
    }

    // Color attachment specific
    let clear_color = ClearColor::new(0.2, 0.4, 0.6, 0.8);
    color_attachment.set_clear_color(clear_color);
    assert_eq!(color_attachment.clear_color(), clear_color);

    // Depth attachment
    let depth_attachment = render_pass_desc.depth_attachment();
    depth_attachment.set_clear_depth(0.5);
    assert_eq!(depth_attachment.clear_depth(), 0.5);

    // Stencil attachment
    let stencil_attachment = render_pass_desc.stencil_attachment();
    stencil_attachment.set_clear_stencil(42);
    assert_eq!(stencil_attachment.clear_stencil(), 42);

    // Render pass descriptor properties
    render_pass_desc.set_render_target_array_length(2);
    assert_eq!(render_pass_desc.render_target_array_length(), 2);

    render_pass_desc.set_imageblock_sample_length(8);
    assert_eq!(render_pass_desc.imageblock_sample_length(), 8);

    render_pass_desc.set_tile_width(16);
    assert_eq!(render_pass_desc.tile_width(), 16);

    render_pass_desc.set_tile_height(16);
    assert_eq!(render_pass_desc.tile_height(), 16);

    // Sample buffer attachments Some/None round trips
    let render_sample_buffers = render_pass_desc.sample_buffer_attachments();
    let render_attachment0 = render_sample_buffers.object_at_indexed_subscript(0);

    render_attachment0.set_start_of_vertex_sample_index(0);
    assert_eq!(render_attachment0.start_of_vertex_sample_index(), 0);
    render_attachment0.set_end_of_vertex_sample_index(1);
    assert_eq!(render_attachment0.end_of_vertex_sample_index(), 1);

    render_attachment0.set_start_of_fragment_sample_index(2);
    assert_eq!(render_attachment0.start_of_fragment_sample_index(), 2);
    render_attachment0.set_end_of_fragment_sample_index(3);
    assert_eq!(render_attachment0.end_of_fragment_sample_index(), 3);

    // Some/None check for sample_buffer itself
    assert!(render_attachment0.sample_buffer().is_none());

    // 2. ComputePassDescriptor
    let compute_class = class(b"MTLComputePassDescriptor\0");
    if !compute_class.is_null() {
        let compute_pass_desc = ComputePassDescriptor::new()?;
        assert!(!compute_pass_desc.raw.is_null());
        compute_pass_desc.set_dispatch_type(DispatchType::Concurrent);
        assert_eq!(compute_pass_desc.dispatch_type()?, DispatchType::Concurrent);

        let compute_sample_buffers = compute_pass_desc.sample_buffer_attachments();
        let compute_attachment0 = compute_sample_buffers.object_at_indexed_subscript(0);
        compute_attachment0.set_start_of_encoder_sample_index(2);
        assert_eq!(compute_attachment0.start_of_encoder_sample_index(), 2);
        compute_attachment0.set_end_of_encoder_sample_index(3);
        assert_eq!(compute_attachment0.end_of_encoder_sample_index(), 3);
    } else {
        println!(
            "MTLComputePassDescriptor class not available, skipping compute pass descriptor tests."
        );
    }

    // 3. BlitPassDescriptor
    let blit_class = class(b"MTLBlitPassDescriptor\0");
    if !blit_class.is_null() {
        let blit_pass_desc = BlitPassDescriptor::new()?;
        assert!(!blit_pass_desc.raw.is_null());

        let blit_sample_buffers = blit_pass_desc.sample_buffer_attachments();
        let blit_attachment0 = blit_sample_buffers.object_at_indexed_subscript(0);
        blit_attachment0.set_start_of_encoder_sample_index(4);
        assert_eq!(blit_attachment0.start_of_encoder_sample_index(), 4);
        blit_attachment0.set_end_of_encoder_sample_index(5);
        assert_eq!(blit_attachment0.end_of_encoder_sample_index(), 5);
    } else {
        println!("MTLBlitPassDescriptor class not available, skipping blit pass descriptor tests.");
    }

    // 4. ResourceStatePassDescriptor
    let resource_class = class(b"MTLResourceStatePassDescriptor\0");
    if !resource_class.is_null() {
        let resource_pass_desc = ResourceStatePassDescriptor::new()?;
        assert!(!resource_pass_desc.raw.is_null());

        let resource_sample_buffers = resource_pass_desc.sample_buffer_attachments();
        let resource_attachment0 = resource_sample_buffers.object_at_indexed_subscript(0);
        resource_attachment0.set_start_of_encoder_sample_index(6);
        assert_eq!(resource_attachment0.start_of_encoder_sample_index(), 6);
        resource_attachment0.set_end_of_encoder_sample_index(7);
        assert_eq!(resource_attachment0.end_of_encoder_sample_index(), 7);
    } else {
        println!(
            "MTLResourceStatePassDescriptor class not available, skipping resource state pass descriptor tests."
        );
    }

    // 5. Device-backed tests
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed pass tests.");
        return Ok(());
    };

    let texture_desc = TextureDescriptor::texture_2d(PixelFormat::Bgra8Unorm, 256, 256, false);
    texture_desc.set_storage_mode(StorageMode::Shared);
    let texture = device.new_texture(&texture_desc)?;

    // Some and None round trips for base texture & resolve_texture
    base.set_texture(None);
    assert!(base.texture().is_none());
    base.set_texture(Some(&texture));
    assert!(base.texture().is_some());

    base.set_resolve_texture(None);
    assert!(base.resolve_texture().is_none());
    base.set_resolve_texture(Some(&texture));
    assert!(base.resolve_texture().is_some());

    // Some and None round trips for visibility buffer
    render_pass_desc.set_visibility_result_buffer(None);
    assert!(render_pass_desc.visibility_result_buffer().is_none());
    let visibility_buf = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
    render_pass_desc.set_visibility_result_buffer(Some(&visibility_buf));
    assert!(render_pass_desc.visibility_result_buffer().is_some());

    // Some and None round trips for sample buffers
    let counter_desc = CounterSampleBufferDescriptor::new();
    counter_desc.set_sample_count(4);
    if let Ok(c_set) = device.new_counter_sample_buffer(&counter_desc) {
        render_attachment0.set_sample_buffer(Some(&c_set));
        assert!(render_attachment0.sample_buffer().is_some());
        render_attachment0.set_sample_buffer(None);
        assert!(render_attachment0.sample_buffer().is_none());
    }

    // Some and None round trips for rasterization rate map
    let rate_desc = RasterizationRateMapDescriptor::with_screen_size(Size::new(100, 100, 0));
    rate_desc.set_label("rate-map-desc");
    if let Ok(rate_map) = device.new_rasterization_rate_map(&rate_desc) {
        if responds_to_selector(render_pass_desc.raw, sel(b"setRasterizationRateMap:\0")) {
            render_pass_desc.set_rasterization_rate_map(Some(&rate_map))?;
            assert!(render_pass_desc.rasterization_rate_map().is_some());
            render_pass_desc.set_rasterization_rate_map(None)?;
            assert!(render_pass_desc.rasterization_rate_map().is_none());
        }
    }

    // Setters helper verification
    render_pass_desc.set_color_attachment(
        0,
        &texture,
        LoadAction::Clear,
        StoreAction::Store,
        clear_color,
    );
    render_pass_desc.set_color_attachment_resolve_texture(0, &texture);
    render_pass_desc.set_depth_attachment(
        &texture,
        LoadAction::DontCare,
        StoreAction::DontCare,
        1.0,
    );
    render_pass_desc.set_depth_resolve_texture(&texture);
    render_pass_desc.set_stencil_attachment(
        &texture,
        LoadAction::DontCare,
        StoreAction::DontCare,
        0,
    );

    Ok(())
}
