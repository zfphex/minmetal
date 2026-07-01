use minmetal::*;
use std::io::Write;

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

fn main() {
    println!("Running indirect_draw example...");
    std::io::stdout().flush().unwrap();

    let Some(device) = Device::system_default() else {
        println!("No Metal device available.");
        return;
    };
    println!("Got device.");
    std::io::stdout().flush().unwrap();

    let queue = device.new_command_queue().unwrap();
    let cmd_buf = queue.command_buffer().unwrap();
    println!("Got queue and command buffer.");
    std::io::stdout().flush().unwrap();

    // Compile simple shaders
    let library = device
        .new_library_with_source(
            r#"
        #include <metal_stdlib>
        using namespace metal;
        vertex float4 vertex_main(uint vid [[vertex_id]]) {
            return float4(0.0, 0.0, 0.0, 1.0);
        }
        fragment float4 fragment_main() {
            return float4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
        )
        .expect("failed to compile library");
    let vertex_func = library
        .function("vertex_main")
        .expect("vertex function not found");
    let fragment_func = library
        .function("fragment_main")
        .expect("fragment function not found");
    println!("Compiled shaders.");
    std::io::stdout().flush().unwrap();

    // Create a 2D texture to bind to the render pass color attachment
    let texture_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 128, 128, false);
    let texture = device
        .new_texture(&texture_desc)
        .expect("failed to create texture");
    println!("Created texture.");
    std::io::stdout().flush().unwrap();

    let render_pass_desc = RenderPassDescriptor::new();
    let attachment0 = render_pass_desc.color_attachment(0);
    let base = attachment0.base();
    base.set_texture(Some(&texture));
    base.set_load_action(LoadAction::Clear);
    base.set_store_action(StoreAction::Store);
    println!("Configured render pass.");
    std::io::stdout().flush().unwrap();

    // Create render pipeline state
    let pipeline_desc = RenderPipelineDescriptor::new();
    pipeline_desc.set_vertex_function(&vertex_func);
    pipeline_desc.set_fragment_function(&fragment_func);
    pipeline_desc.set_color_attachment_pixel_format(0, PixelFormat::Rgba8Unorm);
    let pipeline_state = device
        .new_render_pipeline_state(&pipeline_desc)
        .expect("failed to create pipeline state");
    println!("Created render pipeline state.");
    std::io::stdout().flush().unwrap();

    let render_encoder = cmd_buf.render_command_encoder(&render_pass_desc).unwrap();
    render_encoder.set_render_pipeline_state(&pipeline_state);
    println!("Created render encoder and set pipeline state.");
    std::io::stdout().flush().unwrap();

    // Create indirect buffer and index buffer
    let indirect_buffer = device
        .new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)
        .unwrap();
    let index_buffer = device
        .new_buffer(256, ResourceOptions::STORAGE_MODE_SHARED)
        .unwrap();
    println!("Created buffers.");
    std::io::stdout().flush().unwrap();

    // Populate buffers with valid draw arguments
    let args1 = DrawPrimitivesIndirectArguments {
        vertex_count: 3,
        instance_count: 1,
        vertex_start: 0,
        base_instance: 0,
    };
    indirect_buffer.write(&args1);

    let args2 = DrawIndexedPrimitivesIndirectArguments {
        index_count: 3,
        instance_count: 1,
        index_start: 0,
        base_vertex: 0,
        base_instance: 0,
    };
    // Offset by 32 bytes as specified in the draw call
    unsafe {
        let ptr = (indirect_buffer.contents() as *mut u8).add(32)
            as *mut DrawIndexedPrimitivesIndirectArguments;
        *ptr = args2;
    }
    indirect_buffer.did_modify_range(Range::new(0, 128));
    println!("Populated buffers.");
    std::io::stdout().flush().unwrap();

    // Exercise indirect draw commands
    println!("Calling draw_primitives_indirect...");
    std::io::stdout().flush().unwrap();
    render_encoder.draw_primitives_indirect(PrimitiveType::Triangle, &indirect_buffer, 0);

    println!("Calling draw_indexed_primitives_indirect...");
    std::io::stdout().flush().unwrap();
    render_encoder.draw_indexed_primitives_indirect(
        PrimitiveType::Triangle,
        IndexType::UInt16,
        &index_buffer,
        0,
        &indirect_buffer,
        32,
    );

    println!("Ending encoding...");
    std::io::stdout().flush().unwrap();
    render_encoder.end_encoding();
    println!("Indirect draw commands encoded successfully!");
    std::io::stdout().flush().unwrap();
}
