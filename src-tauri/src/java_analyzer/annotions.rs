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
    ConstantValue(u16),
    EnumConstantValue(u16, u16),
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

impl  ElementValue {
    pub fn read(buffer: &mut Buffer) -> Result<Self> {
        let tag = buffer.read_u8()?;
        match tag {
            0x01 => Ok(ElementValue::ConstantValue(buffer.read_u16()?)),
            0x02 => Ok(ElementValue::EnumConstantValue(buffer.read_u16()?, buffer.read_u16()?)),
            0x03 => Ok(ElementValue::ClassInfoIndex(buffer.read_u16()?)),
            0x04 => Ok(ElementValue::AnnotationValue(Annotation::read(buffer)?)),
            0x05 => {
                let num_values = buffer.read_u16()?;
                let mut values = Vec::new();
                for _ in 0..num_values {
                    values.push(ElementValue::read(buffer)?);
                }
                Ok(ElementValue::ArrayValue(values))
            }
            _ => Err(JavaAnalyzeError::InvalidClassData("Unknown element value tag".to_owned())),
        }
    }
    
}

pub fn read_annotation(buffer: &mut Buffer) -> Result<Annotation> {
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