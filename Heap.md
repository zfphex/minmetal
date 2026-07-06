 ### File-by-File Heap Allocation Audit

  #### 1. lib.rs

  • Status: No allocations in library code.
  • Note: Contains some heap allocations ( to_string() , etc.) inside the  mod tests  block, but these are for
  testing only.

  #### 2. ffi.rs

  • Allocations found:
      • ffi.rs: Allocates a  Vec<id>  on the heap ( Vec::with_capacity(count) ).
      • ffi.rs: Allocates a Rust  String  via  CStr::to_str().ok().map(ToOwned::to_owned) .
      • ffi.rs: Returns a heap-allocated  String  (and constructs temporary  NSString  via  NSString::new ).
      • ffi.rs: Allocates an  NSString  on the Objective-C heap.
      • ffi.rs: Returns a heap-allocated  Vec<u8> .
      • ffi.rs & ffi.rs: Return heap-allocated  Option<String> .


  #### 3. capture.rs

  • Allocations found:
      • capture.rs: Returns  Option<String> .
      • capture.rs: Allocates an  NSString  via  ns_url_from_path .
      • capture.rs: Returns  Option<String> .
      • capture.rs: Allocates an  NSString  on the Objective-C heap.
      • Error paths: Functions returning  Result<T, MetalError>  allocate a  MetalError  on the heap if they fail,
      since  MetalError  contains a  String  message.


  #### 4. counters.rs

  • Allocations found:
      • counters.rs & counters.rs: Return a heap-allocated  String .
      • counters.rs: Returns a heap-allocated  Vec<Counter> .
      • counters.rs & counters.rs: Return  Option<String> .
      • counters.rs: Allocates an  NSString  on the Objective-C heap.
      • counters.rs: Returns  Result<Vec<u8>, MetalError>  (heap-allocated  Vec<u8>  and error path allocation).
      • counters.rs: Returns  Result<Vec<CounterSet>, MetalError>  (heap-allocated  Vec<CounterSet> ).


  #### 5. device.rs

  • Allocations found:
      • device.rs: Returns a  Vec<String> .
      • device.rs: Returns a  Vec<Device> .
      • device.rs: Returns  String .
      • device.rs: Returns  Vec<Device> .
      • device.rs: Dynamically boxes the closure via  Box::new(handler)  and returns a  Vec<Device> .
      • device.rs: Returns a  Vec<VertexAttribute> .
      • device.rs: Returns  Option<String> .
      • device.rs & device.rs: Return  Vec<Binding> .
      • device.rs: Returns  Vec<FunctionLog> .


  #### 6. encoder.rs

  • Allocations found:
      • All command encoders ( RenderCommandEncoder ,  ComputeCommandEncoder ,  BlitCommandEncoder , etc.) use heap
      allocations in  label()  (returns  Option<String> ),  set_label()  (allocates  NSString ), and debug signpost
      methods like  push_debug_group  /  insert_debug_signpost  (allocate  NSString s).
      • Error paths return  Result<(), MetalError>  which can heap allocate a  MetalError  message.


  #### 7. indirect.rs

  • Allocations found:
      • Error paths return  Result<(), MetalError> .


  #### 8. io.rs

  • Allocations found:
      •  label()  /  set_label()  allocate  String / NSString .
      • Error paths returning  Result<T, MetalError>  allocate error messages.


  #### 9. layer.rs

  • Status: No allocations.

  #### 10. mtl4.rs

  • Allocations found:
      • mtl4.rs, mtl4.rs, and mtl4.rs: Return heap-allocated  Vec s.
      • mtl4.rs: Returns  Vec<M4DynamicLibrary> .
      • mtl4.rs: Returns  Vec<Binding> .
      • mtl4.rs: Returns  Vec<M4AccelerationStructureGeometryDescriptor> .
      • mtl4.rs: Allocates a temporary  Vec<id>  on the heap to convert raw descriptors to an  NSArray .
      • Getters/setters for name, label, and source fields allocate  String / NSString .


  #### 11. pass.rs

  • Allocations found:
      •  label()  /  set_label()  and debug signpost methods allocate  String / NSString .
      • Error paths return  Result<(), MetalError> .


  #### 12. pipeline.rs

  • Allocations found:
      • pipeline.rs: Returns  Vec<Function> .
      • pipeline.rs: Returns  Vec<Function> .
      • pipeline.rs: Returns  Result<Vec<BinaryArchive>, MetalError> .
      • pipeline.rs: Returns  Vec<Function> .
      • pipeline.rs: Returns  Vec<(String, Vec<Function>)>  (highly allocating due to returning nested  Vec s and
      String s).
      • Getters/setters for label, name, installName, functionName allocate  String / NSString .
      • Error paths returning  Result<T, MetalError>  allocate error messages.


  #### 13. rasterization_rate.rs

  • Allocations found:
      •  label()  /  set_label()  allocate  String / NSString .
      • Error paths return  Result<(), MetalError> .


  #### 14. raytracing.rs

  • Allocations found:
      • raytracing.rs: Returns  Vec<AccelerationStructureGeometryDescriptor> .
      • raytracing.rs: Returns  Vec<AccelerationStructure> .
      •  label()  /  set_label()  and debug signposts allocate  String / NSString .


  #### 15. reflection.rs

  • Allocations found:
      • reflection.rs: Returns  Option<String> .
      • reflection.rs: Returns  Vec<StructMember> .
      • reflection.rs: Returns  String .
      • reflection.rs: Returns  Vec<Binding> .
      • reflection.rs: Returns  Vec<String> .


  #### 16. residency.rs

  • Allocations found:
      •  label()  /  set_label()  allocate  String / NSString .


  #### 17. resource.rs

  • Allocations found:
      •  label()  /  set_label()  /  marker  allocate  String / NSString .
      • Error paths returning  Result<T, MetalError>  allocate error messages.


  #### 18. sparse.rs

  • Status: No allocations.

  #### 19. stitching.rs

  • Allocations found:
      • stitching.rs / stitching.rs: Allocate temporary  Vec<id>  instances on the heap to convert input slices
      to  NSArray .
      • stitching.rs: Returns  Vec<FunctionStitchingNode> .
      • stitching.rs: Returns  Vec<FunctionStitchingFunctionNode> .
      •  name ,  function_name ,  set_name ,  set_function_name  allocate  String / NSString .
      • stitching.rs & stitching.rs: Return heap-allocated  Vec s.
      • stitching.rs, stitching.rs, and stitching.rs: Return heap-allocated  Vec s.
      • All corresponding array setters (e.g.  set_nodes ,  set_attributes ,  set_function_graphs ,  set_functions )
      allocate temporary  Vec<id>  instances.


  #### 20. tensor.rs

  • Allocations found:
      •  label()  /  set_label()  allocate  String / NSString .


  #### 21. types.rs

  • Allocations found:
      • types.rs: Holds a  message: String , which is heap-allocated.


  #### 22. view_pool.rs

  • Allocations found:
      •  label()  /  set_label()  allocate  String / NSString .

  ──────
  ### Proposed Fix Strategies for Discussion

  To eliminate all heap allocations, we can implement the following strategies:

  #### 1. Eliminate  Vec  Returns (Retrieving Arrays)

  Instead of returning  Vec<T> :

  • Option A: Custom Iterators (Zero Allocation):
  We can implement custom iterator types that wrap the raw  NSArray  pointer and index.
    pub struct CounterSetIterator {
        array: id,
        index: usize,
        count: usize,
    }
    impl Iterator for CounterSetIterator {
        type Item = CounterSet;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.count { return None; }
            let item = retain(msg_id_usize(self.array, sel(b"objectAtIndex:\0"), self.index));
            self.index += 1;
            (!item.is_null()).then_some(CounterSet { raw: item })
        }
    }
    
  • Option B: Visitor / Callback Pattern:
    pub fn for_each_counter_set(&self, mut f: impl FnMut(CounterSet)) { ... }
    

  #### 2. Eliminate Temporary  Vec  Allocations (Passing Arrays)

  Instead of allocating a temporary  Vec<id>  to pass to  NSArray  creation in setters:

  • Stack buffer with Heap Fallback:
  Use a fixed stack buffer for the common case (e.g. up to 32 or 64 elements) to avoid allocation.
    let mut stack_buf = [std::ptr::null_mut(); 32];
    let raw = if descriptors.len() <= 32 {
        for (i, d) in descriptors.iter().enumerate() {
            stack_buf[i] = d.raw;
        }
        &stack_buf[..descriptors.len()]
    } else {
        descriptors.iter().map(|d| d.raw).collect::<Vec<_>>()
    };
    

  #### 3. Eliminate  String  /  NSString  Allocations

  Instead of returning  String  or  Option<String>  for labels, names, or domains:

  • Option A: Borrowed / Callback-based String Access:
  Query the  UTF8String  pointer from the  NSString  directly and pass it to a closure.
    pub fn with_label<R>(&self, f: impl FnOnce(Option<&str>) -> R) -> R {
        let raw = msg_id(self.raw, sel(b"label\0"));
        if raw.is_null() {
            f(None)
        } else {
            unsafe {
                let ptr = utf8_ptr(raw); // UTF8String selector
                let cstr = std::ffi::CStr::from_ptr(ptr);
                f(cstr.to_str().ok())
            }
        }
    }

  • Option B: Stack-based string conversion:
  Accept a user-provided buffer to write strings into.