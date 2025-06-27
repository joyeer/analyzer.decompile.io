
use crate::java_analyzer::attributes::{read_raw_attribute, Attribute};
use crate::java_analyzer::constantpool::{ConstantPool, ConstantPoolReader};
use crate::java_analyzer::io::Buffer;

use crate::java_analyzer::error::{JavaAnalyzeError, Result};
use crate::java_analyzer::method::{read_jvm_method, JvmMethod};
use crate::java_analyzer::field::{JvmField, read_jvm_field};
const MAGIC_NUMBER: u32 = 0xCAFEBABE;

/*
The ClassFile structure represents the entire class file.
It contains the magic number, version information, constant pool,
access flags, this class, super class, interfaces, fields, methods,
and attributes.          
*/

#[derive(Default)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<JvmField>,
    pub methods_count: u16,
    pub methods: Vec<JvmMethod>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

// ClassFileReader is responsible for reading the class file
pub struct ClassFileReader<'a> {
    buffer: Buffer<'a>,
    pub class_file: ClassFile,
}

impl<'a> ClassFileReader<'a> {
    pub fn new(data: &'a [u8]) -> ClassFileReader<'a> {
        ClassFileReader {
            buffer: Buffer::new(data),
            class_file: Default::default(),
        }
    }

    pub fn read(mut self) ->Result<ClassFile> {
        self.read_magic_number()?;
        self.class_file.minor_version = self.buffer.read_u16()?;
        self.class_file.major_version = self.buffer.read_u16()?;
        
        self.read_constant_pool()?;
        // Read access flags
        self.class_file.access_flags = self.buffer.read_u16()?;
        self.class_file.this_class = self.buffer.read_u16()?;
        self.class_file.super_class = self.buffer.read_u16()?;

        self.read_interfaces()?;
        self.read_fields()?;
        self.read_methods()?;
        self.read_class_attributes()?;
        
        Ok(self.class_file)
    }

    fn read_constant_pool(&mut self) -> Result<()> {
        
        let mut reader = ConstantPoolReader::new();
        let constant_pool = reader.read_constant_pool(&mut self.buffer)?;
        self.class_file.constant_pool = constant_pool;
        Ok(())
    }
    fn read_magic_number(&mut self) -> Result<()> {

        match self.buffer.read_u32()  {
            Ok(MAGIC_NUMBER) => {
                self.class_file.magic = 0xCAFEBABE;
                Ok(())
            },
            Ok(_) => {
                Err(JavaAnalyzeError::InvalidClassData("Invalid magic number".to_owned()))
            },
            Err(e) => {
                Err(e.into())
            }
        }
    }    

    fn read_interfaces(&mut self) -> Result<()> {
        self.class_file.interfaces_count = self.buffer.read_u16()?;
        for _ in 0..self.class_file.interfaces_count {
            let interface = self.buffer.read_u16()?;
            self.class_file.interfaces.push(interface);
        }
        Ok(())
    }
    
    fn read_fields(&mut self) -> Result<()> {
        let field_count = self.buffer.read_u16()?;
        self.class_file.fields_count = field_count;
        self.class_file.fields = (0 ..field_count).map(|_| {
            let field = read_jvm_field(&mut self.buffer, &self.class_file)?;
            Ok(field)
        }).collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    
    fn read_methods(&mut self) -> Result<()> {
        let method_count: u16 = self.buffer.read_u16()?;
        self.class_file.methods_count = method_count;
        self.class_file.methods = (0..method_count).map(|_| {
            let method = read_jvm_method(&mut self.buffer, &self.class_file)?;
            Ok(method)
        }).collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    /// Reads the class attributes from the class file.
    /// This includes attributes like the class's version, source file, etc.
    /// The attributes are stored in the `attributes` field of the `ClassFile` struct.
    fn read_class_attributes(&mut self) -> Result<()> {
        let attributes_count = self.buffer.read_u16()?;
        self.class_file.attributes_count = attributes_count;
        for _ in 0..attributes_count {
            let raw_atrribute = read_raw_attribute(&mut self.buffer,& self.class_file)?;
            self.class_file.attributes.push(raw_atrribute);
        }
        Ok(())
    }

}
