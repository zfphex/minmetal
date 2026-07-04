#[repr(C)]
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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SizeAndAlign {
    pub size: usize,
    pub align: usize,
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
pub struct CGSize {
    pub width: f64,
    pub height: f64,
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
    pub const fn as_raw(self) -> usize {
        self as usize
    }

    pub fn from_raw(raw: usize) -> Self {
        match raw {
            10 => Self::R8Unorm,
            13 => Self::R8Uint,
            25 => Self::R16Float,
            30 => Self::Rg8Unorm,
            65 => Self::Rg16Float,
            70 => Self::Rgba8Unorm,
            71 => Self::Rgba8UnormSrgb,
            80 => Self::Bgra8Unorm,
            81 => Self::Bgra8UnormSrgb,
            90 => Self::Rgb10A2Unorm,
            92 => Self::Rg11B10Float,
            93 => Self::Rgb9E5Float,
            115 => Self::Rgba16Float,
            125 => Self::Rgba32Float,
            130 => Self::Bc1Rgba,
            131 => Self::Bc1RgbaSrgb,
            132 => Self::Bc2Rgba,
            133 => Self::Bc2RgbaSrgb,
            134 => Self::Bc3Rgba,
            135 => Self::Bc3RgbaSrgb,
            140 => Self::Bc4RUnorm,
            142 => Self::Bc5RgUnorm,
            150 => Self::Bc6HRgbFloat,
            152 => Self::Bc7RgbaUnorm,
            153 => Self::Bc7RgbaUnormSrgb,
            250 => Self::Depth16Unorm,
            252 => Self::Depth32Float,
            253 => Self::Stencil8,
            255 => Self::Depth24UnormStencil8,
            260 => Self::Depth32FloatStencil8,
            _ => Self::Invalid,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StorageMode {
    Shared = 0,
    Managed = 1,
    Private = 2,
    Memoryless = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum HeapType {
    Automatic = 0,
    Placement = 1,
    Sparse = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SparsePageSize {
    Size16 = 101,
    Size64 = 102,
    Size256 = 103,
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
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const CPU_CACHE_MODE_DEFAULT: Self = Self(0);
    pub const CPU_CACHE_MODE_WRITE_COMBINED: Self = Self(1);
    pub const STORAGE_MODE_SHARED: Self = Self(StorageMode::Shared.as_resource_bits());
    pub const STORAGE_MODE_MANAGED: Self = Self(StorageMode::Managed.as_resource_bits());
    pub const STORAGE_MODE_PRIVATE: Self = Self(StorageMode::Private.as_resource_bits());
    pub const STORAGE_MODE_MEMORYLESS: Self = Self(StorageMode::Memoryless.as_resource_bits());
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

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceUsage(usize);

impl ResourceUsage {
    pub const READ: Self = Self(1);
    pub const WRITE: Self = Self(2);
    pub const SAMPLE: Self = Self(4);

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

impl std::ops::BitOr for ResourceUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureUsage(usize);

impl TextureUsage {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const UNKNOWN: Self = Self(0);
    pub const SHADER_READ: Self = Self(1);
    pub const SHADER_WRITE: Self = Self(1 << 1);
    pub const RENDER_TARGET: Self = Self(1 << 2);
    pub const PIXEL_FORMAT_VIEW: Self = Self(1 << 4);
    pub const SHADER_ATOMIC: Self = Self(1 << 5);

    pub const fn as_raw(self) -> usize {
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

impl LoadAction {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::DontCare),
            1 => Some(Self::Load),
            2 => Some(Self::Clear),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
    StoreAndMultisampleResolve = 3,
    Unknown = 4,
    CustomSampleDepthStore = 5,
}

impl StoreAction {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::DontCare),
            1 => Some(Self::Store),
            2 => Some(Self::MultisampleResolve),
            3 => Some(Self::StoreAndMultisampleResolve),
            4 => Some(Self::Unknown),
            5 => Some(Self::CustomSampleDepthStore),
            _ => None,
        }
    }
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
    TextureBuffer = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TextureSwizzle {
    Zero = 0,
    One = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Alpha = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct TextureSwizzleChannels {
    pub red: TextureSwizzle,
    pub green: TextureSwizzle,
    pub blue: TextureSwizzle,
    pub alpha: TextureSwizzle,
}

impl Default for TextureSwizzleChannels {
    fn default() -> Self {
        Self {
            red: TextureSwizzle::Red,
            green: TextureSwizzle::Green,
            blue: TextureSwizzle::Blue,
            alpha: TextureSwizzle::Alpha,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TextureCompressionType {
    Lossless = 0,
    Lossy = 1,
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

    pub const fn as_raw(self) -> usize {
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
pub enum AttributeFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Char2 = 4,
    Char3 = 5,
    Char4 = 6,
    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,
    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,
    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,
    Short2 = 16,
    Short3 = 17,
    Short4 = 18,
    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,
    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,
    Half2 = 25,
    Half3 = 26,
    Half4 = 27,
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
    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,
    UChar4Normalized_BGRA = 42,
    UChar = 45,
    Char = 46,
    UCharNormalized = 47,
    CharNormalized = 48,
    UShort = 49,
    Short = 50,
    UShortNormalized = 51,
    ShortNormalized = 52,
    Half = 53,
    FloatRG11B10 = 54,
    FloatRGB9E5 = 55,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum StepFunction {
    Constant = 0,
    PerVertex = 1,
    PerInstance = 2,
    PerPatch = 3,
    PerPatchControlPoint = 4,
    ThreadPositionInGridX = 5,
    ThreadPositionInGridY = 6,
    ThreadPositionInGridXIndexed = 7,
    ThreadPositionInGridYIndexed = 8,
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
pub enum SamplerBorderColor {
    TransparentBlack = 0,
    OpaqueBlack = 1,
    OpaqueWhite = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SamplerReductionMode {
    WeightedAverage = 0,
    Minimum = 1,
    Maximum = 2,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum DataType {
    None = 0,
    Struct = 1,
    Array = 2,
    Float = 3,
    Float2 = 4,
    Float3 = 5,
    Float4 = 6,
    Float2x2 = 7,
    Float2x3 = 8,
    Float2x4 = 9,
    Float3x2 = 10,
    Float3x3 = 11,
    Float3x4 = 12,
    Float4x2 = 13,
    Float4x3 = 14,
    Float4x4 = 15,
    Half = 16,
    Half2 = 17,
    Half3 = 18,
    Half4 = 19,
    Half2x2 = 20,
    Half2x3 = 21,
    Half2x4 = 22,
    Half3x2 = 23,
    Half3x3 = 24,
    Half3x4 = 25,
    Half4x2 = 26,
    Half4x3 = 27,
    Half4x4 = 28,
    Int = 29,
    Int2 = 30,
    Int3 = 31,
    Int4 = 32,
    UInt = 33,
    UInt2 = 34,
    UInt3 = 35,
    UInt4 = 36,
    Short = 37,
    Short2 = 38,
    Short3 = 39,
    Short4 = 40,
    UShort = 41,
    UShort2 = 42,
    UShort3 = 43,
    UShort4 = 44,
    Char = 45,
    Char2 = 46,
    Char3 = 47,
    Char4 = 48,
    UChar = 49,
    UChar2 = 50,
    UChar3 = 51,
    UChar4 = 52,
    Bool = 53,
    Bool2 = 54,
    Bool3 = 55,
    Bool4 = 56,
    Texture = 58,
    Sampler = 59,
    Pointer = 60,
    R8Unorm = 62,
    R8Snorm = 63,
    R16Unorm = 64,
    R16Snorm = 65,
    RG8Unorm = 66,
    RG8Snorm = 67,
    RG16Unorm = 68,
    RG16Snorm = 69,
    RGBA8Unorm = 70,
    RGBA8Unorm_sRGB = 71,
    RGBA8Snorm = 72,
    RGBA16Unorm = 73,
    RGBA16Snorm = 74,
    RGB10A2Unorm = 75,
    RG11B10Float = 76,
    RGB9E5Float = 77,
    RenderPipeline = 78,
    ComputePipeline = 79,
    IndirectCommandBuffer = 80,
    Long = 81,
    Long2 = 82,
    Long3 = 83,
    Long4 = 84,
    ULong = 85,
    ULong2 = 86,
    ULong3 = 87,
    ULong4 = 88,
    VisibleFunctionTable = 115,
    IntersectionFunctionTable = 116,
    PrimitiveAccelerationStructure = 117,
    InstanceAccelerationStructure = 118,
    BFloat = 121,
    BFloat2 = 122,
    BFloat3 = 123,
    BFloat4 = 124,
    DepthStencilState = 139,
    Tensor = 140,
}

impl DataType {
    pub fn size(&self) -> Option<usize> {
        match self {
            DataType::Char | DataType::UChar | DataType::Bool => Some(1),
            DataType::Char2 | DataType::UChar2 | DataType::Bool2 => Some(2),
            DataType::Char3 | DataType::UChar3 | DataType::Bool3 => Some(4),
            DataType::Char4 | DataType::UChar4 | DataType::Bool4 => Some(4),
            
            DataType::Short | DataType::UShort | DataType::Half | DataType::BFloat => Some(2),
            DataType::Short2 | DataType::UShort2 | DataType::Half2 | DataType::BFloat2 => Some(4),
            DataType::Short3 | DataType::UShort3 | DataType::Half3 | DataType::BFloat3 => Some(8),
            DataType::Short4 | DataType::UShort4 | DataType::Half4 | DataType::BFloat4 => Some(8),
            
            DataType::Int | DataType::UInt | DataType::Float => Some(4),
            DataType::Int2 | DataType::UInt2 | DataType::Float2 => Some(8),
            DataType::Int3 | DataType::UInt3 | DataType::Float3 => Some(16),
            DataType::Int4 | DataType::UInt4 | DataType::Float4 => Some(16),
            
            DataType::Long | DataType::ULong => Some(8),
            DataType::Long2 | DataType::ULong2 => Some(16),
            DataType::Long3 | DataType::ULong3 => Some(32),
            DataType::Long4 | DataType::ULong4 => Some(32),
            
            DataType::Float2x2 | DataType::Half4x2 => Some(16),
            DataType::Float2x3 | DataType::Float2x4 => Some(32),
            DataType::Float3x2 => Some(24),
            DataType::Float3x3 | DataType::Float3x4 => Some(48),
            DataType::Float4x2 => Some(32),
            DataType::Float4x3 | DataType::Float4x4 => Some(64),
            
            DataType::Half2x2 => Some(8),
            DataType::Half2x3 | DataType::Half2x4 => Some(16),
            DataType::Half3x2 => Some(12),
            DataType::Half3x3 | DataType::Half3x4 => Some(24),
            DataType::Half4x3 | DataType::Half4x4 => Some(32),
            
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum FunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
    Visible = 5,
    Intersection = 6,
    Mesh = 7,
    Object = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum FunctionLogType {
    Validation = 0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum CaptureError {
    NotSupported = 1,
    AlreadyCapturing = 2,
    InvalidDescriptor = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ArgumentAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum BindingAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum BindingType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
    ImageblockData = 16,
    Imageblock = 17,
    VisibleFunctionTable = 24,
    PrimitiveAccelerationStructure = 25,
    InstanceAccelerationStructure = 26,
    IntersectionFunctionTable = 27,
    ObjectPayload = 34,
    Tensor = 37,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ArgumentType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
    ImageblockData = 16,
    Imageblock = 17,
    VisibleFunctionTable = 24,
    PrimitiveAccelerationStructure = 25,
    InstanceAccelerationStructure = 26,
    IntersectionFunctionTable = 27,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndirectCommandType(usize);

impl IndirectCommandType {
    pub const DRAW: Self = Self(1);
    pub const DRAW_INDEXED: Self = Self(1 << 1);
    pub const DRAW_PATCH: Self = Self(1 << 2);
    pub const DRAW_INDEXED_PATCH: Self = Self(1 << 3);
    pub const CONCURRENT_DISPATCH: Self = Self(1 << 5);
    pub const CONCURRENT_DISPATCH_THREADS: Self = Self(1 << 6);

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

impl std::ops::BitOr for IndirectCommandType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndirectCommandBufferOptions(usize);

impl IndirectCommandBufferOptions {
    pub const NONE: Self = Self(0);
    pub const STORAGE_MODE_PRIVATE: Self = Self(StorageMode::Private.as_resource_bits());

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionConstantValue {
    Bool(bool),
    U32(u32),
    I32(i32),
    F32(f32),
    Bytes {
        ptr: *const std::ffi::c_void,
        len: usize,
    },
}

#[derive(Debug, Clone)]
pub struct MetalError {
    message: String,
}

impl MetalError {
    pub fn new(message: impl Into<String>) -> Self {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

impl CommandBufferStatus {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::NotEnqueued),
            1 => Some(Self::Enqueued),
            2 => Some(Self::Committed),
            3 => Some(Self::Scheduled),
            4 => Some(Self::Completed),
            5 => Some(Self::Error),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    DeviceRemoved = 11,
    StackOverflow = 12,
}

impl CommandBufferError {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::Internal),
            2 => Some(Self::Timeout),
            3 => Some(Self::PageFault),
            4 => Some(Self::AccessRevoked),
            7 => Some(Self::NotPermitted),
            8 => Some(Self::OutOfMemory),
            9 => Some(Self::InvalidResource),
            10 => Some(Self::Memoryless),
            11 => Some(Self::DeviceRemoved),
            12 => Some(Self::StackOverflow),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum CommandBufferErrorOption {
    None = 0,
    EncoderExecutionStatus = 1 << 0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum CommandEncoderErrorState {
    Unknown = 0,
    Completed = 1,
    Affected = 2,
    Pending = 3,
    Faulted = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreActionOptions(pub usize);
impl StoreActionOptions {
    pub const NONE: Self = Self(0);
    pub const CUSTOM_SAMPLE_POSITIONS: Self = Self(1 << 0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum DispatchType {
    Serial = 0,
    Concurrent = 1,
}

impl DispatchType {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::Serial),
            1 => Some(Self::Concurrent),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionOptions(pub usize);
impl FunctionOptions {
    pub const NONE: Self = Self(0);
    pub const COMPILE_TO_BINARY: Self = Self(1 << 0);
    pub const STORE_FUNCTION_IN_METAL_PIPELINES_SCRIPT: Self = Self(1 << 1);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntersectionFunctionSignature(pub usize);
impl IntersectionFunctionSignature {
    pub const NONE: Self = Self(0);
    pub const TRIANGLE_DATA: Self = Self(1 << 1);
    pub const WORLD_SPACE_DATA: Self = Self(1 << 2);
    pub const INSTANCE_MOTION: Self = Self(1 << 3);
    pub const PRIMITIVE_MOTION: Self = Self(1 << 4);
    pub const EXTENDED_LIMITS: Self = Self(1 << 5);
    pub const MAX_LEVELS: Self = Self(1 << 6);
    pub const CURVE_DATA: Self = Self(1 << 7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum LogLevel {
    Undefined = -1,
    Debug = 0,
    Info = 1,
    Notice = 2,
    Error = 3,
    Fault = 4,
}

impl LogLevel {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            -1 => Some(Self::Undefined),
            0 => Some(Self::Debug),
            1 => Some(Self::Info),
            2 => Some(Self::Notice),
            3 => Some(Self::Error),
            4 => Some(Self::Fault),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderStages(pub usize);
impl RenderStages {
    pub const VERTEX: Self = Self(1 << 0);
    pub const FRAGMENT: Self = Self(1 << 1);
    pub const TILE: Self = Self(1 << 2);
    pub const OBJECT: Self = Self(1 << 3);
    pub const MESH: Self = Self(1 << 4);
}

impl std::ops::BitOr for RenderStages {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PurgeableState {
    KeepCurrent = 1,
    NonVolatile = 2,
    Volatile = 3,
    Empty = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum LibraryType {
    Executable = 0,
    Dynamic = 1,
}

impl LibraryType {
    pub fn from_raw(raw: usize) -> Option<Self> {
        match raw {
            0 => Some(Self::Executable),
            1 => Some(Self::Dynamic),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum LibraryOptimizationLevel {
    Default = 0,
    Size = 1,
}

impl LibraryOptimizationLevel {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            1 => Some(Self::Size),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum LibraryError {
    Unsupported = 1,
    Internal = 2,
    CompileFailure = 3,
    CompileWarning = 4,
    FunctionNotFound = 5,
    FileNotFound = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceID {
    pub impl_: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AxisAlignedBoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccelerationStructureInstanceOptions(pub u32);
impl AccelerationStructureInstanceOptions {
    pub const NONE: Self = Self(0);
    pub const DISABLE_TRIANGLE_CULLING: Self = Self(1 << 0);
    pub const TRIANGLE_FRONT_FACING_WINDING_COUNTER_CLOCKWISE: Self = Self(1 << 1);
    pub const OPAQUE: Self = Self(1 << 2);
    pub const NON_OPAQUE: Self = Self(1 << 3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccelerationStructureRefitOptions(pub usize);
impl AccelerationStructureRefitOptions {
    pub const VERTEX_DATA: Self = Self(1 << 0);
    pub const PER_PRIMITIVE_DATA: Self = Self(1 << 1);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelerationStructureInstanceDescriptor {
    pub transformation_matrix: [[f32; 3]; 4],
    pub options: u32,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub acceleration_structure_index: u32,
}
