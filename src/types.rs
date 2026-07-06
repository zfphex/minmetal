use crate::*;

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

/// Sample position within a pixel (`MTLSamplePosition`). Origin is top-left with range [0, 1).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SamplePosition {
    pub x: f32,
    pub y: f32,
}

impl SamplePosition {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 64-bit unsigned integer type appropriate for storing GPU addresses (`MTLGPUAddress`).
pub type GpuAddress = u64;

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
    A8Unorm = 1,
    R8Unorm = 10,
    R8UnormSrgb = 11,
    R8Snorm = 12,
    R8Uint = 13,
    R8Sint = 14,
    R16Unorm = 20,
    R16Snorm = 22,
    R16Uint = 23,
    R16Sint = 24,
    R16Float = 25,
    Rg8Unorm = 30,
    Rg8UnormSrgb = 31,
    Rg8Snorm = 32,
    Rg8Uint = 33,
    Rg8Sint = 34,
    B5G6R5Unorm = 40,
    A1Bgr5Unorm = 41,
    Abgr4Unorm = 42,
    Bgr5A1Unorm = 43,
    R32Uint = 53,
    R32Sint = 54,
    R32Float = 55,
    Rg16Unorm = 60,
    Rg16Snorm = 62,
    Rg16Uint = 63,
    Rg16Sint = 64,
    Rg16Float = 65,
    Rgba8Unorm = 70,
    Rgba8UnormSrgb = 71,
    Rgba8Snorm = 72,
    Rgba8Uint = 73,
    Rgba8Sint = 74,
    Bgra8Unorm = 80,
    Bgra8UnormSrgb = 81,
    Rgb10A2Unorm = 90,
    Rgb10A2Uint = 91,
    Rg11B10Float = 92,
    Rgb9E5Float = 93,
    Bgr10A2Unorm = 94,
    Rg32Uint = 103,
    Rg32Sint = 104,
    Rg32Float = 105,
    Rgba16Unorm = 110,
    Rgba16Snorm = 112,
    Rgba16Uint = 113,
    Rgba16Sint = 114,
    Rgba16Float = 115,
    Rgba32Uint = 123,
    Rgba32Sint = 124,
    Rgba32Float = 125,
    Bc1Rgba = 130,
    Bc1RgbaSrgb = 131,
    Bc2Rgba = 132,
    Bc2RgbaSrgb = 133,
    Bc3Rgba = 134,
    Bc3RgbaSrgb = 135,
    Bc4RUnorm = 140,
    Bc4RSnorm = 141,
    Bc5RgUnorm = 142,
    Bc5RgSnorm = 143,
    Bc6HRgbFloat = 150,
    Bc6HRgbUfloat = 151,
    Bc7RgbaUnorm = 152,
    Bc7RgbaUnormSrgb = 153,
    EacR11Unorm = 170,
    EacR11Snorm = 172,
    EacRg11Unorm = 174,
    EacRg11Snorm = 176,
    EacRgba8 = 178,
    EacRgba8Srgb = 179,
    Etc2Rgb8 = 180,
    Etc2Rgb8Srgb = 181,
    Etc2Rgb8A1 = 182,
    Etc2Rgb8A1Srgb = 183,
    Astc4x4Srgb = 186,
    Astc4x4Ldr = 204,
    Astc4x4Hdr = 222,
    Gbgr422 = 240,
    Bgrg422 = 241,
    Depth16Unorm = 250,
    Depth32Float = 252,
    Stencil8 = 253,
    Depth24UnormStencil8 = 255,
    Depth32FloatStencil8 = 260,
    X32Stencil8 = 261,
    X24Stencil8 = 262,
    Bgra10Xr = 552,
    Bgra10XrSrgb = 553,
    Bgr10Xr = 554,
    Bgr10XrSrgb = 555,
}

impl PixelFormat {
    pub const fn as_raw(self) -> usize {
        self as usize
    }

    pub fn from_raw(raw: usize) -> Self {
        match raw {
            0 => Self::Invalid,
            1 => Self::A8Unorm,
            10 => Self::R8Unorm,
            11 => Self::R8UnormSrgb,
            12 => Self::R8Snorm,
            13 => Self::R8Uint,
            14 => Self::R8Sint,
            20 => Self::R16Unorm,
            22 => Self::R16Snorm,
            23 => Self::R16Uint,
            24 => Self::R16Sint,
            25 => Self::R16Float,
            30 => Self::Rg8Unorm,
            31 => Self::Rg8UnormSrgb,
            32 => Self::Rg8Snorm,
            33 => Self::Rg8Uint,
            34 => Self::Rg8Sint,
            40 => Self::B5G6R5Unorm,
            41 => Self::A1Bgr5Unorm,
            42 => Self::Abgr4Unorm,
            43 => Self::Bgr5A1Unorm,
            53 => Self::R32Uint,
            54 => Self::R32Sint,
            55 => Self::R32Float,
            60 => Self::Rg16Unorm,
            62 => Self::Rg16Snorm,
            63 => Self::Rg16Uint,
            64 => Self::Rg16Sint,
            65 => Self::Rg16Float,
            70 => Self::Rgba8Unorm,
            71 => Self::Rgba8UnormSrgb,
            72 => Self::Rgba8Snorm,
            73 => Self::Rgba8Uint,
            74 => Self::Rgba8Sint,
            80 => Self::Bgra8Unorm,
            81 => Self::Bgra8UnormSrgb,
            90 => Self::Rgb10A2Unorm,
            91 => Self::Rgb10A2Uint,
            92 => Self::Rg11B10Float,
            93 => Self::Rgb9E5Float,
            94 => Self::Bgr10A2Unorm,
            103 => Self::Rg32Uint,
            104 => Self::Rg32Sint,
            105 => Self::Rg32Float,
            110 => Self::Rgba16Unorm,
            112 => Self::Rgba16Snorm,
            113 => Self::Rgba16Uint,
            114 => Self::Rgba16Sint,
            115 => Self::Rgba16Float,
            123 => Self::Rgba32Uint,
            124 => Self::Rgba32Sint,
            125 => Self::Rgba32Float,
            130 => Self::Bc1Rgba,
            131 => Self::Bc1RgbaSrgb,
            132 => Self::Bc2Rgba,
            133 => Self::Bc2RgbaSrgb,
            134 => Self::Bc3Rgba,
            135 => Self::Bc3RgbaSrgb,
            140 => Self::Bc4RUnorm,
            141 => Self::Bc4RSnorm,
            142 => Self::Bc5RgUnorm,
            143 => Self::Bc5RgSnorm,
            150 => Self::Bc6HRgbFloat,
            151 => Self::Bc6HRgbUfloat,
            152 => Self::Bc7RgbaUnorm,
            153 => Self::Bc7RgbaUnormSrgb,
            170 => Self::EacR11Unorm,
            172 => Self::EacR11Snorm,
            174 => Self::EacRg11Unorm,
            176 => Self::EacRg11Snorm,
            178 => Self::EacRgba8,
            179 => Self::EacRgba8Srgb,
            180 => Self::Etc2Rgb8,
            181 => Self::Etc2Rgb8Srgb,
            182 => Self::Etc2Rgb8A1,
            183 => Self::Etc2Rgb8A1Srgb,
            186 => Self::Astc4x4Srgb,
            204 => Self::Astc4x4Ldr,
            222 => Self::Astc4x4Hdr,
            240 => Self::Gbgr422,
            241 => Self::Bgrg422,
            250 => Self::Depth16Unorm,
            252 => Self::Depth32Float,
            253 => Self::Stencil8,
            255 => Self::Depth24UnormStencil8,
            260 => Self::Depth32FloatStencil8,
            261 => Self::X32Stencil8,
            262 => Self::X24Stencil8,
            552 => Self::Bgra10Xr,
            553 => Self::Bgra10XrSrgb,
            554 => Self::Bgr10Xr,
            555 => Self::Bgr10XrSrgb,
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
    pub const DRAW_MESH_THREADGROUPS: Self = Self(1 << 7);
    pub const DRAW_MESH_THREADS: Self = Self(1 << 8);

    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn as_raw(self) -> usize {
        self.0
    }

    pub const fn is_render(self) -> bool {
        self.0
            & (Self::DRAW.0
                | Self::DRAW_INDEXED.0
                | Self::DRAW_PATCH.0
                | Self::DRAW_INDEXED_PATCH.0
                | Self::DRAW_MESH_THREADGROUPS.0
                | Self::DRAW_MESH_THREADS.0)
            != 0
    }

    pub const fn is_compute(self) -> bool {
        self.0 & (Self::CONCURRENT_DISPATCH.0 | Self::CONCURRENT_DISPATCH_THREADS.0) != 0
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

#[derive(Debug)]
pub struct MetalError {
    pub(crate) repr: ErrorRepr,
}

#[derive(Debug)]
pub(crate) enum ErrorRepr {
    Static(&'static str),
    Runtime { error: id, fallback: &'static str },
    Custom(String),
}

impl Clone for ErrorRepr {
    fn clone(&self) -> Self {
        match self {
            Self::Static(s) => Self::Static(s),
            Self::Runtime { error, fallback } => Self::Runtime {
                error: retain(*error),
                fallback,
            },
            Self::Custom(s) => Self::Custom(s.clone()),
        }
    }
}

impl Clone for MetalError {
    fn clone(&self) -> Self {
        Self {
            repr: self.repr.clone(),
        }
    }
}

impl Drop for MetalError {
    fn drop(&mut self) {
        if let ErrorRepr::Runtime { error, .. } = self.repr {
            release(error);
        }
    }
}

unsafe impl Send for MetalError {}
unsafe impl Sync for MetalError {}

pub trait IntoMetalError {
    fn into_error(self) -> MetalError;
}

impl IntoMetalError for &'static str {
    fn into_error(self) -> MetalError {
        MetalError {
            repr: ErrorRepr::Static(self),
        }
    }
}

impl IntoMetalError for String {
    fn into_error(self) -> MetalError {
        MetalError {
            repr: ErrorRepr::Custom(self),
        }
    }
}

impl IntoMetalError for LazyErrorMessage {
    fn into_error(self) -> MetalError {
        MetalError {
            repr: ErrorRepr::Runtime {
                error: retain(self.error),
                fallback: self.fallback,
            },
        }
    }
}

impl IntoMetalError for &LazyErrorMessage {
    fn into_error(self) -> MetalError {
        MetalError {
            repr: ErrorRepr::Runtime {
                error: retain(self.error),
                fallback: self.fallback,
            },
        }
    }
}

impl IntoMetalError for &String {
    fn into_error(self) -> MetalError {
        MetalError {
            repr: ErrorRepr::Custom(self.clone()),
        }
    }
}

impl MetalError {
    pub fn new<T: IntoMetalError>(message: T) -> Self {
        message.into_error()
    }
}

impl std::fmt::Display for MetalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            ErrorRepr::Static(s) => f.write_str(s),
            ErrorRepr::Runtime { error, fallback } => {
                let msg = crate::ffi::format_error_message(*error, fallback);
                f.write_str(&msg)
            }
            ErrorRepr::Custom(s) => f.write_str(s),
        }
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

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlitOption(pub usize);
impl BlitOption {
    pub const NONE: Self = Self(0);
    pub const DEPTH_FROM_DEPTH_STENCIL: Self = Self(1 << 0);
    pub const STENCIL_FROM_DEPTH_STENCIL: Self = Self(1 << 1);
    pub const ROW_LINEAR_PVRTC: Self = Self(1 << 2);

    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum CounterSampleBufferError {
    OutOfMemory = 0,
    Invalid = 1,
    Internal = 2,
}

impl CounterSampleBufferError {
    pub fn from_raw(raw: isize) -> Option<Self> {
        match raw {
            0 => Some(Self::OutOfMemory),
            1 => Some(Self::Invalid),
            2 => Some(Self::Internal),
            _ => None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CounterResultTimestamp {
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CounterResultStageUtilization {
    pub total_cycles: u64,
    pub vertex_cycles: u64,
    pub tessellation_cycles: u64,
    pub post_tessellation_vertex_cycles: u64,
    pub fragment_cycles: u64,
    pub render_target_cycles: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CounterResultStatistic {
    pub tessellation_input_patches: u64,
    pub vertex_invocations: u64,
    pub post_tessellation_vertex_invocations: u64,
    pub clipper_invocations: u64,
    pub clipper_primitives_out: u64,
    pub fragment_invocations: u64,
    pub fragments_passed: u64,
    pub compute_kernel_invocations: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MapIndirectArguments {
    pub region_origin_x: u32,
    pub region_origin_y: u32,
    pub region_origin_z: u32,
    pub region_size_width: u32,
    pub region_size_height: u32,
    pub region_size_depth: u32,
    pub mip_map_level: u32,
    pub slice_id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum BinaryArchiveError {
    None = 0,
    InvalidFile = 1,
    UnexpectedElement = 2,
    CompilationFailure = 3,
    InternalError = 4,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarrierScope(pub usize);
impl BarrierScope {
    pub const BUFFERS: Self = Self(1 << 0);
    pub const TEXTURES: Self = Self(1 << 1);
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceLocation {
    BuiltIn = 0,
    Slot = 1,
    External = 2,
    Unspecified = 100,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadWriteTextureTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgumentBuffersTier {
    Tier1 = 0,
    Tier2 = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPUFamily {
    Apple1 = 1001,
    Apple2 = 1002,
    Apple3 = 1003,
    Apple4 = 1004,
    Apple5 = 1005,
    Apple6 = 1006,
    Apple7 = 1007,
    Apple8 = 1008,
    Apple9 = 1009,
    Mac1 = 2001,
    Mac2 = 2002,
    Common1 = 3001,
    Common2 = 3002,
    Common3 = 3003,
    Metal3 = 5001,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatureSet {
    iOS_GPUFamily1_v1 = 0,
    iOS_GPUFamily2_v1 = 1,
    iOS_GPUFamily1_v2 = 2,
    iOS_GPUFamily2_v2 = 3,
    iOS_GPUFamily3_v1 = 4,
    iOS_GPUFamily1_v3 = 5,
    iOS_GPUFamily2_v3 = 6,
    iOS_GPUFamily3_v2 = 7,
    iOS_GPUFamily1_v4 = 8,
    iOS_GPUFamily2_v4 = 9,
    iOS_GPUFamily3_v3 = 10,
    iOS_GPUFamily4_v1 = 11,
    tvOS_GPUFamily1_v1 = 30000,
    tvOS_GPUFamily1_v2 = 30001,
    macOS_GPUFamily1_v1 = 10000,
    macOS_GPUFamily1_v2 = 10001,
    macOS_ReadWriteTextureTier2 = 10002,
    macOS_GPUFamily1_v3 = 10003,
    macOS_GPUFamily1_v4 = 10004,
    macOS_GPUFamily2_v1 = 10005,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DynamicLibraryError {
    None = 0,
    InvalidFile = 1,
    CompilationFailure = 2,
    UnresolvedInstallName = 3,
    DependencyLoadFailure = 4,
    Unsupported = 5,
}
