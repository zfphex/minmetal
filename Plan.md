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

## V5 Advanced Metal Systems Bindings

V5 expands `minmetal` with support for advanced, raw Metal systems: capture tooling, performance counters, sparse resource mapping, and hardware ray tracing. This remains a bindings-only phase: no high-level frameworks or automatic setups.

### V5 Decisions

- Maintain a flat structure in `src/` by adding focused files: `capture.rs`, `counters.rs`, `sparse.rs`, and `raytracing.rs`.
- Expose all modules through `src/lib.rs` and make them publicly accessible.
- Ensure OS/device-unsupported APIs fail gracefully by returning `Result::Err(MetalError)` instead of crash or abort, using selector-safety queries via `respondsToSelector:`.

### V5 Binding Additions

- **Capture Tooling**:
  - `CaptureDestination` (enum representing `MTLCaptureDestination`)
  - `CaptureDescriptor` (wraps `MTLCaptureDescriptor`)
  - `CaptureManager` (wraps `MTLCaptureManager`) with `shared()`, `supports_destination()`, `start_capture()`, `stop_capture()`, and `is_capturing()`
- **Counters**:
  - `CounterSet` and `Counter` discovery and querying APIs
  - `CounterSampleBufferDescriptor` and `CounterSampleBuffer` creation
  - `supports_counter_sampling()` query on `Device`
  - `sample_counters_in_buffer()` on render, compute, and blit encoders, and `resolve_counters()` on blit encoders
- **Sparse Resources**:
  - `supports_sparse_textures()` and `sparse_tile_size()` query on `Device`
  - `SparseTextureMappingMode` mapping enum (`Map`, `Unmap`)
  - `update_texture_mapping()` mapping commands on `ResourceStateCommandEncoder`
- **Ray Tracing**:
  - `supports_raytracing()` query on `Device`
  - `AccelerationStructureSizes` query logic
  - `AccelerationStructureTriangleGeometryDescriptor` and `PrimitiveAccelerationStructureDescriptor` configuration
  - `new_acceleration_structure()` allocation and `acceleration_structure_command_encoder()` commands for building structures

### V5 Validation

- Added unit tests in `src/lib.rs` covering V5 systems:
  - `test_selector_availability_helper` to verify selector-safety logic works.
  - `test_unsupported_api_paths_returning_metal_error` to verify that unsupported API calls return clean `MetalError` results instead of crashes.
  - `test_nil_object_creation_paths` to check handling of dummy/nil Handles.
  - `test_wrapper_drop_coverage` to check destructor behaviors.
- Added four smoke examples:
  - `examples/capture.rs`
  - `examples/counters.rs`
  - `examples/sparse_resources.rs`
  - `examples/raytracing.rs`

## V6 Metal API Coverage Foundation

V6 shifts `minmetal` from milestone-driven feature slices toward systematic Metal API coverage. The crate remains a safe, zero-dependency, macro-free Metal binding library, not a renderer and not a renderer framework.

### V6 Decisions

- Maintain a flat structure in `src/` and public `minmetal::*` exports.
- Use Apple SDK headers as the source of truth for selectors, enum values, struct layouts, and ownership.
- Prefer safe fallible wrappers (nil object creation returns `MetalError`, unavailable selectors return `MetalError`, owned objects release in `Drop`, borrowed objects are not retained/released unless explicitly converted).
- No new dependencies, macros, build scripts, or renderer abstractions.

### V6 Binding Additions

- **Command and Pass Descriptor Coverage**:
  - `MTLRenderPassDescriptor` remaining attachment properties, store action options, visibility result buffer, render target array length, imageblock sample length, tile size, and sample buffer attachments.
  - `MTLComputePassDescriptor`, compute sample buffer attachments, and `computeCommandEncoderWithDescriptor:`.
  - `MTLBlitPassDescriptor`, blit sample buffer attachments, and `blitCommandEncoderWithDescriptor:`.
  - `MTLResourceStatePassDescriptor`, resource-state sample buffer attachments, and `resourceStateCommandEncoderWithDescriptor:`.
  - `MTLParallelRenderCommandEncoder` creation, child render encoders, and end encoding.

- **Encoder Completeness**:
  - indirect draw calls using ordinary indirect buffers.
  - staged resource usage APIs such as `useResource:usage:stages:` and heap staged variants.
  - render fence update/wait with stage masks.
  - tile shader bindings and dispatch.
  - object/mesh shader bindings and mesh draw calls.
  - compute acceleration-structure, visible-function-table, and intersection-function-table bindings.
  - array/range binding variants for buffers, textures, samplers, heaps, and function tables.

- **Pipeline, Library, and Function Coverage**:
  - `MTLFunctionDescriptor`, `MTLLinkedFunctions`, `MTLDynamicLibrary`, `MTLFunctionHandle`, `MTLFunctionLog`, `MTLLogState`.
  - `MTLVisibleFunctionTableDescriptor`, `MTLVisibleFunctionTable`.
  - `MTLIntersectionFunctionTableDescriptor`, `MTLIntersectionFunctionTable`.
  - render/compute pipeline descriptor fields not yet bound, including linked functions, support flags, max buffers, binary archives, and function tables.
  - pipeline state reflection-free queries that return simple values.

- **Expanded Ray Tracing**:
  - bounding-box geometry descriptors.
  - instance acceleration structure descriptors.
  - acceleration-structure instance options and descriptors.
  - acceleration-structure command encoder copy, compact, refit, write-size, and sample-counter methods.
  - heap acceleration-structure allocation.
  - compute/render bindings for acceleration structures.
  - intersection function table binding and resource setup.

- **Resource and Argument Completeness**:
  - labels on all label-bearing objects.
  - resource queries: allocated size, storage mode, hazard tracking mode, resource options, heap, heap offset, CPU cache mode, purgeable state, GPU resource ID where available.
  - texture queries for texture type, usage, storage mode, sample count, mip count, array length, depth, parent texture, buffer-backed layout, sparse properties, and remote storage where available.
  - argument encoder array/range setters for buffers, textures, samplers, indirect command buffers, visible function tables, intersection function tables, and acceleration structures.
  - argument descriptor fields including constant block alignment/data size where supported.

### V6 Smoke Examples

- `examples/pass_descriptors.rs`
- `examples/parallel_render.rs`
- `examples/function_tables.rs`
- `examples/raytracing_instances.rs`
- `examples/indirect_draw.rs`

## V7 MetalIO, Residency, and Rasterization-Rate Bindings

V7 closes important Metal coverage gaps while keeping `minmetal` a safe bindings library, not a renderer. The focus is asset streaming and residency control first, plus variable rasterization-rate descriptors.

### V7 Decisions

- Remain macOS-only by assumption with no `#[cfg(target_os = "macos")]` gates.
- Keep the API zero-dependency, macro-free, and publicly exported through `minmetal::*`.
- Runtime availability still matters: MetalIO is macOS 13+, newer IO file handles are macOS 14+ (with legacy selector fallback), and residency sets are macOS 15+.
- V7 does not implement a renderer, streaming system, asset format, VRS renderer, or resource manager.
- Function stitching remains deferred to a later focused phase.

### V7 Binding Additions

- **MetalIO** (`src/io.rs`):
  - `MTLIOCommandQueueDescriptor`, `MTLIOCommandQueue`, `MTLIOCommandBuffer`, `MTLIOFileHandle`.
  - `IOPriority`, `IOCommandQueueType`, `IOStatus`, `IOError`, `IOCompressionStatus`, `IOCompressionMethod`.
  - Device methods: `new_io_command_queue`, `new_io_file_handle`, `new_io_file_handle_compressed` (with legacy selector fallback).
  - IO command buffer methods: load into raw memory, buffers, and textures; status copy; barriers; enqueue/commit/wait/cancel; labels; errors; shared event wait/signal.
  - `MTLIOCompressor` C API wrappers: default chunk size, context creation, append data, flush/destroy.

- **Residency Sets** (`src/residency.rs`):
  - `MTLResidencySetDescriptor`, `MTLResidencySet`, and `Allocation` protocol handle.
  - Device `new_residency_set`.
  - Add/remove allocation APIs, allocation count, allocated size, contains checks, request/end residency, and commit.

- **Rasterization Rate** (`src/rasterization_rate.rs`):
  - `MTLRasterizationRateSampleArray`, `MTLRasterizationRateLayerDescriptor`, `MTLRasterizationRateLayerArray`, `MTLRasterizationRateMapDescriptor`, `MTLRasterizationRateMap`.
  - Device support query and map creation.
  - `RenderPassDescriptor` rasterization rate map setter/getter.
  - Map queries: screen size, physical granularity, layer count, parameter buffer size/alignment, physical size, coordinate mapping, parameter buffer copy.

### V7 Smoke Examples

- `examples/io_buffer.rs`
- `examples/io_texture.rs`
- `examples/io_compression.rs`
- `examples/residency_set.rs`
- `examples/rasterization_rate.rs`

### V7 Validation

- `cargo check --all-targets`
- `cargo test` with real Metal runtime
- Run existing non-interactive examples plus new V7 examples
- Unsupported OS/API selectors return `MetalError` or graceful skip instead of crashing
- Fallible Objective-C creation methods propagate readable `NSError` descriptions
- Newly owned Objective-C objects release through existing `Drop` wrappers

## V8 Function Stitching Bindings

V8 adds bindings for MTLFunctionStitching.h, closing the last major user-facing Metal API coverage gap. This remains bindings-only: no renderer, no shader build tooling, no .metallib pipeline, and no higher-level graph DSL.

### V8 Decisions

- Keep bindings direct and close to Metal's object model.
- Stay zero-dependency, macro-free, runtime-shader-only, and bindings-focused.
- Implement macOS-only guarded options and binary archives support (macOS 15+).

### V8 Binding Additions

- **Foundation Helpers** (`src/ffi.rs`):
  - `ns_array_count`, `ns_array_object_at_index`, and `ns_array_to_vec` to query `NSArray` count and elements.

- **Function Stitching Types** (`src/stitching.rs`):
  - `StitchedLibraryOptions` options bitfield.
  - `FunctionStitchingAttribute` protocol wrapper and `FunctionStitchingAttributeAlwaysInline` implementation.
  - `FunctionStitchingNode` protocol wrapper, `FunctionStitchingInputNode` and `FunctionStitchingFunctionNode` implementations.
  - `FunctionStitchingGraph` and `StitchedLibraryDescriptor`.

- **Device Entry Point** (`src/device.rs`):
  - `Device::new_library_with_stitched_descriptor` compiling function stitching graphs into an `Library`.
  - `CompileOptions` with `set_library_type` and `set_install_name` to allow compiling dynamic libraries.

### V8 Smoke Examples

- `examples/function_stitching.rs`

### V8 Validation

- `cargo check --all-targets`
- `cargo test` with real Metal runtime access
- `cargo run --example function_stitching` executing a dynamically stitched function linked to a compute kernel
- Re-run existing smoke examples: `compute`, `function_constants`, and `function_tables`
- Validate that graph compilation failures return readable `MetalError` details
- Validate that all owned Objective-C objects release through `Drop` wrappers
