use minmetal::*;
use std::io::Write;

fn main() {
    println!("Running parallel_render example...");
    std::io::stdout().flush().unwrap();

    let Some(device) = Device::system_default() else {
        println!("No Metal device available.");
        return;
    };
    println!("Got system default device.");
    std::io::stdout().flush().unwrap();

    let queue = device.new_command_queue().expect("failed to create queue");
    println!("Got command queue.");
    std::io::stdout().flush().unwrap();

    let cmd_buf = queue
        .command_buffer()
        .expect("failed to create command buffer");
    println!("Got command buffer.");
    std::io::stdout().flush().unwrap();

    // Create a 2D texture to bind to the render pass color attachment
    let texture_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 128, 128, false);
    let texture = device
        .new_texture(&texture_desc)
        .expect("failed to create texture");
    println!("Created 128x128 texture.");
    std::io::stdout().flush().unwrap();

    let render_pass_desc = RenderPassDescriptor::new();
    println!("Created RenderPassDescriptor.");
    std::io::stdout().flush().unwrap();

    let attachment0 = render_pass_desc.color_attachment(0);
    let base = attachment0.base();
    base.set_texture(Some(&texture));
    base.set_load_action(LoadAction::Clear);
    base.set_store_action(StoreAction::Store);
    println!("Set attachment texture and properties.");
    std::io::stdout().flush().unwrap();

    println!("Creating ParallelRenderCommandEncoder...");
    std::io::stdout().flush().unwrap();

    match cmd_buf.parallel_render_command_encoder(&render_pass_desc) {
        Ok(parallel_encoder) => {
            println!("Successfully created ParallelRenderCommandEncoder!");
            std::io::stdout().flush().unwrap();

            let sub_encoder = parallel_encoder.render_command_encoder().unwrap();
            println!("Created child RenderCommandEncoder.");
            std::io::stdout().flush().unwrap();

            sub_encoder.end_encoding();
            println!("Ended child RenderCommandEncoder encoding.");
            std::io::stdout().flush().unwrap();

            parallel_encoder.end_encoding();
            println!("Ended ParallelRenderCommandEncoder encoding.");
            std::io::stdout().flush().unwrap();
        }
        Err(e) => {
            println!("ParallelRenderCommandEncoder failed to create: {:?}", e);
            std::io::stdout().flush().unwrap();
        }
    }

    println!("Example finished successfully.");
    std::io::stdout().flush().unwrap();
}
