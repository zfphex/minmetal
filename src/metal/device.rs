use super::encoder::{BlitCommandEncoder, ComputeCommandEncoder, RenderCommandEncoder};
use super::ffi::*;
use super::layer::Drawable;
use super::pass::RenderPassDescriptor;
use super::pipeline::{
    ComputePipelineDescriptor, ComputePipelineState, DepthStencilDescriptor, DepthStencilState,
    Fence, RenderPipelineDescriptor, RenderPipelineState, SamplerDescriptor, SamplerState,
};
use super::resource::{Buffer, Texture, TextureDescriptor};
use super::types::*;
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr;

#[derive(Debug)]
pub struct Device {
    pub(crate) raw: id,
}

impl Device {
    pub fn system_default() -> Option<Self> {
        unsafe {
            let raw = MTLCreateSystemDefaultDevice();
            (!raw.is_null()).then_some(Self { raw })
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
                .unwrap_or_else(|| "Unknown Metal Device".to_string())
        }
    }

    pub fn new_command_queue(&self) -> Result<CommandQueue, MetalError> {
        unsafe {
            let raw = msg_id(self.raw, sel(b"newCommandQueue\0"));
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal command queue"))
            } else {
                Ok(CommandQueue { raw })
            }
        }
    }

    pub fn new_library_with_source(&self, source: &str) -> Result<Library, MetalError> {
        unsafe {
            let source = NSString::new(source);
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newLibraryWithSource:options:error:\0"),
                source.raw(),
                NIL,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to compile Metal shader source",
                )))
            } else {
                Ok(Library { raw })
            }
        }
    }

    pub fn new_render_pipeline_state(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<RenderPipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newRenderPipelineStateWithDescriptor:error:\0"),
                descriptor.raw,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal render pipeline state",
                )))
            } else {
                Ok(RenderPipelineState { raw })
            }
        }
    }

    pub fn new_compute_pipeline_state_with_function(
        &self,
        function: &Function,
    ) -> Result<ComputePipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newComputePipelineStateWithFunction:error:\0"),
                function.raw,
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal compute pipeline state",
                )))
            } else {
                Ok(ComputePipelineState { raw })
            }
        }
    }

    pub fn new_compute_pipeline_state(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<ComputePipelineState, MetalError> {
        unsafe {
            let mut error = NIL;
            let f: unsafe extern "C" fn(id, SEL, id, usize, *mut id, *mut id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newComputePipelineStateWithDescriptor:options:reflection:error:\0"),
                descriptor.raw,
                0,
                ptr::null_mut(),
                &mut error,
            );
            if raw.is_null() {
                Err(MetalError::new(error_message(
                    error,
                    "failed to create Metal compute pipeline state",
                )))
            } else {
                Ok(ComputePipelineState { raw })
            }
        }
    }

    pub fn new_depth_stencil_state(
        &self,
        descriptor: &DepthStencilDescriptor,
    ) -> Result<DepthStencilState, MetalError> {
        unsafe {
            let raw = msg_id_id(
                self.raw,
                sel(b"newDepthStencilStateWithDescriptor:\0"),
                descriptor.raw,
            );
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal depth stencil state",
                ))
            } else {
                Ok(DepthStencilState { raw })
            }
        }
    }

    pub fn new_sampler_state(
        &self,
        descriptor: &SamplerDescriptor,
    ) -> Result<SamplerState, MetalError> {
        unsafe {
            let raw = msg_id_id(
                self.raw,
                sel(b"newSamplerStateWithDescriptor:\0"),
                descriptor.raw,
            );
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal sampler state"))
            } else {
                Ok(SamplerState { raw })
            }
        }
    }

    pub fn new_fence(&self) -> Result<Fence, MetalError> {
        unsafe {
            let raw = msg_id(self.raw, sel(b"newFence\0"));
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal fence"))
            } else {
                Ok(Fence { raw })
            }
        }
    }

    pub fn new_buffer(
        &self,
        length: usize,
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        unsafe {
            let f: unsafe extern "C" fn(id, SEL, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newBufferWithLength:options:\0"),
                length,
                options.as_raw(),
            );
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal buffer"))
            } else {
                Ok(Buffer { raw })
            }
        }
    }

    pub fn new_buffer_with_data<T>(
        &self,
        data: &[T],
        options: ResourceOptions,
    ) -> Result<Buffer, MetalError> {
        unsafe {
            let length = std::mem::size_of_val(data);
            let f: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = f(
                self.raw,
                sel(b"newBufferWithBytes:length:options:\0"),
                data.as_ptr() as *const c_void,
                length,
                options.as_raw(),
            );
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal buffer from data"))
            } else {
                Ok(Buffer { raw })
            }
        }
    }

    pub fn new_texture(&self, descriptor: &TextureDescriptor) -> Result<Texture, MetalError> {
        unsafe {
            let raw = msg_id_id(
                self.raw,
                sel(b"newTextureWithDescriptor:\0"),
                descriptor.raw,
            );
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal texture"))
            } else {
                Ok(Texture { raw })
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct CommandQueue {
    pub(crate) raw: id,
}

impl CommandQueue {
    pub fn command_buffer(&self) -> Result<CommandBuffer, MetalError> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"commandBuffer\0")));
            if raw.is_null() {
                Err(MetalError::new("failed to create Metal command buffer"))
            } else {
                Ok(CommandBuffer { raw })
            }
        }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    pub(crate) raw: id,
}

impl CommandBuffer {
    pub fn render_command_encoder(
        &self,
        descriptor: &RenderPassDescriptor,
    ) -> Result<RenderCommandEncoder, MetalError> {
        unsafe {
            let raw = retain(msg_id_id(
                self.raw,
                sel(b"renderCommandEncoderWithDescriptor:\0"),
                descriptor.raw,
            ));
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal render command encoder",
                ))
            } else {
                Ok(RenderCommandEncoder { raw })
            }
        }
    }

    pub fn blit_command_encoder(&self) -> Result<BlitCommandEncoder, MetalError> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"blitCommandEncoder\0")));
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal blit command encoder",
                ))
            } else {
                Ok(BlitCommandEncoder { raw })
            }
        }
    }

    pub fn compute_command_encoder(&self) -> Result<ComputeCommandEncoder, MetalError> {
        unsafe {
            let raw = retain(msg_id(self.raw, sel(b"computeCommandEncoder\0")));
            if raw.is_null() {
                Err(MetalError::new(
                    "failed to create Metal compute command encoder",
                ))
            } else {
                Ok(ComputeCommandEncoder { raw })
            }
        }
    }

    pub fn present_drawable(&self, drawable: &Drawable) {
        unsafe {
            msg_void_id(self.raw, sel(b"presentDrawable:\0"), drawable.raw);
        }
    }

    pub fn commit(&self) {
        unsafe { msg_void(self.raw, sel(b"commit\0")) };
    }

    pub fn wait_until_completed(&self) {
        unsafe { msg_void(self.raw, sel(b"waitUntilCompleted\0")) };
    }

    pub fn status(&self) -> usize {
        unsafe { msg_usize(self.raw, sel(b"status\0")) }
    }

    pub fn error(&self) -> Option<String> {
        unsafe {
            let error = msg_id(self.raw, sel(b"error\0"));
            (!error.is_null()).then(|| error_message(error, "Metal command buffer failed"))
        }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Library {
    pub(crate) raw: id,
}

impl Library {
    pub fn function(&self, name: &str) -> Result<Function, MetalError> {
        unsafe {
            let name = NSString::new(name);
            let raw = msg_id_id(self.raw, sel(b"newFunctionWithName:\0"), name.raw());
            if raw.is_null() {
                Err(MetalError::new("failed to load Metal function"))
            } else {
                Ok(Function { raw })
            }
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}

#[derive(Debug)]
pub struct Function {
    pub(crate) raw: id,
}

impl Function {
    pub fn name(&self) -> String {
        unsafe {
            ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
                .unwrap_or_else(|| "unknown".to_string())
        }
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe { release(self.raw) };
    }
}
