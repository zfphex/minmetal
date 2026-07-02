use minmetal::*;

#[test]
fn capture_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Enum variants
    let destinations = [
        CaptureDestination::DeveloperTools,
        CaptureDestination::GpuTraceDocument,
    ];
    for &dest in &destinations {
        match dest {
            CaptureDestination::DeveloperTools => assert_eq!(dest as usize, 1),
            CaptureDestination::GpuTraceDocument => assert_eq!(dest as usize, 2),
        }
    }

    // 2. CaptureDescriptor constructors, methods, and drop
    let desc = CaptureDescriptor::new();
    let desc_default = CaptureDescriptor::default();

    // Check set functions on a dummy object or null/nil if safe
    desc.set_destination(CaptureDestination::DeveloperTools);
    desc.set_output_url("/tmp/test_trace.gputrace");

    // Optional real Device/Queue for set_capture_object
    if let Some(device) = Device::system_default() {
        if let Ok(queue) = device.new_command_queue() {
            desc.set_capture_object(queue.raw);
        }
    } else {
        desc.set_capture_object(std::ptr::null_mut());
    }

    drop(desc);
    drop(desc_default);

    // 3. CaptureManager methods
    let capture_manager = match CaptureManager::shared() {
        Ok(mgr) => mgr,
        Err(e) => {
            println!(
                "CaptureManager not available: {}. Skipping remaining capture tests.",
                e
            );
            return Ok(());
        }
    };

    // supports_destination
    let tools_supported = capture_manager.supports_destination(CaptureDestination::DeveloperTools);
    let doc_supported = capture_manager.supports_destination(CaptureDestination::GpuTraceDocument);
    println!(
        "DeveloperTools supported: {}, GpuTraceDocument supported: {}",
        tools_supported, doc_supported
    );

    // Check is_capturing
    let _ = capture_manager.is_capturing();

    // start_capture error path (using an incomplete/invalid/unsupported setup or when developer tools are not active)
    let desc_err = CaptureDescriptor::new();
    desc_err.set_destination(CaptureDestination::DeveloperTools);
    // Safe to attempt, should either succeed (if system is configured for trace) or return Err
    if tools_supported {
        match capture_manager.start_capture(&desc_err) {
            Ok(_) => {
                assert!(capture_manager.is_capturing());
            }
            Err(e) => {
                println!("start_capture returned expected error: {}", e);
            }
        }
    }

    // stop_capture is safe to call even if not capturing
    capture_manager.stop_capture();

    Ok(())
}
