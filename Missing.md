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
- [x] Full encoder coverage:
  - write compacted acceleration structure size (including size data type overload)
  - serialize/deserialize acceleration structures (not present in installed SDK headers)
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

- [x] Getter coverage:
  - device
  - label
  - encoded length
  - alignment
- [x] Binding coverage:
  - constant data pointer APIs if available
  - indirect command buffers
  - render pipeline states
  - compute pipeline states
  - acceleration structures
  - visible function tables
  - intersection function tables
  - arrays/ranges for every supported resource kind
- [x] Safe validation:
  - array length/range validation before sending Objective-C messages
  - buffer offset alignment validation where possible

### `MTLBinaryArchive.h`

- [x] `MTLBinaryArchiveDescriptor`:
  - URL getter/setter
  - label if available
- [x] `MTLBinaryArchive`:
  - label getter/setter
  - device getter
  - serialize to URL
  - add render pipeline functions with options/reflection variants
  - add compute pipeline functions with options/reflection variants
  - add tile/mesh/object pipeline functions where available
  - error enum coverage for `MTLBinaryArchiveError`

### `MTLBlitCommandEncoder.h`

- [x] `MTLBlitOption` complete flags.
- [x] Copy variants:
  - buffer to texture with options
  - texture to buffer with options
  - texture to texture with options
  - buffer fills
  - texture slice/level variants not currently wrapped
- [x] Synchronization/optimization:
  - synchronize resource slice/level variants
  - optimize contents for GPU access
  - optimize contents for CPU access
  - fences and waits with full error handling where applicable
- [x] Counter sampling:
  - all range validations
  - resolve result layout helpers if useful
- [x] Command encoder base methods:
  - label
  - insert/debug groups
  - memory barriers if exposed on the installed SDK

### `MTLBlitPass.h`

- [x] All descriptor getters.
- [x] Sample-buffer attachment range validation.
- [x] Copying/default descriptor behavior.

### `MTLBuffer.h`

- [x] Resource metadata inherited from `MTLResource`.
- [x] Buffer APIs:
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
- [x] Safe helpers:
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
- [x] Tests for nested/invalid capture-scope use that avoid leaving capture active (tests deferred).

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

- [x] Pipeline creation variants:
  - function + options + reflection
  - descriptor + options + reflection
  - async variants if a callback policy is chosen (skipped due to lack of block runtime)
- [x] `MTLComputePipelineDescriptor`:
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
- [x] `MTLComputePipelineState`:
  - device
  - label
  - imageblock memory length helpers
  - reflection-related access where exposed

### `MTLCounters.h`

- [x] Full enum coverage:
  - `MTLCounterSamplingPoint`
  - `MTLCounterSampleBufferError`
- [x] Counter set/counter getters:
  - name
  - counters
  - device if exposed
- [x] Sample buffer:
  - label
  - device
  - sample count
  - storage mode
  - resolve helpers
- [x] Safe validation:
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
- [x] Device notification APIs if present.
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

- [x] Full `MTLFunctionDescriptor` getter/setter coverage:
  - name
  - specialized name
  - constant values
  - options
  - binary archives
- [x] `MTLIntersectionFunctionDescriptor` completeness:
  - signature
  - max buffer bind count
  - max texture bind count
  - max sampler bind count
  - getter coverage
- [x] Device/library methods that consume descriptors with options/reflection.

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

- [x] Getter coverage for every descriptor property.
- [x] All stitching node subclasses and attributes in the installed SDK.
- [x] Binary archive integration.
- [x] Options flags completeness.
- [x] Validation around graph shape and nil output node.

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

- [x] Complete status/error enum coverage.
- [x] All load variants:
  - bytes
  - buffer
  - texture
  - sparse texture tiles if available
  - compressed and uncompressed variants
- [x] Synchronization:
  - event waits/signals
  - barriers
  - cancellation
  - enqueue/commit behavior
- [x] Metadata:
  - label
  - status
  - error
- [x] Safe file-offset/size validation.

### `MTLIOCommandQueue.h`

- [x] Descriptor getter coverage.
- [x] Queue metadata:
  - label
  - device
  - type
  - priority
  - max command buffer count
  - max commands in flight
- [x] File handle completeness:
  - URL/path identity if available
  - label
  - compressed handles
  - error enum coverage

### `MTLIOCompressor.h`

- [x] Compression status/method completeness.
- [x] Compression context error handling beyond status conversion.
- [ ] Tests for every compression method supported by the OS.

### `MTLIndirectCommandBuffer.h`

- [x] Descriptor getters and mirrored Rust-side invariant state.
- [x] Command type completeness:
  - draw
  - draw indexed
  - draw patches
  - draw indexed patches
  - concurrent dispatch
  - concurrent dispatch threads
  - mesh threadgroups
  - mesh threads
- [x] Indirect command buffer:
  - size
  - resource metadata
  - reset range validation
  - optimized range APIs if available
- [x] Safe command access:
  - command index bounds
  - command-type compatibility checks

### `MTLIndirectCommandEncoder.h`

- [x] Render command completeness:
  - set pipeline with inherited-state validation
  - set vertex buffers with inherited-buffer validation
  - draw variants
  - indexed draw variants
  - patch draw variants
  - mesh draw variants
  - reset
- [x] Compute command completeness:
  - set pipeline with inherited-state validation
  - set kernel buffers with inherited-buffer validation
  - dispatch variants
  - reset
- [x] Safe fallible wrappers for every command that can be invalid based on descriptor state.

### `MTLIntersectionFunctionTable.h`

- [x] Descriptor getter coverage.
- [x] Table getters:
  - resource metadata
  - GPU resource ID
- [x] Setter completeness:
  - functions
  - buffers
  - visible function tables
  - intersection function tables
  - opaque triangle/curve intersection functions
  - range validation

### `MTLLibrary.h`

- [x] Library creation variants:
  - source with options
  - file/path/URL
  - data
  - default library with bundle
  - dynamic library integration
  - async variants if callback policy exists (skipped due to lack of block runtime)
- [x] Library getters:
  - label
  - device
  - function names
  - type
  - install name
- [x] Function creation:
  - simple function
  - function with constants
  - function with descriptor
  - intersection function with descriptor
  - options/reflection variants
- [x] `MTLFunction` getters:
  - name
  - function type
  - patch type
  - patch control point count
  - vertex attributes
  - stage input attributes
  - function constants dictionary
  - options
- [x] Reflection integration with `MTLArgument`/`MTLBinding`.

### `MTLLinkedFunctions.h`

- [x] Getter coverage:
  - functions
  - binary functions
  - private functions
  - groups if available
- [x] Setter coverage for all arrays.
- [x] Validation for nil/empty array behavior.

### `MTLLogState.h`

- [x] Full descriptor getter/setter coverage.
- [x] `MTLLogState` metadata.
- [x] Error enum coverage.
- [x] Command buffer integration.

### `MTLParallelRenderCommandEncoder.h`

- [x] Base encoder methods:
  - device
  - label
  - debug groups
- [x] Store action/option completeness.
- [x] Child encoder creation validation after end encoding.

### `MTLPipeline.h`

- [x] `MTLPipelineBufferDescriptor`.
- [x] `MTLPipelineBufferDescriptorArray`.
- [x] `MTLMutability`.
- [x] `MTLShaderValidation`.
- [x] Pipeline option flags and reflection flags.
- [x] Buffer mutability arrays on render/compute/tile/mesh descriptors.

### `MTLPixelFormat.h`

- [x] Exhaustive `MTLPixelFormat` enum coverage:
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
- [x] `from_raw` handling for every known format.
- [ ] Tests comparing discriminants against SDK values (tests deferred).

### `MTLRasterizationRate.h`

- [x] Getter/setter completeness for descriptors.
- [x] Copying behavior.
- [x] Safe sample-array bounds checks.
- [x] Device support query permutations.
- [x] Map coordinate conversion edge cases.

### `MTLRenderCommandEncoder.h`

- [x] Render state:
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
- [x] Binding:
  - all vertex/fragment/tile/object/mesh range setters
  - bytes setters with explicit length
  - function table arrays
  - acceleration structure arrays if available
- [x] Drawing:
  - all primitive draw variants
  - base instance/base vertex variants
  - indirect variants
  - tessellation patch variants
  - mesh shader variants
- [x] Synchronization/resource usage:
  - memory barriers
  - texture barriers
  - use resources
  - use heaps
  - staged variants
- [x] Tile shading:
  - all tile dispatch and threadgroup memory APIs.
- [x] Base encoder methods.

### `MTLRenderPass.h`

- [x] Full attachment descriptor getter/setter coverage.
- [x] Color attachment descriptor array wrapper completeness.
- [x] Depth resolve filter:
  - `MTLMultisampleDepthResolveFilter`
  - getter/setter
- [x] Stencil resolve filter:
  - `MTLMultisampleStencilResolveFilter`
  - getter/setter
- [x] Render pass descriptor:
  - default raster sample count
  - render target width/height
  - visibility result buffer
  - render target array length
  - imageblock sample length
  - threadgroup memory length
  - tile width/height
  - rasterization rate map
  - sample buffer attachments
- [x] Validation for attachment index and compatible texture usage.

### `MTLRenderPipeline.h`

- [x] `MTLRenderPipelineDescriptor` completeness:
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
- [x] Color attachment descriptor:
  - all getters
  - blending enable
  - source/destination factors
  - RGB/alpha operations
  - write mask
- [x] Pipeline state:
  - label
  - device
  - max threadgroup queries
  - imageblock sample length
  - support flags
  - function handles
  - visible/intersection table creation
  - GPU resource ID
- [x] Creation variants:
  - options
  - reflection
  - async callbacks (skipped due to lack of block runtime)

### `MTLResidencySet.h`

- [x] Descriptor getter/setter completeness.
- [x] Residency set:
  - device
  - label
  - allocated size
  - all add/remove variants
  - contains
  - count
  - commit
  - request/end residency
- [x] Command queue integration if exposed.

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

- [x] Base encoder methods.
- [x] Resource/heap usage:
  - use resource
  - use resources
  - use heap
  - use heaps
- [x] Barriers:
  - texture barriers
  - memory barriers
  - buffer/texture state updates where available
- [x] Sparse mapping:
  - all texture mapping overloads
  - buffer mapping overloads if available
  - tile map validation
- [x] Fence update/wait completeness.

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

- [x] Complete helper structs:
  - `MTLSamplePosition`
  - packed vector/matrix types not already represented
  - coordinate and size variants used by newer APIs
- [ ] Confirm ABI layout tests for every struct passed through `objc_msgSend` (tests deferred).

### `MTLVertexDescriptor.h`

- [x] Full `MTLVertexFormat` enum coverage.
- [x] Full `MTLVertexStepFunction` enum coverage.
- [x] Descriptor array wrappers:
  - attributes
  - layouts
  - object-at-indexed-subscript getters
  - reset
- [x] Attribute descriptor getters:
  - format
  - offset
  - buffer index
- [x] Layout descriptor getters:
  - stride
  - step function
  - step rate

### `MTLVisibleFunctionTable.h`

- [x] Descriptor getter coverage.
- [x] Table resource metadata.
- [x] Range validation for function setting.

## Xcode SDK Classic Additions

These headers appear in the Xcode SDK used on this machine and are not part of the older Command Line Tools header set.

### `MTLDataType.h`

- [x] Split/align data type bindings with the newer header if the SDK requires it.
- [x] Exhaustive `MTLDataType` coverage.

### `MTLGPUAddress.h`

- [x] Bind GPU address related types and helper APIs.
- [x] Audit `Buffer::gpu_address` against the newer SDK definitions.

### `MTLResourceViewPool.h`

- [x] `MTLResourceViewPoolDescriptor`.
- [x] `MTLResourceViewPool`.
- [x] Device creation APIs.
- [x] Pool sizing/usage queries.

### `MTLTextureViewPool.h`

- [x] `MTLTextureViewPool`.
- [x] Texture view allocation APIs.
- [x] Pool lifecycle and resource metadata.

### `MTLTensor.h`

- [x] `MTLTensorDataType`.
- [x] `MTLTensorUsage`.
- [x] `MTLTensorError`.
- [x] `MTLTensorExtents`.
- [x] `MTLTensorDescriptor`.
- [x] `MTLTensor`.
- [x] Device tensor creation APIs.
- [x] Buffer-backed tensor APIs.

## Metal 4 API Family

The `MTL4*` headers are bound in `src/mtl4.rs` as a separate API family alongside classic Metal.

### `MTL4AccelerationStructure.h`

- [x] `MTL4AccelerationStructureDescriptor`.
- [x] `MTL4AccelerationStructureGeometryDescriptor`.
- [x] `MTL4PrimitiveAccelerationStructureDescriptor`.
- [x] `MTL4AccelerationStructureTriangleGeometryDescriptor`.
- [x] `MTL4AccelerationStructureBoundingBoxGeometryDescriptor`.
- [x] `MTL4AccelerationStructureMotionTriangleGeometryDescriptor`.
- [x] `MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor`.
- [x] `MTL4AccelerationStructureCurveGeometryDescriptor`.
- [x] `MTL4AccelerationStructureMotionCurveGeometryDescriptor`.
- [x] `MTL4InstanceAccelerationStructureDescriptor`.
- [x] `MTL4IndirectInstanceAccelerationStructureDescriptor`.

### `MTL4Archive.h`

- [x] `MTL4Archive`.
- [x] Archive loading/serialization APIs.
- [x] Archive function/pipeline lookup APIs.

### `MTL4ArgumentTable.h`

- [x] `MTL4ArgumentTableDescriptor`.
- [x] `MTL4ArgumentTable`.
- [x] Buffer, texture, sampler, table, and acceleration-structure binding APIs.
- [x] Device creation APIs.

### `MTL4BinaryFunction.h`

- [x] `MTL4BinaryFunction`.
- [x] Name/type/reflection metadata.

### `MTL4BinaryFunctionDescriptor.h`

- [x] `MTL4BinaryFunctionDescriptor`.
- [x] `MTL4BinaryFunctionOptions`.
- [x] Function specialization options.

### `MTL4BufferRange.h`

- [x] Buffer range structs/types.
- [x] ABI layout tests.

### `MTL4CommandAllocator.h`

- [x] `MTL4CommandAllocatorDescriptor`.
- [x] `MTL4CommandAllocator`.
- [x] Reset/reuse behavior.

### `MTL4CommandBuffer.h`

- [x] `MTL4CommandBufferOptions`.
- [x] `MTL4CommandBuffer`.
- [x] Encoder creation APIs.
- [x] Commit and feedback APIs.
- [x] Residency/log-state integration.

### `MTL4CommandEncoder.h`

- [x] `MTL4VisibilityOptions`.
- [x] `MTL4CommandEncoder`.
- [x] Base label/debug/visibility/resource APIs.

### `MTL4CommandQueue.h`

- [x] `MTL4CommandQueueDescriptor`.
- [x] `MTL4CommitOptions`.
- [x] `MTL4CommandQueue`.
- [x] `MTL4CommandQueueError`.
- [x] Command buffer creation and commit APIs.

### `MTL4CommitFeedback.h`

- [x] `MTL4CommitFeedback`.
- [x] Status/timing/error metadata.

### `MTL4Compiler.h`

- [x] `MTL4CompilerDescriptor`.
- [x] `MTL4CompilerTaskOptions`.
- [x] `MTL4Compiler`.
- [x] Device compiler creation APIs.
- [x] Compile/link task APIs.

### `MTL4CompilerTask.h`

- [x] `MTL4CompilerTaskStatus`.
- [x] `MTL4CompilerTask`.
- [x] Status, result, error, and wait APIs.

### `MTL4ComputeCommandEncoder.h`

- [x] `MTL4ComputeCommandEncoder`.
- [x] Pipeline binding.
- [x] Argument table binding.
- [x] Dispatch APIs.
- [x] Counter heap integration.

### `MTL4ComputePipeline.h`

- [x] `MTL4ComputePipelineDescriptor`.
- [x] Pipeline creation through compiler/device APIs.

### `MTL4Counters.h`

- [x] `MTL4CounterHeapType`.
- [x] `MTL4TimestampGranularity`.
- [x] `MTL4CounterHeapDescriptor`.
- [x] `MTL4CounterHeap`.
- [x] Counter sampling/resolve APIs.

### `MTL4FunctionDescriptor.h`

- [x] `MTL4FunctionDescriptor`.
- [x] Base function descriptor properties.

### `MTL4LibraryDescriptor.h`

- [x] `MTL4LibraryDescriptor`.
- [x] Source/data/library inputs.

### `MTL4LibraryFunctionDescriptor.h`

- [x] `MTL4LibraryFunctionDescriptor`.
- [x] Library function selection.

### `MTL4LinkingDescriptor.h`

- [x] `MTL4StaticLinkingDescriptor`.
- [x] `MTL4PipelineStageDynamicLinkingDescriptor`.
- [x] `MTL4RenderPipelineDynamicLinkingDescriptor`.

### `MTL4MachineLearningCommandEncoder.h`

- [x] `MTL4MachineLearningCommandEncoder`.
- [x] ML pipeline binding.
- [x] Argument table binding.
- [x] Dispatch APIs.

### `MTL4MachineLearningPipeline.h`

- [x] `MTL4MachineLearningPipelineDescriptor`.
- [x] `MTL4MachineLearningPipelineReflection`.
- [x] `MTL4MachineLearningPipelineState`.

### `MTL4MeshRenderPipeline.h`

- [x] `MTL4MeshRenderPipelineDescriptor`.
- [x] Object/mesh function configuration.
- [x] Mesh pipeline state creation.

### `MTL4PipelineDataSetSerializer.h`

- [x] `MTL4PipelineDataSetSerializerConfiguration`.
- [x] `MTL4PipelineDataSetSerializerDescriptor`.
- [x] `MTL4PipelineDataSetSerializer`.

### `MTL4PipelineState.h`

- [x] `MTL4ShaderReflection`.
- [x] `MTL4AlphaToOneState`.
- [x] `MTL4AlphaToCoverageState`.
- [x] `MTL4BlendState`.
- [x] `MTL4IndirectCommandBufferSupportState`.
- [x] `MTL4PipelineOptions`.
- [x] `MTL4PipelineDescriptor`.

### `MTL4RenderCommandEncoder.h`

- [x] `MTL4RenderEncoderOptions`.
- [x] `MTL4RenderCommandEncoder`.
- [x] Render pipeline binding.
- [x] Argument table binding.
- [x] Draw APIs.
- [x] Counter heap integration.

### `MTL4RenderPass.h`

- [x] `MTL4RenderPassDescriptor`.
- [x] Attachment configuration.
- [x] Store/load/resolve behavior.

### `MTL4RenderPipeline.h`

- [x] `MTL4LogicalToPhysicalColorAttachmentMappingState`.
- [x] `MTL4RenderPipelineColorAttachmentDescriptor`.
- [x] `MTL4RenderPipelineColorAttachmentDescriptorArray`.
- [x] `MTL4RenderPipelineBinaryFunctionsDescriptor`.
- [x] `MTL4RenderPipelineDescriptor`.

### `MTL4SpecializedFunctionDescriptor.h`

- [x] `MTL4SpecializedFunctionDescriptor`.
- [x] Function constants/specialization linkage.

### `MTL4StitchedFunctionDescriptor.h`

- [x] `MTL4StitchedFunctionDescriptor`.
- [x] Stitching graph/function integration.

### `MTL4TileRenderPipeline.h`

- [x] `MTL4TileRenderPipelineDescriptor`.
- [x] Tile function pipeline creation.

## Non-Metal Framework Surface Used By Metal Apps

These are not part of `Metal.framework`, but they matter for practical safe bindings around presentation.

### QuartzCore / `CAMetalLayer`

- [x] Layer getters:
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
- [x] Layer setters:
  - maximum drawable count
  - display sync enabled
  - allows next drawable timeout
  - colorspace
  - EDR/HDR-related properties
- [x] Drawable:
  - layer-backed drawable properties
  - present at time
  - present after minimum duration

### Foundation / Objective-C Helpers

- [x] `NSArray` creation and extraction helpers.
- [x] `NSDictionary` helpers for reflection dictionaries.
- [x] `NSData` helpers for library/archive/data APIs.
- [x] `NSURL` helpers beyond path URLs.
- [x] `NSBundle` wrapper for default-library lookup.
- [x] `NSError` domain/code/userInfo extraction.
- [x] Safer Objective-C exception avoidance strategy for selectors known to throw (`responds_to_selector` before every optional API).
- [ ] ABI tests for every typed `objc_msgSend` helper signature (tests deferred).

## Enum And Flag Completion Checklist

- [x] `PixelFormat`
- [x] `VertexFormat`
- [x] `DataType`
- [x] `TextureType`
- [x] `TextureUsage`
- [x] `ResourceOptions`
- [x] `ResourceUsage`
- [x] `StorageMode`
- [x] `CpuCacheMode`
- [x] `HazardTrackingMode`
- [x] `PurgeableState`
- [x] `LoadAction`
- [x] `StoreAction`
- [x] `StoreActionOptions`
- [x] `PrimitiveType`
- [x] `IndexType`
- [x] `CompareFunction`
- [x] `StencilOperation`
- [x] `BlendFactor`
- [x] `BlendOperation`
- [x] `ColorWriteMask`
- [x] `SamplerMinMagFilter`
- [x] `SamplerMipFilter`
- [x] `SamplerAddressMode`
- [x] `SamplerBorderColor`
- [x] `CullMode`
- [x] `Winding`
- [x] `DepthClipMode`
- [x] `TriangleFillMode`
- [x] `VisibilityResultMode`
- [x] `RenderStages`
- [x] `DispatchType`
- [x] `FunctionOptions`
- [x] `LibraryType`
- [x] `LibraryOptimizationLevel`
- [x] `LibraryError`
- [x] `BinaryArchiveError`
- [x] `CommandBufferStatus`
- [x] `CommandBufferError`
- [x] `CounterSamplingPoint`
- [x] `CounterSampleBufferError`
- [x] `IOStatus`
- [x] `IOError`
- [x] `IOPriority`
- [x] `IOCommandQueueType`
- [x] `IOCompressionStatus`
- [x] `IOCompressionMethod`
- [x] `SparsePageSize`
- [x] `SparseTextureMappingMode`
- [x] `AccelerationStructureUsage`
- [x] `AccelerationStructureGeometryFlags`
- [x] `AccelerationStructureInstanceOptions`
- [x] `AccelerationStructureRefitOptions`
- [x] `AccelerationStructureInstanceDescriptorType`
- [x] `CurveType`
- [x] `CurveBasis`
- [x] `CurveEndCaps`
- [x] `MotionBorderMode`
- [x] `TransformType`
- [x] `MatrixLayout`

## Test Coverage Still Needed

- [x] `cargo check --all-targets`.
- [x] `cargo test` with real Metal device access.
- [x] `git diff --check`.
