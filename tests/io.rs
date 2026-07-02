use minmetal::*;
use std::ffi::c_void;
use std::fs::{remove_file, File};
use std::io::Write;

#[test]
fn io_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Pure enum/API permutations
    let priorities = [IOPriority::High, IOPriority::Normal, IOPriority::Low];
    for &p in &priorities {
        match p {
            IOPriority::High => assert_eq!(p as usize, 0),
            IOPriority::Normal => assert_eq!(p as usize, 1),
            IOPriority::Low => assert_eq!(p as usize, 2),
        }
        assert_eq!(IOPriority::from_raw(p as usize), Some(p));
    }
    assert_eq!(IOPriority::from_raw(99), None);

    let queue_types = [IOCommandQueueType::Concurrent, IOCommandQueueType::Serial];
    for &q in &queue_types {
        match q {
            IOCommandQueueType::Concurrent => assert_eq!(q as usize, 0),
            IOCommandQueueType::Serial => assert_eq!(q as usize, 1),
        }
        assert_eq!(IOCommandQueueType::from_raw(q as usize), Some(q));
    }
    assert_eq!(IOCommandQueueType::from_raw(99), None);

    let io_statuses = [
        IOStatus::Pending,
        IOStatus::Cancelled,
        IOStatus::Error,
        IOStatus::Complete,
    ];
    for &s in &io_statuses {
        match s {
            IOStatus::Pending => assert_eq!(s as isize, 0),
            IOStatus::Cancelled => assert_eq!(s as isize, 1),
            IOStatus::Error => assert_eq!(s as isize, 2),
            IOStatus::Complete => assert_eq!(s as isize, 3),
        }
        assert_eq!(IOStatus::from_raw(s as isize), Some(s));
    }
    assert_eq!(IOStatus::from_raw(99), None);

    let io_errors = [IOError::UrlInvalid, IOError::Internal];
    for &e in &io_errors {
        match e {
            IOError::UrlInvalid => assert_eq!(e as isize, 1),
            IOError::Internal => assert_eq!(e as isize, 2),
        }
        assert_eq!(IOError::from_raw(e as isize), Some(e));
    }
    assert_eq!(IOError::from_raw(99), None);

    let compression_statuses = [IOCompressionStatus::Complete, IOCompressionStatus::Error];
    for &cs in &compression_statuses {
        match cs {
            IOCompressionStatus::Complete => assert_eq!(cs as isize, 0),
            IOCompressionStatus::Error => assert_eq!(cs as isize, 1),
        }
        assert_eq!(IOCompressionStatus::from_raw(cs as isize), Some(cs));
    }
    assert_eq!(IOCompressionStatus::from_raw(99), None);

    let compression_methods = [
        IOCompressionMethod::Zlib,
        IOCompressionMethod::Lzfse,
        IOCompressionMethod::Lz4,
        IOCompressionMethod::Lzma,
        IOCompressionMethod::LzBitmap,
    ];
    for &cm in &compression_methods {
        match cm {
            IOCompressionMethod::Zlib => assert_eq!(cm as isize, 0),
            IOCompressionMethod::Lzfse => assert_eq!(cm as isize, 1),
            IOCompressionMethod::Lz4 => assert_eq!(cm as isize, 2),
            IOCompressionMethod::Lzma => assert_eq!(cm as isize, 3),
            IOCompressionMethod::LzBitmap => assert_eq!(cm as isize, 4),
        }
        assert_eq!(IOCompressionMethod::from_raw(cm as isize), Some(cm));
    }
    assert_eq!(IOCompressionMethod::from_raw(99), None);

    // 2. IOCommandQueueDescriptor tests
    let desc = IOCommandQueueDescriptor::new();
    let desc_default = IOCommandQueueDescriptor::default();

    desc.set_max_command_buffer_count(0);
    desc.set_max_command_buffer_count(1);
    desc.set_max_command_buffer_count(4);

    desc.set_max_commands_in_flight(0);
    desc.set_max_commands_in_flight(1);
    desc.set_max_commands_in_flight(4);

    for &p in &priorities {
        desc.set_priority(p);
        assert_eq!(desc.priority()?, p);
    }

    for &q in &queue_types {
        desc.set_type(q);
        assert_eq!(desc.queue_type()?, q);
    }

    drop(desc_default);

    // 3. Compression context tests
    let interior_nul_res = IOCompressionContext::new("abc\0def", IOCompressionMethod::Lzfse, 4096);
    assert!(interior_nul_res.is_err());

    let chunk_size = io_compression_context_default_chunk_size();
    println!("Default chunk size: {}", chunk_size);

    // Define temp paths
    let temp_uncompressed_path = "temp_payload.bin";
    let temp_compressed_path = "temp_payload.lzfse";

    // Best-effort cleanup before starting
    let _ = remove_file(temp_uncompressed_path);
    let _ = remove_file(temp_compressed_path);

    let mut payload = vec![0u8; 1024];
    for (i, val) in payload.iter_mut().enumerate() {
        *val = (i % 256) as u8;
    }

    // Write uncompressed payload for general testing
    {
        let mut file = File::create(temp_uncompressed_path)?;
        file.write_all(&payload)?;
    }

    // Try creating a compressed file using context
    let mut compression_supported = true;
    if let Ok(comp_ctx) = IOCompressionContext::new(temp_compressed_path, IOCompressionMethod::Lzfse, chunk_size) {
        comp_ctx.append_data(&[]);
        comp_ctx.append_data(&payload);
        let flush_res = comp_ctx.flush_and_destroy()?;
        assert_eq!(flush_res, IOCompressionStatus::Complete);
    } else {
        println!("IOCompressionContext creation unsupported in this environment. Skipping compressed readback path.");
        compression_supported = false;
    }

    // 4. MetalIO device tests
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping remaining IO tests.");
        let _ = remove_file(temp_uncompressed_path);
        let _ = remove_file(temp_compressed_path);
        return Ok(());
    };

    let dummy_desc = IOCommandQueueDescriptor::new();
    if device.new_io_command_queue(&dummy_desc).is_err() {
        println!("newIOCommandQueueWithDescriptor:error: is not supported on this device/OS. Skipping remaining IO tests.");
        let _ = remove_file(temp_uncompressed_path);
        let _ = remove_file(temp_compressed_path);
        return Ok(());
    }

    // File Handle Tests
    let file_handle = device.new_io_file_handle(temp_uncompressed_path)?;
    file_handle.set_label("");
    assert_eq!(file_handle.label().as_deref(), Some(""));
    file_handle.set_label("minmetal-io-file");
    assert_eq!(file_handle.label().as_deref(), Some("minmetal-io-file"));

    let invalid_handle_res = device.new_io_file_handle("non_existent_file_xyz_999.bin");
    assert!(invalid_handle_res.is_err());

    // Command Queue Tests
    let desc_serial = IOCommandQueueDescriptor::new();
    desc_serial.set_type(IOCommandQueueType::Serial);
    let queue_serial = device.new_io_command_queue(&desc_serial)?;
    queue_serial.set_label("serial-queue");
    assert_eq!(queue_serial.label().as_deref(), Some("serial-queue"));

    let desc_concurrent = IOCommandQueueDescriptor::new();
    desc_concurrent.set_type(IOCommandQueueType::Concurrent);
    let queue_concurrent = device.new_io_command_queue(&desc_concurrent)?;

    queue_serial.enqueue_barrier()?;

    // 5. Command Buffer and Operations
    let cmd_buf = queue_concurrent.command_buffer()?;
    cmd_buf.set_label("my-io-cmd-buf");
    assert_eq!(cmd_buf.label().as_deref(), Some("my-io-cmd-buf"));

    // Exercise command_buffer_with_unretained_references
    let unretained_cmd_buf = queue_concurrent.command_buffer_with_unretained_references()?;
    unretained_cmd_buf.commit()?;
    unretained_cmd_buf.wait_until_completed()?;

    // load_bytes
    let mut host_bytes = vec![0u8; 1024];
    cmd_buf.load_bytes(
        host_bytes.as_mut_ptr() as *mut c_void,
        payload.len(),
        &file_handle,
        0,
    )?;

    // load_buffer
    let shared_buffer = device.new_buffer(2048, ResourceOptions::STORAGE_MODE_SHARED)?;
    cmd_buf.load_buffer(&shared_buffer, 0, 512, &file_handle, 0)?;
    cmd_buf.load_buffer(&shared_buffer, 512, 512, &file_handle, 512)?;

    // load_texture
    let tex_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 4, 4, false);
    tex_desc.set_storage_mode(StorageMode::Shared);
    tex_desc.set_usage(TextureUsage::SHADER_READ);
    let shared_texture = device.new_texture(&tex_desc)?;
    cmd_buf.load_texture(
        &shared_texture,
        0,
        0,
        Size::new(4, 4, 1),
        16,
        16,
        Origin::new(0, 0, 0),
        &file_handle,
        0,
    )?;

    // copy_status_to_buffer
    let status_buffer = device.new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)?;
    cmd_buf.copy_status_to_buffer(&status_buffer, 0)?;

    // wait_for_event and signal_event
    let shared_event = device.new_shared_event()?;
    cmd_buf.wait_for_event(&shared_event, 0)?;
    cmd_buf.signal_event(&shared_event, 2)?;

    cmd_buf.add_barrier()?;

    // try_cancel
    let fresh_cmd_buf = queue_concurrent.command_buffer()?;
    fresh_cmd_buf.try_cancel()?;

    // Commit and wait
    cmd_buf.enqueue()?;
    cmd_buf.commit()?;
    cmd_buf.wait_until_completed()?;

    if let Some(err) = cmd_buf.error() {
        panic!("IO Command Buffer failed with error: {}", err);
    }
    assert_eq!(cmd_buf.status()?, IOStatus::Complete);

    // Verify host_bytes
    assert_eq!(host_bytes, payload);

    // Verify buffer payload
    let mut buf_data = vec![0u8; 1024];
    shared_buffer.read_slice(&mut buf_data);
    assert_eq!(buf_data, payload);

    // Verify texture payload (4x4 RGBA8 = 64 bytes)
    let mut tex_data = vec![0u8; 64];
    shared_texture.get_bytes(
        Region::new_2d(0, 0, 4, 4),
        0,
        &mut tex_data,
        16,
    );
    assert_eq!(tex_data[..64], payload[..64]);

    // 6. Test compressed readback if supported
    let has_compressed_new = responds_to_selector(device.raw, sel(b"newIOFileHandleWithURL:compressionMethod:error:\0"));
    let has_compressed_legacy = responds_to_selector(device.raw, sel(b"newIOHandleWithURL:compressionMethod:error:\0"));
    let compressed_supported = has_compressed_new || has_compressed_legacy;

    if compressed_supported && compression_supported {
        let comp_file_handle = device.new_io_file_handle_compressed(temp_compressed_path, IOCompressionMethod::Lzfse)?;
        let decomp_buf = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
        let comp_cmd_buf = queue_concurrent.command_buffer()?;
        comp_cmd_buf.load_buffer(&decomp_buf, 0, 1024, &comp_file_handle, 0)?;
        comp_cmd_buf.commit()?;
        comp_cmd_buf.wait_until_completed()?;

        if let Some(err) = comp_cmd_buf.error() {
            panic!("Compressed IO Command Buffer failed: {}", err);
        }
        assert_eq!(comp_cmd_buf.status()?, IOStatus::Complete);

        let mut decomp_data = vec![0u8; 1024];
        decomp_buf.read_slice(&mut decomp_data);
        assert_eq!(decomp_data, payload);
        println!("Compressed readback verified successfully.");
    } else {
        println!("Compressed file handle creation not supported on this device/OS, skipping.");
    }

    // Cleanup temp files
    let _ = remove_file(temp_uncompressed_path);
    let _ = remove_file(temp_compressed_path);

    Ok(())
}
