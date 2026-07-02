use minmetal::*;

#[test]
fn layer_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping layer tests.");
        return Ok(());
    };

    // 1. MetalLayer::new
    let layer = match MetalLayer::new() {
        Ok(l) => l,
        Err(e) => {
            println!(
                "CAMetalLayer class not available: {}. Skipping layer tests.",
                e
            );
            return Ok(());
        }
    };

    // 2. set_device
    layer.set_device(&device);

    // 3. all pixel formats that are reasonable for layer use
    let reasonable_formats = [
        PixelFormat::Bgra8Unorm,
        PixelFormat::Bgra8UnormSrgb,
        PixelFormat::Rgba16Float,
        PixelFormat::Rgb10A2Unorm,
    ];
    for &format in &reasonable_formats {
        layer.set_pixel_format(format);
    }

    // 4. set_framebuffer_only(true/false)
    layer.set_framebuffer_only(true);
    layer.set_framebuffer_only(false);

    // 5. set_presents_with_transaction(true/false)
    layer.set_presents_with_transaction(true);
    layer.set_presents_with_transaction(false);

    // 6. set_contents_scale representative values
    let scales = [1.0, 2.0, 3.0];
    for &scale in &scales {
        layer.set_contents_scale(scale);
    }

    // 7. set_drawable_size representative sizes
    let sizes = [(0, 0), (1, 1), (1024, 768), (3840, 2160)];
    for &(w, h) in &sizes {
        layer.set_drawable_size(w, h);
    }

    // 8. next_drawable nil-safe path
    // Since the layer is not attached to an active NSView or window, next_drawable might return None, which we must handle safely.
    if let Some(drawable) = layer.next_drawable() {
        // Drawable::texture and present only if a drawable is available
        if let Ok(texture) = drawable.texture() {
            println!("Got texture with raw: {:?}", texture.raw);
        }
        drawable.present();
    } else {
        println!(
            "next_drawable returned None, which is expected for off-screen / unattached layers."
        );
    }

    Ok(())
}
