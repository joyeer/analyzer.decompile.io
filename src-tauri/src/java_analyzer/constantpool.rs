use crate::java_analyzer::io::Buffer;
use crate::java_analyzer::error::{JavaAnalyzeError, Result};

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_INTERFACEMETHODREF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAMEANDTYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHODHANDLE: u8 = 15;
const CONSTANT_METHODTYPE: u8 = 16;
const CONSTANT_INVOKEDYNAMIC: u8 = 18;
const CONSTANT_MODULE: u8 = 19;
const CONSTANT_PACKAGE: u8 = 20;
const CONSTANT_DYNAMIC: u8 = 17;

#[derive(Clone)]
pub enum ConstantPoolEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassRef(u16),
    StringRef(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndTypeRef(u16, u16),
    MethodHandleRef(u8, u16),
    MethodTypeRef(u16),
    InvokeDynamicRef(u16, u16),
    Module(u16),
    Package(u16),
    Dynamic(u16, u16),
}

#[derive(Default)]
pub struct ConstantPool {
    pub constant_pool: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_entry(&self, index: usize) -> Option<&ConstantPoolEntry> {
        self.constant_pool.get(index - 1)
    }

    pub fn get_utf8(&self, index: usize) -> Option<&String> {
        if let Some(ConstantPoolEntry::Utf8(utf8)) = self.get_entry(index) {
            Some(utf8)
        } else {
            None
        }
    }

    pub fn get_integer(&self, index: usize) -> Option<i32> {
        if let Some(ConstantPoolEntry::Integer(value)) = self.get_entry(index) {
            Some(*value)
        } else {
            None
        }
    }
    pub fn get_float(&self, index: usize) -> Option<f32> {
        if let Some(ConstantPoolEntry::Float(value)) = self.get_entry(index) {
            Some(*value)
        } else {
            None
        }
    }

    pub fn get_method_ref(&self, index: usize) -> Option<(u16, u16)> {
        if let Some(ConstantPoolEntry::MethodRef(class_index, name_and_type_index)) = self.get_entry(index) {
            Some((*class_index, *name_and_type_index))
        } else {
            None
        }
    }

    pub fn get_name_and_type(&self, index: usize) -> Option<(u16, u16)> {
        if let Some(ConstantPoolEntry::NameAndTypeRef(name_index, descriptor_index)) = self.get_entry(index) {
            Some((*name_index, *descriptor_index))
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct ConstantPoolReader {
}

impl ConstantPoolReader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read_constant_pool(&mut self, buffer:&mut Buffer) ->Result<ConstantPool> {
        let mut constant_pool = Vec::<ConstantPoolEntry>::new();
        let constant_pool_count = buffer.read_u16()?;

        let mut i = 1;
        while i < constant_pool_count {
            let tag = buffer.read_u8()?;
            let constant = match tag {
                CONSTANT_UTF8 => self.read_utf8_constant(buffer)?,
                CONSTANT_INTEGER => self.read_int_constant(buffer)?,
                CONSTANT_FLOAT => self.read_float_constant(buffer)?,
                CONSTANT_LONG => self.read_long_constant(buffer)?,
                CONSTANT_DOUBLE => self.read_double_constant(buffer)?,
                CONSTANT_CLASS => self.read_class_ref_constant(buffer)?,
                CONSTANT_STRING => self.read_string_constant(buffer)?,
                CONSTANT_FIELDREF => self.read_field_ref_constant(buffer)?,
                CONSTANT_METHODREF => self.read_method_ref_constant(buffer)?, 
                CONSTANT_INTERFACEMETHODREF => self.read_interface_method_ref_constant(buffer)?,
                CONSTANT_NAMEANDTYPE => self.read_name_and_type_constant(buffer)?,
                CONSTANT_METHODHANDLE => self.read_method_handle_constant(buffer)?,
                CONSTANT_METHODTYPE => self.read_method_type_constant(buffer)?,
                CONSTANT_INVOKEDYNAMIC => self.read_invoke_dynamic_constant(buffer)?,
                CONSTANT_MODULE => self.read_module_constant(buffer)?,
                CONSTANT_PACKAGE => self.read_package_constant(buffer)?,
                CONSTANT_DYNAMIC => self.read_dynamic_constant(buffer)?,
                _ => {
                    return Err(JavaAnalyzeError::InvalidClassData(format!(
                        "Unsupported constant pool tag: {}",
                        tag
                    )));    
                }
            };
            
            constant_pool.push(constant.clone());
            i += 1;
            if tag == CONSTANT_LONG || tag == CONSTANT_DOUBLE {
                constant_pool.push(constant);
                i += 1; // Long and Double take up two entries in the constant pool
            }
        }
        Ok(ConstantPool { constant_pool })
    }


    fn read_utf8_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let length = buffer.read_u16()?;
        buffer
            .read_utf8(length as usize)
            .map(ConstantPoolEntry::Utf8)
            .map_err(|err| err.into())
    }

    fn read_int_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        buffer
            .read_i32()
            .map(ConstantPoolEntry::Integer)
            .map_err(|err| err.into())
    }

    fn read_float_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        buffer
            .read_f32()
            .map(ConstantPoolEntry::Float)
            .map_err(|err| err.into())
    }

    fn read_long_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        buffer
            .read_i64()
            .map(ConstantPoolEntry::Long)
            .map_err(|err| err.into())
    }

    fn read_double_constant(&mut self,buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        buffer
            .read_f64()
            .map(ConstantPoolEntry::Double)
            .map_err(|err| err.into())
    }

    fn read_class_ref_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        buffer
            .read_u16()
            .map(ConstantPoolEntry::ClassRef)
            .map_err(|err| err.into())
    }

    fn read_string_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        buffer
            .read_u16()
            .map(ConstantPoolEntry::StringRef)
            .map_err(|err| err.into())
    }

    fn read_field_ref_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let class_index = buffer.read_u16()?;
        let name_and_type_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::FieldRef(class_index, name_and_type_index))
    }
    
    fn read_method_ref_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let class_index = buffer.read_u16()?;
        let name_and_type_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::MethodRef(class_index, name_and_type_index))
    }

    fn read_interface_method_ref_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let class_index = buffer.read_u16()?;
        let name_and_type_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::InterfaceMethodRef(class_index, name_and_type_index))
    }

    fn read_name_and_type_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let name_index = buffer.read_u16()?;
        let descriptor_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::NameAndTypeRef(name_index, descriptor_index))
    }

    fn read_method_handle_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let reference_kind = buffer.read_u8()?;
        let reference_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::MethodHandleRef(reference_kind, reference_index))
    }

    fn read_method_type_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let descriptor_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::MethodTypeRef(descriptor_index))
    }

    fn read_invoke_dynamic_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let bootstrap_method_attr_index = buffer.read_u16()?;
        let name_and_type_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::InvokeDynamicRef(bootstrap_method_attr_index, name_and_type_index))
    }

    fn read_module_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let name_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::Module(name_index))
    }

    fn read_package_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let name_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::Package(name_index))
    }

    fn read_dynamic_constant(&mut self, buffer:&mut Buffer) -> Result<ConstantPoolEntry> {
        let bootstrap_method_attr_index = buffer.read_u16()?;
        let name_and_type_index = buffer.read_u16()?;
        Ok(ConstantPoolEntry::Dynamic(bootstrap_method_attr_index, name_and_type_index))
    }

}