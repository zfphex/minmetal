use miniwin::*;
use minmetal::*;

const GRID_SIZE: usize = 32;
const PROPAGATION_PASSES: usize = 6;
const INJECT_RADIUS: f32 = 1.25;

const VOLUME_MIN: [f32; 3] = [-4.5, -0.5, -4.5];
const VOLUME_MAX: [f32; 3] = [4.5, 6.5, 4.5];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Mat4 {
    columns: [[f32; 4]; 4],
}

impl Mat4 {
    fn identity() -> Self {
        Self {
            columns: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
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
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    albedo: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct SurfaceSample {
    position: [f32; 4],
    normal: [f32; 4],
    albedo: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct LpvUniforms {
    volume_min: [f32; 4],
    volume_max: [f32; 4],
    light_pos: [f32; 4],
    light_color: [f32; 4],
    grid_size: u32,
    sample_count: u32,
    inject_radius: f32,
    _pad: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct SceneUniforms {
    mvp: Mat4,
    model: Mat4,
    volume_min: [f32; 4],
    volume_max: [f32; 4],
    light_pos: [f32; 4],
    light_color: [f32; 4],
    ambient_color: [f32; 4],
    debug_mode: u32,
    lpv_strength: f32,
    lpv_visual_scale: f32,
    _pad: f32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DebugMode {
    Full = 0,
    DirectOnly = 1,
    BounceOnly = 2,
    AmbientOnly = 3,
    Albedo = 4,
    InjectedLpv = 5,
    PropagatedLpvRaw = 6,
    DirectAmbient = 7,
}

impl DebugMode {
    fn from_digit(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::Full),
            '2' => Some(Self::DirectOnly),
            '3' => Some(Self::BounceOnly),
            '4' => Some(Self::AmbientOnly),
            '5' => Some(Self::Albedo),
            '6' => Some(Self::InjectedLpv),
            '7' => Some(Self::PropagatedLpvRaw),
            '8' => Some(Self::DirectAmbient),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Full => "1: full (direct + bounce + ambient)",
            Self::DirectOnly => "2: direct lighting only",
            Self::BounceOnly => "3: LPV bounce/indirect only",
            Self::AmbientOnly => "4: ambient only",
            Self::Albedo => "5: albedo only",
            Self::InjectedLpv => "6: injected LPV volume (pre-propagation)",
            Self::PropagatedLpvRaw => "7: propagated LPV volume (raw radiance)",
            Self::DirectAmbient => "8: direct + ambient (no LPV)",
        }
    }
}

struct MeshData {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

struct MeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl MeshBuilder {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn push_quad(
        &mut self,
        p0: [f32; 3],
        p1: [f32; 3],
        p2: [f32; 3],
        p3: [f32; 3],
        normal: [f32; 3],
        albedo: [f32; 3],
    ) {
        let base = self.vertices.len() as u16;
        self.vertices.push(Vertex {
            position: p0,
            normal,
            albedo,
        });
        self.vertices.push(Vertex {
            position: p1,
            normal,
            albedo,
        });
        self.vertices.push(Vertex {
            position: p2,
            normal,
            albedo,
        });
        self.vertices.push(Vertex {
            position: p3,
            normal,
            albedo,
        });
        self.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    fn push_box(&mut self, center: [f32; 3], half: [f32; 3], albedo: [f32; 3]) {
        let cx = center[0];
        let cy = center[1];
        let cz = center[2];
        let hx = half[0];
        let hy = half[1];
        let hz = half[2];

        let faces = [
            (
                [0.0, 0.0, 1.0],
                [
                    [cx - hx, cy - hy, cz + hz],
                    [cx + hx, cy - hy, cz + hz],
                    [cx + hx, cy + hy, cz + hz],
                    [cx - hx, cy + hy, cz + hz],
                ],
            ),
            (
                [0.0, 0.0, -1.0],
                [
                    [cx + hx, cy - hy, cz - hz],
                    [cx - hx, cy - hy, cz - hz],
                    [cx - hx, cy + hy, cz - hz],
                    [cx + hx, cy + hy, cz - hz],
                ],
            ),
            (
                [0.0, 1.0, 0.0],
                [
                    [cx - hx, cy + hy, cz - hz],
                    [cx - hx, cy + hy, cz + hz],
                    [cx + hx, cy + hy, cz + hz],
                    [cx + hx, cy + hy, cz - hz],
                ],
            ),
            (
                [0.0, -1.0, 0.0],
                [
                    [cx - hx, cy - hy, cz + hz],
                    [cx - hx, cy - hy, cz - hz],
                    [cx + hx, cy - hy, cz - hz],
                    [cx + hx, cy - hy, cz + hz],
                ],
            ),
            (
                [1.0, 0.0, 0.0],
                [
                    [cx + hx, cy - hy, cz + hz],
                    [cx + hx, cy - hy, cz - hz],
                    [cx + hx, cy + hy, cz - hz],
                    [cx + hx, cy + hy, cz + hz],
                ],
            ),
            (
                [-1.0, 0.0, 0.0],
                [
                    [cx - hx, cy - hy, cz - hz],
                    [cx - hx, cy - hy, cz + hz],
                    [cx - hx, cy + hy, cz + hz],
                    [cx - hx, cy + hy, cz - hz],
                ],
            ),
        ];

        for (normal, corners) in faces {
            self.push_quad(
                corners[0], corners[1], corners[2], corners[3], normal, albedo,
            );
        }
    }

    fn finish(self) -> MeshData {
        MeshData {
            vertices: self.vertices,
            indices: self.indices,
        }
    }
}

fn build_test_scene() -> MeshData {
    let mut builder = MeshBuilder::new();
    let room_half = 4.0;
    let wall_height = 6.0;

    // Floor
    builder.push_quad(
        [-room_half, 0.0, -room_half],
        [room_half, 0.0, -room_half],
        [room_half, 0.0, room_half],
        [-room_half, 0.0, room_half],
        [0.0, 1.0, 0.0],
        [0.75, 0.75, 0.75],
    );

    // Back wall (warm red)
    builder.push_quad(
        [-room_half, 0.0, -room_half],
        [room_half, 0.0, -room_half],
        [room_half, wall_height, -room_half],
        [-room_half, wall_height, -room_half],
        [0.0, 0.0, 1.0],
        [0.85, 0.25, 0.2],
    );

    // Left wall (green)
    builder.push_quad(
        [-room_half, 0.0, room_half],
        [-room_half, 0.0, -room_half],
        [-room_half, wall_height, -room_half],
        [-room_half, wall_height, room_half],
        [1.0, 0.0, 0.0],
        [0.2, 0.75, 0.3],
    );

    // Right wall (neutral)
    builder.push_quad(
        [room_half, 0.0, -room_half],
        [room_half, 0.0, room_half],
        [room_half, wall_height, room_half],
        [room_half, wall_height, -room_half],
        [-1.0, 0.0, 0.0],
        [0.7, 0.7, 0.72],
    );

    // Ceiling (slightly dark to emphasize LPV bounce)
    builder.push_quad(
        [-room_half, wall_height, room_half],
        [room_half, wall_height, room_half],
        [room_half, wall_height, -room_half],
        [-room_half, wall_height, -room_half],
        [0.0, -1.0, 0.0],
        [0.55, 0.55, 0.58],
    );

    // Occluder blocks
    builder.push_box([-1.2, 0.6, -0.8], [0.6, 0.6, 0.6], [0.92, 0.92, 0.9]);
    builder.push_box([1.6, 0.85, 0.9], [0.85, 0.85, 0.85], [0.95, 0.85, 0.35]);
    builder.push_box([-2.0, 0.35, 2.0], [0.35, 0.35, 0.35], [0.35, 0.75, 0.9]);
    builder.push_box([0.2, 0.25, 2.3], [1.2, 0.25, 0.25], [0.8, 0.45, 0.25]);

    builder.finish()
}

fn build_surface_samples(mesh: &MeshData) -> Vec<SurfaceSample> {
    let mut samples = Vec::with_capacity(mesh.indices.len() / 3);
    for tri in mesh.indices.chunks(3) {
        let v0 = &mesh.vertices[tri[0] as usize];
        let v1 = &mesh.vertices[tri[1] as usize];
        let v2 = &mesh.vertices[tri[2] as usize];

        let position = [
            (v0.position[0] + v1.position[0] + v2.position[0]) / 3.0,
            (v0.position[1] + v1.position[1] + v2.position[1]) / 3.0,
            (v0.position[2] + v1.position[2] + v2.position[2]) / 3.0,
            1.0,
        ];
        let normal = normalize3([
            (v0.normal[0] + v1.normal[0] + v2.normal[0]) / 3.0,
            (v0.normal[1] + v1.normal[1] + v2.normal[1]) / 3.0,
            (v0.normal[2] + v1.normal[2] + v2.normal[2]) / 3.0,
        ]);
        let albedo = [
            (v0.albedo[0] + v1.albedo[0] + v2.albedo[0]) / 3.0,
            (v0.albedo[1] + v1.albedo[1] + v2.albedo[1]) / 3.0,
            (v0.albedo[2] + v1.albedo[2] + v2.albedo[2]) / 3.0,
            1.0,
        ];

        samples.push(SurfaceSample {
            position,
            normal: [normal[0], normal[1], normal[2], 0.0],
            albedo,
        });
    }
    samples
}

fn create_lpv_texture(device: &Device, grid: usize) -> Result<Texture, MetalError> {
    let desc = TextureDescriptor::new();
    desc.set_texture_type(TextureType::D3);
    desc.set_pixel_format(PixelFormat::Rgba16Float);
    desc.set_width(grid);
    desc.set_height(grid);
    desc.set_depth(grid);
    desc.set_storage_mode(StorageMode::Private);
    desc.set_usage(TextureUsage::SHADER_READ | TextureUsage::SHADER_WRITE);
    device.new_texture(&desc)
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-6 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [0.0, 1.0, 0.0]
    }
}

const SHADERS: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Vertex {
    packed_float3 position;
    packed_float3 normal;
    packed_float3 albedo;
};

struct SurfaceSample {
    float4 position;
    float4 normal;
    float4 albedo;
};

struct LpvUniforms {
    float4 volume_min;
    float4 volume_max;
    float4 light_pos;
    float4 light_color;
    uint grid_size;
    uint sample_count;
    float inject_radius;
    float _pad;
};

struct SceneUniforms {
    float4x4 mvp;
    float4x4 model;
    float4 volume_min;
    float4 volume_max;
    float4 light_pos;
    float4 light_color;
    float4 ambient_color;
    uint debug_mode;
    float lpv_strength;
    float lpv_visual_scale;
    float _pad;
};

constant uint DEBUG_FULL = 0u;
constant uint DEBUG_DIRECT = 1u;
constant uint DEBUG_BOUNCE = 2u;
constant uint DEBUG_AMBIENT = 3u;
constant uint DEBUG_ALBEDO = 4u;
constant uint DEBUG_INJECTED = 5u;
constant uint DEBUG_PROPAGATED_RAW = 6u;
constant uint DEBUG_DIRECT_AMBIENT = 7u;

struct VertexOut {
    float4 position [[position]];
    float3 world_pos;
    float3 normal;
    float3 albedo;
};

inline float3 world_from_grid(uint3 gid, constant LpvUniforms& u) {
    float3 extent = u.volume_max.xyz - u.volume_min.xyz;
    float grid = float(u.grid_size);
    return u.volume_min.xyz + ((float3(gid) + 0.5) / grid) * extent;
}

inline float3 grid_coord(float3 world_pos, constant LpvUniforms& u) {
    float3 extent = u.volume_max.xyz - u.volume_min.xyz;
    float3 t = (world_pos - u.volume_min.xyz) / extent;
    return clamp(t, 0.0, 1.0);
}

kernel void clear_lpv(texture3d<half, access::write> volume [[texture(0)]],
                      constant LpvUniforms& u [[buffer(0)]],
                      uint3 gid [[thread_position_in_grid]]) {
    if (any(gid >= uint3(u.grid_size))) {
        return;
    }
    volume.write(half4(0.0h), gid);
}

kernel void inject_lpv(device const SurfaceSample* samples [[buffer(0)]],
                       constant LpvUniforms& u [[buffer(1)]],
                       texture3d<half, access::write> volume [[texture(0)]],
                       uint3 gid [[thread_position_in_grid]]) {
    if (any(gid >= uint3(u.grid_size))) {
        return;
    }

    float3 cell_pos = world_from_grid(gid, u);
    float3 accum = float3(0.0);
    float radius_sq = u.inject_radius * u.inject_radius;

    for (uint i = 0; i < u.sample_count; i++) {
        SurfaceSample s = samples[i];
        float3 surf_pos = s.position.xyz;
        float3 delta = surf_pos - cell_pos;
        float dist_sq = dot(delta, delta);
        if (dist_sq > radius_sq) {
            continue;
        }

        float3 N = normalize(s.normal.xyz);
        float3 L = normalize(u.light_pos.xyz - surf_pos);
        float diffuse = max(dot(N, L), 0.0);
        float3 radiance = s.albedo.xyz * diffuse * u.light_color.rgb * u.light_color.a;

        float w = exp(-dist_sq / max(radius_sq * 0.35, 1e-4));
        accum += radiance * w;
    }

    volume.write(half4(half3(accum), 1.0h), gid);
}

kernel void propagate_lpv(texture3d<half, access::read> src [[texture(0)]],
                          texture3d<half, access::write> dst [[texture(1)]],
                          constant LpvUniforms& u [[buffer(0)]],
                          uint3 gid [[thread_position_in_grid]]) {
    if (any(gid >= uint3(u.grid_size))) {
        return;
    }

    float3 center = float3(src.read(gid).rgb);
    float3 accum = center * 0.35;

    const int3 offsets[6] = {
        int3( 1,  0,  0), int3(-1,  0,  0),
        int3( 0,  1,  0), int3( 0, -1,  0),
        int3( 0,  0,  1), int3( 0,  0, -1),
    };

    int grid = int(u.grid_size);
    for (int i = 0; i < 6; i++) {
        int3 neighbor = int3(gid) + offsets[i];
        if (neighbor.x < 0 || neighbor.y < 0 || neighbor.z < 0) {
            continue;
        }
        if (neighbor.x >= grid || neighbor.y >= grid || neighbor.z >= grid) {
            continue;
        }
        accum += float3(src.read(uint3(neighbor)).rgb) * 0.11;
    }

    dst.write(half4(half3(accum), 1.0h), gid);
}

vertex VertexOut vertex_main(device const Vertex* vertices [[buffer(0)]],
                             constant SceneUniforms& uniforms [[buffer(1)]],
                             uint vid [[vertex_id]]) {
    Vertex v = vertices[vid];
    VertexOut out;
    float3 pos = v.position;
    out.world_pos = pos;
    out.position = uniforms.mvp * float4(pos, 1.0);
    out.normal = (uniforms.model * float4(v.normal, 0.0)).xyz;
    out.albedo = v.albedo;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]],
                              constant SceneUniforms& uniforms [[buffer(1)]],
                              texture3d<half> lpv [[texture(0)]],
                              sampler lpv_sampler [[sampler(0)]]) {
    float3 N = normalize(in.normal);
    float3 L = normalize(uniforms.light_pos.xyz - in.world_pos);
    float direct = max(dot(N, L), 0.0);
    float3 direct_color = in.albedo * direct * uniforms.light_color.rgb;
    float3 ambient_lit = in.albedo * uniforms.ambient_color.rgb * uniforms.ambient_color.a;

    float3 extent = uniforms.volume_max.xyz - uniforms.volume_min.xyz;
    float3 uvw = (in.world_pos - uniforms.volume_min.xyz) / extent;
    uvw = clamp(uvw, 0.001, 0.999);
    float3 lpv_radiance = float3(lpv.sample(lpv_sampler, uvw).rgb);
    float3 bounce_color = in.albedo * lpv_radiance * uniforms.lpv_strength;
    float3 lpv_debug = lpv_radiance * uniforms.lpv_strength * uniforms.lpv_visual_scale;

    switch (uniforms.debug_mode) {
        case DEBUG_DIRECT:
            return float4(direct_color, 1.0);
        case DEBUG_BOUNCE:
            return float4(bounce_color, 1.0);
        case DEBUG_AMBIENT:
            return float4(ambient_lit, 1.0);
        case DEBUG_ALBEDO:
            return float4(in.albedo, 1.0);
        case DEBUG_INJECTED:
            return float4(lpv_debug, 1.0);
        case DEBUG_PROPAGATED_RAW:
            return float4(lpv_debug, 1.0);
        case DEBUG_DIRECT_AMBIENT:
            return float4(direct_color + ambient_lit, 1.0);
        default:
            return float4(direct_color + bounce_color + ambient_lit, 1.0);
    }
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = create_window(
        "Light Propagation Volumes Demo",
        None,
        1024,
        768,
        true,
        WindowStyle::Standard,
    );

    let device = Device::required_system_default()?;
    eprintln!("Using device: {}", device.name());

    let command_queue = device.new_command_queue()?;
    let library = device.new_library_with_source(SHADERS)?;

    let clear_lpv_fn = library.function("clear_lpv")?;
    let inject_lpv_fn = library.function("inject_lpv")?;
    let propagate_lpv_fn = library.function("propagate_lpv")?;
    let vertex_fn = library.function("vertex_main")?;
    let fragment_fn = library.function("fragment_main")?;

    let clear_pipeline = device.new_compute_pipeline_state_with_function(&clear_lpv_fn)?;
    let inject_pipeline = device.new_compute_pipeline_state_with_function(&inject_lpv_fn)?;
    let propagate_pipeline = device.new_compute_pipeline_state_with_function(&propagate_lpv_fn)?;

    let render_pipeline_desc = RenderPipelineDescriptor::new();
    render_pipeline_desc.set_vertex_function(&vertex_fn);
    render_pipeline_desc.set_fragment_function(&fragment_fn);
    render_pipeline_desc.set_color_attachment_pixel_format(0, PixelFormat::Bgra8Unorm);
    render_pipeline_desc.set_depth_attachment_pixel_format(PixelFormat::Depth32Float);
    let render_pipeline = device.new_render_pipeline_state(&render_pipeline_desc)?;

    let depth_stencil_desc = DepthStencilDescriptor::new();
    depth_stencil_desc.set_depth_compare_function(CompareFunction::Less);
    depth_stencil_desc.set_depth_write_enabled(true);
    let depth_state = device.new_depth_stencil_state(&depth_stencil_desc)?;

    let sampler_desc = SamplerDescriptor::new();
    sampler_desc.set_min_filter(SamplerMinMagFilter::Linear);
    sampler_desc.set_mag_filter(SamplerMinMagFilter::Linear);
    sampler_desc.set_mip_filter(SamplerMipFilter::NotMipmapped);
    sampler_desc.set_address_mode(SamplerAddressMode::ClampToEdge);
    let lpv_sampler = device.new_sampler_state(&sampler_desc)?;

    let scene = build_test_scene();
    let surface_samples = build_surface_samples(&scene);

    let vertex_buffer = device.new_buffer(
        scene.vertices.len() * std::mem::size_of::<Vertex>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    vertex_buffer.write_slice(&scene.vertices);

    let index_buffer = device.new_buffer(
        scene.indices.len() * std::mem::size_of::<u16>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    index_buffer.write_slice(&scene.indices);

    let surface_buffer = device.new_buffer(
        surface_samples.len() * std::mem::size_of::<SurfaceSample>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    surface_buffer.write_slice(&surface_samples);

    let lpv_uniform_buffer = device.new_buffer(
        std::mem::size_of::<LpvUniforms>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;
    let scene_uniform_buffer = device.new_buffer(
        std::mem::size_of::<SceneUniforms>(),
        ResourceOptions::STORAGE_MODE_SHARED,
    )?;

    let lpv_a = create_lpv_texture(&device, GRID_SIZE)?;
    let lpv_b = create_lpv_texture(&device, GRID_SIZE)?;
    let lpv_injected = create_lpv_texture(&device, GRID_SIZE)?;

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

    let depth_texture_desc = TextureDescriptor::texture_2d(
        PixelFormat::Depth32Float,
        drawable_width,
        drawable_height,
        false,
    );
    depth_texture_desc.set_storage_mode(StorageMode::Private);
    depth_texture_desc.set_usage(TextureUsage::RENDER_TARGET);
    let mut depth_texture = device.new_texture(&depth_texture_desc)?;

    let threadgroup = Size::new(4, 4, 4);
    let grid_threads = Size::new(GRID_SIZE, GRID_SIZE, GRID_SIZE);

    let mut time = 0.0f32;
    let mut debug_mode = DebugMode::Full;
    eprintln!("Debug views (number keys):");
    for mode in [
        DebugMode::Full,
        DebugMode::DirectOnly,
        DebugMode::BounceOnly,
        DebugMode::AmbientOnly,
        DebugMode::Albedo,
        DebugMode::InjectedLpv,
        DebugMode::PropagatedLpvRaw,
        DebugMode::DirectAmbient,
    ] {
        eprintln!("  {}", mode.label());
    }

    while window.open() {
        let _pool = AutoreleasePool::new();

        if window.pressed(Key::Escape) {
            window.close();
        }
        for digit in '1'..='8' {
            if window.pressed(Key::Char(digit)) {
                let new_mode = DebugMode::from_digit(digit).unwrap();
                if new_mode != debug_mode {
                    debug_mode = new_mode;
                    eprintln!("Debug mode: {}", debug_mode.label());
                }
            }
        }

        window.draw(|win| {
            let win_scale = win.scale_factor();
            let (win_width, win_height) = win.content_size();
            let current_draw_w = (win_width as f64 * win_scale).max(1.0) as usize;
            let current_draw_h = (win_height as f64 * win_scale).max(1.0) as usize;

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
            // Fixed camera outside the open front of the room, elevated and yawed to frame the blocks.
            let view = Mat4::translation(0.0, -2.4, -9.0).mul(&Mat4::rotation_y(0.3));
            let proj = Mat4::perspective(50.0f32.to_radians(), aspect, 0.1, 50.0);
            let model = Mat4::identity();
            let mvp = proj.mul(&view).mul(&model);

            let light_pos = [
                1.5 * (time * 0.7).sin(),
                5.2,
                -1.0 + 1.2 * (time * 0.5).cos(),
                1.0,
            ];
            let light_color = [1.0, 0.95, 0.85, 2.5];

            let lpv_uniforms = LpvUniforms {
                volume_min: [VOLUME_MIN[0], VOLUME_MIN[1], VOLUME_MIN[2], 0.0],
                volume_max: [VOLUME_MAX[0], VOLUME_MAX[1], VOLUME_MAX[2], 0.0],
                light_pos,
                light_color,
                grid_size: GRID_SIZE as u32,
                sample_count: surface_samples.len() as u32,
                inject_radius: INJECT_RADIUS,
                _pad: 0.0,
            };
            lpv_uniform_buffer.write(&lpv_uniforms);

            let scene_uniforms = SceneUniforms {
                mvp,
                model,
                volume_min: lpv_uniforms.volume_min,
                volume_max: lpv_uniforms.volume_max,
                light_pos,
                light_color,
                ambient_color: [0.55, 0.58, 0.65, 0.22],
                debug_mode: debug_mode as u32,
                lpv_strength: 1.35,
                lpv_visual_scale: 2.0,
                _pad: 0.0,
            };
            scene_uniform_buffer.write(&scene_uniforms);

            let Ok(command_buffer) = command_queue.command_buffer() else {
                return;
            };

            if let Ok(compute_encoder) = command_buffer.compute_command_encoder() {
                compute_encoder.set_compute_pipeline_state(&clear_pipeline);
                compute_encoder.set_bytes(0, &lpv_uniforms);
                compute_encoder.set_texture(0, &lpv_a);
                compute_encoder.dispatch_threads(grid_threads, threadgroup);

                compute_encoder.set_compute_pipeline_state(&inject_pipeline);
                compute_encoder.set_buffer(0, &surface_buffer, 0);
                compute_encoder.set_buffer(1, &lpv_uniform_buffer, 0);
                compute_encoder.set_texture(0, &lpv_a);
                compute_encoder.dispatch_threads(grid_threads, threadgroup);

                compute_encoder.end_encoding();
            }

            if let Ok(blit_encoder) = command_buffer.blit_command_encoder() {
                blit_encoder.copy_texture_to_texture(
                    &lpv_a,
                    Origin::new(0, 0, 0),
                    Size::new(GRID_SIZE, GRID_SIZE, GRID_SIZE),
                    &lpv_injected,
                    Origin::new(0, 0, 0),
                );
                blit_encoder.end_encoding();
            }

            if let Ok(compute_encoder) = command_buffer.compute_command_encoder() {
                let mut src = &lpv_a;
                let mut dst = &lpv_b;
                for _ in 0..PROPAGATION_PASSES {
                    compute_encoder.set_compute_pipeline_state(&propagate_pipeline);
                    compute_encoder.set_buffer(0, &lpv_uniform_buffer, 0);
                    compute_encoder.set_texture(0, src);
                    compute_encoder.set_texture(1, dst);
                    compute_encoder.dispatch_threads(grid_threads, threadgroup);
                    std::mem::swap(&mut src, &mut dst);
                }

                compute_encoder.end_encoding();
            }

            let final_lpv = if PROPAGATION_PASSES % 2 == 0 {
                &lpv_a
            } else {
                &lpv_b
            };

            let lpv_sample_texture = match debug_mode {
                DebugMode::InjectedLpv => &lpv_injected,
                _ => final_lpv,
            };

            let Ok(color_texture) = drawable.texture() else {
                return;
            };

            let pass = RenderPassDescriptor::new();
            pass.set_color_attachment(
                0,
                &color_texture,
                LoadAction::Clear,
                StoreAction::Store,
                ClearColor::new(0.03, 0.03, 0.05, 1.0),
            );
            pass.set_depth_attachment(
                &depth_texture,
                LoadAction::Clear,
                StoreAction::DontCare,
                1.0,
            );

            let Ok(render_encoder) = command_buffer.render_command_encoder(&pass) else {
                return;
            };

            render_encoder.set_render_pipeline_state(&render_pipeline);
            render_encoder.set_depth_stencil_state(&depth_state);
            render_encoder.set_vertex_buffer(0, &vertex_buffer, 0);
            render_encoder.set_vertex_buffer(1, &scene_uniform_buffer, 0);
            render_encoder.set_fragment_buffer(1, &scene_uniform_buffer, 0);
            render_encoder.set_fragment_texture(0, lpv_sample_texture);
            render_encoder.set_fragment_sampler_state(0, &lpv_sampler);

            render_encoder.draw_indexed_primitives(
                PrimitiveType::Triangle,
                scene.indices.len(),
                IndexType::UInt16,
                &index_buffer,
                0,
            );

            render_encoder.end_encoding();
            command_buffer.present_drawable(&drawable);
            command_buffer.commit();

            time += 0.012;
        });

        window.wait_for_vsync();
    }

    Ok(())
}
