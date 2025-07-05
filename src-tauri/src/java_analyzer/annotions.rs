use crate::java_analyzer::io::Buffer;
use crate::java_analyzer::error::{Result, JavaAnalyzeError};

#[derive(Debug)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn read(buffer: &mut Buffer) -> Result<Self> {
        let type_index = buffer.read_u16()?;
        let num_element_value_pairs = buffer.read_u16()?;
        let mut element_value_pairs = Vec::new();

        for _ in 0..num_element_value_pairs {
            let element_name_index = buffer.read_u16()?;
            let value = ElementValue::read(buffer)?;
            element_value_pairs.push(ElementValuePair {
                element_name_index,
                value,
            });
        }

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug)]
pub enum ElementValue {
    // 基本类型和字符串 - tag 'B', 'C', 'D', 'F', 'I', 'J', 'S', 'Z', 's'
    Byte(u16),           // 'B' (0x42)
    Char(u16),           // 'C' (0x43) 
    Double(u16),         // 'D' (0x44)
    Float(u16),          // 'F' (0x46)
    Int(u16),            // 'I' (0x49)
    Long(u16),           // 'J' (0x4A)
    Short(u16),          // 'S' (0x53)
    Boolean(u16),        // 'Z' (0x5A)
    String(u16),         // 's' (0x73)
    
    // 枚举类型 - tag 'e'
    EnumConstValue { type_name_index: u16, const_name_index: u16 }, // 'e' (0x65)
    
    // 类类型 - tag 'c'
    Class(u16),          // 'c' (0x63)
    
    // 注解类型 - tag '@'
    Annotation(Box<Annotation>), // '@' (0x40)
    
    // 数组类型 - tag '['
    Array(Vec<ElementValue>),    // '[' (0x5B)
}

impl ElementValue {
    pub fn read(buffer: &mut Buffer) -> Result<Self> {
        let tag = buffer.read_u8()?;
        match tag {
            // 基本类型和字符串
            b'B' => Ok(ElementValue::Byte(buffer.read_u16()?)),          // 0x42
            b'C' => Ok(ElementValue::Char(buffer.read_u16()?)),          // 0x43
            b'D' => Ok(ElementValue::Double(buffer.read_u16()?)),        // 0x44
            b'F' => Ok(ElementValue::Float(buffer.read_u16()?)),         // 0x46
            b'I' => Ok(ElementValue::Int(buffer.read_u16()?)),           // 0x49
            b'J' => Ok(ElementValue::Long(buffer.read_u16()?)),          // 0x4A
            b'S' => Ok(ElementValue::Short(buffer.read_u16()?)),         // 0x53
            b'Z' => Ok(ElementValue::Boolean(buffer.read_u16()?)),       // 0x5A
            b's' => Ok(ElementValue::String(buffer.read_u16()?)),        // 0x73
            
            // 枚举类型
            b'e' => {
                let type_name_index = buffer.read_u16()?;
                let const_name_index = buffer.read_u16()?;
                Ok(ElementValue::EnumConstValue { type_name_index, const_name_index })
            },
            
            // 类类型
            b'c' => Ok(ElementValue::Class(buffer.read_u16()?)),         // 0x63
            
            // 注解类型
            b'@' => Ok(ElementValue::Annotation(Box::new(Annotation::read(buffer)?))), // 0x40
            
            // 数组类型
            b'[' => {
                let num_values = buffer.read_u16()?;
                let mut values = Vec::new();
                for _ in 0..num_values {
                    values.push(ElementValue::read(buffer)?);
                }
                Ok(ElementValue::Array(values))
            },
            
            _ => Err(JavaAnalyzeError::InvalidClassData(
                format!("Unknown element value tag: 0x{:02X} ('{}')", tag, tag as char)
            )),
        }
    }
}

