use crate::java_analyzer::attributes::{read_raw_attribute, Attribute};
use crate::java_analyzer::classfile::ClassFile;
use crate::java_analyzer::error::Result;
use crate::java_analyzer::io::Buffer;

#[derive(Debug)]
pub struct JvmField {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,

    pub name: String,
    pub descriptor: String,
}

pub fn read_jvm_field(buffer: &mut Buffer, class_file: &ClassFile) -> Result<JvmField> {
    let access_flags = buffer.read_u16()?;
    let name_index = buffer.read_u16()?;
    let descriptor_index = buffer.read_u16()?;
    let attributes_count = buffer.read_u16()?;
    let mut attributes = vec![];
    for _ in 0..attributes_count {
        let raw_atrribute = read_raw_attribute(buffer, class_file)?;
        attributes.push(raw_atrribute);
    }

    let name =class_file.constant_pool.get_utf8(name_index as usize).unwrap();
    let descriptor = class_file.constant_pool.get_utf8(descriptor_index as usize).unwrap();

    Ok(JvmField {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
        name: name.to_string(),
        descriptor: descriptor.to_string(),
    })
}