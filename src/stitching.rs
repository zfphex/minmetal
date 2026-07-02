use crate::*;
use std::ffi::c_void;
use std::mem::transmute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StitchedLibraryOptions(pub usize);

impl StitchedLibraryOptions {
    pub const NONE: Self = Self(0);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 0);
    pub const STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT: Self = Self(1 << 1);

    pub const fn as_raw(self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct FunctionStitchingAttribute {
    pub raw: id,
}

impl Drop for FunctionStitchingAttribute {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct FunctionStitchingAttributeAlwaysInline {
    pub raw: id,
}

impl FunctionStitchingAttributeAlwaysInline {
    pub fn new() -> Self {
        let raw = msg_id(
            class(b"MTLFunctionStitchingAttributeAlwaysInline\0"),
            sel(b"alloc\0"),
        );
        let raw = msg_id(raw, sel(b"init\0"));
        Self { raw }
    }
}

impl Drop for FunctionStitchingAttributeAlwaysInline {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl From<FunctionStitchingAttributeAlwaysInline> for FunctionStitchingAttribute {
    fn from(attr: FunctionStitchingAttributeAlwaysInline) -> Self {
        let raw = attr.raw;
        std::mem::forget(attr);
        Self { raw }
    }
}

#[derive(Debug)]
pub struct FunctionStitchingNode {
    pub raw: id,
}

impl Drop for FunctionStitchingNode {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct FunctionStitchingInputNode {
    pub raw: id,
}

impl FunctionStitchingInputNode {
    pub fn new(argument_index: usize) -> Self {
        let raw = msg_id(class(b"MTLFunctionStitchingInputNode\0"), sel(b"alloc\0"));
        let raw = msg_id_usize(raw, sel(b"initWithArgumentIndex:\0"), argument_index);
        Self { raw }
    }

    pub fn argument_index(&self) -> usize {
        msg_usize(self.raw, sel(b"argumentIndex\0"))
    }

    pub fn set_argument_index(&self, index: usize) {
        msg_void_usize(self.raw, sel(b"setArgumentIndex:\0"), index);
    }
}

impl Drop for FunctionStitchingInputNode {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl From<FunctionStitchingInputNode> for FunctionStitchingNode {
    fn from(node: FunctionStitchingInputNode) -> Self {
        let raw = node.raw;
        std::mem::forget(node);
        Self { raw }
    }
}

#[derive(Debug)]
pub struct FunctionStitchingFunctionNode {
    pub raw: id,
}

impl FunctionStitchingFunctionNode {
    pub fn new(
        name: &str,
        arguments: &[&FunctionStitchingNode],
        control_dependencies: &[&FunctionStitchingFunctionNode],
    ) -> Self {
        unsafe {
            let ns_name = NSString::new(name);
            let raw_args: Vec<id> = arguments.iter().map(|n| n.raw).collect();
            let ns_args = ns_array_from_ids(&raw_args);

            let raw_deps: Vec<id> = control_dependencies.iter().map(|n| n.raw).collect();
            let ns_deps = ns_array_from_ids(&raw_deps);

            let allocated = msg_id(
                class(b"MTLFunctionStitchingFunctionNode\0"),
                sel(b"alloc\0"),
            );
            let init: unsafe extern "C" fn(id, SEL, id, id, id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = init(
                allocated,
                sel(b"initWithName:arguments:controlDependencies:\0"),
                ns_name.raw(),
                ns_args,
                ns_deps,
            );
            Self { raw }
        }
    }

    pub fn name(&self) -> String {
        ns_string_to_string(msg_id(self.raw, sel(b"name\0"))).unwrap_or_default()
    }

    pub fn set_name(&self, name: &str) {
        let ns_name = NSString::new(name);
        msg_void_id(self.raw, sel(b"setName:\0"), ns_name.raw());
    }

    pub fn arguments(&self) -> Vec<FunctionStitchingNode> {
        let array = msg_id(self.raw, sel(b"arguments\0"));
        ns_array_to_vec(array)
            .into_iter()
            .map(|raw| FunctionStitchingNode { raw: retain(raw) })
            .collect()
    }

    pub fn set_arguments(&self, arguments: &[&FunctionStitchingNode]) {
        let raw_args: Vec<id> = arguments.iter().map(|n| n.raw).collect();
        let array = ns_array_from_ids(&raw_args);
        msg_void_id(self.raw, sel(b"setArguments:\0"), array);
    }

    pub fn control_dependencies(&self) -> Vec<FunctionStitchingFunctionNode> {
        let array = msg_id(self.raw, sel(b"controlDependencies\0"));
        ns_array_to_vec(array)
            .into_iter()
            .map(|raw| FunctionStitchingFunctionNode { raw: retain(raw) })
            .collect()
    }

    pub fn set_control_dependencies(
        &self,
        control_dependencies: &[&FunctionStitchingFunctionNode],
    ) {
        let raw_deps: Vec<id> = control_dependencies.iter().map(|n| n.raw).collect();
        let array = ns_array_from_ids(&raw_deps);
        msg_void_id(self.raw, sel(b"setControlDependencies:\0"), array);
    }
}

impl Drop for FunctionStitchingFunctionNode {
    fn drop(&mut self) {
        release(self.raw);
    }
}

impl From<FunctionStitchingFunctionNode> for FunctionStitchingNode {
    fn from(node: FunctionStitchingFunctionNode) -> Self {
        let raw = node.raw;
        std::mem::forget(node);
        Self { raw }
    }
}

#[derive(Debug)]
pub struct FunctionStitchingGraph {
    pub raw: id,
}

impl FunctionStitchingGraph {
    pub fn new(
        function_name: &str,
        nodes: &[&FunctionStitchingFunctionNode],
        output_node: Option<&FunctionStitchingFunctionNode>,
        attributes: &[&FunctionStitchingAttribute],
    ) -> Self {
        unsafe {
            let ns_name = NSString::new(function_name);

            let raw_nodes: Vec<id> = nodes.iter().map(|n| n.raw).collect();
            let ns_nodes = ns_array_from_ids(&raw_nodes);

            let raw_output = output_node.map(|n| n.raw).unwrap_or(NIL);

            let raw_attrs: Vec<id> = attributes.iter().map(|a| a.raw).collect();
            let ns_attrs = ns_array_from_ids(&raw_attrs);

            let allocated = msg_id(class(b"MTLFunctionStitchingGraph\0"), sel(b"alloc\0"));
            let init: unsafe extern "C" fn(id, SEL, id, id, id, id) -> id =
                transmute(objc_msgSend as *const c_void);
            let raw = init(
                allocated,
                sel(b"initWithFunctionName:nodes:outputNode:attributes:\0"),
                ns_name.raw(),
                ns_nodes,
                raw_output,
                ns_attrs,
            );
            Self { raw }
        }
    }

    pub fn function_name(&self) -> String {
        ns_string_to_string(msg_id(self.raw, sel(b"functionName\0"))).unwrap_or_default()
    }

    pub fn set_function_name(&self, function_name: &str) {
        let ns_name = NSString::new(function_name);
        msg_void_id(self.raw, sel(b"setFunctionName:\0"), ns_name.raw());
    }

    pub fn nodes(&self) -> Vec<FunctionStitchingFunctionNode> {
        let array = msg_id(self.raw, sel(b"nodes\0"));
        ns_array_to_vec(array)
            .into_iter()
            .map(|raw| FunctionStitchingFunctionNode { raw: retain(raw) })
            .collect()
    }

    pub fn set_nodes(&self, nodes: &[&FunctionStitchingFunctionNode]) {
        let raw_nodes: Vec<id> = nodes.iter().map(|n| n.raw).collect();
        let array = ns_array_from_ids(&raw_nodes);
        msg_void_id(self.raw, sel(b"setNodes:\0"), array);
    }

    pub fn output_node(&self) -> Option<FunctionStitchingFunctionNode> {
        let raw = msg_id(self.raw, sel(b"outputNode\0"));
        if raw.is_null() {
            None
        } else {
            Some(FunctionStitchingFunctionNode { raw: retain(raw) })
        }
    }

    pub fn set_output_node(&self, output_node: Option<&FunctionStitchingFunctionNode>) {
        let raw_output = output_node.map(|n| n.raw).unwrap_or(NIL);
        msg_void_id(self.raw, sel(b"setOutputNode:\0"), raw_output);
    }

    pub fn attributes(&self) -> Vec<FunctionStitchingAttribute> {
        let array = msg_id(self.raw, sel(b"attributes\0"));
        ns_array_to_vec(array)
            .into_iter()
            .map(|raw| FunctionStitchingAttribute { raw: retain(raw) })
            .collect()
    }

    pub fn set_attributes(&self, attributes: &[&FunctionStitchingAttribute]) {
        let raw_attrs: Vec<id> = attributes.iter().map(|a| a.raw).collect();
        let array = ns_array_from_ids(&raw_attrs);
        msg_void_id(self.raw, sel(b"setAttributes:\0"), array);
    }
}

impl Drop for FunctionStitchingGraph {
    fn drop(&mut self) {
        release(self.raw);
    }
}

#[derive(Debug)]
pub struct StitchedLibraryDescriptor {
    pub raw: id,
}

impl StitchedLibraryDescriptor {
    pub fn new() -> Self {
        let raw = msg_id(class(b"MTLStitchedLibraryDescriptor\0"), sel(b"alloc\0"));
        let raw = msg_id(raw, sel(b"init\0"));
        Self { raw }
    }

    pub fn function_graphs(&self) -> Vec<FunctionStitchingGraph> {
        let array = msg_id(self.raw, sel(b"functionGraphs\0"));
        ns_array_to_vec(array)
            .into_iter()
            .map(|raw| FunctionStitchingGraph { raw: retain(raw) })
            .collect()
    }

    pub fn set_function_graphs(&self, graphs: &[&FunctionStitchingGraph]) {
        let raw_graphs: Vec<id> = graphs.iter().map(|g| g.raw).collect();
        let array = ns_array_from_ids(&raw_graphs);
        msg_void_id(self.raw, sel(b"setFunctionGraphs:\0"), array);
    }

    pub fn functions(&self) -> Vec<Function> {
        let array = msg_id(self.raw, sel(b"functions\0"));
        ns_array_to_vec(array)
            .into_iter()
            .map(|raw| Function { raw: retain(raw) })
            .collect()
    }

    pub fn set_functions(&self, functions: &[&Function]) {
        let raw_funcs: Vec<id> = functions.iter().map(|f| f.raw).collect();
        let array = ns_array_from_ids(&raw_funcs);
        msg_void_id(self.raw, sel(b"setFunctions:\0"), array);
    }

    pub fn binary_archives(&self) -> Result<Vec<BinaryArchive>, MetalError> {
        let selector = sel(b"binaryArchives\0");
        if responds_to_selector(self.raw, selector) {
            let array = msg_id(self.raw, selector);
            Ok(ns_array_to_vec(array)
                .into_iter()
                .map(|raw| BinaryArchive { raw: retain(raw) })
                .collect())
        } else {
            Err(MetalError::new(
                "binaryArchives property not supported on this platform",
            ))
        }
    }

    pub fn set_binary_archives(&self, archives: &[&BinaryArchive]) -> Result<(), MetalError> {
        let selector = sel(b"setBinaryArchives:\0");
        if responds_to_selector(self.raw, selector) {
            let raw_archives: Vec<id> = archives.iter().map(|a| a.raw).collect();
            let array = ns_array_from_ids(&raw_archives);
            msg_void_id(self.raw, selector, array);
            Ok(())
        } else {
            Err(MetalError::new(
                "setBinaryArchives: property not supported on this platform",
            ))
        }
    }

    pub fn options(&self) -> Result<StitchedLibraryOptions, MetalError> {
        let selector = sel(b"options\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            Ok(StitchedLibraryOptions(val))
        } else {
            Err(MetalError::new(
                "options property not supported on this platform",
            ))
        }
    }

    pub fn set_options(&self, options: StitchedLibraryOptions) -> Result<(), MetalError> {
        let selector = sel(b"setOptions:\0");
        if responds_to_selector(self.raw, selector) {
            msg_void_usize(self.raw, selector, options.as_raw());
            Ok(())
        } else {
            Err(MetalError::new(
                "setOptions: property not supported on this platform",
            ))
        }
    }
}

impl Drop for StitchedLibraryDescriptor {
    fn drop(&mut self) {
        release(self.raw);
    }
}
