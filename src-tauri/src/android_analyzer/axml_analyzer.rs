use std::io::{Read, Seek};
use std::collections::HashMap;
use crate::android_analyzer::error::{AndroidAnalyzeError, Result};
use crate::android_analyzer::dex_reader::DexReader;
use crate::android::{AndroidManifest, Activity, Service, Receiver, Provider, Permission};

/// AXML (Android XML) analyzer for parsing binary XML files
pub struct AXMLAnalyzer<R: Read + Seek> {
    reader: DexReader<R>,
}

impl<R: Read + Seek> AXMLAnalyzer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: DexReader::new(reader),
        }
    }

    /// Analyze the binary XML and return the parsed manifest
    pub fn analyze_manifest(&mut self) -> Result<AndroidManifest> {
        let header = self.read_axml_header()?;
        let string_pool = self.read_string_pool()?;
        let _resource_ids = self.read_resource_ids()?;
        let xml_content = self.read_xml_content(&string_pool)?;
        
        // Parse manifest-specific content
        self.parse_manifest_content(&xml_content, &string_pool)
    }

    /// Analyze the binary XML and return the parsed manifest
    pub fn analyze(&mut self) -> Result<AndroidManifest> {
        self.analyze_manifest()
    }

    /// Read AXML header
    fn read_axml_header(&mut self) -> Result<AXMLHeader> {
        let magic = self.reader.read_u32()?;
        if magic != 0x00080003 {
            return Err(AndroidAnalyzeError::InvalidManifest("Invalid AXML magic".to_string()));
        }

        let file_size = self.reader.read_u32()?;
        
        Ok(AXMLHeader {
            magic,
            file_size,
        })
    }

    /// Read string pool
    fn read_string_pool(&mut self) -> Result<Vec<String>> {
        let chunk_type = self.reader.read_u32()?;
        if chunk_type != 0x001C0001 {
            return Err(AndroidAnalyzeError::InvalidManifest("Invalid string pool chunk".to_string()));
        }

        let chunk_size = self.reader.read_u32()?;
        let string_count = self.reader.read_u32()?;
        let style_count = self.reader.read_u32()?;
        let flags = self.reader.read_u32()?;
        let strings_start = self.reader.read_u32()?;
        let styles_start = self.reader.read_u32()?;

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

    /// Read resource IDs
    fn read_resource_ids(&mut self) -> Result<Vec<u32>> {
        // Check if there's a resource chunk
        let chunk_type = self.reader.read_u32()?;
        if chunk_type != 0x00080180 {
            // No resource chunk, seek back
            let current_pos = self.reader.stream_position()?;
            self.reader.seek(current_pos - 4)?;
            return Ok(Vec::new());
        }

        let chunk_size = self.reader.read_u32()?;
        let resource_count = (chunk_size - 8) / 4;
        
        let mut resource_ids = Vec::new();
        for _ in 0..resource_count {
            resource_ids.push(self.reader.read_u32()?);
        }

        Ok(resource_ids)
    }

    /// Read XML content
    fn read_xml_content(&mut self, string_pool: &[String]) -> Result<XmlNode> {
        let mut nodes = Vec::new();
        
        while let Ok(node) = self.read_xml_node(string_pool) {
            nodes.push(node);
        }
        
        // Find the root node (manifest)
        for node in nodes {
            if let XmlNode::StartElement { name, .. } = &node {
                if name == "manifest" {
                    return Ok(node);
                }
            }
        }
        
        Err(AndroidAnalyzeError::InvalidManifest("No manifest root element found".to_string()))
    }

    /// Read XML node
    fn read_xml_node(&mut self, string_pool: &[String]) -> Result<XmlNode> {
        let chunk_type = self.reader.read_u32()?;
        let chunk_size = self.reader.read_u32()?;
        
        match chunk_type {
            0x00100100 => {
                // START_NAMESPACE
                let line_number = self.reader.read_u32()?;
                let comment = self.reader.read_u32()?;
                let prefix = self.reader.read_u32()?;
                let uri = self.reader.read_u32()?;
                
                Ok(XmlNode::StartNamespace {
                    line_number,
                    prefix: if prefix != 0xFFFFFFFF { Some(string_pool[prefix as usize].clone()) } else { None },
                    uri: if uri != 0xFFFFFFFF { Some(string_pool[uri as usize].clone()) } else { None },
                })
            }
            0x00100101 => {
                // END_NAMESPACE
                Ok(XmlNode::EndNamespace)
            }
            0x00100102 => {
                // START_TAG
                let line_number = self.reader.read_u32()?;
                let comment = self.reader.read_u32()?;
                let namespace = self.reader.read_u32()?;
                let name = self.reader.read_u32()?;
                let attribute_start = self.reader.read_u32()?;
                let attribute_size = self.reader.read_u32()?;
                let attribute_count = self.reader.read_u32()?;
                let id_attribute = self.reader.read_u32()?;
                let class_attribute = self.reader.read_u32()?;
                let style_attribute = self.reader.read_u32()?;
                
                let element_name = if name != 0xFFFFFFFF {
                    string_pool[name as usize].clone()
                } else {
                    "unknown".to_string()
                };
                
                let mut attributes = HashMap::new();
                for _ in 0..attribute_count {
                    let attr_namespace = self.reader.read_u32()?;
                    let attr_name = self.reader.read_u32()?;
                    let attr_value = self.reader.read_u32()?;
                    let attr_type = self.reader.read_u32()?;
                    let attr_data = self.reader.read_u32()?;
                    
                    if attr_name != 0xFFFFFFFF {
                        let attr_name_str = string_pool[attr_name as usize].clone();
                        let attr_value_str = if attr_value != 0xFFFFFFFF {
                            string_pool[attr_value as usize].clone()
                        } else {
                            format!("0x{:08x}", attr_data)
                        };
                        
                        attributes.insert(attr_name_str, attr_value_str);
                    }
                }
                
                Ok(XmlNode::StartElement {
                    name: element_name,
                    attributes,
                })
            }
            0x00100103 => {
                // END_TAG
                Ok(XmlNode::EndElement)
            }
            0x00100104 => {
                // TEXT
                let line_number = self.reader.read_u32()?;
                let comment = self.reader.read_u32()?;
                let name = self.reader.read_u32()?;
                let _unknown1 = self.reader.read_u32()?;
                let _unknown2 = self.reader.read_u32()?;
                
                let text = if name != 0xFFFFFFFF {
                    string_pool[name as usize].clone()
                } else {
                    String::new()
                };
                
                Ok(XmlNode::Text { content: text })
            }
            _ => Err(AndroidAnalyzeError::InvalidManifest(format!("Unknown chunk type: 0x{:08x}", chunk_type))),
        }
    }

    /// Parse manifest-specific content
    fn parse_manifest_content(&mut self, xml_node: &XmlNode, string_pool: &[String]) -> Result<AndroidManifest> {
        if let XmlNode::StartElement { name, attributes } = xml_node {
            if name != "manifest" {
                return Err(AndroidAnalyzeError::InvalidManifest("Expected manifest root element".to_string()));
            }
            
            let package_name = attributes.get("package")
                .ok_or_else(|| AndroidAnalyzeError::InvalidManifest("Missing package attribute".to_string()))?
                .clone();
            
            let version_code = attributes.get("versionCode")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            
            let version_name = attributes.get("versionName")
                .cloned()
                .unwrap_or_else(|| "1.0".to_string());
            
            // TODO: Parse activities, services, receivers, providers, permissions
            let activities = Vec::new();
            let services = Vec::new();
            let receivers = Vec::new();
            let providers = Vec::new();
            let permissions = Vec::new();
            let uses_permissions = Vec::new();
            
            Ok(AndroidManifest {
                package_name,
                version_code,
                version_name,
                activities,
                services,
                receivers,
                providers,
                permissions,
                uses_permissions,
            })
        } else {
            Err(AndroidAnalyzeError::InvalidManifest("Invalid XML structure".to_string()))
        }
    }
}

/// AXML header structure
#[derive(Debug, Clone)]
struct AXMLHeader {
    magic: u32,
    file_size: u32,
}

/// XML node types
#[derive(Debug, Clone)]
enum XmlNode {
    StartNamespace {
        line_number: u32,
        prefix: Option<String>,
        uri: Option<String>,
    },
    EndNamespace,
    StartElement {
        name: String,
        attributes: HashMap<String, String>,
    },
    EndElement,
    Text {
        content: String,
    },
}
