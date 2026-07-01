use minmetal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::required_system_default()?;

    let selector = sel(b"supportsRasterizationRateMapWithLayerCount:\0");
    if !responds_to_selector(device.raw, selector) {
        println!("Rasterization rate maps are not available, skipping example");
        return Ok(());
    }

    if !device.supports_rasterization_rate_map_with_layer_count(1)? {
        println!("Device does not support rasterization rate map with 1 layer, skipping");
        return Ok(());
    }

    let screen_size = Size::new(64, 64, 0);
    let layer = RasterizationRateLayerDescriptor::new(Size::new(2, 2, 0));
    layer.horizontal().set_object_at_indexed_subscript(1.0, 0);
    layer.horizontal().set_object_at_indexed_subscript(0.5, 1);
    layer.vertical().set_object_at_indexed_subscript(1.0, 0);
    layer.vertical().set_object_at_indexed_subscript(0.5, 1);

    let map_descriptor =
        RasterizationRateMapDescriptor::with_screen_size_and_layer(screen_size, &layer);
    map_descriptor.set_label("minmetal rasterization rate smoke test");

    let rate_map = device.new_rasterization_rate_map(&map_descriptor)?;
    assert_eq!(rate_map.screen_size().width, 64);
    assert_eq!(rate_map.layer_count(), 1);

    let param_size = rate_map.parameter_buffer_size_and_align();
    let param_buffer = device.new_buffer(
        param_size.size + param_size.align,
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    let aligned_offset = param_size.align;
    rate_map.copy_parameter_data_to_buffer(&param_buffer, aligned_offset)?;

    let physical_size = rate_map.physical_size_for_layer(0)?;
    let mapped = rate_map.map_screen_to_physical_coordinates(
        Coordinate2D::new(32.0, 32.0),
        0,
    )?;
    assert!(mapped.x <= 32.0);
    assert!(mapped.y <= 32.0);

    let pass_descriptor = RenderPassDescriptor::new();
    pass_descriptor.set_rasterization_rate_map(Some(&rate_map))?;
    assert!(pass_descriptor.rasterization_rate_map().is_some());

    println!("rasterization_rate smoke test passed");
    println!("screen size: {}x{}", screen_size.width, screen_size.height);
    println!(
        "physical size for layer 0: {}x{}",
        physical_size.width, physical_size.height
    );
    println!("mapped (32,32) -> ({}, {})", mapped.x, mapped.y);
    Ok(())
}
