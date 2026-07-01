use minmetal::*;

fn main() {
    println!("Running pass_descriptors example...");
    let Some(device) = Device::system_default() else {
        println!("No Metal device available.");
        return;
    };
    println!("Device name: {}", device.name());

    let queue = device.new_command_queue().unwrap();
    let _cmd_buf = queue.command_buffer().unwrap();

    // 1. Create Render Pass Descriptor
    let render_pass_desc = RenderPassDescriptor::new();
    let attachment0 = render_pass_desc.color_attachment(0);
    let base = attachment0.base();
    base.set_load_action(LoadAction::Clear);
    base.set_store_action(StoreAction::Store);
    base.set_store_action_options(StoreActionOptions::NONE)
        .unwrap();

    // 2. Create Compute Pass Descriptor
    let compute_pass_desc = ComputePassDescriptor::new().unwrap();
    let compute_sample_buffers = compute_pass_desc.sample_buffer_attachments();
    let compute_attachment0 = compute_sample_buffers.object_at_indexed_subscript(0);
    compute_attachment0.set_start_of_encoder_sample_index(0);
    compute_attachment0.set_end_of_encoder_sample_index(1);

    // 3. Create Blit Pass Descriptor
    let blit_pass_desc = BlitPassDescriptor::new().unwrap();
    let blit_sample_buffers = blit_pass_desc.sample_buffer_attachments();
    let blit_attachment0 = blit_sample_buffers.object_at_indexed_subscript(0);
    blit_attachment0.set_start_of_encoder_sample_index(0);
    blit_attachment0.set_end_of_encoder_sample_index(1);

    // 4. Create Resource State Pass Descriptor
    let resource_pass_desc = ResourceStatePassDescriptor::new().unwrap();
    let resource_sample_buffers = resource_pass_desc.sample_buffer_attachments();
    let resource_attachment0 = resource_sample_buffers.object_at_indexed_subscript(0);
    resource_attachment0.set_start_of_encoder_sample_index(0);
    resource_attachment0.set_end_of_encoder_sample_index(1);

    println!("All pass descriptors configured successfully!");
}
