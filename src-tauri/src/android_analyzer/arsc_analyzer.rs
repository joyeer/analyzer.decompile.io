use std::io::{Read, Seek};
use std::collections::HashMap;
use crate::android_analyzer::error::{AndroidAnalyzeError, Result};
use crate::android_analyzer::dex_reader::DexReader;
use crate::android::{ResourceTable, ResourcePackage, ResourceType, ResourceEntry, ResourceValue};

/// ARSC (Android Resource) analyzer for parsing resource.arsc files
pub struct ARSCAnalyzer<R: Read + Seek> {
    reader: DexReader<R>,
}

impl<R: Read + Seek> ARSCAnalyzer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: DexReader::new(reader),
        }
    }

    /// Analyze the ARSC file and return the parsed resource table
    pub fn analyze(&mut self) -> Result<ResourceTable> {
        let header = self.read_resource_table_header()?;
        let string_pool = self.read_string_pool()?;
        let packages = self.read_packages(&string_pool)?;
        
        Ok(ResourceTable {
            packages,
        })
    }

    /// Read resource table header
    fn read_resource_table_header(&mut self) -> Result<ResourceTableHeader> {
        let chunk_type = self.reader.read_u16()?;
        let header_size = self.reader.read_u16()?;
        let chunk_size = self.reader.read_u32()?;
        let package_count = self.reader.read_u32()?;
        
        if chunk_type != 0x0002 {
            return Err(AndroidAnalyzeError::InvalidResource("Invalid resource table chunk type".to_string()));
        }
        
        Ok(ResourceTableHeader {
            chunk_type,
            header_size,
            chunk_size,
            package_count,
        })
    }

    /// Read string pool
    fn read_string_pool(&mut self) -> Result<Vec<String>> {
        let chunk_type = self.reader.read_u16()?;
        let header_size = self.reader.read_u16()?;
        let chunk_size = self.reader.read_u32()?;
        let string_count = self.reader.read_u32()?;
        let style_count = self.reader.read_u32()?;
        let flags = self.reader.read_u32()?;
        let strings_start = self.reader.read_u32()?;
        let styles_start = self.reader.read_u32()?;

        if chunk_type != 0x0001 {
            return Err(AndroidAnalyzeError::InvalidResource("Invalid string pool chunk type".to_string()));
        }

        let is_utf8 = (flags & 0x00000100) != 0;
        
        // Read string offsets
        let mut string_offsets = Vec::new();
        for _ in 0..string_count {
            string_offsets.push(self.reader.read_u32()?);
        }

        // Skip style offsets
        for _ in 0..style_count {
            self.reader.read_u32()?;
        }

        // Read strings
        let mut strings = Vec::new();
        let base_offset = self.reader.stream_position()? as u32;
        
        for offset in string_offsets {
            self.reader.seek((base_offset + offset) as u64)?;
            
            let string = if is_utf8 {
                self.read_utf8_string()?
            } else {
                self.read_utf16_string()?
            };
            
            strings.push(string);
        }

        Ok(strings)
    }

    /// Read UTF-8 string
    fn read_utf8_string(&mut self) -> Result<String> {
        let _char_count = self.reader.read_u8()?;
        let byte_count = self.reader.read_u8()?;
        
        let mut bytes = vec![0u8; byte_count as usize];
        self.reader.read_exact(&mut bytes)?;
        
        // Skip null terminator
        self.reader.read_u8()?;
        
        String::from_utf8(bytes)
            .map_err(|e| AndroidAnalyzeError::ParseError(format!("Invalid UTF-8 string: {}", e)))
    }

    /// Read UTF-16 string
    fn read_utf16_string(&mut self) -> Result<String> {
        let char_count = self.reader.read_u16()?;
        
        let mut chars = Vec::new();
        for _ in 0..char_count {
            chars.push(self.reader.read_u16()?);
        }
        
        // Skip null terminator
        self.reader.read_u16()?;
        
        String::from_utf16(&chars)
            .map_err(|e| AndroidAnalyzeError::ParseError(format!("Invalid UTF-16 string: {}", e)))
    }

    /// Read resource packages
    fn read_packages(&mut self, string_pool: &[String]) -> Result<Vec<ResourcePackage>> {
        let mut packages = Vec::new();
        
        // Read package header
        let chunk_type = self.reader.read_u16()?;
        let header_size = self.reader.read_u16()?;
        let chunk_size = self.reader.read_u32()?;
        let package_id = self.reader.read_u32()?;
        
        if chunk_type != 0x0200 {
            return Err(AndroidAnalyzeError::InvalidResource("Invalid package chunk type".to_string()));
        }
        
        // Read package name (128 UTF-16 characters)
        let mut package_name_chars = Vec::new();
        for _ in 0..128 {
            package_name_chars.push(self.reader.read_u16()?);
        }
        
        // Find the end of the string
        let package_name = if let Some(null_pos) = package_name_chars.iter().position(|&c| c == 0) {
            String::from_utf16(&package_name_chars[..null_pos])
                .map_err(|e| AndroidAnalyzeError::ParseError(format!("Invalid package name: {}", e)))?
        } else {
            String::from_utf16(&package_name_chars)
                .map_err(|e| AndroidAnalyzeError::ParseError(format!("Invalid package name: {}", e)))?
        };
        
        let type_strings_offset = self.reader.read_u32()?;
        let last_public_type = self.reader.read_u32()?;
        let key_strings_offset = self.reader.read_u32()?;
        let last_public_key = self.reader.read_u32()?;
        let type_id_offset = self.reader.read_u32()?;
        
        // Read type strings
        let current_pos = self.reader.stream_position()?;
        self.reader.seek(type_strings_offset as u64)?;
        let type_strings = self.read_string_pool()?;
        
        // Read key strings
        self.reader.seek(key_strings_offset as u64)?;
        let key_strings = self.read_string_pool()?;
        
        // Read resource types
        self.reader.seek(current_pos)?;
        let types = self.read_resource_types(&type_strings, &key_strings)?;
        
        packages.push(ResourcePackage {
            id: package_id,
            name: package_name,
            types,
        });
        
        Ok(packages)
    }

    /// Read resource types
    fn read_resource_types(&mut self, type_strings: &[String], key_strings: &[String]) -> Result<Vec<ResourceType>> {
        let mut types = Vec::new();
        
        loop {
            // Try to read next chunk
            let chunk_type = match self.reader.read_u16() {
                Ok(ct) => ct,
                Err(_) => break, // End of file
            };
            
            match chunk_type {
                0x0201 => {
                    // TYPE_SPEC
                    let header_size = self.reader.read_u16()?;
                    let chunk_size = self.reader.read_u32()?;
                    let type_id = self.reader.read_u8()?;
                    let _reserved0 = self.reader.read_u8()?;
                    let _reserved1 = self.reader.read_u16()?;
                    let entry_count = self.reader.read_u32()?;
                    
                    // Skip spec flags
                    for _ in 0..entry_count {
                        self.reader.read_u32()?;
                    }
                    
                    // Read corresponding TYPE chunk
                    let type_chunk = self.read_type_chunk(type_id, type_strings, key_strings)?;
                    types.push(type_chunk);
                }
                0x0202 => {
                    // TYPE (without spec)
                    let header_size = self.reader.read_u16()?;
                    let chunk_size = self.reader.read_u32()?;
                    let type_id = self.reader.read_u8()?;
                    
                    // Seek back to read the full TYPE chunk
                    let current_pos = self.reader.stream_position()?;
                    self.reader.seek(current_pos - 7)?;
                    let type_chunk = self.read_type_chunk(type_id, type_strings, key_strings)?;
                    types.push(type_chunk);
                }
                _ => {
                    // Unknown chunk, skip
                    let header_size = self.reader.read_u16()?;
                    let chunk_size = self.reader.read_u32()?;
                    let current_pos = self.reader.stream_position()?;
                    self.reader.seek(current_pos + chunk_size as u64 - 8)?;
                }
            }
        }
        
        Ok(types)
    }

    /// Read a TYPE chunk
    fn read_type_chunk(&mut self, type_id: u8, type_strings: &[String], key_strings: &[String]) -> Result<ResourceType> {
        let chunk_type = self.reader.read_u16()?;
        let header_size = self.reader.read_u16()?;
        let chunk_size = self.reader.read_u32()?;
        let id = self.reader.read_u8()?;
        let _reserved0 = self.reader.read_u8()?;
        let _reserved1 = self.reader.read_u16()?;
        let entry_count = self.reader.read_u32()?;
        let entries_start = self.reader.read_u32()?;
        
        // Read config
        let config_size = self.reader.read_u32()?;
        // Skip config data
        for _ in 0..config_size - 4 {
            self.reader.read_u8()?;
        }
        
        // Read entry offsets
        let mut entry_offsets = Vec::new();
        for _ in 0..entry_count {
            entry_offsets.push(self.reader.read_u32()?);
        }
        
        // Read entries
        let mut entries = Vec::new();
        let base_offset = self.reader.stream_position()? as u32;
        
        for (i, &offset) in entry_offsets.iter().enumerate() {
            if offset != 0xFFFFFFFF {
                self.reader.seek((base_offset + offset) as u64)?;
                let entry = self.read_resource_entry(i as u32, key_strings)?;
                entries.push(entry);
            }
        }
        
        let type_name = if type_id as usize - 1 < type_strings.len() {
            type_strings[type_id as usize - 1].clone()
        } else {
            format!("type_{}", type_id)
        };
        
        Ok(ResourceType {
            id: type_id as u32,
            name: type_name,
            entries,
        })
    }

    /// Read a resource entry
    fn read_resource_entry(&mut self, entry_id: u32, key_strings: &[String]) -> Result<ResourceEntry> {
        let entry_size = self.reader.read_u16()?;
        let flags = self.reader.read_u16()?;
        let key_index = self.reader.read_u32()?;
        
        let name = if (key_index as usize) < key_strings.len() {
            key_strings[key_index as usize].clone()
        } else {
            format!("entry_{}", entry_id)
        };
        
        let is_complex = (flags & 0x0001) != 0;
        
        if is_complex {
            // Complex entry (bag)
            let parent = self.reader.read_u32()?;
            let count = self.reader.read_u32()?;
            
            let mut values = HashMap::new();
            for _ in 0..count {
                let name_ref = self.reader.read_u32()?;
                let value = self.read_resource_value()?;
                values.insert(name_ref, value);
            }
            
            Ok(ResourceEntry {
                id: entry_id,
                name,
                value: ResourceValue::Complex { parent, values },
            })
        } else {
            // Simple entry
            let value = self.read_resource_value()?;
            Ok(ResourceEntry {
                id: entry_id,
                name,
                value,
            })
        }
    }

    /// Read a resource value
    fn read_resource_value(&mut self) -> Result<ResourceValue> {
        let size = self.reader.read_u16()?;
        let _reserved = self.reader.read_u8()?;
        let data_type = self.reader.read_u8()?;
        let data = self.reader.read_u32()?;
        
        let value = match data_type {
            0x00 => ResourceValue::Null,
            0x01 => ResourceValue::Reference(data),
            0x02 => ResourceValue::Attribute(data),
            0x03 => ResourceValue::String(data),
            0x04 => ResourceValue::Float(f32::from_bits(data)),
            0x05 => ResourceValue::Dimension(data),
            0x06 => ResourceValue::Fraction(data),
            0x10 => ResourceValue::Integer(data as i32),
            0x11 => ResourceValue::Boolean(data != 0),
            0x1C => ResourceValue::Color(data),
            0x1D => ResourceValue::ColorStateList(data),
            _ => ResourceValue::Unknown(data),
        };
        
        Ok(value)
    }
}

/// Resource table header
#[derive(Debug, Clone)]
struct ResourceTableHeader {
    chunk_type: u16,
    header_size: u16,
    chunk_size: u32,
    package_count: u32,
}
