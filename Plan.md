# Zero-Dependency Rust Metal Core Bindings

## Summary

`minmetal` will provide a small, idiomatic Rust wrapper over the core Metal APIs needed for framebuffer rendering on macOS. The first version targets device creation, `CAMetalLayer` presentation, runtime shader compilation, buffers, textures, render passes, render pipelines, and command encoding.

The crate stays zero-dependency apart from the local `miniwin` example dependency, and it does not use macros. The bindings are intentionally not a full mirror of the Metal API.

## Key Decisions

- Keep Metal bindings in `minmetal`.
- Modify `miniwin` only to expose macOS native handles.
- Attach a `CAMetalLayer` to the existing `miniwin` `NSView`.
- Use runtime Metal shader source compilation for v1.
- Skip compile-time `.metal`/`.metallib` tooling until the Apple shader compiler tools are available.
- Document later phases separately instead of binding the whole Metal API up front.

## Required Binding Surface

### Objective-C Runtime Helpers

- `objc_getClass`
- `sel_registerName`
- `objc_msgSend`
- Typed message-send helpers for:
  - `id` return
  - `BOOL` return
  - `usize` return
  - `u64` return
  - `f64` return
  - struct return and struct argument methods used by Metal
  - `void` return
- Ownership helpers:
  - `retain`
  - `release`
  - autorelease-pool helpers where needed

### Foundation Helpers

- `NSString alloc/initWithBytes:length:encoding:`
- `NSString UTF8String`
- `NSError localizedDescription`
- `NSAutoreleasePool alloc/init/drain`

### QuartzCore / `CAMetalLayer`

- `CAMetalLayer layer`
- `setDevice:`
- `setPixelFormat:`
- `setFramebufferOnly:`
- `setDrawableSize:`
- `setContentsScale:`
- `setPresentsWithTransaction:`
- `nextDrawable`
- `CAMetalDrawable texture`
- `CAMetalDrawable present`

### `miniwin` Integration

- Access the public `window.ns_view` handle directly.
- Use existing `content_size()` and `scale_factor()`.
- Attach the Metal layer with `setWantsLayer:` and `setLayer:`.

### Device And Queues

- `MTLCreateSystemDefaultDevice`
- `MTLDevice name`
- `MTLDevice newCommandQueue`
- `MTLCommandQueue commandBuffer`

### Command Buffers

- `renderCommandEncoderWithDescriptor:`
- `blitCommandEncoder`
- `presentDrawable:`
- `commit`
- `waitUntilCompleted`
- `status`
- `error`

### Runtime Shader Compilation

- `newLibraryWithSource:options:error:`
- `newFunctionWithName:`
- `MTLFunction name`

### Render Pipeline

- `MTLRenderPipelineDescriptor alloc/init`
- `setVertexFunction:`
- `setFragmentFunction:`
- `colorAttachments`
- `objectAtIndexedSubscript:`
- `setPixelFormat:`
- `newRenderPipelineStateWithDescriptor:error:`

### Render Pass

- `MTLRenderPassDescriptor renderPassDescriptor`
- `colorAttachments`
- `objectAtIndexedSubscript:`
- `setTexture:`
- `setLoadAction:`
- `setStoreAction:`
- `setClearColor:`

### Render Encoder

- `setRenderPipelineState:`
- `setVertexBuffer:offset:atIndex:`
- `setFragmentBuffer:offset:atIndex:`
- `setFragmentTexture:atIndex:`
- `drawPrimitives:vertexStart:vertexCount:`
- `endEncoding`

### Buffers

- `newBufferWithLength:options:`
- `newBufferWithBytes:length:options:`
- `contents`
- `length`

### Textures

- `MTLTextureDescriptor texture2DDescriptorWithPixelFormat:width:height:mipmapped:`
- `setUsage:`
- `setStorageMode:`
- `newTextureWithDescriptor:`
- `replaceRegion:mipmapLevel:withBytes:bytesPerRow:`
- `width`
- `height`
- `pixelFormat`

### Blit Encoder

- `copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:`
- `endEncoding`

## Rust Types

The v1 API should include:

- `PixelFormat`
- `TextureUsage`
- `StorageMode`
- `ResourceOptions`
- `LoadAction`
- `StoreAction`
- `PrimitiveType`
- `ClearColor`
- `Origin`
- `Size`
- `Region`
- `Device`
- `CommandQueue`
- `CommandBuffer`
- `RenderCommandEncoder`
- `BlitCommandEncoder`
- `Buffer`
- `Texture`
- `Library`
- `Function`
- `RenderPipelineState`
- `RenderPassDescriptor`
- `RenderPipelineDescriptor`
- `MetalLayer`
- `Drawable`

## Test Plan

- Run `cargo check` on macOS.
- Keep the existing `miniwin` event loop behavior.
- Present a changing GPU-rendered gradient through `CAMetalLayer`.
- Resize the window and confirm drawable size follows `content_size * scale_factor`.
- Confirm shader and pipeline creation errors return readable Rust errors.
- Confirm owned Objective-C objects are released via `Drop`.

## Later Phases

- Precompiled `.metallib` loading and optional `build.rs` shader compilation.
- Compute pipelines and compute command encoders.
- Samplers, depth/stencil, multisampling, and indexed drawing.
- Capture tooling, counters, sparse resources, and ray tracing.

## V2 Core Renderer Binding Surface

V2 expands the crate from framebuffer presentation into bindings needed by a Forward+ renderer. It remains a bindings-only release: no renderer framework, no IBL implementation, no clustered light system, no HZB builder, no shadows, no decals, no SSR, and no post stack yet.

### V2 Decisions

- Keep the single-crate structure.
- Keep runtime shader source compilation only.
- Keep examples as single files under `examples/`.
- Keep the API zero-dependency and macro-free.
- Avoid advanced Metal systems in this phase: argument buffers, indirect command buffers, heaps, sparse resources, counters, capture tooling, ray tracing, and full renderer architecture.

### V2 Binding Additions

- Compute:
  - compute pipeline descriptors and states
  - compute command encoders
  - compute buffer, texture, sampler, and bytes binding
  - threadgroup and thread-grid dispatch

- Depth/stencil:
  - depth/stencil descriptors and states
  - compare functions, stencil operations, stencil masks
  - depth and stencil render pass attachments
  - depth resolve attachment support

- Pipeline state:
  - vertex descriptors
  - multiple color attachment configuration
  - blending factors and operations
  - color write masks
  - sample count and raster sample count
  - depth and stencil attachment pixel formats
  - alpha-to-coverage

- Drawing and encoder state:
  - indexed drawing
  - instanced drawing
  - viewport and scissor state
  - cull mode, front-facing winding, triangle fill mode
  - depth bias
  - vertex and fragment texture/sampler bindings
  - small uniform byte bindings

- Textures and resources:
  - 1D, 2D, 2D array, 3D, cube, cube-array, and multisample texture types
  - expanded color, HDR, integer, compressed, depth, and stencil pixel formats
  - mip level count, array length, depth, sample count, and texture usage flags
  - texture views
  - texture CPU readback
  - buffer modified-range notification

- Samplers:
  - sampler descriptors and states
  - min/mag/mip filters
  - address modes
  - compare function
  - anisotropy
  - LOD clamp

- Blit and synchronization:
  - buffer-to-buffer copy
  - buffer-to-texture copy
  - texture-to-texture copy
  - mipmap generation
  - managed resource synchronization
  - fences for render, compute, and blit encoders

### V2 Smoke Examples

- `examples/compute.rs` validates compute pipeline creation, dispatch, command completion, and CPU-visible buffer readback.
- `examples/depth_triangle.rs` validates depth state, depth attachments, vertex descriptors, indexed drawing, viewport/scissor state, and presentation.

## V3 GPU-Driven Core Binding Surface

V3 adds GPU-driven Metal features needed by scalable Forward+ renderer architecture while staying bindings-only, runtime-shader-only, zero-dependency, and macro-free.

### V3 Decisions

- Keep direct `window.ns_view` access for `CAMetalLayer` attachment.
- Keep diagnostics, capture tooling, counters, sparse resources, ray tracing, and renderer architecture out of v3.
- Keep advanced API bindings available through `minmetal::*`.

### V3 Binding Additions

- Function constants:
  - `MTLFunctionConstantValues`
  - typed bool, integer, float, and byte setters
  - `newFunctionWithName:constantValues:error:`

- Binary archives:
  - `MTLBinaryArchiveDescriptor`
  - `newBinaryArchiveWithDescriptor:error:`
  - render and compute pipeline function insertion
  - render and compute descriptor archive lists

- Argument buffers:
  - `MTLArgumentDescriptor`
  - `MTLArgumentEncoder`
  - encoded length and alignment
  - buffer, texture, sampler, and byte encoding
  - argument buffer allocation and binding

- Indirect command buffers:
  - `MTLIndirectCommandBufferDescriptor`
  - command type flags and inheritance controls
  - render and compute indirect commands
  - indirect draw, indexed draw, and compute dispatch
  - render and compute encoder execution

- Heaps:
  - `MTLHeapDescriptor`
  - heap type, storage mode, CPU cache mode, hazard tracking mode, and size
  - device heap size/alignment queries
  - heap buffer and texture allocation
  - heap usage/availability queries

- Shared events:
  - `MTLSharedEvent`
  - signaled value get/set
  - command buffer signal and wait encoding

- Resource-state commands:
  - `MTLResourceStateCommandEncoder`
  - fence update/wait
  - sparse texture mapping entry points later

- Render/compute resource usage declarations:
  - `useResource:usage:`
  - `useHeap:`
  - fence update/wait

### V3 Smoke Examples

- `examples/function_constants.rs`
- `examples/heap_resources.rs`
- `examples/resource_state.rs`
- `examples/argument_buffer.rs`
- `examples/indirect_commands.rs`

## V4 Stabilized Metal Binding Core

V4 stabilizes the existing V1-V3 binding surface to make the current framebuffer, renderer-core, and GPU-driven APIs safer to use, harder to misconfigure, and easier to validate.

### V4 Decisions

- Keep the current flat `src/` layout; no nested files, selective per-file imports, or conditional compilation constraints.
- Focus on correctness, ownership, error handling, FFI struct alignment, and API consistency rather than adding renderer features.
- Eliminate Objective-C reference leaks from double-retains on `new` prefix methods (e.g., `new_argument_encoder` and `new_texture_view`).
- Ensure all FFI structs use `#[repr(C)]` and match Apple headers exactly.

### V4 Binding Additions and Adjustments

- Device convenience:
  - `Device::required_system_default() -> Result<Device, MetalError>`
- Typed accessors:
  - `CommandBufferStatus` (enum representing `MTLCommandBufferStatus`)
  - `CommandBuffer::status() -> CommandBufferStatus`
  - `CommandBuffer::error() -> Option<MetalError>`
- Nil-safety:
  - Changed `Drawable::texture()` from direct wrapper return to `Result<Texture, MetalError>`
- Safe helper wrappers for FFI calls to eliminate redundant raw transmutes:
  - `msg_id_id_err`
  - `msg_bool_id_err`
  - `msg_void_id_usize_usize`
  - `msg_void_id_usize`
  - `msg_void_ptr_usize_usize`
  - `msg_void_id_range`
  - `msg_void_range`
  - `msg_void_size_size`
  - `msg_void_id_u64`
- Buffer reading/writing helper methods:
  - `Buffer::write<T: Copy>(&self, value: &T)` (added validation checking if `contents()` is null)
  - `Buffer::write_slice<T: Copy>(&self, data: &[T])` (added validation checking if `contents()` is null)
  - `Buffer::read_slice<T: Copy>(&self, out: &mut [T])` (added validation checking if `contents()` is null)

### V4 Validation

- Added unit tests in `src/lib.rs` covering error-reporting and invalid cases:
  - Tests check for the presence of a system default Metal device and skip gracefully in headless or CI environments without panic.
  - Invalid shader compilation returns `MetalError` with translated details.
  - Requesting a missing function name returns `MetalError` specifying the name.
  - Invalid render pipeline descriptors return `MetalError` instead of crashing.
  - Nil indirect command buffer accesses safely return `MetalError` rather than causing a segmentation fault.
- Refactored all existing examples (`compute`, `depth_triangle`, `function_constants`, `heap_resources`, `resource_state`, `argument_buffer`, `indirect_commands`) to align with the new safe methods and design guidelines.

