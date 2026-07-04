use crate::*;

// --- MTLType ---

#[derive(Debug)]
pub struct Type {
    pub raw: id,
}

impl Type {
    pub fn new_with_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn data_type(&self) -> DataType {
        if self.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.raw, sel(b"dataType\0"));
        // Safely transmute/map raw usize to DataType if it's in range
        // Since DataType is repr(usize), we can check if it's valid or just cast
        // Let's do a safe fallback
        unsafe { std::mem::transmute(val) }
    }
}

impl Clone for Type {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl Drop for Type {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// --- MTLStructMember ---

#[derive(Debug)]
pub struct StructMember {
    pub raw: id,
}

impl StructMember {
    pub fn new_with_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn name(&self) -> Option<String> {
        if self.raw.is_null() {
            return None;
        }
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
    }

    pub fn offset(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"offset\0"))
    }

    pub fn data_type(&self) -> DataType {
        if self.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.raw, sel(b"dataType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn struct_type(&self) -> Option<StructType> {
        if self.raw.is_null() {
            return None;
        }
        let ptr = msg_id(self.raw, sel(b"structType\0"));
        if ptr.is_null() {
            None
        } else {
            Some(StructType {
                Type: Type::new_with_raw(ptr),
            })
        }
    }

    pub fn array_type(&self) -> Option<ArrayType> {
        if self.raw.is_null() {
            return None;
        }
        let ptr = msg_id(self.raw, sel(b"arrayType\0"));
        if ptr.is_null() {
            None
        } else {
            Some(ArrayType {
                Type: Type::new_with_raw(ptr),
            })
        }
    }

    pub fn texture_reference_type(&self) -> Option<TextureReferenceType> {
        if self.raw.is_null() {
            return None;
        }
        let selector = sel(b"textureReferenceType\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if !ptr.is_null() {
                return Some(TextureReferenceType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    pub fn pointer_type(&self) -> Option<PointerType> {
        if self.raw.is_null() {
            return None;
        }
        let selector = sel(b"pointerType\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if !ptr.is_null() {
                return Some(PointerType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    pub fn argument_index(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"argumentIndex\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }
}

impl Clone for StructMember {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl Drop for StructMember {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// --- MTLStructType ---

#[derive(Debug)]
pub struct StructType {
    pub Type: Type,
}

impl StructType {
    pub fn new_with_raw(raw: id) -> Self {
        Self {
            Type: Type::new_with_raw(raw),
        }
    }

    pub fn members(&self) -> Vec<StructMember> {
        if self.Type.raw.is_null() {
            return Vec::new();
        }
        let array = msg_id(self.Type.raw, sel(b"members\0"));
        if array.is_null() {
            return Vec::new();
        }
        let count = msg_usize(array, sel(b"count\0"));
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let item = msg_id_usize(array, sel(b"objectAtIndexedSubscript:\0"), i);
            if !item.is_null() {
                result.push(StructMember::new_with_raw(item));
            }
        }
        result
    }

    pub fn member_by_name(&self, name: &str) -> Option<StructMember> {
        if self.Type.raw.is_null() {
            return None;
        }
        let ns_name = NSString::new(name);
        let ptr = msg_id_id(self.Type.raw, sel(b"memberByName:\0"), ns_name.raw());
        if ptr.is_null() {
            None
        } else {
            Some(StructMember::new_with_raw(ptr))
        }
    }
}

// --- MTLArrayType ---

#[derive(Debug)]
pub struct ArrayType {
    pub Type: Type,
}

impl ArrayType {
    pub fn new_with_raw(raw: id) -> Self {
        Self {
            Type: Type::new_with_raw(raw),
        }
    }

    pub fn element_type(&self) -> DataType {
        if self.Type.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.Type.raw, sel(b"elementType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn array_length(&self) -> usize {
        if self.Type.raw.is_null() {
            return 0;
        }
        msg_usize(self.Type.raw, sel(b"arrayLength\0"))
    }

    pub fn stride(&self) -> usize {
        if self.Type.raw.is_null() {
            return 0;
        }
        msg_usize(self.Type.raw, sel(b"stride\0"))
    }

    pub fn argument_index_stride(&self) -> usize {
        if self.Type.raw.is_null() {
            return 0;
        }
        let selector = sel(b"argumentIndexStride\0");
        if responds_to_selector(self.Type.raw, selector) {
            msg_usize(self.Type.raw, selector)
        } else {
            0
        }
    }

    pub fn element_struct_type(&self) -> Option<StructType> {
        if self.Type.raw.is_null() {
            return None;
        }
        let ptr = msg_id(self.Type.raw, sel(b"elementStructType\0"));
        if ptr.is_null() {
            None
        } else {
            Some(StructType {
                Type: Type::new_with_raw(ptr),
            })
        }
    }

    pub fn element_array_type(&self) -> Option<ArrayType> {
        if self.Type.raw.is_null() {
            return None;
        }
        let ptr = msg_id(self.Type.raw, sel(b"elementArrayType\0"));
        if ptr.is_null() {
            None
        } else {
            Some(ArrayType {
                Type: Type::new_with_raw(ptr),
            })
        }
    }

    pub fn element_texture_reference_type(&self) -> Option<TextureReferenceType> {
        if self.Type.raw.is_null() {
            return None;
        }
        let selector = sel(b"elementTextureReferenceType\0");
        if responds_to_selector(self.Type.raw, selector) {
            let ptr = msg_id(self.Type.raw, selector);
            if !ptr.is_null() {
                return Some(TextureReferenceType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    pub fn element_pointer_type(&self) -> Option<PointerType> {
        if self.Type.raw.is_null() {
            return None;
        }
        let selector = sel(b"elementPointerType\0");
        if responds_to_selector(self.Type.raw, selector) {
            let ptr = msg_id(self.Type.raw, selector);
            if !ptr.is_null() {
                return Some(PointerType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }
}

// --- MTLPointerType ---

#[derive(Debug)]
pub struct PointerType {
    pub Type: Type,
}

impl PointerType {
    pub fn new_with_raw(raw: id) -> Self {
        Self {
            Type: Type::new_with_raw(raw),
        }
    }

    pub fn element_type(&self) -> DataType {
        if self.Type.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.Type.raw, sel(b"elementType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn access(&self) -> BindingAccess {
        if self.Type.raw.is_null() {
            return BindingAccess::ReadOnly;
        }
        let val = msg_usize(self.Type.raw, sel(b"access\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn alignment(&self) -> usize {
        if self.Type.raw.is_null() {
            return 0;
        }
        msg_usize(self.Type.raw, sel(b"alignment\0"))
    }

    pub fn data_size(&self) -> usize {
        if self.Type.raw.is_null() {
            return 0;
        }
        msg_usize(self.Type.raw, sel(b"dataSize\0"))
    }

    pub fn element_is_argument_buffer(&self) -> bool {
        if self.Type.raw.is_null() {
            return false;
        }
        let selector = sel(b"elementIsArgumentBuffer\0");
        if responds_to_selector(self.Type.raw, selector) {
            msg_bool(self.Type.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn element_struct_type(&self) -> Option<StructType> {
        if self.Type.raw.is_null() {
            return None;
        }
        let selector = sel(b"elementStructType\0");
        if responds_to_selector(self.Type.raw, selector) {
            let ptr = msg_id(self.Type.raw, selector);
            if !ptr.is_null() {
                return Some(StructType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    pub fn element_array_type(&self) -> Option<ArrayType> {
        if self.Type.raw.is_null() {
            return None;
        }
        let selector = sel(b"elementArrayType\0");
        if responds_to_selector(self.Type.raw, selector) {
            let ptr = msg_id(self.Type.raw, selector);
            if !ptr.is_null() {
                return Some(ArrayType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }
}

// --- MTLTextureReferenceType ---

#[derive(Debug)]
pub struct TextureReferenceType {
    pub Type: Type,
}

impl TextureReferenceType {
    pub fn new_with_raw(raw: id) -> Self {
        Self {
            Type: Type::new_with_raw(raw),
        }
    }

    pub fn texture_data_type(&self) -> DataType {
        if self.Type.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.Type.raw, sel(b"textureDataType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn texture_type(&self) -> TextureType {
        if self.Type.raw.is_null() {
            return TextureType::D2;
        }
        let val = msg_usize(self.Type.raw, sel(b"textureType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn access(&self) -> BindingAccess {
        if self.Type.raw.is_null() {
            return BindingAccess::ReadOnly;
        }
        let val = msg_usize(self.Type.raw, sel(b"access\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn is_depth_texture(&self) -> bool {
        if self.Type.raw.is_null() {
            return false;
        }
        msg_bool(self.Type.raw, sel(b"isDepthTexture\0")) != 0
    }
}

// --- MTLArgument ---

#[derive(Debug)]
pub struct Argument {
    pub raw: id,
}

impl Argument {
    pub fn new_with_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn name(&self) -> Option<String> {
        if self.raw.is_null() {
            return None;
        }
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
    }

    pub fn type_(&self) -> ArgumentType {
        if self.raw.is_null() {
            return ArgumentType::Buffer;
        }
        let val = msg_usize(self.raw, sel(b"type\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn access(&self) -> BindingAccess {
        if self.raw.is_null() {
            return BindingAccess::ReadOnly;
        }
        let val = msg_usize(self.raw, sel(b"access\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn index(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"index\0"))
    }

    pub fn is_active(&self) -> bool {
        if self.raw.is_null() {
            return false;
        }
        msg_bool(self.raw, sel(b"isActive\0")) != 0
    }

    pub fn buffer_alignment(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"bufferAlignment\0"))
    }

    pub fn buffer_data_size(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"bufferDataSize\0"))
    }

    pub fn buffer_data_type(&self) -> DataType {
        if self.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.raw, sel(b"bufferDataType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn buffer_struct_type(&self) -> Option<StructType> {
        if self.raw.is_null() {
            return None;
        }
        let ptr = msg_id(self.raw, sel(b"bufferStructType\0"));
        if ptr.is_null() {
            None
        } else {
            Some(StructType {
                Type: Type::new_with_raw(ptr),
            })
        }
    }

    pub fn buffer_pointer_type(&self) -> Option<PointerType> {
        if self.raw.is_null() {
            return None;
        }
        let selector = sel(b"bufferPointerType\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if !ptr.is_null() {
                return Some(PointerType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    pub fn threadgroup_memory_alignment(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"threadgroupMemoryAlignment\0"))
    }

    pub fn threadgroup_memory_data_size(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"threadgroupMemoryDataSize\0"))
    }

    pub fn texture_type(&self) -> TextureType {
        if self.raw.is_null() {
            return TextureType::D2;
        }
        let val = msg_usize(self.raw, sel(b"textureType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn texture_data_type(&self) -> DataType {
        if self.raw.is_null() {
            return DataType::None;
        }
        let val = msg_usize(self.raw, sel(b"textureDataType\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn is_depth_texture(&self) -> bool {
        if self.raw.is_null() {
            return false;
        }
        let selector = sel(b"isDepthTexture\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn array_length(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"arrayLength\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }
}

impl Clone for Argument {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl Drop for Argument {
    fn drop(&mut self) {
        release(self.raw);
    }
}

// --- MTLBinding ---

#[derive(Debug)]
pub struct Binding {
    pub raw: id,
}

impl Binding {
    pub fn new_with_raw(raw: id) -> Self {
        Self { raw: retain(raw) }
    }

    pub fn name(&self) -> Option<String> {
        if self.raw.is_null() {
            return None;
        }
        ns_string_to_string(msg_id(self.raw, sel(b"name\0")))
    }

    pub fn type_(&self) -> BindingType {
        if self.raw.is_null() {
            return BindingType::Buffer;
        }
        let val = msg_usize(self.raw, sel(b"type\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn access(&self) -> BindingAccess {
        if self.raw.is_null() {
            return BindingAccess::ReadOnly;
        }
        let val = msg_usize(self.raw, sel(b"access\0"));
        unsafe { std::mem::transmute(val) }
    }

    pub fn index(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        msg_usize(self.raw, sel(b"index\0"))
    }

    pub fn is_used(&self) -> bool {
        if self.raw.is_null() {
            return false;
        }
        let selector = sel(b"isUsed\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn is_argument(&self) -> bool {
        if self.raw.is_null() {
            return false;
        }
        let selector = sel(b"isArgument\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    // BufferBinding protocol properties
    pub fn buffer_alignment(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"bufferAlignment\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn buffer_data_size(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"bufferDataSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn buffer_data_type(&self) -> DataType {
        if self.raw.is_null() {
            return DataType::None;
        }
        let selector = sel(b"bufferDataType\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            unsafe { std::mem::transmute(val) }
        } else {
            DataType::None
        }
    }

    pub fn buffer_struct_type(&self) -> Option<StructType> {
        if self.raw.is_null() {
            return None;
        }
        let selector = sel(b"bufferStructType\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if !ptr.is_null() {
                return Some(StructType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    pub fn buffer_pointer_type(&self) -> Option<PointerType> {
        if self.raw.is_null() {
            return None;
        }
        let selector = sel(b"bufferPointerType\0");
        if responds_to_selector(self.raw, selector) {
            let ptr = msg_id(self.raw, selector);
            if !ptr.is_null() {
                return Some(PointerType {
                    Type: Type::new_with_raw(ptr),
                });
            }
        }
        None
    }

    // ThreadgroupBinding protocol properties
    pub fn threadgroup_memory_alignment(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"threadgroupMemoryAlignment\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn threadgroup_memory_data_size(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"threadgroupMemoryDataSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    // TextureBinding protocol properties
    pub fn texture_type(&self) -> TextureType {
        if self.raw.is_null() {
            return TextureType::D2;
        }
        let selector = sel(b"textureType\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            unsafe { std::mem::transmute(val) }
        } else {
            TextureType::D2
        }
    }

    pub fn texture_data_type(&self) -> DataType {
        if self.raw.is_null() {
            return DataType::None;
        }
        let selector = sel(b"textureDataType\0");
        if responds_to_selector(self.raw, selector) {
            let val = msg_usize(self.raw, selector);
            unsafe { std::mem::transmute(val) }
        } else {
            DataType::None
        }
    }

    pub fn is_depth_texture(&self) -> bool {
        if self.raw.is_null() {
            return false;
        }
        let selector = sel(b"isDepthTexture\0");
        if responds_to_selector(self.raw, selector) {
            msg_bool(self.raw, selector) != 0
        } else {
            false
        }
    }

    pub fn array_length(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"arrayLength\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    // ObjectPayloadBinding protocol properties
    pub fn object_payload_alignment(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"objectPayloadAlignment\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }

    pub fn object_payload_data_size(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        let selector = sel(b"objectPayloadDataSize\0");
        if responds_to_selector(self.raw, selector) {
            msg_usize(self.raw, selector)
        } else {
            0
        }
    }
}

impl Clone for Binding {
    fn clone(&self) -> Self {
        Self {
            raw: retain(self.raw),
        }
    }
}

impl Drop for Binding {
    fn drop(&mut self) {
        release(self.raw);
    }
}
