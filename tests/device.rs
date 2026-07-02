use minmetal::*;
#[test]
fn device_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CompileOptions permutations
    let opts = CompileOptions::new();
    let opts_def = CompileOptions::default();
    assert!(!opts.raw.is_null());
    assert!(!opts_def.raw.is_null());

    opts.set_library_type(LibraryType::Executable);
    assert_eq!(opts.library_type(), Some(LibraryType::Executable));
    opts.set_library_type_raw(1); // Dynamic
    assert_eq!(opts.library_type(), Some(LibraryType::Dynamic));

    opts.set_install_name("minmetal-install-name");
    if responds_to_selector(opts.raw, sel(b"installName\0")) {
        assert_eq!(
            opts.install_name().as_deref(),
            Some("minmetal-install-name")
        );
    }

    opts.set_optimization_level(LibraryOptimizationLevel::Size);
    if responds_to_selector(opts.raw, sel(b"optimizationLevel\0")) {
        assert_eq!(
            opts.optimization_level(),
            Some(LibraryOptimizationLevel::Size)
        );
    }

    // 2. Device-backed tests
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping device-backed tests.");
        return Ok(());
    };

    let required_device = Device::required_system_default()?;
    assert!(!required_device.raw.is_null());
    assert!(!device.name().is_empty());

    let queue = device.new_command_queue()?;
    assert!(!queue.raw.is_null());
    let empty_command_buffer = queue.command_buffer()?;
    assert_eq!(
        empty_command_buffer.status(),
        CommandBufferStatus::NotEnqueued
    );
    empty_command_buffer.commit();
    empty_command_buffer.wait_until_completed();
    assert_eq!(
        empty_command_buffer.status(),
        CommandBufferStatus::Completed
    );
    assert!(empty_command_buffer.error().is_none());

    let fence = device.new_fence()?;
    assert!(!fence.raw.is_null());

    let shared_event = device.new_shared_event()?;
    shared_event.set_signaled_value(7);
    assert_eq!(shared_event.signaled_value(), 7);

    let depth_desc = DepthStencilDescriptor::new();
    depth_desc.set_depth_compare_function(CompareFunction::LessEqual);
    depth_desc.set_depth_write_enabled(true);
    let depth_state = device.new_depth_stencil_state(&depth_desc)?;
    assert!(!depth_state.raw.is_null());

    // Compile library from source
    let source = r#"
        #include <metal_stdlib>
        using namespace metal;
        kernel void test_kernel() {}
    "#;
    let lib = device.new_library_with_source(source)?;
    assert!(!lib.raw.is_null());

    lib.set_label("my-library");
    assert_eq!(lib.label().as_deref(), Some("my-library"));

    if let Ok(t) = lib.library_type() {
        assert_eq!(t, LibraryType::Executable);
    }

    if let Ok(install_name) = lib.install_name() {
        println!("Library install name: {:?}", install_name);
    }

    // Function validation
    let func = lib.function("test_kernel")?;
    assert_eq!(func.name(), "test_kernel");

    // Source compilation with options
    let lib_opts = device.new_library_with_source_and_options(source, &opts)?;
    assert!(!lib_opts.raw.is_null());

    let bad_source = device.new_library_with_source("this is not metal");
    assert!(bad_source.is_err());
    assert!(!bad_source.unwrap_err().to_string().is_empty());

    let missing_func = lib.function("definitely_missing_kernel");
    assert!(missing_func.is_err());
    assert!(!missing_func.unwrap_err().to_string().is_empty());

    // Load precompiled library
    let metallib_path = "tests/shaders/precompiled_basic.metallib";
    let file_lib = device.new_library_with_file(metallib_path)?;
    assert!(!file_lib.raw.is_null());

    let url_lib = device.new_library_with_url_path(metallib_path)?;
    assert!(!url_lib.raw.is_null());

    let precompiled_func = file_lib.function("add_one")?;
    assert_eq!(precompiled_func.name(), "add_one");

    let missing_file = device.new_library_with_file("/tmp/minmetal_missing_file.metallib");
    assert!(missing_file.is_err());
    assert!(!missing_file.unwrap_err().to_string().is_empty());

    let missing_url = device.new_library_with_url_path("/tmp/minmetal_missing_url.metallib");
    assert!(missing_url.is_err());
    assert!(!missing_url.unwrap_err().to_string().is_empty());

    match device.new_default_library() {
        Ok(default_lib) => assert!(!default_lib.raw.is_null()),
        Err(e) => assert!(!e.to_string().is_empty()),
    }

    if responds_to_selector(device.raw, sel(b"newBinaryArchiveWithDescriptor:error:\0")) {
        let archive_desc = BinaryArchiveDescriptor::new();
        let archive = device.new_binary_archive(&archive_desc)?;
        assert!(!archive.raw.is_null());
    }

    let icb_desc = IndirectCommandBufferDescriptor::new();
    icb_desc.set_command_types(IndirectCommandType::CONCURRENT_DISPATCH);
    icb_desc.set_max_kernel_buffer_bind_count(1);
    match device.new_indirect_command_buffer(&icb_desc, 1, IndirectCommandBufferOptions::NONE) {
        Ok(icb) => assert!(!icb.raw.is_null()),
        Err(e) => assert!(!e.to_string().is_empty()),
    }

    Ok(())
}
