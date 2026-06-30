#[cfg(target_os = "macos")]
use miniwin::{Event, Key, Window, WindowStyle, create_window};

#[cfg(target_os = "macos")]
use minmetal::{
    AutoreleasePool, ClearColor, Device, LoadAction, MetalLayer, PixelFormat, PrimitiveType,
    RenderPassDescriptor, RenderPipelineDescriptor, ResourceOptions, StoreAction,
};

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Clone, Copy)]
struct Uniforms {
    frame: f32,
    width: f32,
    height: f32,
    _pad: f32,
}

#[cfg(target_os = "macos")]
const SHADERS: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Uniforms {
    float frame;
    float width;
    float height;
    float _pad;
};

struct VertexOut {
    float4 position [[position]];
    float2 uv;
};

vertex VertexOut fullscreen_vertex(uint vertex_id [[vertex_id]]) {
    float2 positions[3] = {
        float2(-1.0, -1.0),
        float2( 3.0, -1.0),
        float2(-1.0,  3.0),
    };

    VertexOut out;
    out.position = float4(positions[vertex_id], 0.0, 1.0);
    out.uv = positions[vertex_id] * 0.5 + 0.5;
    return out;
}

fragment float4 gradient_fragment(VertexOut in [[stage_in]],
                                  constant Uniforms& uniforms [[buffer(0)]]) {
    float2 uv = in.uv;
    float time = uniforms.frame * 0.018;
    float pulse = 0.5 + 0.5 * sin(time + uv.x * 8.0 + uv.y * 4.0);
    float red = mix(0.08, 0.95, uv.x);
    float green = mix(0.12, 0.85, uv.y);
    float blue = mix(0.25, 1.0, pulse);
    return float4(red, green, blue, 1.0);
}
"#;

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = create_window("Metal Demo", None, 800, 600, WindowStyle::Standard);

    let device = Device::system_default().ok_or("no Metal device is available")?;
    eprintln!("Using {}", device.name());

    let command_queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADERS)?;
    let vertex = library.function("fullscreen_vertex")?;
    let fragment = library.function("gradient_fragment")?;

    let pipeline_descriptor = RenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(&vertex);
    pipeline_descriptor.set_fragment_function(&fragment);
    pipeline_descriptor.set_color_attachment_pixel_format(0, PixelFormat::Bgra8Unorm);
    let pipeline = device.new_render_pipeline_state(&pipeline_descriptor)?;

    let uniform_buffer = device.new_buffer(
        std::mem::size_of::<Uniforms>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    let scale = window.scale_factor();
    let (width, height) = window.content_size();
    let layer = unsafe {
        MetalLayer::attach_to_view(
            window.raw_ns_view(),
            &device,
            PixelFormat::Bgra8Unorm,
            (width as f64 * scale) as usize,
            (height as f64 * scale) as usize,
            scale,
        )?
    };

    let mut frame = 0.0f32;
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

            let Some(drawable) = layer.next_drawable() else {
                return;
            };

            let uniforms = Uniforms {
                frame,
                width: drawable_width as f32,
                height: drawable_height as f32,
                _pad: 0.0,
            };
            uniform_buffer.write(&uniforms);

            let texture = drawable.texture();
            let pass = RenderPassDescriptor::new();
            pass.set_color_attachment(
                0,
                &texture,
                LoadAction::Clear,
                StoreAction::Store,
                ClearColor::new(0.02, 0.02, 0.03, 1.0),
            );

            let Ok(command_buffer) = command_queue.command_buffer() else {
                return;
            };
            let Ok(encoder) = command_buffer.render_command_encoder(&pass) else {
                return;
            };

            encoder.set_render_pipeline_state(&pipeline);
            encoder.set_fragment_buffer(0, &uniform_buffer, 0);
            encoder.draw_primitives(PrimitiveType::Triangle, 0, 3);
            encoder.end_encoding();

            command_buffer.present_drawable(&drawable);
            command_buffer.commit();

            frame += 1.0;
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

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("minmetal currently targets macOS because Metal is an Apple platform API.");
}
