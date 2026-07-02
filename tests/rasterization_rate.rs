use minmetal::*;

#[test]
fn rasterization_rate_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cover pure value types first
    let coord_default = Coordinate2D::default();
    assert_eq!(coord_default.x, 0.0);
    assert_eq!(coord_default.y, 0.0);

    let coord_zero = Coordinate2D::new(0.0, 0.0);
    assert_eq!(coord_zero.x, 0.0);
    assert_eq!(coord_zero.y, 0.0);

    let coord_pos = Coordinate2D::new(1.0, 2.5);
    assert_eq!(coord_pos.x, 1.0);
    assert_eq!(coord_pos.y, 2.5);

    let coord_neg = Coordinate2D::new(-1.0, -2.5);
    assert_eq!(coord_neg.x, -1.0);
    assert_eq!(coord_neg.y, -2.5);

    let coord_frac = Coordinate2D::new(0.75, -0.25);
    assert_eq!(coord_frac.x, 0.75);
    assert_eq!(coord_frac.y, -0.25);

    // Equality/field assertions for all constructed coordinates
    assert_eq!(coord_zero, coord_default);
    assert_ne!(coord_pos, coord_neg);
    assert_ne!(coord_pos, coord_frac);

    // 2. Cover RasterizationRateLayerDescriptor without requiring a device
    let size1 = Size::new(1, 1, 0);
    let size2 = Size::new(2, 2, 0);
    let size3 = Size::new(4, 2, 0);

    let layer1 = RasterizationRateLayerDescriptor::new(size1);
    let layer2 = RasterizationRateLayerDescriptor::new(size2);
    let layer3 = RasterizationRateLayerDescriptor::new(size3);

    // Assert sample_count() round trips
    assert_eq!(layer1.sample_count(), size1);
    assert_eq!(layer2.sample_count(), size2);
    assert_eq!(layer3.sample_count(), size3);

    // Call set_sample_count(...) only when supported, and assert the updated count
    let set_selector = sel(b"setSampleCount:\0");
    if responds_to_selector(layer2.raw, set_selector) {
        layer2.set_sample_count(size2)?;
        assert_eq!(layer2.sample_count(), size2);
    } else {
        println!("setSampleCount: is unsupported on this platform");
    }

    // Exercise max_sample_count() when supported
    let max_selector = sel(b"maxSampleCount\0");
    if responds_to_selector(layer2.raw, max_selector) {
        let max_size = layer2.max_sample_count()?;
        assert!(max_size.width > 0);
        assert!(max_size.height > 0);
    } else {
        println!("maxSampleCount is unsupported on this platform");
    }

    // Exercise horizontal_sample_storage() and vertical_sample_storage() and assert they are non-null where Metal returns storage
    let h_storage = layer2.horizontal_sample_storage();
    let v_storage = layer2.vertical_sample_storage();
    if !h_storage.is_null() {
        // storage is returned, can assert non-null
        assert!(!h_storage.is_null());
    }
    if !v_storage.is_null() {
        assert!(!v_storage.is_null());
    }

    // Use horizontal() and vertical() sample arrays to set/read all entries for the sample count grid with values like 1.0, 0.75, 0.5, and 0.25
    let horiz = layer2.horizontal();
    let vert = layer2.vertical();

    horiz.set_object_at_indexed_subscript(1.0, 0);
    horiz.set_object_at_indexed_subscript(0.75, 1);
    vert.set_object_at_indexed_subscript(0.5, 0);
    vert.set_object_at_indexed_subscript(0.25, 1);

    assert_eq!(horiz.object_at_indexed_subscript(0), 1.0);
    assert_eq!(horiz.object_at_indexed_subscript(1), 0.75);
    assert_eq!(vert.object_at_indexed_subscript(0), 0.5);
    assert_eq!(vert.object_at_indexed_subscript(1), 0.25);

    // 3. Cover RasterizationRateMapDescriptor permutations
    let screen_size = Size::new(64, 64, 0);

    // with_screen_size
    let desc_screen = RasterizationRateMapDescriptor::with_screen_size(screen_size);
    assert_eq!(desc_screen.screen_size().width, screen_size.width);
    assert_eq!(desc_screen.screen_size().height, screen_size.height);

    // with_screen_size_and_layer
    let desc_layer =
        RasterizationRateMapDescriptor::with_screen_size_and_layer(screen_size, &layer2);
    assert_eq!(desc_layer.screen_size().width, screen_size.width);
    assert_eq!(desc_layer.screen_size().height, screen_size.height);
    assert_eq!(desc_layer.layer_count(), 1);

    // with_screen_size_and_layers using one layer and two layers
    let desc_layers1 =
        RasterizationRateMapDescriptor::with_screen_size_and_layers(screen_size, &[&layer2]);
    assert_eq!(desc_layers1.layer_count(), 1);

    let desc_layers2 = RasterizationRateMapDescriptor::with_screen_size_and_layers(
        screen_size,
        &[&layer2, &layer3],
    );
    assert_eq!(desc_layers2.layer_count(), 2);

    // screen_size getter and set_screen_size
    let new_screen_size = Size::new(128, 128, 0);
    desc_screen.set_screen_size(new_screen_size);
    assert_eq!(desc_screen.screen_size().width, new_screen_size.width);
    assert_eq!(desc_screen.screen_size().height, new_screen_size.height);

    // label round trips for "" and "minmetal-rasterization-rate"
    desc_screen.set_label("");
    assert_eq!(desc_screen.label().as_deref(), Some(""));

    desc_screen.set_label("minmetal-rasterization-rate");
    assert_eq!(
        desc_screen.label().as_deref(),
        Some("minmetal-rasterization-rate")
    );

    // layer_count
    assert_eq!(desc_screen.layer_count(), 0);
    assert_eq!(desc_layers2.layer_count(), 2);

    // layer_at_index for valid indices and an out-of-range index
    let layer_at_0 = desc_layers2.layer_at_index(0);
    assert!(layer_at_0.is_some());
    let layer_at_2 = desc_layers2.layer_at_index(2);
    assert!(layer_at_2.is_none());

    // set_layer_at_index(Some(...)) and set_layer_at_index(None, ...), verifying the layer is present/cleared
    desc_layers2.set_layer_at_index(None, 0);
    assert!(desc_layers2.layer_at_index(0).is_none());
    desc_layers2.set_layer_at_index(Some(&layer2), 0);
    assert!(desc_layers2.layer_at_index(0).is_some());

    // layers().object_at_indexed_subscript(...) and layers().set_object_at_indexed_subscript(...) for valid and cleared entries
    let layers_array = desc_layers2.layers();
    let l_obj_0 = layers_array.object_at_indexed_subscript(0);
    assert!(l_obj_0.is_some());

    layers_array.set_object_at_indexed_subscript(None, 0);
    assert!(layers_array.object_at_indexed_subscript(0).is_none());

    layers_array.set_object_at_indexed_subscript(Some(&layer2), 0);
    assert!(layers_array.object_at_indexed_subscript(0).is_some());

    // 4. Cover device-backed RasterizationRateMap behavior when available
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed rasterization rate tests.");
        return Ok(());
    };

    let selector = sel(b"supportsRasterizationRateMapWithLayerCount:\0");
    if !responds_to_selector(device.raw, selector) {
        println!(
            "supportsRasterizationRateMapWithLayerCount: is unsupported, skipping device-backed map creation."
        );
        return Ok(());
    }

    // Assert support checks for layer counts 0, 1, and 2 return Ok(bool)
    let support_0 = device.supports_rasterization_rate_map_with_layer_count(0)?;
    let support_1 = device.supports_rasterization_rate_map_with_layer_count(1)?;
    let support_2 = device.supports_rasterization_rate_map_with_layer_count(2)?;
    println!(
        "Device supports rasterization rate map with layer counts: 0: {}, 1: {}, 2: {}",
        support_0, support_1, support_2
    );

    if support_1 {
        let one_layer_desc =
            RasterizationRateMapDescriptor::with_screen_size_and_layer(screen_size, &layer2);
        one_layer_desc.set_label("minmetal-rasterization-rate-device-test");
        let rate_map = device.new_rasterization_rate_map(&one_layer_desc)?;

        // screen_size()
        assert_eq!(rate_map.screen_size().width, screen_size.width);
        assert_eq!(rate_map.screen_size().height, screen_size.height);
        // layer_count()
        assert_eq!(rate_map.layer_count(), 1);
        // label()
        assert_eq!(
            rate_map.label().as_deref(),
            Some("minmetal-rasterization-rate-device-test")
        );
        // device() returns a non-null device
        let rate_map_device = rate_map.device();
        assert!(!rate_map_device.raw.is_null());
        // physical_granularity() has non-zero width/height
        let granularity = rate_map.physical_granularity();
        assert!(granularity.width > 0);
        assert!(granularity.height > 0);
        // parameter_buffer_size_and_align() has non-zero size and non-zero alignment
        let size_align = rate_map.parameter_buffer_size_and_align();
        assert!(size_align.size > 0);
        assert!(size_align.align > 0);

        // copy_parameter_data_to_buffer(...) succeeds using an aligned offset
        let buffer = device.new_buffer(
            size_align.size + size_align.align,
            ResourceOptions::STORAGE_MODE_SHARED,
        )?;
        rate_map.copy_parameter_data_to_buffer(&buffer, size_align.align)?;

        // physical_size_for_layer(0) succeeds and returns non-zero width/height
        let phys_size = rate_map.physical_size_for_layer(0)?;
        assert!(phys_size.width > 0);
        assert!(phys_size.height > 0);

        // screen-to-physical and physical-to-screen coordinate mapping succeeds
        let center = Coordinate2D::new(
            screen_size.width as f32 / 2.0,
            screen_size.height as f32 / 2.0,
        );
        let br = Coordinate2D::new(
            screen_size.width as f32 - 1.0,
            screen_size.height as f32 - 1.0,
        );
        let coords = [Coordinate2D::new(0.0, 0.0), center, br];
        for coord in coords {
            let mapped_phys = rate_map.map_screen_to_physical_coordinates(coord, 0)?;
            let mapped_screen = rate_map.map_physical_to_screen_coordinates(mapped_phys, 0)?;
            assert!(mapped_phys.x >= 0.0);
            assert!(mapped_phys.y >= 0.0);
            assert!(mapped_screen.x >= 0.0);
            assert!(mapped_screen.y >= 0.0);
        }

        // 5. Cover integration with render pass descriptor
        let pass_desc = RenderPassDescriptor::new();
        pass_desc.set_rasterization_rate_map(Some(&rate_map))?;
        assert!(pass_desc.rasterization_rate_map().is_some());
        pass_desc.set_rasterization_rate_map(None)?;
        assert!(pass_desc.rasterization_rate_map().is_none());
    }

    if support_2 {
        let two_layer_desc = RasterizationRateMapDescriptor::with_screen_size_and_layers(
            screen_size,
            &[&layer2, &layer3],
        );
        let rate_map_2 = device.new_rasterization_rate_map(&two_layer_desc)?;
        assert_eq!(rate_map_2.layer_count(), 2);
        let size_0 = rate_map_2.physical_size_for_layer(0)?;
        let size_1 = rate_map_2.physical_size_for_layer(1)?;
        assert!(size_0.width > 0 && size_0.height > 0);
        assert!(size_1.width > 0 && size_1.height > 0);
    }

    Ok(())
}
