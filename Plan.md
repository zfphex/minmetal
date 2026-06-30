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

- `raw_ns_window() -> *mut c_void`
- `raw_ns_view() -> *mut c_void`
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
- Heaps, fences, shared events, capture tooling, argument buffers, sparse resources, counters, and ray tracing.
