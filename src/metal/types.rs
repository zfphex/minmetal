#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Origin {
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Region {
    pub origin: Origin,
    pub size: Size,
}

impl Region {
    pub const fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: Origin::new(x, y, 0),
            size: Size::new(width, height, 1),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport {
    pub origin_x: f64,
    pub origin_y: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

impl Viewport {
    pub const fn new(
        origin_x: f64,
        origin_y: f64,
        width: f64,
        height: f64,
        znear: f64,
        zfar: f64,
    ) -> Self {
        Self {
            origin_x,
            origin_y,
            width,
            height,
            znear,
            zfar,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl ScissorRect {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Range {
    pub location: usize,
    pub length: usize,
}

impl Range {
    pub const fn new(location: usize, length: usize) -> Self {
        Self { location, length }
    }
}

impl ClearColor {
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CGSize {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PixelFormat {
    Invalid = 0,
    R8Unorm = 10,
    R8Uint = 13,
    R16Float = 25,
    Rg8Unorm = 30,
    Rg16Float = 65,
    Rgba8Unorm = 70,
    Rgba8UnormSrgb = 71,
    Bgra8Unorm = 80,
    Bgra8UnormSrgb = 81,
    Rgb10A2Unorm = 90,
    Rg11B10Float = 92,
    Rgb9E5Float = 93,
    Rgba16Float = 115,
    Rgba32Float = 125,
    Bc1Rgba = 130,
    Bc1RgbaSrgb = 131,
    Bc2Rgba = 132,
    Bc2RgbaSrgb = 133,
    Bc3Rgba = 134,
    Bc3RgbaSrgb = 135,
    Bc4RUnorm = 140,
    Bc5RgUnorm = 142,
    Bc6HRgbFloat = 150,
    Bc7RgbaUnorm = 152,
    Bc7RgbaUnormSrgb = 153,
    Depth16Unorm = 250,
    Depth32Float = 252,
    Stencil8 = 253,
    Depth24UnormStencil8 = 255,
    Depth32FloatStencil8 = 260,
}

impl PixelFormat {
    pub(crate) const fn as_raw(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StorageMode {
    Shared = 0,
    Managed = 1,
    Private = 2,
}

impl StorageMode {
    const fn as_resource_bits(self) -> usize {
        (self as usize) << 4
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CpuCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum HazardTrackingMode {
    Default = 0,
    Untracked = 1,
    Tracked = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceOptions(usize);

impl ResourceOptions {
    pub const CPU_CACHE_MODE_DEFAULT: Self = Self(0);
    pub const CPU_CACHE_MODE_WRITE_COMBINED: Self = Self(1);
    pub const STORAGE_MODE_SHARED: Self = Self(StorageMode::Shared.as_resource_bits());
    pub const STORAGE_MODE_MANAGED: Self = Self(StorageMode::Managed.as_resource_bits());
    pub const STORAGE_MODE_PRIVATE: Self = Self(StorageMode::Private.as_resource_bits());
    pub const HAZARD_TRACKING_MODE_UNTRACKED: Self = Self(1 << 8);
    pub const HAZARD_TRACKING_MODE_TRACKED: Self = Self(2 << 8);

    pub const fn new(
        cpu_cache_mode: CpuCacheMode,
        storage_mode: StorageMode,
        hazard_tracking_mode: HazardTrackingMode,
    ) -> Self {
        Self(
            cpu_cache_mode as usize
                | ((storage_mode as usize) << 4)
                | ((hazard_tracking_mode as usize) << 8),
        )
    }

    pub(crate) const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureUsage(usize);

impl TextureUsage {
    pub const UNKNOWN: Self = Self(0);
    pub const SHADER_READ: Self = Self(1);
    pub const SHADER_WRITE: Self = Self(1 << 1);
    pub const RENDER_TARGET: Self = Self(1 << 2);
    pub const PIXEL_FORMAT_VIEW: Self = Self(1 << 4);

    pub(crate) const fn as_raw(self) -> usize {
        self.0
    }
}

impl std::ops::BitOr for TextureUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TextureType {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D2Multisample = 4,
    Cube = 5,
    CubeArray = 6,
    D3 = 7,
    D2MultisampleArray = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum BlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorWriteMask(usize);

impl ColorWriteMask {
    pub const NONE: Self = Self(0);
    pub const RED: Self = Self(1 << 3);
    pub const GREEN: Self = Self(1 << 2);
    pub const BLUE: Self = Self(1 << 1);
    pub const ALPHA: Self = Self(1);
    pub const ALL: Self = Self(0xf);

    pub(crate) const fn as_raw(self) -> usize {
        self.0
    }
}

impl std::ops::BitOr for ColorWriteMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum VertexFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,
    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,
    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,
    Half = 40,
    Half2 = 41,
    Half3 = 42,
    Half4 = 43,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum VertexStepFunction {
    Constant = 0,
    PerVertex = 1,
    PerInstance = 2,
    PerPatch = 3,
    PerPatchControlPoint = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum IndexType {
    UInt16 = 0,
    UInt32 = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SamplerMipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Winding {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TriangleFillMode {
    Fill = 0,
    Lines = 1,
}

#[derive(Debug, Clone)]
pub struct MetalError {
    message: String,
}

impl MetalError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MetalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MetalError {}
