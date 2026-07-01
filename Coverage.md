# Metal API Coverage Matrix

This file tracks the status of all Metal framework headers. Statuses can be `Bound`, `Partially Bound`, `Planned` (with phase), `Skipped` (with reason), or `N/A`.

| Header File | Binding Status | Phase | Notes / Excluded / Deprecated |
| :--- | :--- | :--- | :--- |
| `Metal.h` | Bound | V1 | Umbrella header |
| `MTLAccelerationStructure.h` | Bound | V6 | Bounding box / instance descriptors, type enum, options |
| `MTLAccelerationStructureCommandEncoder.h` | Bound | V6 | build, refit, copy, compact, write size, sample counters |
| `MTLAccelerationStructureTypes.h` | Bound | V6 | MTLPackedFloat3, MTLPackedFloat4x3, AxisAlignedBoundingBox |
| `MTLAllocation.h` | Skipped | - | Internal/system memory allocation tracking |
| `MTLArgument.h` | Bound | V3 | Argument descriptors and types |
| `MTLArgumentEncoder.h` | Bound | V6 | Array/range setters |
| `MTLBinaryArchive.h` | Bound | V3 | Binary archives configuration and creation |
| `MTLBlitCommandEncoder.h` | Bound | V6 | complete blit commands and counters |
| `MTLBlitPass.h` | Bound | V6 | MTLBlitPassDescriptor and counters |
| `MTLBuffer.h` | Bound | V6 | CPU read/write helpers, queries |
| `MTLCaptureManager.h` | Bound | V5 | MTLCaptureManager, MTLCaptureDescriptor |
| `MTLCaptureScope.h` | Bound | V5 | MTLCaptureScope |
| `MTLCommandBuffer.h` | Bound | V6 | pass descriptors, parallel command encoders, log states |
| `MTLCommandEncoder.h` | Bound | V1 | Base command encoder methods, labels |
| `MTLCommandQueue.h` | Bound | V1 | Command queue creation and labeling |
| `MTLComputeCommandEncoder.h` | Bound | V6 | acceleration structure / table bindings, array ranges |
| `MTLComputePass.h` | Bound | V6 | MTLComputePassDescriptor and counters |
| `MTLComputePipeline.h` | Bound | V6 | Reflection-free queries, linked functions |
| `MTLCounters.h` | Bound | V5 | Counter sets, sample buffers |
| `MTLDefines.h` | N/A | - | Compiler macros |
| `MTLDepthStencil.h` | Bound | V2 | Depth/stencil states and descriptors |
| `MTLDevice.h` | Bound | V6 | required system default, queries, new tables/logs |
| `MTLDeviceCertification.h` | Skipped | - | App Store / system certification APIs |
| `MTLDrawable.h` | Bound | V1 | Metal drawables |
| `MTLDynamicLibrary.h` | Bound | V6 | Dynamic library loading and serializing |
| `MTLEvent.h` | Bound | V3 | Shared events synchronization |
| `MTLFence.h` | Bound | V6 | Fence awaits/updates with stages |
| `MTLFunctionConstantValues.h` | Bound | V3 | Shader specialization constant values |
| `MTLFunctionDescriptor.h` | Bound | V6 | MTLFunctionDescriptor, MTLIntersectionFunctionDescriptor |
| `MTLFunctionHandle.h` | Bound | V6 | MTLFunctionHandle |
| `MTLFunctionLog.h` | Bound | V6 | Function log debugging location, logs |
| `MTLFunctionStitching.h` | Skipped | - | Stitching descriptors (complex runtime graph generation) |
| `MTLHeap.h` | Bound | V6 | Heap allocation for acceleration structures, staged resource use |
| `MTLIndirectCommandBuffer.h` | Bound | V3 | ICB options, execution |
| `MTLIndirectCommandEncoder.h` | Bound | V3 | ICB draw / dispatch commands |
| `MTLIntersectionFunctionTable.h` | Bound | V6 | Intersection function table creation / binding |
| `MTLIOCommandBuffer.h` | Planned | V7+ | IO/Fast-loading storage queues and command buffers |
| `MTLIOCommandQueue.h` | Planned | V7+ | IO/Fast-loading queue management |
| `MTLIOCompressor.h` | Planned | V7+ | File compression helper classes |
| `MTLLibrary.h` | Bound | V6 | Functions, custom constant compilation, logs |
| `MTLLinkedFunctions.h` | Bound | V6 | Linked functions for dynamic linking |
| `MTLLogState.h` | Bound | V6 | GPU log state and log level configuration |
| `MTLParallelRenderCommandEncoder.h` | Bound | V6 | Parallel render passes, child render encoders |
| `MTLPipeline.h` | Bound | V6 | Pipeline support flags |
| `MTLPixelFormat.h` | Bound | V2 | Enums for color/depth/stencil textures |
| `MTLRasterizationRate.h` | Skipped | - | Variable rate shading (VRS) descriptors (highly complex/platform-dependent) |
| `MTLRenderCommandEncoder.h` | Bound | V6 | indirect draw, range bindings, fences, tile/mesh shaders |
| `MTLRenderPass.h` | Bound | V6 | RenderPassDescriptor color/depth/stencil store action options |
| `MTLRenderPipeline.h` | Bound | V6 | RenderPipelineDescriptor linked functions, support flags, max buffers |
| `MTLResidencySet.h` | Skipped | - | macOS 14+ residency sets (managed heap is preferred) |
| `MTLResource.h` | Bound | V6 | Labels, hazard tracking, storage, options |
| `MTLResourceStateCommandEncoder.h` | Bound | V6 | Update texture mapping, staged fence updates/waits |
| `MTLResourceStatePass.h` | Bound | V6 | MTLResourceStatePassDescriptor |
| `MTLSampler.h` | Bound | V2 | Sampler descriptors and states |
| `MTLStageInputOutputDescriptor.h` | Bound | V2 | Vertex descriptors and FFI layouts |
| `MTLTexture.h` | Bound | V6 | Parent texture, usage, storage, format, buffer-backed texture |
| `MTLTypes.h` | Bound | V1 | Size, Region, Origin, Range |
| `MTLVertexDescriptor.h` | Bound | V2 | Vertex buffer layouts |
| `MTLVisibleFunctionTable.h` | Bound | V6 | Visible function table creation, indexing, binding |
