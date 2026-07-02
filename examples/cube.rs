use miniwin::{Event, Key, Window, WindowStyle, create_window};
use minmetal::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Mat4 {
    columns: [[f32; 4]; 4],
}

impl Mat4 {
    fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            columns: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, s, 0.0],
                [0.0, -s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            columns: [
                [c, 0.0, -s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut out = [[0.0; 4]; 4];
        for col in 0..4 {
            for row in 0..4 {
                out[col][row] = self.columns[0][row] * other.columns[col][0]
                    + self.columns[1][row] * other.columns[col][1]
                    + self.columns[2][row] * other.columns[col][2]
                    + self.columns[3][row] * other.columns[col][3];
            }
        }
        Self { columns: out }
    }

    fn perspective(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let g = 1.0 / (fov_y_rad * 0.5).tan();
        let range_inv = 1.0 / (near - far);
        Self {
            columns: [
                [g / aspect, 0.0, 0.0, 0.0],
                [0.0, g, 0.0, 0.0],
                [0.0, 0.0, far * range_inv, -1.0],
                [0.0, 0.0, near * far * range_inv, 0.0],
            ],
        }
    }

    fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            columns: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Uniforms {
    mvp: Mat4,
    model: Mat4,
    light_dir: [f32; 4],
    light_color: [f32; 4],
}

const SHADERS: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Vertex {
    packed_float3 position;
    packed_float3 normal;
    float2 uv;
};

struct Uniforms {
    float4x4 mvp;
    float4x4 model;
    float4 light_dir;
    float4 light_color;
};

struct VertexOut {
    float4 position [[position]];
    float3 normal;
    float2 uv;
};

vertex VertexOut vertex_main(device const Vertex* vertices [[buffer(0)]],
                            constant Uniforms& uniforms [[buffer(1)]],
                            uint vid [[vertex_id]]) {
    VertexOut out;
    float3 pos = vertices[vid].position;
    out.position = uniforms.mvp * float4(pos, 1.0);
    out.normal = (uniforms.model * float4(vertices[vid].normal, 0.0)).xyz;
    out.uv = vertices[vid].uv;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]],
                              constant Uniforms& uniforms [[buffer(1)]],
                              texture2d<float> tex [[texture(0)]]) {
    constexpr sampler s(mip_filter::linear, mag_filter::linear, min_filter::linear);
    float4 tex_color = tex.sample(s, in.uv);

    float3 N = normalize(in.normal);
    float3 L = normalize(uniforms.light_dir.xyz);
    float diffuse = max(dot(N, L), 0.0);
    float ambient = 0.2;

    float3 lit_color = tex_color.rgb * (diffuse * uniforms.light_color.rgb + ambient);
    return float4(lit_color, tex_color.a);
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = create_window("Metal 3D Cube Demo", None, 800, 600, WindowStyle::Standard);

    let device = Device::required_system_default()?;
    eprintln!("Using device: {}", device.name());

    let command_queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADERS)?;
    let vertex_func = library.function("vertex_main")?;
    let fragment_func = library.function("fragment_main")?;

    let pipeline_desc = RenderPipelineDescriptor::new();
    pipeline_desc.set_vertex_function(&vertex_func);
    pipeline_desc.set_fragment_function(&fragment_func);
    pipeline_desc.set_color_attachment_pixel_format(0, PixelFormat::Bgra8Unorm);
    pipeline_desc.set_depth_attachment_pixel_format(PixelFormat::Depth32Float);
    let pipeline = device.new_render_pipeline_state(&pipeline_desc)?;

    let depth_desc = DepthStencilDescriptor::new();
    depth_desc.set_depth_compare_function(CompareFunction::Less);
    depth_desc.set_depth_write_enabled(true);
    let depth_state = device.new_depth_stencil_state(&depth_desc)?;

    // 3D Cube Data: vertices and indices
    #[rustfmt::skip]
    let vertices = [
        // Front Face (normal Z+)
        Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 1.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 1.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 0.0] },
        Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 0.0] },
        // Back Face (normal Z-)
        Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 1.0] },
        Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 0.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 1.0] },
        // Top Face (normal Y+)
        Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 0.0] },
        Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 0.0] },
        // Bottom Face (normal Y-)
        Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 1.0] },
        Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 1.0] },
        // Right Face (normal X+)
        Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 0.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 1.0] },
        // Left Face (normal X-)
        Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [0.0, 1.0] },
        Vertex { position: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [1.0, 1.0] },
        Vertex { position: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [1.0, 0.0] },
        Vertex { position: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [0.0, 0.0] },
    ];

    #[rustfmt::skip]
    let indices: [u16; 36] = [
         0,  1,  2,  2,  3,  0, // front
         4,  5,  6,  6,  7,  4, // back
         8,  9, 10, 10, 11,  8, // top
        12, 13, 14, 14, 15, 12, // bottom
        16, 17, 18, 18, 19, 16, // right
        20, 21, 22, 22, 23, 20, // left
    ];

    let vertex_buffer = device.new_buffer(
        std::mem::size_of_val(&vertices),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    vertex_buffer.write_slice(&vertices);

    let index_buffer = device.new_buffer(
        std::mem::size_of_val(&indices),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    index_buffer.write_slice(&indices);

    let uniform_buffer = device.new_buffer(
        std::mem::size_of::<Uniforms>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    // Generate a Checkerboard Texture
    let texture_width = 256;
    let texture_height = 256;
    let mut texture_data = vec![0u8; texture_width * texture_height * 4];
    for y in 0..texture_height {
        for x in 0..texture_width {
            let offset = (y * texture_width + x) * 4;
            let check = ((x / 32) + (y / 32)) % 2 == 0;
            if check {
                texture_data[offset] = 255;     // B
                texture_data[offset + 1] = 255; // G
                texture_data[offset + 2] = 255; // R
                texture_data[offset + 3] = 255; // A
            } else {
                texture_data[offset] = 100;     // B
                texture_data[offset + 1] = 100; // G
                texture_data[offset + 2] = 200; // R
                texture_data[offset + 3] = 255; // A
            }
        }
    }

    let texture_desc = TextureDescriptor::texture_2d(
        PixelFormat::Bgra8Unorm,
        texture_width,
        texture_height,
        false,
    );
    let checker_texture = device.new_texture(&texture_desc)?;
    checker_texture.replace_region(
        Region::new_2d(0, 0, texture_width, texture_height),
        0,
        &texture_data,
        texture_width * 4,
    );

    let scale = window.scale_factor();
    let (width, height) = window.content_size();
    let mut drawable_width = (width as f64 * scale).max(1.0) as usize;
    let mut drawable_height = (height as f64 * scale).max(1.0) as usize;

    let layer = unsafe {
        MetalLayer::attach_to_view(
            window.ns_view,
            &device,
            PixelFormat::Bgra8Unorm,
            drawable_width,
            drawable_height,
            scale,
        )?
    };

    // Prepare depth texture descriptor
    let depth_desc = TextureDescriptor::texture_2d(
        PixelFormat::Depth32Float,
        drawable_width,
        drawable_height,
        false,
    );
    depth_desc.set_storage_mode(StorageMode::Private);
    depth_desc.set_usage(TextureUsage::RENDER_TARGET);
    let mut depth_texture = device.new_texture(&depth_desc)?;

    let mut angle = 0.0f32;
    let mut running = true;

    while running {
        let _pool = AutoreleasePool::new();

        window.draw(|win| {
            let win_scale = win.scale_factor();
            let (win_width, win_height) = win.content_size();
            let current_draw_w = (win_width as f64 * win_scale).max(1.0) as usize;
            let current_draw_h = (win_height as f64 * win_scale).max(1.0) as usize;

            // Recreate depth texture on window resize
            if current_draw_w != drawable_width || current_draw_h != drawable_height {
                drawable_width = current_draw_w;
                drawable_height = current_draw_h;

                let new_depth_desc = TextureDescriptor::texture_2d(
                    PixelFormat::Depth32Float,
                    drawable_width,
                    drawable_height,
                    false,
                );
                new_depth_desc.set_storage_mode(StorageMode::Private);
                new_depth_desc.set_usage(TextureUsage::RENDER_TARGET);
                if let Ok(tex) = device.new_texture(&new_depth_desc) {
                    depth_texture = tex;
                }
            }

            layer.set_contents_scale(win_scale);
            layer.set_drawable_size(drawable_width, drawable_height);

            let Some(drawable) = layer.next_drawable() else {
                return;
            };

            let aspect = drawable_width as f32 / drawable_height as f32;
            let proj = Mat4::perspective(60.0f32.to_radians(), aspect, 0.1, 10.0);
            let view = Mat4::translation(0.0, 0.0, -4.0);

            // Compute spinning rotation matrix
            let rot_x = Mat4::rotation_x(angle);
            let rot_y = Mat4::rotation_y(angle * 0.5);
            let model = rot_x.mul(&rot_y);

            let mvp = proj.mul(&view).mul(&model);

            let uniforms = Uniforms {
                mvp,
                model,
                light_dir: [1.0, 1.0, 1.0, 0.0],
                light_color: [0.9, 0.9, 1.0, 1.0],
            };
            uniform_buffer.write(&uniforms);

            let Ok(texture) = drawable.texture() else {
                return;
            };

            let pass = RenderPassDescriptor::new();
            pass.set_color_attachment(
                0,
                &texture,
                LoadAction::Clear,
                StoreAction::Store,
                ClearColor::new(0.05, 0.05, 0.07, 1.0),
            );
            pass.set_depth_attachment(
                &depth_texture,
                LoadAction::Clear,
                StoreAction::DontCare,
                1.0,
            );

            let Ok(command_buffer) = command_queue.command_buffer() else {
                return;
            };
            let Ok(encoder) = command_buffer.render_command_encoder(&pass) else {
                return;
            };

            encoder.set_render_pipeline_state(&pipeline);
            encoder.set_depth_stencil_state(&depth_state);

            encoder.set_vertex_buffer(0, &vertex_buffer, 0);
            encoder.set_vertex_buffer(1, &uniform_buffer, 0);
            encoder.set_fragment_buffer(1, &uniform_buffer, 0);
            encoder.set_fragment_texture(0, &checker_texture);

            encoder.draw_indexed_primitives(
                PrimitiveType::Triangle,
                36,
                IndexType::UInt16,
                &index_buffer,
                0,
            );

            encoder.end_encoding();
            command_buffer.present_drawable(&drawable);
            command_buffer.commit();

            angle += 0.02;
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
