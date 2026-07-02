use minmetal::*;

#[test]
fn counters_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CounterSamplingPoint variants
    let points = [
        CounterSamplingPoint::AtStageBoundary,
        CounterSamplingPoint::AtDrawBoundary,
        CounterSamplingPoint::AtDispatchBoundary,
        CounterSamplingPoint::AtTileBoundary,
        CounterSamplingPoint::AtBlitBoundary,
    ];

    for &point in &points {
        match point {
            CounterSamplingPoint::AtStageBoundary => assert_eq!(point as usize, 0),
            CounterSamplingPoint::AtDrawBoundary => assert_eq!(point as usize, 1),
            CounterSamplingPoint::AtDispatchBoundary => assert_eq!(point as usize, 2),
            CounterSamplingPoint::AtTileBoundary => assert_eq!(point as usize, 3),
            CounterSamplingPoint::AtBlitBoundary => assert_eq!(point as usize, 4),
        }
    }

    // 2. Device availability
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping counters test.");
        return Ok(());
    };

    // Device::supports_counter_sampling
    for &point in &points {
        let supported = device.supports_counter_sampling(point);
        println!("Sampling point {:?} supported: {}", point, supported);
    }

    // Device::counter_sets
    let counter_sets = match device.counter_sets() {
        Ok(sets) => sets,
        Err(e) => {
            println!(
                "Counter sets query not supported on this device/OS: {}. Graceful skip.",
                e
            );
            return Ok(());
        }
    };

    if counter_sets.is_empty() {
        println!("No counter sets available on this device. Graceful skip.");
        return Ok(());
    }

    for set in &counter_sets {
        let set_name = set.name();
        assert!(!set_name.is_empty());
        for counter in set.counters() {
            let counter_name = counter.name();
            assert!(!counter_name.is_empty());
        }
    }

    // 3. CounterSampleBufferDescriptor
    let desc = CounterSampleBufferDescriptor::new();
    let desc_default = CounterSampleBufferDescriptor::default();

    desc.set_counter_set(&counter_sets[0]);
    desc.set_label("");
    desc.set_label("test-counter-buffer");
    desc.set_storage_mode(StorageMode::Shared);
    desc.set_sample_count(1);
    desc.set_sample_count(4);

    drop(desc_default);

    // 4. Device::new_counter_sample_buffer
    let sample_buffer = match device.new_counter_sample_buffer(&desc) {
        Ok(buf) => buf,
        Err(e) => {
            println!(
                "Failed to create counter sample buffer: {}. Skipping encoder tests.",
                e
            );
            return Ok(());
        }
    };

    let queue = device.new_command_queue()?;
    let command_buffer = queue.command_buffer()?;

    // AtBlitBoundary / AtDispatchBoundary / AtDrawBoundary tests depending on support
    let mut encoded_any = false;

    // Blit Command Encoder tests
    if device.supports_counter_sampling(CounterSamplingPoint::AtBlitBoundary) {
        let blit = command_buffer.blit_command_encoder()?;
        blit.sample_counters_in_buffer(&sample_buffer, 0, true)?;

        let dest_buf = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;
        blit.resolve_counters(&sample_buffer, Range::new(0, 1), &dest_buf, 0)?;

        blit.end_encoding();
        encoded_any = true;
    }

    // Compute Command Encoder tests
    if device.supports_counter_sampling(CounterSamplingPoint::AtDispatchBoundary) {
        let compute = command_buffer.compute_command_encoder()?;
        compute.sample_counters_in_buffer(&sample_buffer, 1, true)?;
        compute.end_encoding();

        // Also test resolve_counters via blit if needed
        let blit = command_buffer.blit_command_encoder()?;
        let dest_buf = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;
        blit.resolve_counters(&sample_buffer, Range::new(1, 1), &dest_buf, 0)?;
        blit.end_encoding();
        encoded_any = true;
    }

    // Render Command Encoder tests
    if device.supports_counter_sampling(CounterSamplingPoint::AtDrawBoundary) {
        let render_desc = RenderPassDescriptor::new();
        // Since we need a render encoder, we can configure a minimal render pass
        let dummy_tex_desc = TextureDescriptor::texture_2d(PixelFormat::Bgra8Unorm, 16, 16, false);
        dummy_tex_desc.set_storage_mode(StorageMode::Private);
        dummy_tex_desc.set_usage(TextureUsage::RENDER_TARGET);
        let dummy_tex = device.new_texture(&dummy_tex_desc)?;

        render_desc.set_color_attachment(
            0,
            &dummy_tex,
            LoadAction::Clear,
            StoreAction::Store,
            ClearColor::new(0.0, 0.0, 0.0, 1.0),
        );

        let render = command_buffer.render_command_encoder(&render_desc)?;
        render.sample_counters_in_buffer(&sample_buffer, 2, true)?;
        render.end_encoding();

        let blit = command_buffer.blit_command_encoder()?;
        let dest_buf = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;
        blit.resolve_counters(&sample_buffer, Range::new(2, 1), &dest_buf, 0)?;
        blit.end_encoding();
        encoded_any = true;
    }

    if encoded_any {
        command_buffer.commit();
        command_buffer.wait_until_completed();

        assert_eq!(command_buffer.status(), CommandBufferStatus::Completed);
        if let Some(err) = command_buffer.error() {
            return Err(Box::new(err));
        }
    }

    Ok(())
}
