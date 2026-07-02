use minmetal::*;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running parallel_render example...");
    let _ = std::io::stdout().flush();

    let Some(device) = Device::system_default() else {
        println!("No Metal device available.");
        return Ok(());
    };
    println!("Got system default device.");
    let _ = std::io::stdout().flush();

    let queue = device.new_command_queue()?;
    println!("Got command queue.");
    let _ = std::io::stdout().flush();

    let cmd_buf = queue.command_buffer()?;
    println!("Got command buffer.");
    let _ = std::io::stdout().flush();

    // Create a 2D texture to bind to the render pass color attachment
    let texture_desc = TextureDescriptor::texture_2d(PixelFormat::Rgba8Unorm, 128, 128, false);
    let texture = device.new_texture(&texture_desc)?;
    println!("Created 128x128 texture.");
    let _ = std::io::stdout().flush();

    let render_pass_desc = RenderPassDescriptor::new();
    println!("Created RenderPassDescriptor.");
    let _ = std::io::stdout().flush();

    let attachment0 = render_pass_desc.color_attachment(0);
    let base = attachment0.base();
    base.set_texture(Some(&texture));
    base.set_load_action(LoadAction::Clear);
    base.set_store_action(StoreAction::Store);
    println!("Set attachment texture and properties.");
    let _ = std::io::stdout().flush();

    println!("Creating ParallelRenderCommandEncoder...");
    let _ = std::io::stdout().flush();

    match cmd_buf.parallel_render_command_encoder(&render_pass_desc) {
        Ok(parallel_encoder) => {
            println!("Successfully created ParallelRenderCommandEncoder!");
            let _ = std::io::stdout().flush();

            let sub_encoder = parallel_encoder.render_command_encoder()?;
            println!("Created child RenderCommandEncoder.");
            let _ = std::io::stdout().flush();

            sub_encoder.end_encoding();
            println!("Ended child RenderCommandEncoder encoding.");
            let _ = std::io::stdout().flush();

            parallel_encoder.end_encoding();
            println!("Ended ParallelRenderCommandEncoder encoding.");
            let _ = std::io::stdout().flush();
        }
        Err(e) => {
            println!("ParallelRenderCommandEncoder failed to create: {:?}", e);
            let _ = std::io::stdout().flush();
        }
    }

    println!("Example finished successfully.");
    let _ = std::io::stdout().flush();
    Ok(())
}
