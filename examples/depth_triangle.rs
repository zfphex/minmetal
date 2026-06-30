use miniwin::{Event, Key, Window, WindowStyle, create_window};

use minmetal::{
    AutoreleasePool, ClearColor, CompareFunction, DepthStencilDescriptor, Device, IndexType,
    LoadAction, MetalLayer, PixelFormat, PrimitiveType, RenderPassDescriptor,
    RenderPipelineDescriptor, ResourceOptions, ScissorRect, StoreAction, TextureDescriptor,
    TextureUsage, VertexDescriptor, VertexFormat, VertexStepFunction, Viewport,
};

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float3 position [[attribute(0)]];
};

struct VertexOut {
    float4 position [[position]];
    float3 color;
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]]) {
    VertexOut out;
    out.position = float4(in.position, 1.0);
    out.color = in.position * 0.5 + 0.5;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return float4(in.color, 1.0);
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = create_window("Depth Triangle", None, 800, 600, WindowStyle::Standard);

    let device = Device::system_default().ok_or("no Metal device is available")?;
    let queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADER)?;
    let vertex_function = library.function("vertex_main")?;
    let fragment_function = library.function("fragment_main")?;

    let vertex_descriptor = VertexDescriptor::new();
    vertex_descriptor.set_attribute(0, VertexFormat::Float3, 0, 0);
    vertex_descriptor.set_layout(
        0,
        std::mem::size_of::<Vertex>(),
        VertexStepFunction::PerVertex,
        1,
    );

    let pipeline_descriptor = RenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(&vertex_function);
    pipeline_descriptor.set_fragment_function(&fragment_function);
    pipeline_descriptor.set_vertex_descriptor(&vertex_descriptor);
    pipeline_descriptor.set_color_attachment_pixel_format(0, PixelFormat::Bgra8Unorm);
    pipeline_descriptor.set_depth_attachment_pixel_format(PixelFormat::Depth32Float);
    let pipeline = device.new_render_pipeline_state(&pipeline_descriptor)?;

    let depth_descriptor = DepthStencilDescriptor::new();
    depth_descriptor.set_depth_compare_function(CompareFunction::Less);
    depth_descriptor.set_depth_write_enabled(true);
    let depth_state = device.new_depth_stencil_state(&depth_descriptor)?;

    let vertices = [
        Vertex {
            position: [0.0, 0.65, 0.2],
        },
        Vertex {
            position: [-0.65, -0.55, 0.5],
        },
        Vertex {
            position: [0.65, -0.55, 0.8],
        },
    ];
    let indices = [0u16, 1, 2];
    let vertex_buffer =
        device.new_buffer_with_data(&vertices, ResourceOptions::STORAGE_MODE_SHARED)?;
    let index_buffer =
        device.new_buffer_with_data(&indices, ResourceOptions::STORAGE_MODE_SHARED)?;

    let scale = window.scale_factor();
    let (width, height) = window.content_size();
    let layer = unsafe {
        MetalLayer::attach_to_view(
            window.ns_view,
            &device,
            PixelFormat::Bgra8Unorm,
            (width as f64 * scale) as usize,
            (height as f64 * scale) as usize,
            scale,
        )?
    };

    let mut running = true;

    while running {
        let _pool = AutoreleasePool::new();

        window.draw(|win| {
            let scale = win.scale_factor();
            let (width, height) = win.content_size();
            let drawable_width = (width as f64 * scale).max(1.0) as usize;
            let drawable_height = (height as f64 * scale).max(1.0) as usize;

            layer.set_contents_scale(scale);
            layer.set_drawable_size(drawable_width, drawable_height);

            let depth_texture_descriptor = TextureDescriptor::texture_2d(
                PixelFormat::Depth32Float,
                drawable_width,
                drawable_height,
                false,
            );
            depth_texture_descriptor.set_usage(TextureUsage::RENDER_TARGET);
            let Ok(depth_texture) = device.new_texture(&depth_texture_descriptor) else {
                return;
            };

            let Some(drawable) = layer.next_drawable() else {
                return;
            };
            let color_texture = drawable.texture();

            let pass = RenderPassDescriptor::new();
            pass.set_color_attachment(
                0,
                &color_texture,
                LoadAction::Clear,
                StoreAction::Store,
                ClearColor::new(0.03, 0.03, 0.04, 1.0),
            );
            pass.set_depth_attachment(
                &depth_texture,
                LoadAction::Clear,
                StoreAction::DontCare,
                1.0,
            );

            let Ok(command_buffer) = queue.command_buffer() else {
                return;
            };
            let Ok(encoder) = command_buffer.render_command_encoder(&pass) else {
                return;
            };

            encoder.set_render_pipeline_state(&pipeline);
            encoder.set_depth_stencil_state(&depth_state);
            encoder.set_viewport(Viewport::new(
                0.0,
                0.0,
                drawable_width as f64,
                drawable_height as f64,
                0.0,
                1.0,
            ));
            encoder.set_scissor_rect(ScissorRect::new(0, 0, drawable_width, drawable_height));
            encoder.set_vertex_buffer(0, &vertex_buffer, 0);
            encoder.draw_indexed_primitives(
                PrimitiveType::Triangle,
                indices.len(),
                IndexType::UInt16,
                &index_buffer,
                0,
            );
            encoder.end_encoding();

            command_buffer.present_drawable(&drawable);
            command_buffer.commit();
        });

        while let Some(event) = window.event() {
            match event {
                Event::Quit
                | Event::CloseRequested
                | Event::KeyDown {
                    key: Key::Escape, ..
                } => running = false,
                _ => {}
            }
        }

        window.wait_for_vsync();
    }

    Ok(())
}
