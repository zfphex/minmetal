use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Debug)]
pub struct FunctionConstantValues {
    pub raw: id,
}

impl FunctionConstantValues {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLFunctionConstantValues\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_bool(&self, index: usize, value: bool) {
        let value: bool = value;
        self.set_raw(
            index,
            DataType::Bool,
            &value as *const bool as *const c_void,
        );
    }

    pub fn set_u32(&self, index: usize, value: u32) {
        self.set_raw(index, DataType::UInt, &value as *const u32 as *const c_void);
    }

    pub fn set_i32(&self, index: usize, value: i32) {
        self.set_raw(index, DataType::Int, &value as *const i32 as *const c_void);
    }

    pub fn set_f32(&self, index: usize, value: f32) {
        self.set_raw(
            index,
            DataType::Float,
            &value as *const f32 as *const c_void,
        );
    }

    pub fn set_bytes(&self, index: usize, data_type: DataType, bytes: &[u8]) {
        self.set_raw(index, data_type, bytes.as_ptr() as *const c_void);
    }

    fn set_raw(&self, index: usize, data_type: DataType, ptr: *const c_void) {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) =
                transmute(objc_msgSend as *const c_void);
            f(
                self.raw,
                sel(b"setConstantValue:type:atIndex:\0"),
                ptr,
                data_type as usize,
                index,
            );
        }
    }
}

impl Default for FunctionConstantValues {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FunctionConstantValues {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct BinaryArchiveDescriptor {
    pub raw: id,
}

impl BinaryArchiveDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLBinaryArchiveDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }
}

impl Default for BinaryArchiveDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BinaryArchiveDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct BinaryArchive {
    pub raw: id,
}

impl BinaryArchive {
    pub fn add_render_pipeline_functions(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(
                self.raw,
                sel(b"addRenderPipelineFunctionsWithDescriptor:error:\0"),
                descriptor.raw,
                &mut error,
            );
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add render pipeline functions to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_compute_pipeline_functions(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<(), MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> BOOL =
                transmute(objc_msgSend as *const c_void);
            let ok = f(
                self.raw,
                sel(b"addComputePipelineFunctionsWithDescriptor:error:\0"),
                descriptor.raw,
                &mut error,
            );
            if ok == NO {
                Err(MetalError::new(error_message(
                    error,
                    "failed to add compute pipeline functions to Metal binary archive",
                )))
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for BinaryArchive {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct ComputePipelineDescriptor {
    pub raw: id,
}

impl ComputePipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLComputePipelineDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_compute_function(&self, function: &Function) {
        unsafe {
            msg_void_id(self.raw, sel(b"setComputeFunction:\0"), function.raw);
        }
    }

    pub fn set_binary_archives(&self, archives: &[&BinaryArchive]) {
        unsafe {
            let raw: Vec<id> = archives.iter().map(|archive| archive.raw).collect();
            let array = ns_array_from_ids(&raw);
            msg_void_id(self.raw, sel(b"setBinaryArchives:\0"), array);
        }
    }
}

impl Default for ComputePipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ComputePipelineDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct ComputePipelineState {
    pub raw: id,
}

impl Drop for ComputePipelineState {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct VertexDescriptor {
    pub raw: id,
}

impl VertexDescriptor {
    pub fn new() -> Self {
        unsafe {
            let raw = retain(msg_id(
                class(b"MTLVertexDescriptor\0"),
                sel(b"vertexDescriptor\0"),
            ));
            Self { raw }
        }
    }

    pub fn set_attribute(
        &self,
        index: usize,
        format: VertexFormat,
        offset: usize,
        buffer_index: usize,
    ) {
        unsafe {
            let attributes = msg_id(self.raw, sel(b"attributes\0"));
            let attribute = msg_id_usize(attributes, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_usize(attribute, sel(b"setFormat:\0"), format as usize);
            msg_void_usize(attribute, sel(b"setOffset:\0"), offset);
            msg_void_usize(attribute, sel(b"setBufferIndex:\0"), buffer_index);
        }
    }

    pub fn set_layout(
        &self,
        index: usize,
        stride: usize,
        step_function: VertexStepFunction,
        step_rate: usize,
    ) {
        unsafe {
            let layouts = msg_id(self.raw, sel(b"layouts\0"));
            let layout = msg_id_usize(layouts, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_usize(layout, sel(b"setStride:\0"), stride);
            msg_void_usize(layout, sel(b"setStepFunction:\0"), step_function as usize);
            msg_void_usize(layout, sel(b"setStepRate:\0"), step_rate);
        }
    }
}

impl Default for VertexDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VertexDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct RenderPipelineDescriptor {
    pub raw: id,
}

impl RenderPipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLRenderPipelineDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_vertex_function(&self, function: &Function) {
        unsafe {
            msg_void_id(self.raw, sel(b"setVertexFunction:\0"), function.raw);
        }
    }

    pub fn set_fragment_function(&self, function: &Function) {
        unsafe {
            msg_void_id(self.raw, sel(b"setFragmentFunction:\0"), function.raw);
        }
    }

    pub fn set_color_attachment_pixel_format(&self, index: usize, pixel_format: PixelFormat) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_usize(attachment, sel(b"setPixelFormat:\0"), pixel_format.as_raw());
        }
    }

    pub fn set_vertex_descriptor(&self, vertex_descriptor: &VertexDescriptor) {
        unsafe {
            msg_void_id(
                self.raw,
                sel(b"setVertexDescriptor:\0"),
                vertex_descriptor.raw,
            );
        }
    }

    pub fn set_sample_count(&self, sample_count: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setSampleCount:\0"), sample_count);
        }
    }

    pub fn set_raster_sample_count(&self, raster_sample_count: usize) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setRasterSampleCount:\0"),
                raster_sample_count,
            );
        }
    }

    pub fn set_depth_attachment_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setDepthAttachmentPixelFormat:\0"),
                pixel_format.as_raw(),
            );
        }
    }

    pub fn set_stencil_attachment_pixel_format(&self, pixel_format: PixelFormat) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setStencilAttachmentPixelFormat:\0"),
                pixel_format.as_raw(),
            );
        }
    }

    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setAlphaToCoverageEnabled:\0"),
                if enabled { YES } else { NO },
            );
        }
    }

    pub fn set_color_attachment_blending(
        &self,
        index: usize,
        enabled: bool,
        source_rgb: BlendFactor,
        destination_rgb: BlendFactor,
        rgb_operation: BlendOperation,
        source_alpha: BlendFactor,
        destination_alpha: BlendFactor,
        alpha_operation: BlendOperation,
    ) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_bool(
                attachment,
                sel(b"setBlendingEnabled:\0"),
                if enabled { YES } else { NO },
            );
            msg_void_usize(
                attachment,
                sel(b"setSourceRGBBlendFactor:\0"),
                source_rgb as usize,
            );
            msg_void_usize(
                attachment,
                sel(b"setDestinationRGBBlendFactor:\0"),
                destination_rgb as usize,
            );
            msg_void_usize(
                attachment,
                sel(b"setRgbBlendOperation:\0"),
                rgb_operation as usize,
            );
            msg_void_usize(
                attachment,
                sel(b"setSourceAlphaBlendFactor:\0"),
                source_alpha as usize,
            );
            msg_void_usize(
                attachment,
                sel(b"setDestinationAlphaBlendFactor:\0"),
                destination_alpha as usize,
            );
            msg_void_usize(
                attachment,
                sel(b"setAlphaBlendOperation:\0"),
                alpha_operation as usize,
            );
        }
    }

    pub fn set_color_attachment_write_mask(&self, index: usize, mask: ColorWriteMask) {
        unsafe {
            let attachments = msg_id(self.raw, sel(b"colorAttachments\0"));
            let attachment = msg_id_usize(attachments, sel(b"objectAtIndexedSubscript:\0"), index);
            msg_void_usize(attachment, sel(b"setWriteMask:\0"), mask.as_raw());
        }
    }

    pub fn set_binary_archives(&self, archives: &[&BinaryArchive]) {
        unsafe {
            let raw: Vec<id> = archives.iter().map(|archive| archive.raw).collect();
            let array = ns_array_from_ids(&raw);
            msg_void_id(self.raw, sel(b"setBinaryArchives:\0"), array);
        }
    }
}

impl Default for RenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct RenderPipelineState {
    pub raw: id,
}

impl Drop for RenderPipelineState {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct StencilDescriptor {
    pub raw: id,
}

impl StencilDescriptor {
    fn borrowed(raw: id) -> Self {
        Self { raw }
    }

    pub fn set_stencil_compare_function(&self, compare_function: CompareFunction) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setStencilCompareFunction:\0"),
                compare_function as usize,
            );
        }
    }

    pub fn set_stencil_failure_operation(&self, operation: StencilOperation) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setStencilFailureOperation:\0"),
                operation as usize,
            );
        }
    }

    pub fn set_depth_failure_operation(&self, operation: StencilOperation) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setDepthFailureOperation:\0"),
                operation as usize,
            );
        }
    }

    pub fn set_depth_stencil_pass_operation(&self, operation: StencilOperation) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setDepthStencilPassOperation:\0"),
                operation as usize,
            );
        }
    }

    pub fn set_read_mask(&self, mask: u32) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setReadMask:\0"), mask as usize);
        }
    }

    pub fn set_write_mask(&self, mask: u32) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setWriteMask:\0"), mask as usize);
        }
    }
}

#[derive(Debug)]
pub struct DepthStencilDescriptor {
    pub raw: id,
}

impl DepthStencilDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLDepthStencilDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_depth_compare_function(&self, compare_function: CompareFunction) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setDepthCompareFunction:\0"),
                compare_function as usize,
            );
        }
    }

    pub fn set_depth_write_enabled(&self, enabled: bool) {
        unsafe {
            msg_void_bool(
                self.raw,
                sel(b"setDepthWriteEnabled:\0"),
                if enabled { YES } else { NO },
            );
        }
    }

    pub fn front_face_stencil(&self) -> StencilDescriptor {
        unsafe { StencilDescriptor::borrowed(msg_id(self.raw, sel(b"frontFaceStencil\0"))) }
    }

    pub fn back_face_stencil(&self) -> StencilDescriptor {
        unsafe { StencilDescriptor::borrowed(msg_id(self.raw, sel(b"backFaceStencil\0"))) }
    }
}

impl Default for DepthStencilDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DepthStencilDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct DepthStencilState {
    pub raw: id,
}

impl Drop for DepthStencilState {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct SamplerDescriptor {
    pub raw: id,
}

impl SamplerDescriptor {
    pub fn new() -> Self {
        unsafe {
            let allocated = msg_id(class(b"MTLSamplerDescriptor\0"), sel(b"alloc\0"));
            Self {
                raw: msg_id(allocated, sel(b"init\0")),
            }
        }
    }

    pub fn set_min_filter(&self, filter: SamplerMinMagFilter) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMinFilter:\0"), filter as usize);
        }
    }

    pub fn set_mag_filter(&self, filter: SamplerMinMagFilter) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMagFilter:\0"), filter as usize);
        }
    }

    pub fn set_mip_filter(&self, filter: SamplerMipFilter) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMipFilter:\0"), filter as usize);
        }
    }

    pub fn set_address_mode(&self, mode: SamplerAddressMode) {
        self.set_s_address_mode(mode);
        self.set_t_address_mode(mode);
        self.set_r_address_mode(mode);
    }

    pub fn set_s_address_mode(&self, mode: SamplerAddressMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setSAddressMode:\0"), mode as usize);
        }
    }

    pub fn set_t_address_mode(&self, mode: SamplerAddressMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setTAddressMode:\0"), mode as usize);
        }
    }

    pub fn set_r_address_mode(&self, mode: SamplerAddressMode) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setRAddressMode:\0"), mode as usize);
        }
    }

    pub fn set_compare_function(&self, compare_function: CompareFunction) {
        unsafe {
            msg_void_usize(
                self.raw,
                sel(b"setCompareFunction:\0"),
                compare_function as usize,
            );
        }
    }

    pub fn set_max_anisotropy(&self, max_anisotropy: usize) {
        unsafe {
            msg_void_usize(self.raw, sel(b"setMaxAnisotropy:\0"), max_anisotropy);
        }
    }

    pub fn set_lod_min_clamp(&self, value: f64) {
        unsafe {
            msg_void_f64(self.raw, sel(b"setLodMinClamp:\0"), value);
        }
    }

    pub fn set_lod_max_clamp(&self, value: f64) {
        unsafe {
            msg_void_f64(self.raw, sel(b"setLodMaxClamp:\0"), value);
        }
    }
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SamplerDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct SamplerState {
    pub raw: id,
}

impl Drop for SamplerState {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Fence {
    pub raw: id,
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}
