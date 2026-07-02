use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void compute_main(device uint* values [[buffer(0)]],
                         uint index [[thread_position_in_grid]]) {
    values[index] = index;
}

vertex float4 vertex_main(uint vid [[vertex_id]]) {
    return float4(0.0, 0.0, 0.0, 1.0);
}

fragment float4 fragment_main() {
    return float4(1.0, 1.0, 1.0, 1.0);
}
"#;

#[derive(Clone, Copy)]
#[repr(C)]
struct DrawPrimitivesIndirectArguments {
    vertex_count: u32,
    instance_count: u32,
    vertex_start: u32,
    base_instance: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct DrawIndexedPrimitivesIndirectArguments {
    index_count: u32,
    instance_count: u32,
    index_start: u32,
    base_vertex: i32,
    base_instance: u32,
}

fn finish_command_buffer(
    label: &str,
    command_buffer: &CommandBuffer,
) -> Result<(), Box<dyn std::error::Error>> {
    command_buffer.commit();
    command_buffer.wait_until_completed();
    if let Some(err) = command_buffer.error() {
        return Err(Box::new(MetalError::new(format!("{label}: {err}"))));
    }
    assert_eq!(command_buffer.status(), CommandBufferStatus::Completed);
    Ok(())
}

#[test]
fn encoder_module_permutations() -> Result<(), Box<dyn std::error::Error>> {
    let Some(device) = Device::system_default() else {
        println!("No Metal device available, skipping encoder tests.");
        return Ok(());
    };

    let queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADER)?;
    let compute_func = library.function("compute_main")?;
    let vertex_func = library.function("vertex_main")?;
    let fragment_func = library.function("fragment_main")?;

    let compute_pipeline = device.new_compute_pipeline_state_with_function(&compute_func)?;

    let render_desc = RenderPipelineDescriptor::new();
    render_desc.set_vertex_function(&vertex_func);
    render_desc.set_fragment_function(&fragment_func);
    render_desc.set_color_attachment_pixel_format(0, PixelFormat::Rgba8Unorm);
    let render_pipeline = device.new_render_pipeline_state(&render_desc)?;

    // Shared resources
    let buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
    let render_texture_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 64, 64, false);
    render_texture_desc.set_usage(
        TextureUsage::RENDER_TARGET | TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE,
    );
    render_texture_desc.set_storage_mode(StorageMode::Shared);
    let texture = device.new_texture(&render_texture_desc)?;

    let sampled_texture_desc =
        TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 64, 64, false);
    sampled_texture_desc.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    sampled_texture_desc.set_storage_mode(StorageMode::Shared);
    let sampled_texture = device.new_texture(&sampled_texture_desc)?;

    let mip_texture_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 64, 64, true);
    mip_texture_desc.set_usage(
        TextureUsage::RENDER_TARGET | TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE,
    );
    mip_texture_desc.set_storage_mode(StorageMode::Shared);
    let mip_texture = device.new_texture(&mip_texture_desc)?;
    let sampler_desc = SamplerDescriptor::new();
    let sampler = device.new_sampler_state(&sampler_desc)?;

    // 1. RenderCommandEncoder tests
    {
        let cmd_buf = queue.command_buffer()?;
        let render_pass = RenderPassDescriptor::new();
        let att0 = render_pass.color_attachment(0);
        att0.base().set_texture(Some(&texture));
        att0.base().set_load_action(LoadAction::Clear);
        att0.base().set_store_action(StoreAction::Store);

        let encoder = cmd_buf.render_command_encoder(&render_pass)?;
        encoder.set_render_pipeline_state(&render_pipeline);

        encoder.set_vertex_buffer(0, &buffer, 0);
        encoder.set_vertex_texture(0, &sampled_texture);
        encoder.set_vertex_sampler_state(0, &sampler);
        encoder.set_vertex_bytes(1, &12345u32);

        encoder.set_fragment_buffer(0, &buffer, 0);
        encoder.set_fragment_texture(0, &sampled_texture);
        encoder.set_fragment_sampler_state(0, &sampler);
        encoder.set_fragment_bytes(1, &54321u32);

        encoder.set_viewport(Viewport::new(0.0, 0.0, 64.0, 64.0, 0.0, 1.0));
        encoder.set_scissor_rect(ScissorRect::new(0, 0, 64, 64));
        encoder.set_cull_mode(CullMode::Back);
        encoder.set_front_facing_winding(Winding::Clockwise);
        encoder.set_triangle_fill_mode(TriangleFillMode::Fill);
        encoder.set_depth_bias(0.1, 0.2, 0.3);

        // Drawing
        let index_buffer = device
            .new_buffer_with_data(&[0u16, 0u16, 0u16], ResourceOptions::STORAGE_MODE_SHARED)?;
        encoder.draw_primitives(PrimitiveType::Triangle, 0, 3);
        encoder.draw_primitives_instanced(PrimitiveType::Triangle, 0, 3, 2);
        encoder.draw_indexed_primitives(
            PrimitiveType::Triangle,
            3,
            IndexType::UInt16,
            &index_buffer,
            0,
        );
        encoder.draw_indexed_primitives_instanced(
            PrimitiveType::Triangle,
            3,
            IndexType::UInt16,
            &index_buffer,
            0,
            2,
        );

        // Indirect drawing
        let args = DrawPrimitivesIndirectArguments {
            vertex_count: 3,
            instance_count: 1,
            vertex_start: 0,
            base_instance: 0,
        };
        let indirect_buffer =
            device.new_buffer_with_data(&[args], ResourceOptions::STORAGE_MODE_SHARED)?;
        let indexed_args = DrawIndexedPrimitivesIndirectArguments {
            index_count: 3,
            instance_count: 1,
            index_start: 0,
            base_vertex: 0,
            base_instance: 0,
        };
        let indexed_indirect_buffer =
            device.new_buffer_with_data(&[indexed_args], ResourceOptions::STORAGE_MODE_SHARED)?;
        encoder.draw_primitives_indirect(PrimitiveType::Triangle, &indirect_buffer, 0);
        encoder.draw_indexed_primitives_indirect(
            PrimitiveType::Triangle,
            IndexType::UInt16,
            &index_buffer,
            0,
            &indexed_indirect_buffer,
            0,
        );

        // Resource usage
        encoder.use_buffer(&buffer, ResourceUsage::READ);
        encoder.use_texture(&sampled_texture, ResourceUsage::READ);

        let heap_desc = HeapDescriptor::new();
        heap_desc.set_size(1024 * 1024);
        heap_desc.set_storage_mode(StorageMode::Shared);
        if let Ok(heap) = device.new_heap(&heap_desc) {
            encoder.use_heap(&heap);
        }

        encoder.end_encoding();
        finish_command_buffer("render encoder", &cmd_buf)?;
    }

    // 2. ComputeCommandEncoder tests
    {
        let cmd_buf = queue.command_buffer()?;
        let encoder = cmd_buf.compute_command_encoder()?;
        encoder.set_compute_pipeline_state(&compute_pipeline);

        encoder.set_buffer(0, &buffer, 0);
        encoder.set_texture(0, &sampled_texture);
        encoder.set_sampler_state(0, &sampler);
        encoder.set_bytes(1, &100u32);

        encoder.dispatch_threads(Size::new(16, 1, 1), Size::new(16, 1, 1));
        encoder.dispatch_threadgroups(Size::new(1, 1, 1), Size::new(16, 1, 1));

        encoder.use_buffer(&buffer, ResourceUsage::READ);
        encoder.use_texture(&sampled_texture, ResourceUsage::READ);

        encoder.end_encoding();
        finish_command_buffer("compute encoder", &cmd_buf)?;
        let mut values = [0u32; 16];
        buffer.read_slice(&mut values);
        for (index, value) in values.iter().enumerate() {
            assert_eq!(*value, index as u32);
        }
    }

    // 3. BlitCommandEncoder tests
    {
        let cmd_buf = queue.command_buffer()?;
        let encoder = cmd_buf.blit_command_encoder()?;
        let src_buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
        let dst_buffer = device.new_buffer(1024, ResourceOptions::STORAGE_MODE_SHARED)?;
        let source_pattern: Vec<u8> = (0..1024).map(|i| (i % 251) as u8).collect();
        src_buffer.write_slice(&source_pattern);

        encoder.copy_texture_to_texture(
            &sampled_texture,
            Origin::new(0, 0, 0),
            Size::new(16, 16, 1),
            &mip_texture,
            Origin::new(16, 16, 0),
        );

        encoder.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 256, 128);
        encoder.copy_buffer_to_texture(
            &src_buffer,
            0,
            64 * 4,
            64 * 64 * 4,
            Size::new(16, 16, 1),
            &mip_texture,
            Origin::new(0, 0, 0),
        );

        encoder.generate_mipmaps(&mip_texture);
        encoder.synchronize_resource(&buffer);
        encoder.synchronize_texture(&mip_texture);

        encoder.end_encoding();
        finish_command_buffer("blit encoder", &cmd_buf)?;
        let mut copied = vec![0u8; 1024];
        dst_buffer.read_slice(&mut copied);
        assert_eq!(&copied[256..384], &source_pattern[0..128]);
    }

    // 4. ResourceStateCommandEncoder tests
    {
        let cmd_buf = queue.command_buffer()?;
        let encoder = cmd_buf.resource_state_command_encoder()?;
        let fence = device.new_fence()?;
        let mut sparse_heap_keepalive = None;
        let mut sparse_texture_keepalive = None;

        encoder.update_fence(&fence);
        encoder.wait_for_fence(&fence);

        // update_texture_mapping mapping tests
        let sparse_tile = device.sparse_tile_size(TextureType::D2, PixelFormat::Rgba8Unorm, 1);
        if device.supports_sparse_textures() && sparse_tile.width > 0 && sparse_tile.height > 0 {
            let heap_desc = HeapDescriptor::new();
            heap_desc.set_heap_type(HeapType::Sparse);
            heap_desc.set_size(1024 * 1024);
            heap_desc.set_storage_mode(StorageMode::Private);
            heap_desc.set_sparse_page_size(SparsePageSize::Size64);
            if let Ok(sparse_heap) = device.new_heap(&heap_desc) {
                let sparse_desc =
                    TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 256, 256, false);
                sparse_desc.set_storage_mode(StorageMode::Private);
                sparse_desc.set_usage(TextureUsage::SHADER_READ);
                let sparse_tex = sparse_heap.new_texture(&sparse_desc)?;
                encoder.update_texture_mapping(
                    &sparse_tex,
                    SparseTextureMappingMode::Map,
                    Region::new_2d(0, 0, 64, 64),
                    0,
                    0,
                )?;
                encoder.update_texture_mapping(
                    &sparse_tex,
                    SparseTextureMappingMode::Unmap,
                    Region::new_2d(0, 0, 64, 64),
                    0,
                    0,
                )?;
                sparse_texture_keepalive = Some(sparse_tex);
                sparse_heap_keepalive = Some(sparse_heap);
            }
        }

        encoder.end_encoding();
        finish_command_buffer("resource state encoder", &cmd_buf)?;
        drop(sparse_texture_keepalive);
        drop(sparse_heap_keepalive);
    }

    // 5. ParallelRenderCommandEncoder tests
    {
        let cmd_buf = queue.command_buffer()?;
        let render_pass = RenderPassDescriptor::new();
        let att0 = render_pass.color_attachment(0);
        att0.base().set_texture(Some(&texture));

        if let Ok(parallel_encoder) = cmd_buf.parallel_render_command_encoder(&render_pass) {
            let child_encoder = parallel_encoder.render_command_encoder()?;
            child_encoder.end_encoding();
            parallel_encoder.end_encoding();
            finish_command_buffer("parallel render encoder", &cmd_buf)?;
        }
    }

    Ok(())
}
