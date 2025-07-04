use std::io::{Read, Seek};
use crate::android_analyzer::error::{AndroidAnalyzeError, Result};
use crate::android_analyzer::dex_reader::DexReader;
use crate::android::{DexFile, TypeDescriptor, ProtoDescriptor, FieldDescriptor, MethodDescriptor, ClassDef};

/// DEX file analyzer for parsing Android DEX files
pub struct DexAnalyzer<R: Read + Seek> {
    reader: DexReader<R>,
}

impl<R: Read + Seek> DexAnalyzer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: DexReader::new(reader),
        }
    }

    /// Analyze the DEX file and return the parsed structure
    pub fn analyze(&mut self) -> Result<DexFile> {
        // Read DEX header
        let header = self.read_dex_header()?;
        
        // Read string table
        let strings = self.read_string_table(&header)?;
        
        // Read type table
        let types = self.read_type_table(&header, &strings)?;
        
        // Read proto table
        let protos = self.read_proto_table(&header, &strings, &types)?;
        
        // Read field table
        let fields = self.read_field_table(&header, &strings, &types)?;
        
        // Read method table
        let methods = self.read_method_table(&header, &strings, &types, &protos)?;
        
        // Read class definitions
        let classes = self.read_class_definitions(&header, &strings, &types, &fields, &methods)?;

        Ok(DexFile {
            magic: header.magic,
            checksum: header.checksum,
            signature: header.signature,
            file_size: header.file_size,
            header_size: header.header_size,
            endian_tag: header.endian_tag,
            link_size: header.link_size,
            link_offset: header.link_offset,
            map_offset: header.map_offset,
            string_ids_size: header.string_ids_size,
            string_ids_offset: header.string_ids_offset,
            type_ids_size: header.type_ids_size,
            type_ids_offset: header.type_ids_offset,
            proto_ids_size: header.proto_ids_size,
            proto_ids_offset: header.proto_ids_offset,
            field_ids_size: header.field_ids_size,
            field_ids_offset: header.field_ids_offset,
            method_ids_size: header.method_ids_size,
            method_ids_offset: header.method_ids_offset,
            class_defs_size: header.class_defs_size,
            class_defs_offset: header.class_defs_offset,
            data_size: header.data_size,
            data_offset: header.data_offset,
            strings,
            types,
            protos,
            fields,
            methods,
            classes,
        })
    }

    /// Read the DEX file header
    fn read_dex_header(&mut self) -> Result<DexHeader> {
        let mut magic = [0u8; 8];
        self.reader.read_exact(&mut magic)?;
        
        // Validate DEX magic
        if &magic != b"dex\n035\0" && &magic != b"dex\n036\0" && &magic != b"dex\n037\0" {
            return Err(AndroidAnalyzeError::InvalidDexFile("Invalid DEX magic".to_string()));
        }

        let checksum = self.reader.read_u32()?;
        
        let mut signature = [0u8; 20];
        self.reader.read_exact(&mut signature)?;
        
        let file_size = self.reader.read_u32()?;
        let header_size = self.reader.read_u32()?;
        let endian_tag = self.reader.read_u32()?;
        let link_size = self.reader.read_u32()?;
        let link_offset = self.reader.read_u32()?;
        let map_offset = self.reader.read_u32()?;
        let string_ids_size = self.reader.read_u32()?;
        let string_ids_offset = self.reader.read_u32()?;
        let type_ids_size = self.reader.read_u32()?;
        let type_ids_offset = self.reader.read_u32()?;
        let proto_ids_size = self.reader.read_u32()?;
        let proto_ids_offset = self.reader.read_u32()?;
        let field_ids_size = self.reader.read_u32()?;
        let field_ids_offset = self.reader.read_u32()?;
        let method_ids_size = self.reader.read_u32()?;
        let method_ids_offset = self.reader.read_u32()?;
        let class_defs_size = self.reader.read_u32()?;
        let class_defs_offset = self.reader.read_u32()?;
        let data_size = self.reader.read_u32()?;
        let data_offset = self.reader.read_u32()?;

        Ok(DexHeader {
            magic,
            checksum,
            signature,
            file_size,
            header_size,
            endian_tag,
            link_size,
            link_offset,
            map_offset,
            string_ids_size,
            string_ids_offset,
            type_ids_size,
            type_ids_offset,
            proto_ids_size,
            proto_ids_offset,
            field_ids_size,
            field_ids_offset,
            method_ids_size,
            method_ids_offset,
            class_defs_size,
            class_defs_offset,
            data_size,
            data_offset,
        })
    }

    /// Read the string table
    fn read_string_table(&mut self, header: &DexHeader) -> Result<Vec<String>> {
        let mut strings = Vec::new();
        
        // Seek to string IDs
        self.reader.seek(header.string_ids_offset as u64)?;
        
        for _ in 0..header.string_ids_size {
            let string_data_offset = self.reader.read_u32()?;
            let current_pos = self.reader.stream_position()?;
            
            // Read string data
            self.reader.seek(string_data_offset as u64)?;
            let string = self.read_mutf8_string()?;
            strings.push(string);
            
            // Return to string IDs table
            self.reader.seek(current_pos)?;
        }
        
        Ok(strings)
    }

    /// Read a Modified UTF-8 string
    fn read_mutf8_string(&mut self) -> Result<String> {
        let size = self.reader.read_uleb128()?;
        let mut bytes = vec![0u8; size as usize];
        self.reader.read_exact(&mut bytes)?;
        
        // Convert MUTF-8 to UTF-8
        String::from_utf8(bytes)
            .map_err(|e| AndroidAnalyzeError::ParseError(format!("Invalid MUTF-8 string: {}", e)))
    }

    /// Read the type table
    fn read_type_table(&mut self, header: &DexHeader, strings: &[String]) -> Result<Vec<TypeDescriptor>> {
        let mut types = Vec::new();
        
        self.reader.seek(header.type_ids_offset as u64)?;
        
        for _ in 0..header.type_ids_size {
            let descriptor_idx = self.reader.read_u32()?;
            if descriptor_idx as usize >= strings.len() {
                return Err(AndroidAnalyzeError::InvalidDexFile("Invalid type descriptor index".to_string()));
            }
            
            let descriptor = strings[descriptor_idx as usize].clone();
            types.push(TypeDescriptor::from_descriptor(&descriptor)?);
        }
        
        Ok(types)
    }

    /// Read the proto table
    fn read_proto_table(&mut self, header: &DexHeader, strings: &[String], types: &[TypeDescriptor]) -> Result<Vec<ProtoDescriptor>> {
        let mut protos = Vec::new();
        
        self.reader.seek(header.proto_ids_offset as u64)?;
        
        for _ in 0..header.proto_ids_size {
            let shorty_idx = self.reader.read_u32()?;
            let return_type_idx = self.reader.read_u32()?;
            let parameters_off = self.reader.read_u32()?;
            
            if shorty_idx as usize >= strings.len() || return_type_idx as usize >= types.len() {
                return Err(AndroidAnalyzeError::InvalidDexFile("Invalid proto descriptor index".to_string()));
            }
            
            let shorty = strings[shorty_idx as usize].clone();
            let return_type = types[return_type_idx as usize].clone();
            
            let mut parameters = Vec::new();
            if parameters_off > 0 {
                let current_pos = self.reader.stream_position()?;
                self.reader.seek(parameters_off as u64)?;
                
                let size = self.reader.read_u32()?;
                for _ in 0..size {
                    let type_idx = self.reader.read_u16()?;
                    if type_idx as usize >= types.len() {
                        return Err(AndroidAnalyzeError::InvalidDexFile("Invalid parameter type index".to_string()));
                    }
                    parameters.push(types[type_idx as usize].clone());
                }
                
                self.reader.seek(current_pos)?;
            }
            
            protos.push(ProtoDescriptor {
                shorty,
                return_type,
                parameters,
            });
        }
        
        Ok(protos)
    }

    /// Read the field table
    fn read_field_table(&mut self, header: &DexHeader, strings: &[String], types: &[TypeDescriptor]) -> Result<Vec<FieldDescriptor>> {
        let mut fields = Vec::new();
        
        self.reader.seek(header.field_ids_offset as u64)?;
        
        for _ in 0..header.field_ids_size {
            let class_idx = self.reader.read_u16()?;
            let type_idx = self.reader.read_u16()?;
            let name_idx = self.reader.read_u32()?;
            
            if class_idx as usize >= types.len() || 
               type_idx as usize >= types.len() || 
               name_idx as usize >= strings.len() {
                return Err(AndroidAnalyzeError::InvalidDexFile("Invalid field descriptor index".to_string()));
            }
            
            fields.push(FieldDescriptor {
                class_type: types[class_idx as usize].clone(),
                field_type: types[type_idx as usize].clone(),
                name: strings[name_idx as usize].clone(),
            });
        }
        
        Ok(fields)
    }

    /// Read the method table
    fn read_method_table(&mut self, header: &DexHeader, strings: &[String], types: &[TypeDescriptor], protos: &[ProtoDescriptor]) -> Result<Vec<MethodDescriptor>> {
        let mut methods = Vec::new();
        
        self.reader.seek(header.method_ids_offset as u64)?;
        
        for _ in 0..header.method_ids_size {
            let class_idx = self.reader.read_u16()?;
            let proto_idx = self.reader.read_u16()?;
            let name_idx = self.reader.read_u32()?;
            
            if class_idx as usize >= types.len() || 
               proto_idx as usize >= protos.len() || 
               name_idx as usize >= strings.len() {
                return Err(AndroidAnalyzeError::InvalidDexFile("Invalid method descriptor index".to_string()));
            }
            
            methods.push(MethodDescriptor {
                class_type: types[class_idx as usize].clone(),
                proto: protos[proto_idx as usize].clone(),
                name: strings[name_idx as usize].clone(),
            });
        }
        
        Ok(methods)
    }

    /// Read class definitions
    fn read_class_definitions(&mut self, header: &DexHeader, strings: &[String], types: &[TypeDescriptor], fields: &[FieldDescriptor], methods: &[MethodDescriptor]) -> Result<Vec<ClassDef>> {
        let mut classes = Vec::new();
        
        self.reader.seek(header.class_defs_offset as u64)?;
        
        for _ in 0..header.class_defs_size {
            let class_idx = self.reader.read_u32()?;
            let access_flags = self.reader.read_u32()?;
            let superclass_idx = self.reader.read_u32()?;
            let interfaces_off = self.reader.read_u32()?;
            let source_file_idx = self.reader.read_u32()?;
            let annotations_off = self.reader.read_u32()?;
            let class_data_off = self.reader.read_u32()?;
            let static_values_off = self.reader.read_u32()?;
            
            if class_idx as usize >= types.len() {
                return Err(AndroidAnalyzeError::InvalidDexFile("Invalid class type index".to_string()));
            }
            
            let class_type = types[class_idx as usize].clone();
            let super_type = if superclass_idx != 0xFFFFFFFF {
                if superclass_idx as usize >= types.len() {
                    return Err(AndroidAnalyzeError::InvalidDexFile("Invalid superclass type index".to_string()));
                }
                Some(types[superclass_idx as usize].clone())
            } else {
                None
            };
            
            let source_file = if source_file_idx != 0xFFFFFFFF {
                if source_file_idx as usize >= strings.len() {
                    return Err(AndroidAnalyzeError::InvalidDexFile("Invalid source file index".to_string()));
                }
                Some(strings[source_file_idx as usize].clone())
            } else {
                None
            };
            
            // TODO: Read interfaces, annotations, and class data
            let interfaces = Vec::new();
            let annotations = Vec::new();
            let static_fields = Vec::new();
            let instance_fields = Vec::new();
            let direct_methods = Vec::new();
            let virtual_methods = Vec::new();
            
            classes.push(ClassDef {
                class_type,
                access_flags,
                super_type,
                interfaces,
                source_file,
                annotations,
                static_fields,
                instance_fields,
                direct_methods,
                virtual_methods,
            });
        }
        
        Ok(classes)
    }
}

/// DEX file header structure
#[derive(Debug, Clone)]
struct DexHeader {
    magic: [u8; 8],
    checksum: u32,
    signature: [u8; 20],
    file_size: u32,
    header_size: u32,
    endian_tag: u32,
    link_size: u32,
    link_offset: u32,
    map_offset: u32,
    string_ids_size: u32,
    string_ids_offset: u32,
    type_ids_size: u32,
    type_ids_offset: u32,
    proto_ids_size: u32,
    proto_ids_offset: u32,
    field_ids_size: u32,
    field_ids_offset: u32,
    method_ids_size: u32,
    method_ids_offset: u32,
    class_defs_size: u32,
    class_defs_offset: u32,
    data_size: u32,
    data_offset: u32,
}
