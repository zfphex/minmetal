# Missing Metal API Coverage

This document tracks API surface that is still missing or only partially covered by `minmetal`.

The goal of this crate is safe, zero-dependency, macro-free Metal bindings rather than a renderer. A header marked `Bound` in `Coverage.md` can still appear here when the crate only covers the most important calls and not the full SDK surface.

This inventory is organized by SDK area. It should be treated as a living checklist: before a module is considered complete, compare it against the installed Metal SDK headers and add tests that exercise every safe wrapper permutation.

## Highest Priority

- [x] Reflection and argument introspection:
  - `MTLArgument`
  - `MTLType`
  - `MTLStructType`
  - `MTLArrayType`
  - `MTLPointerType`
  - `MTLTextureReferenceType`
  - `MTLBinding`
  - `MTLBufferBinding`
  - `MTLThreadgroupBinding`
  - `MTLTextureBinding`
  - `MTLObjectPayloadBinding`
- [x] Command queue and command buffer completeness:
  - `MTLCommandQueueDescriptor`
  - retained and unretained command buffer creation variants
  - command queue device/label accessors
  - command buffer enqueue/scheduling/completion APIs
  - scheduled/completed handlers (callback handlers skipped due to lack of block runtime in zero-dependency Rust)
  - GPU timing accessors
  - command buffer logs
  - error options and detailed error enums
- [x] Resource base API completeness:
  - shared `MTLResource` wrapper behavior for buffers, textures, heaps, and acceleration structures
  - device, heap, heap offset, allocated size, storage mode, CPU cache mode, hazard tracking mode, and resource options getters
  - purgeable-state setter/result handling
  - aliasing APIs
- [x] Texture and sampler completeness:
  - texture swizzles
  - full texture view overloads
  - parent/relative texture queries
  - buffer-backed texture queries
  - shared texture handles
  - sparse texture metadata
  - compression type and optimization flags
  - sampler border color, normalized coordinates, LOD average, and remaining descriptor/state getters
- [x] Safe validation for APIs that can currently crash Metal when misused:
  - indirect command buffer descriptor invariants
  - indirect render/compute command setter compatibility
  - command index bounds
  - inherited pipeline/buffer modes
  - sample counter range bounds

## Classic Metal Headers

### `MTLAccelerationStructure.h`

- [x] `MTLAccelerationStructureDescriptor` base properties:
  - usage getters/setters
  - motion properties where available
- [x] `MTLPrimitiveAccelerationStructureDescriptor` completeness:
  - geometry descriptor getters
  - motion start/end time
  - motion keyframe count
  - motion transform support
- [x] `MTLAccelerationStructureTriangleGeometryDescriptor` completeness:
  - transformation matrix buffer/offset
  - primitive data buffer/stride/element size
  - allow duplicate intersection function invocation
  - getter coverage for all configured properties
- [x] `MTLAccelerationStructureBoundingBoxGeometryDescriptor` completeness:
  - primitive data buffer/stride/element size
  - getter coverage
- [x] Motion geometry:
  - `MTLMotionKeyframeData`
  - `MTLAccelerationStructureMotionTriangleGeometryDescriptor`
  - `MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor`
- [x] Curve geometry:
  - `MTLCurveType`
  - `MTLCurveBasis`
  - `MTLCurveEndCaps`
  - `MTLAccelerationStructureCurveGeometryDescriptor`
  - `MTLAccelerationStructureMotionCurveGeometryDescriptor`
- [x] Instance descriptor variants:
  - `MTLAccelerationStructureInstanceDescriptorType`
  - user ID/options/mask coverage for all descriptor layouts
  - motion instance descriptors
  - indirect instance descriptors
- [x] Transform support:
  - `MTLMatrixLayout`
  - `MTLTransformType`
  - transform buffer/offset/count/stride APIs
- [x] Acceleration structure object getters:
  - device
  - label
  - allocated size/resource metadata inherited from `MTLResource`

### `MTLAccelerationStructureCommandEncoder.h`

- [x] Acceleration-structure pass descriptors:
  - `MTLAccelerationStructurePassDescriptor`
  - `MTLAccelerationStructurePassSampleBufferAttachmentDescriptor`
  - `MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray`
- [x] Command buffer creation overload using an acceleration-structure pass descriptor.
- [ ] Full encoder coverage:
  - write serialized acceleration structure size
  - serialize/deserialize acceleration structures if available in the target SDK
  - refit/copy/compact overload variants not currently wrapped
- [x] sample counter support for acceleration-structure passes
- [x] label/debug group methods inherited from `MTLCommandEncoder`

### `MTLAllocation.h`

- [x] Direct protocol-level queries where useful:
  - allocated size
  - resource/device identity where exposed
- [x] Confirm whether the current `Allocation` wrapper should expose all protocol methods or remain a tagged wrapper for residency APIs.

### `MTLArgument.h`

- [x] Full reflection model:
  - `MTLDataType` complete enum coverage
  - `MTLBindingType`
  - `MTLArgumentType`
  - `MTLBindingAccess`
  - `MTLType`
  - `MTLStructMember`
  - `MTLStructType`
  - `MTLArrayType`
  - `MTLPointerType`
  - `MTLTextureReferenceType`
  - `MTLArgument`
  - `MTLBinding`
  - `MTLBufferBinding`
  - `MTLThreadgroupBinding`
  - `MTLTextureBinding`
  - `MTLObjectPayloadBinding`
- [x] Reflection getters:
  - name
  - index
  - access
  - active flag
  - array length
  - data type
  - texture type
  - texture data type
  - buffer data type
  - buffer alignment
  - buffer data size
  - struct members
  - pointer element type/access/alignment/data size

### `MTLArgumentEncoder.h`

- [ ] Getter coverage:
  - device
  - label
  - encoded length
  - alignment
- [ ] Binding coverage:
  - constant data pointer APIs if available
  - indirect command buffers
  - render pipeline states
  - compute pipeline states
  - acceleration structures
  - visible function tables
  - intersection function tables
  - arrays/ranges for every supported resource kind
- [ ] Safe validation:
  - array length/range validation before sending Objective-C messages
  - buffer offset alignment validation where possible

### `MTLBinaryArchive.h`

- [ ] `MTLBinaryArchiveDescriptor`:
  - URL getter/setter
  - label if available
- [ ] `MTLBinaryArchive`:
  - label getter/setter
  - device getter
  - serialize to URL
  - add render pipeline functions with options/reflection variants
  - add compute pipeline functions with options/reflection variants
  - add tile/mesh/object pipeline functions where available
  - error enum coverage for `MTLBinaryArchiveError`

### `MTLBlitCommandEncoder.h`

- [ ] `MTLBlitOption` complete flags.
- [ ] Copy variants:
  - buffer to texture with options
  - texture to buffer with options
  - texture to texture with options
  - buffer fills
  - texture slice/level variants not currently wrapped
- [ ] Synchronization/optimization:
  - synchronize resource slice/level variants
  - optimize contents for GPU access
  - optimize contents for CPU access
  - fences and waits with full error handling where applicable
- [ ] Counter sampling:
  - all range validations
  - resolve result layout helpers if useful
- [ ] Command encoder base methods:
  - label
  - insert/debug groups
  - memory barriers if exposed on the installed SDK

### `MTLBlitPass.h`

- [x] All descriptor getters.
- [x] Sample-buffer attachment range validation.
- [x] Copying/default descriptor behavior.

### `MTLBuffer.h`

- [ ] Resource metadata inherited from `MTLResource`.
- [ ] Buffer APIs:
  - device
  - label
  - CPU cache mode
  - storage mode
  - hazard tracking mode
  - resource options
  - heap
  - heap offset
  - allocated size
  - remote storage buffer if available
  - new texture with descriptor/offset/bytes-per-row
  - add debug marker/remove debug markers
  - purgeable state
  - aliasing
- [ ] Safe helpers:
  - checked typed read/write length handling
  - checked `contents` access for private/storage modes

### `MTLCaptureManager.h`

- [x] Capture descriptor getters:
  - capture object
  - destination
  - output URL
- [x] Capture manager:
  - shared manager state coverage
  - default capture scope getter/setter
  - create capture scope with device
  - create capture scope with command queue
  - start capture with device/command queue legacy overloads if retained
  - full `MTLCaptureError` enum coverage

### `MTLCaptureScope.h`

- [x] `MTLCaptureScope` wrapper:
  - label getter/setter
  - device getter if exposed
  - command queue getter if exposed
  - `beginScope`
  - `endScope`
- [x] Tests for nested/invalid capture-scope use that avoid leaving capture active.

### `MTLCommandBuffer.h`

- [x] `MTLCommandBufferDescriptor` if available on the installed SDK.
- [x] `MTLCommandBufferError` and `MTLCommandBufferStatus` complete enum coverage.
- [x] Error option flags.
- [x] Creation paths:
  - retained references
  - unretained references
  - descriptor-based creation
- [x] State and metadata:
  - device
  - command queue
  - label
  - kernel start/end time
  - GPU start/end time
  - logs
- [x] Lifecycle:
  - enqueue
  - commit
  - wait until scheduled
  - wait until completed
  - present drawable at time
  - present drawable after minimum duration
- [x] Handlers:
  - scheduled handler (skipped/deferred)
  - completed handler (skipped/deferred)
  - safe callback/lifetime policy (skipped/deferred)
- [x] Encoder creation overloads:
  - render/compute/blit/resource-state with pass descriptors
  - acceleration-structure command encoder
  - IO integration where applicable

### `MTLCommandEncoder.h`

- [x] Base methods for every encoder wrapper:
  - device
  - label getter/setter
  - end encoding
  - insert debug signpost
  - push debug group
  - pop debug group
- [x] Ensure every encoder implements the base methods consistently.

### `MTLCommandQueue.h`

- [x] `MTLCommandQueueDescriptor`.
- [x] Device creation with descriptor.
- [x] Queue properties:
  - label getter/setter
  - device getter
  - max command buffer count
  - log state
- [x] Creation variants:
  - command buffer
  - command buffer with unretained references
  - command buffer with descriptor
  - command buffer with descriptor and unretained references if available
- [x] Residency-set enqueue APIs if exposed.

### `MTLComputeCommandEncoder.h`

- [x] Full command encoder base methods.
- [x] Binding overloads:
  - buffers with range
  - bytes with length/index
  - texture arrays
  - sampler arrays
  - visible function table arrays
  - intersection function table arrays
  - acceleration-structure arrays if available
- [x] Dispatch variants:
  - indirect dispatch
  - dispatch type/pass descriptor coverage
- [x] Memory APIs:
  - threadgroup memory length
  - imageblock memory length if available
  - memory barriers
  - texture barriers
- [x] Resource usage:
  - use resources
  - use heaps
  - staged variants where available

### `MTLComputePass.h`

- [x] All descriptor getters.
- [x] Dispatch type default/copy behavior.
- [x] Sample-buffer attachment completeness.

### `MTLComputePipeline.h`

- [ ] Pipeline creation variants:
  - function + options + reflection
  - descriptor + options + reflection
  - async variants if a callback policy is chosen
- [ ] `MTLComputePipelineDescriptor`:
  - label
  - compute function getter
  - threadgroup size multiple
  - max total threads per threadgroup
  - stage input descriptor
  - buffers/mutability
  - preloaded libraries
  - linked functions
  - binary archives
  - support adding binary functions
  - support indirect command buffers
  - shader validation
- [ ] `MTLComputePipelineState`:
  - device
  - label
  - imageblock memory length helpers
  - reflection-related access where exposed

### `MTLCounters.h`

- [ ] Full enum coverage:
  - `MTLCounterSamplingPoint`
  - `MTLCounterSampleBufferError`
- [ ] Counter set/counter getters:
  - name
  - counters
  - device if exposed
- [ ] Sample buffer:
  - label
  - device
  - sample count
  - storage mode
  - resolve helpers
- [ ] Safe validation:
  - sample index bounds
  - resolve range bounds
  - unsupported sampling point handling.

### `MTLDepthStencil.h`

- [x] Descriptor getters:
  - depth compare function
  - depth write enabled
  - front/back stencil descriptors
  - label if available
- [x] `MTLDepthStencilState` getters:
  - label
  - device
- [x] Stencil descriptor getters for every setter.

### `MTLDevice.h`

- [x] Device feature and limit queries:
  - registry ID
  - location and location number
  - max threads per threadgroup
  - max buffer length
  - max texture dimensions
  - max argument buffer sampler count
  - max threadgroup memory length
  - max transfer rate
  - recommended working set size
  - current allocated size
  - read/write texture support
  - raster order groups support
  - argument buffer tier
  - sparse texture support by texture type/sample count
  - ray tracing support details
  - counter sampling support details
  - family support
  - feature set support if still present
  - sample count support
  - texture format support
  - BC/ASTC/PVRTC capability queries where exposed
- [x] Device creation methods:
  - command queues with max count and descriptor
  - buffers with bytes-no-copy and deallocator policy
  - textures with IOSurface/shared handles if available
  - dynamic libraries from URL/library/data
  - binary archives with URLs
  - render/compute pipeline creation with options/reflection
  - asynchronous pipeline creation
  - event/shared event handle creation
  - counters, log states, and capture scopes
  - peer group/device APIs if available
- [ ] Device notification APIs if present.
- [x] Full error enum coverage for device-related creation failures.

### `MTLDeviceCertification.h`

- [x] Decide whether to bind or explicitly skip:
  - certification APIs
  - certification enum/result types
- [x] If skipped, document the exact reason in `Coverage.md`.

### `MTLDrawable.h`

- [x] Drawable protocol completeness:
  - presented time
  - drawable ID if available
  - layer identity if exposed through `CAMetalDrawable`
- [x] Present variants:
  - present
  - present at time
  - present after minimum duration

### `MTLDynamicLibrary.h`

- [x] Device creation methods:
  - new dynamic library from URL
  - new dynamic library from library
  - new dynamic library from data if available
- [x] Dynamic library:
  - device
  - label
  - install name
  - serialize to URL
  - error enum coverage

### `MTLEvent.h`

- [x] `MTLEvent` wrapper.
- [x] `MTLSharedEvent` completeness:
  - device
  - label
  - signaled value
  - new shared event handle
  - create shared event from handle
- [x] Listener/callback APIs:
  - `MTLSharedEventListener`
  - notify listener at value
  - safe callback/lifetime strategy

### `MTLFence.h`

- [x] Fence getters:
  - device
  - label
- [x] Consistent use on render/compute/blit/resource-state encoders.

### `MTLFunctionConstantValues.h`

- [x] Complete data type support for constants.
- [x] named constants if exposed.
- [x] reset APIs if available.
- [x] Safe byte-size validation for `setConstantValue:type:atIndex:`.

### `MTLFunctionDescriptor.h`

- [ ] Full `MTLFunctionDescriptor` getter/setter coverage:
  - name
  - specialized name
  - constant values
  - options
  - binary archives
- [ ] `MTLIntersectionFunctionDescriptor` completeness:
  - signature
  - max buffer bind count
  - max texture bind count
  - max sampler bind count
  - getter coverage
- [ ] Device/library methods that consume descriptors with options/reflection.

### `MTLFunctionHandle.h`

- [x] Function handle getters:
  - name
  - function type if exposed
  - device/pipeline identity where exposed

### `MTLFunctionLog.h`

- [x] Complete log type enum.
- [x] Function log getters:
  - type
  - function
  - encoder label
  - debug location
- [x] Debug location getters:
  - function name
  - URL
  - line
  - column
- [x] Integrate command-buffer log retrieval.

### `MTLFunctionStitching.h`

- [ ] Getter coverage for every descriptor property.
- [ ] All stitching node subclasses and attributes in the installed SDK.
- [ ] Binary archive integration.
- [ ] Options flags completeness.
- [ ] Validation around graph shape and nil output node.

### `MTLHeap.h`

- [x] Descriptor getters:
  - type
  - storage mode
  - CPU cache mode
  - hazard tracking mode
  - resource options
  - size
  - sparse page size
- [x] Heap getters:
  - device
  - label
  - storage mode
  - CPU cache mode
  - hazard tracking mode
  - resource options
  - heap type
  - size
  - used size
  - current allocated size
  - max available size
- [x] Allocation methods:
  - textures with descriptors at offset
  - buffers at offset
  - acceleration structures at offset
  - aliasing controls
  - purgeable state

### `MTLIOCommandBuffer.h`

- [ ] Complete status/error enum coverage.
- [ ] All load variants:
  - bytes
  - buffer
  - texture
  - sparse texture tiles if available
  - compressed and uncompressed variants
- [ ] Synchronization:
  - event waits/signals
  - barriers
  - cancellation
  - enqueue/commit behavior
- [ ] Metadata:
  - label
  - status
  - error
- [ ] Safe file-offset/size validation.

### `MTLIOCommandQueue.h`

- [ ] Descriptor getter coverage.
- [ ] Queue metadata:
  - label
  - device
  - type
  - priority
  - max command buffer count
  - max commands in flight
- [ ] File handle completeness:
  - URL/path identity if available
  - label
  - compressed handles
  - error enum coverage

### `MTLIOCompressor.h`

- [ ] Compression status/method completeness.
- [ ] Compression context error handling beyond status conversion.
- [ ] Tests for every compression method supported by the OS.

### `MTLIndirectCommandBuffer.h`

- [ ] Descriptor getters and mirrored Rust-side invariant state.
- [ ] Command type completeness:
  - draw
  - draw indexed
  - draw patches
  - draw indexed patches
  - concurrent dispatch
  - concurrent dispatch threads
  - mesh threadgroups
  - mesh threads
- [ ] Indirect command buffer:
  - size
  - resource metadata
  - reset range validation
  - optimized range APIs if available
- [ ] Safe command access:
  - command index bounds
  - command-type compatibility checks

### `MTLIndirectCommandEncoder.h`

- [ ] Render command completeness:
  - set pipeline with inherited-state validation
  - set vertex buffers with inherited-buffer validation
  - draw variants
  - indexed draw variants
  - patch draw variants
  - mesh draw variants
  - reset
- [ ] Compute command completeness:
  - set pipeline with inherited-state validation
  - set kernel buffers with inherited-buffer validation
  - dispatch variants
  - reset
- [ ] Safe fallible wrappers for every command that can be invalid based on descriptor state.

### `MTLIntersectionFunctionTable.h`

- [ ] Descriptor getter coverage.
- [ ] Table getters:
  - resource metadata
  - GPU resource ID
- [ ] Setter completeness:
  - functions
  - buffers
  - visible function tables
  - intersection function tables
  - opaque triangle/curve intersection functions
  - range validation

### `MTLLibrary.h`

- [ ] Library creation variants:
  - source with options
  - file/path/URL
  - data
  - default library with bundle
  - dynamic library integration
  - async variants if callback policy exists
- [ ] Library getters:
  - label
  - device
  - function names
  - type
  - install name
- [ ] Function creation:
  - simple function
  - function with constants
  - function with descriptor
  - intersection function with descriptor
  - options/reflection variants
- [ ] `MTLFunction` getters:
  - name
  - function type
  - patch type
  - patch control point count
  - vertex attributes
  - stage input attributes
  - function constants dictionary
  - options
- [ ] Reflection integration with `MTLArgument`/`MTLBinding`.

### `MTLLinkedFunctions.h`

- [ ] Getter coverage:
  - functions
  - binary functions
  - private functions
  - groups if available
- [ ] Setter coverage for all arrays.
- [ ] Validation for nil/empty array behavior.

### `MTLLogState.h`

- [ ] Full descriptor getter/setter coverage.
- [ ] `MTLLogState` metadata.
- [ ] Error enum coverage.
- [ ] Command buffer integration.

### `MTLParallelRenderCommandEncoder.h`

- [ ] Base encoder methods:
  - device
  - label
  - debug groups
- [ ] Store action/option completeness.
- [ ] Child encoder creation validation after end encoding.

### `MTLPipeline.h`

- [ ] `MTLPipelineBufferDescriptor`.
- [ ] `MTLPipelineBufferDescriptorArray`.
- [ ] `MTLMutability`.
- [ ] `MTLShaderValidation`.
- [ ] Pipeline option flags and reflection flags.
- [ ] Buffer mutability arrays on render/compute/tile/mesh descriptors.

### `MTLPixelFormat.h`

- [ ] Exhaustive `MTLPixelFormat` enum coverage:
  - all normalized formats
  - all integer formats
  - all float formats
  - depth/stencil formats
  - packed formats
  - shared exponent formats
  - RGB9E5/RGB10A2/BGR10A2 variants
  - BC compressed formats
  - ASTC compressed formats
  - PVRTC formats if present on the installed SDK
  - XR/sRGB variants
- [ ] `from_raw` handling for every known format.
- [ ] Tests comparing discriminants against SDK values.

### `MTLRasterizationRate.h`

- [ ] Getter/setter completeness for descriptors.
- [ ] Copying behavior.
- [ ] Safe sample-array bounds checks.
- [ ] Device support query permutations.
- [ ] Map coordinate conversion edge cases.

### `MTLRenderCommandEncoder.h`

- [ ] Render state:
  - viewport arrays
  - scissor arrays
  - visibility result mode/offset
  - depth clip mode
  - depth bias
  - stencil front/back reference values
  - blend color
  - color store action updates
  - depth/stencil store action updates
  - sample positions
- [ ] Binding:
  - all vertex/fragment/tile/object/mesh range setters
  - bytes setters with explicit length
  - function table arrays
  - acceleration structure arrays if available
- [ ] Drawing:
  - all primitive draw variants
  - base instance/base vertex variants
  - indirect variants
  - tessellation patch variants
  - mesh shader variants
- [ ] Synchronization/resource usage:
  - memory barriers
  - texture barriers
  - use resources
  - use heaps
  - staged variants
- [ ] Tile shading:
  - all tile dispatch and threadgroup memory APIs.
- [ ] Base encoder methods.

### `MTLRenderPass.h`

- [ ] Full attachment descriptor getter/setter coverage.
- [ ] Color attachment descriptor array wrapper completeness.
- [ ] Depth resolve filter:
  - `MTLMultisampleDepthResolveFilter`
  - getter/setter
- [ ] Stencil resolve filter:
  - `MTLMultisampleStencilResolveFilter`
  - getter/setter
- [ ] Render pass descriptor:
  - default raster sample count
  - render target width/height
  - visibility result buffer
  - render target array length
  - imageblock sample length
  - threadgroup memory length
  - tile width/height
  - rasterization rate map
  - sample buffer attachments
- [ ] Validation for attachment index and compatible texture usage.

### `MTLRenderPipeline.h`

- [ ] `MTLRenderPipelineDescriptor` completeness:
  - label
  - vertex/fragment functions getters
  - vertex descriptor getter
  - sample count
  - raster sample count
  - alpha to coverage
  - alpha to one
  - rasterization enabled
  - input primitive topology
  - tessellation properties
  - max vertex amplification count
  - support indirect command buffers
  - support adding binary functions
  - linked functions
  - binary archives
  - buffer mutability arrays
  - preloaded libraries
  - shader validation
  - color/depth/stencil formats
- [ ] Color attachment descriptor:
  - all getters
  - blending enable
  - source/destination factors
  - RGB/alpha operations
  - write mask
- [ ] Pipeline state:
  - label
  - device
  - max threadgroup queries
  - imageblock sample length
  - support flags
  - function handles
  - visible/intersection table creation
  - GPU resource ID
- [ ] Creation variants:
  - options
  - reflection
  - async callbacks

### `MTLResidencySet.h`

- [ ] Descriptor getter/setter completeness.
- [ ] Residency set:
  - device
  - label
  - allocated size
  - all add/remove variants
  - contains
  - count
  - commit
  - request/end residency
- [ ] Command queue integration if exposed.

### `MTLResource.h`

- [x] Complete shared resource wrapper:
  - label
  - device
  - CPU cache mode
  - storage mode
  - hazard tracking mode
  - resource options
  - heap
  - heap offset
  - allocated size
  - purgeable state
  - aliasing
  - set owner with identity if available
- [x] `MTLResourceOptions` complete flags.
- [x] `MTLCPUCacheMode`, `MTLStorageMode`, `MTLHazardTrackingMode`, and `MTLPurgeableState` complete values.

### `MTLResourceStateCommandEncoder.h`

- [ ] Base encoder methods.
- [ ] Resource/heap usage:
  - use resource
  - use resources
  - use heap
  - use heaps
- [ ] Barriers:
  - texture barriers
  - memory barriers
  - buffer/texture state updates where available
- [ ] Sparse mapping:
  - all texture mapping overloads
  - buffer mapping overloads if available
  - tile map validation
- [ ] Fence update/wait completeness.

### `MTLResourceStatePass.h`

- [x] All descriptor getters.
- [x] Sample-buffer attachment completeness.

### `MTLSampler.h`

- [x] `MTLSamplerBorderColor`.
- [x] Descriptor completeness:
  - min/mag/mip filters
  - s/t/r address modes
  - border color
  - normalized coordinates
  - LOD min/max clamp
  - LOD average
  - LOD bias
  - max anisotropy
  - compare function
  - support argument buffers
  - label
- [x] Sampler state:
  - label
  - device
  - GPU resource ID

### `MTLStageInputOutputDescriptor.h`

- [x] `MTLStageInputOutputDescriptor`.
- [x] `MTLAttributeDescriptor`.
- [x] `MTLAttributeDescriptorArray`.
- [x] `MTLBufferLayoutDescriptor`.
- [x] `MTLBufferLayoutDescriptorArray`.
- [x] `MTLAttributeFormat` complete enum.
- [x] `MTLStepFunction` complete enum.
- [x] Getter/setter coverage and reset behavior.

### `MTLTexture.h`

- [x] `MTLTextureSwizzle`.
- [x] `MTLTextureSwizzleChannels`.
- [x] `MTLSharedTextureHandle`.
- [x] `MTLTextureCompressionType`.
- [x] Descriptor completeness:
  - texture type
  - pixel format
  - width/height/depth
  - mipmap level count
  - sample count
  - array length
  - resource options
  - CPU cache mode
  - storage mode
  - hazard tracking mode
  - usage
  - allow GPU optimized contents
  - compression type
  - swizzle
- [x] Texture getters:
  - root resource
  - parent texture
  - parent relative level
  - parent relative slice
  - buffer
  - buffer offset
  - buffer bytes per row
  - iosurface/plane where available
  - texture type
  - sample count
  - array length
  - mipmap level count
  - usage
  - storage mode
  - resource options
  - swizzle
  - is framebuffer only
  - first mip in tail
  - tail size in bytes
  - sparse metadata
- [x] Texture methods:
  - full `replaceRegion` overloads
  - full `getBytes` overloads
  - full texture view overloads
  - new shared texture handle

### `MTLTypes.h`

- [ ] Complete helper structs:
  - `MTLSamplePosition`
  - packed vector/matrix types not already represented
  - coordinate and size variants used by newer APIs
- [ ] Confirm ABI layout tests for every struct passed through `objc_msgSend`.

### `MTLVertexDescriptor.h`

- [ ] Full `MTLVertexFormat` enum coverage.
- [ ] Full `MTLVertexStepFunction` enum coverage.
- [ ] Descriptor array wrappers:
  - attributes
  - layouts
  - object-at-indexed-subscript getters
  - reset
- [ ] Attribute descriptor getters:
  - format
  - offset
  - buffer index
- [ ] Layout descriptor getters:
  - stride
  - step function
  - step rate

### `MTLVisibleFunctionTable.h`

- [ ] Descriptor getter coverage.
- [ ] Table resource metadata.
- [ ] Range validation for function setting.

## Xcode SDK Classic Additions

These headers appear in the Xcode SDK used on this machine and are not part of the older Command Line Tools header set.

### `MTLDataType.h`

- [ ] Split/align data type bindings with the newer header if the SDK requires it.
- [ ] Exhaustive `MTLDataType` coverage.

### `MTLGPUAddress.h`

- [ ] Bind GPU address related types and helper APIs.
- [ ] Audit `Buffer::gpu_address` against the newer SDK definitions.

### `MTLResourceViewPool.h`

- [ ] `MTLResourceViewPoolDescriptor`.
- [ ] `MTLResourceViewPool`.
- [ ] Device creation APIs.
- [ ] Pool sizing/usage queries.

### `MTLTextureViewPool.h`

- [ ] `MTLTextureViewPool`.
- [ ] Texture view allocation APIs.
- [ ] Pool lifecycle and resource metadata.

### `MTLTensor.h`

- [ ] `MTLTensorDataType`.
- [ ] `MTLTensorUsage`.
- [ ] `MTLTensorError`.
- [ ] `MTLTensorExtents`.
- [ ] `MTLTensorDescriptor`.
- [ ] `MTLTensor`.
- [ ] Device tensor creation APIs.
- [ ] Buffer-backed tensor APIs.

## Metal 4 API Family

The `MTL4*` headers are currently effectively unbound. They should probably be planned as a separate major phase because they introduce a new command submission, compiler, pipeline, archive, and argument model.

### `MTL4AccelerationStructure.h`

- [ ] `MTL4AccelerationStructureDescriptor`.
- [ ] `MTL4AccelerationStructureGeometryDescriptor`.
- [ ] `MTL4PrimitiveAccelerationStructureDescriptor`.
- [ ] `MTL4AccelerationStructureTriangleGeometryDescriptor`.
- [ ] `MTL4AccelerationStructureBoundingBoxGeometryDescriptor`.
- [ ] `MTL4AccelerationStructureMotionTriangleGeometryDescriptor`.
- [ ] `MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor`.
- [ ] `MTL4AccelerationStructureCurveGeometryDescriptor`.
- [ ] `MTL4AccelerationStructureMotionCurveGeometryDescriptor`.
- [ ] `MTL4InstanceAccelerationStructureDescriptor`.
- [ ] `MTL4IndirectInstanceAccelerationStructureDescriptor`.

### `MTL4Archive.h`

- [ ] `MTL4Archive`.
- [ ] Archive loading/serialization APIs.
- [ ] Archive function/pipeline lookup APIs.

### `MTL4ArgumentTable.h`

- [ ] `MTL4ArgumentTableDescriptor`.
- [ ] `MTL4ArgumentTable`.
- [ ] Buffer, texture, sampler, table, and acceleration-structure binding APIs.
- [ ] Device creation APIs.

### `MTL4BinaryFunction.h`

- [ ] `MTL4BinaryFunction`.
- [ ] Name/type/reflection metadata.

### `MTL4BinaryFunctionDescriptor.h`

- [ ] `MTL4BinaryFunctionDescriptor`.
- [ ] `MTL4BinaryFunctionOptions`.
- [ ] Function specialization options.

### `MTL4BufferRange.h`

- [ ] Buffer range structs/types.
- [ ] ABI layout tests.

### `MTL4CommandAllocator.h`

- [ ] `MTL4CommandAllocatorDescriptor`.
- [ ] `MTL4CommandAllocator`.
- [ ] Reset/reuse behavior.

### `MTL4CommandBuffer.h`

- [ ] `MTL4CommandBufferOptions`.
- [ ] `MTL4CommandBuffer`.
- [ ] Encoder creation APIs.
- [ ] Commit and feedback APIs.
- [ ] Residency/log-state integration.

### `MTL4CommandEncoder.h`

- [ ] `MTL4VisibilityOptions`.
- [ ] `MTL4CommandEncoder`.
- [ ] Base label/debug/visibility/resource APIs.

### `MTL4CommandQueue.h`

- [ ] `MTL4CommandQueueDescriptor`.
- [ ] `MTL4CommitOptions`.
- [ ] `MTL4CommandQueue`.
- [ ] `MTL4CommandQueueError`.
- [ ] Command buffer creation and commit APIs.

### `MTL4CommitFeedback.h`

- [ ] `MTL4CommitFeedback`.
- [ ] Status/timing/error metadata.

### `MTL4Compiler.h`

- [ ] `MTL4CompilerDescriptor`.
- [ ] `MTL4CompilerTaskOptions`.
- [ ] `MTL4Compiler`.
- [ ] Device compiler creation APIs.
- [ ] Compile/link task APIs.

### `MTL4CompilerTask.h`

- [ ] `MTL4CompilerTaskStatus`.
- [ ] `MTL4CompilerTask`.
- [ ] Status, result, error, and wait APIs.

### `MTL4ComputeCommandEncoder.h`

- [ ] `MTL4ComputeCommandEncoder`.
- [ ] Pipeline binding.
- [ ] Argument table binding.
- [ ] Dispatch APIs.
- [ ] Counter heap integration.

### `MTL4ComputePipeline.h`

- [ ] `MTL4ComputePipelineDescriptor`.
- [ ] Pipeline creation through compiler/device APIs.

### `MTL4Counters.h`

- [ ] `MTL4CounterHeapType`.
- [ ] `MTL4TimestampGranularity`.
- [ ] `MTL4CounterHeapDescriptor`.
- [ ] `MTL4CounterHeap`.
- [ ] Counter sampling/resolve APIs.

### `MTL4FunctionDescriptor.h`

- [ ] `MTL4FunctionDescriptor`.
- [ ] Base function descriptor properties.

### `MTL4LibraryDescriptor.h`

- [ ] `MTL4LibraryDescriptor`.
- [ ] Source/data/library inputs.

### `MTL4LibraryFunctionDescriptor.h`

- [ ] `MTL4LibraryFunctionDescriptor`.
- [ ] Library function selection.

### `MTL4LinkingDescriptor.h`

- [ ] `MTL4StaticLinkingDescriptor`.
- [ ] `MTL4PipelineStageDynamicLinkingDescriptor`.
- [ ] `MTL4RenderPipelineDynamicLinkingDescriptor`.

### `MTL4MachineLearningCommandEncoder.h`

- [ ] `MTL4MachineLearningCommandEncoder`.
- [ ] ML pipeline binding.
- [ ] Argument table binding.
- [ ] Dispatch APIs.

### `MTL4MachineLearningPipeline.h`

- [ ] `MTL4MachineLearningPipelineDescriptor`.
- [ ] `MTL4MachineLearningPipelineReflection`.
- [ ] `MTL4MachineLearningPipelineState`.

### `MTL4MeshRenderPipeline.h`

- [ ] `MTL4MeshRenderPipelineDescriptor`.
- [ ] Object/mesh function configuration.
- [ ] Mesh pipeline state creation.

### `MTL4PipelineDataSetSerializer.h`

- [ ] `MTL4PipelineDataSetSerializerConfiguration`.
- [ ] `MTL4PipelineDataSetSerializerDescriptor`.
- [ ] `MTL4PipelineDataSetSerializer`.

### `MTL4PipelineState.h`

- [ ] `MTL4ShaderReflection`.
- [ ] `MTL4AlphaToOneState`.
- [ ] `MTL4AlphaToCoverageState`.
- [ ] `MTL4BlendState`.
- [ ] `MTL4IndirectCommandBufferSupportState`.
- [ ] `MTL4PipelineOptions`.
- [ ] `MTL4PipelineDescriptor`.

### `MTL4RenderCommandEncoder.h`

- [ ] `MTL4RenderEncoderOptions`.
- [ ] `MTL4RenderCommandEncoder`.
- [ ] Render pipeline binding.
- [ ] Argument table binding.
- [ ] Draw APIs.
- [ ] Counter heap integration.

### `MTL4RenderPass.h`

- [ ] `MTL4RenderPassDescriptor`.
- [ ] Attachment configuration.
- [ ] Store/load/resolve behavior.

### `MTL4RenderPipeline.h`

- [ ] `MTL4LogicalToPhysicalColorAttachmentMappingState`.
- [ ] `MTL4RenderPipelineColorAttachmentDescriptor`.
- [ ] `MTL4RenderPipelineColorAttachmentDescriptorArray`.
- [ ] `MTL4RenderPipelineBinaryFunctionsDescriptor`.
- [ ] `MTL4RenderPipelineDescriptor`.

### `MTL4SpecializedFunctionDescriptor.h`

- [ ] `MTL4SpecializedFunctionDescriptor`.
- [ ] Function constants/specialization linkage.

### `MTL4StitchedFunctionDescriptor.h`

- [ ] `MTL4StitchedFunctionDescriptor`.
- [ ] Stitching graph/function integration.

### `MTL4TileRenderPipeline.h`

- [ ] `MTL4TileRenderPipelineDescriptor`.
- [ ] Tile function pipeline creation.

## Non-Metal Framework Surface Used By Metal Apps

These are not part of `Metal.framework`, but they matter for practical safe bindings around presentation.

### QuartzCore / `CAMetalLayer`

- [ ] Layer getters:
  - device
  - pixel format
  - framebuffer only
  - drawable size
  - presents with transaction
  - maximum drawable count
  - display sync enabled
  - allows next drawable timeout
  - colorspace
  - wants extended dynamic range content
- [ ] Layer setters:
  - maximum drawable count
  - display sync enabled
  - allows next drawable timeout
  - colorspace
  - EDR/HDR-related properties
- [ ] Drawable:
  - layer-backed drawable properties
  - present at time
  - present after minimum duration

### Foundation / Objective-C Helpers

- [ ] `NSArray` creation and extraction helpers.
- [ ] `NSDictionary` helpers for reflection dictionaries.
- [ ] `NSData` helpers for library/archive/data APIs.
- [ ] `NSURL` helpers beyond path URLs.
- [ ] `NSBundle` wrapper for default-library lookup.
- [ ] `NSError` domain/code/userInfo extraction.
- [ ] Safer Objective-C exception avoidance strategy for selectors known to throw.
- [ ] ABI tests for every typed `objc_msgSend` helper signature.

## Enum And Flag Completion Checklist

- [ ] `PixelFormat`
- [ ] `VertexFormat`
- [ ] `DataType`
- [ ] `TextureType`
- [ ] `TextureUsage`
- [ ] `ResourceOptions`
- [ ] `ResourceUsage`
- [ ] `StorageMode`
- [ ] `CpuCacheMode`
- [ ] `HazardTrackingMode`
- [ ] `PurgeableState`
- [ ] `LoadAction`
- [ ] `StoreAction`
- [ ] `StoreActionOptions`
- [ ] `PrimitiveType`
- [ ] `IndexType`
- [ ] `CompareFunction`
- [ ] `StencilOperation`
- [ ] `BlendFactor`
- [ ] `BlendOperation`
- [ ] `ColorWriteMask`
- [ ] `SamplerMinMagFilter`
- [ ] `SamplerMipFilter`
- [ ] `SamplerAddressMode`
- [ ] `SamplerBorderColor`
- [ ] `CullMode`
- [ ] `Winding`
- [ ] `DepthClipMode`
- [ ] `TriangleFillMode`
- [ ] `VisibilityResultMode`
- [ ] `RenderStages`
- [ ] `DispatchType`
- [ ] `FunctionOptions`
- [ ] `LibraryType`
- [ ] `LibraryOptimizationLevel`
- [ ] `LibraryError`
- [ ] `BinaryArchiveError`
- [ ] `CommandBufferStatus`
- [ ] `CommandBufferError`
- [ ] `CounterSamplingPoint`
- [ ] `CounterSampleBufferError`
- [ ] `IOStatus`
- [ ] `IOError`
- [ ] `IOPriority`
- [ ] `IOCommandQueueType`
- [ ] `IOCompressionStatus`
- [ ] `IOCompressionMethod`
- [ ] `SparsePageSize`
- [ ] `SparseTextureMappingMode`
- [ ] `AccelerationStructureUsage`
- [ ] `AccelerationStructureGeometryFlags`
- [ ] `AccelerationStructureInstanceOptions`
- [ ] `AccelerationStructureRefitOptions`
- [ ] `AccelerationStructureInstanceDescriptorType`
- [ ] `CurveType`
- [ ] `CurveBasis`
- [ ] `CurveEndCaps`
- [ ] `MotionBorderMode`
- [ ] `TransformType`
- [ ] `MatrixLayout`

## Test Coverage Still Needed

- [ ] A `tests/<module>.rs` file for every source module.
- [ ] One broad module test per file that exercises all safe permutations for that module.
- [ ] No examples as coverage substitutes.
- [ ] No silent success for unsupported APIs unless support was explicitly queried first.
- [ ] No swallowed Metal errors.
- [ ] No safe API path that can trigger a foreign exception or driver segfault.
- [ ] Discriminant tests against SDK header values for every enum and bitflag.
- [ ] ABI layout tests for every struct passed to Objective-C.
- [ ] Real Metal-device execution for command encoders, resource state, IO, counters, ray tracing, sparse resources, and indirect commands.
- [ ] `cargo check --all-targets`.
- [ ] `cargo test` with real Metal device access.
- [ ] `git diff --check`.
