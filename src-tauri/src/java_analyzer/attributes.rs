use crate::java_analyzer::{ 
    classfile::ClassFile,
    error::{Result, JavaAnalyzeError},
    io::Buffer,
    annotions::Annotation
};
pub struct AttributeNames;

impl AttributeNames {
    pub const CONSTANT_VALUE: &'static str = "ConstantValue";
    pub const CODE: &'static str = "Code";
    pub const STACK_MAP_TABLE: &'static str = "StackMapTable";
    pub const EXCEPTIONS: &'static str = "Exceptions";
    pub const INNER_CLASSES: &'static str = "InnerClasses";
    pub const ENCLOSING_METHOD: &'static str = "EnclosingMethod";
    pub const SYNTHETIC: &'static str = "Synthetic";
    pub const SIGNATURE: &'static str = "Signature";
    pub const SOURCE_FILE: &'static str = "SourceFile";
    pub const SOURCE_DEBUG_EXTENSION: &'static str = "SourceDebugExtension";
    pub const LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
    pub const LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
    pub const LOCAL_VARIABLE_TYPE_TABLE: &'static str = "LocalVariableTypeTable";
    pub const DEPRECATED: &'static str = "Deprecated";
    pub const RUNTIME_VISIBLE_ANNOTATIONS: &'static str = "RuntimeVisibleAnnotations";
    pub const RUNTIME_INVISIBLE_ANNOTATIONS: &'static str = "RuntimeInvisibleAnnotations";
    pub const RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: &'static str = "RuntimeVisibleParameterAnnotations";
    pub const RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: &'static str = "RuntimeInvisibleParameterAnnotations";
    pub const RUNTIME_VISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeVisibleTypeAnnotations";
    pub const RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeInvisibleTypeAnnotations";
    pub const ANNOTATION_DEFAULT: &'static str = "AnnotationDefault";
    pub const BOOTSTRAP_METHODS: &'static str = "BootstrapMethods";
    pub const METHOD_PARAMETERS: &'static str = "MethodParameters";
    pub const MODULE: &'static str = "Module";
    pub const MODULE_PACKAGES: &'static str = "ModulePackages";
    pub const MODULE_MAIN_CLASS: &'static str = "ModuleMainClass";
    pub const NEST_HOST: &'static str = "NestHost";
    pub const NEST_MEMBERS: &'static str = "NestMembers";
    pub const MODULE_TARGET: &'static str = "ModuleTarget";
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(ConstantValueAttribute),
    Code(Code_attribute),
    StackMapTable(StackMapTable_attribute),
    LineNumberTable(LineNumberTable_attribute),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotations_attribute),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations_attribute),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations_attribute),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations_attribute),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotations_attribute),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotations_attribute),
    AnnotationDefault(AnnotationDefault_attribute),
    BootstrapMethods(BootstrapMethods_attribute),
    SourceFile(SourceFile_attribute),
    LocalVariableTable(LocalVariableTable_attribute),
    LocalVariableTypeTable(LocalVariableTypeTable_attribute),
    Deprecated(Deprecated_attribute),
    Signature(Signature_attribute),
    Exceptions(Exceptions_attribute),
    InnerClasses(InnerClasses_attribute),
    EnclosingMethod(EnclosingMethod_attribute),
    SYNTHETIC(Synthetic_attribute),
}

#[derive(Debug)]
pub struct ConstantValueAttribute {
    pub constant_value_index: u16,
}

pub fn read_constant_value_attribute(buffer: &mut Buffer) -> Result<ConstantValueAttribute> {
    let constant_value_index = buffer.read_u16()?;
    Ok(ConstantValueAttribute { constant_value_index })
}

#[derive(Debug)]
pub struct Exceptions_attribute {
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<u16>,
}

fn read_exceptions_attribute(buffer: &mut Buffer) -> Result<Exceptions_attribute> {
    let number_of_exceptions = buffer.read_u16()?;
    let mut exception_index_table = Vec::new();
    for _ in 0..number_of_exceptions {
        let exception_index = buffer.read_u16()?;
        exception_index_table.push(exception_index);
    }
    Ok(Exceptions_attribute { number_of_exceptions, exception_index_table })
}

#[derive(Debug)]
pub struct InnerClasses_attribute {
    pub number_of_classes: u16,
    pub classes: Vec<InnerClass>,
}

#[derive(Debug)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

fn read_innter_class_attribute(buffer: &mut Buffer) -> Result<InnerClasses_attribute> {
    let number_of_classes = buffer.read_u16()?;
    let mut classes = Vec::new();
    for _ in 0..number_of_classes {
        let inner_class_info_index = buffer.read_u16()?;
        let outer_class_info_index = buffer.read_u16()?;
        let inner_name_index = buffer.read_u16()?;
        let inner_class_access_flags = buffer.read_u16()?;
        classes.push(InnerClass {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        });
    }
    Ok(InnerClasses_attribute { number_of_classes, classes })
}

#[derive(Debug)]
pub struct EnclosingMethod_attribute {
    pub class_index: u16,
    pub method_index: u16,
}

fn read_enclosing_method_attribute(buffer: &mut Buffer) -> Result<EnclosingMethod_attribute> {
    let class_index = buffer.read_u16()?;
    let method_index = buffer.read_u16()?;
    Ok(EnclosingMethod_attribute { class_index, method_index })
}

#[derive(Debug)]
pub struct Signature_attribute {
    pub signature_index: u16,
}

fn read_signature_attribute(buffer: &mut Buffer) -> Result<Signature_attribute> {
    let signature_index = buffer.read_u16()?;
    Ok(Signature_attribute { signature_index })
}

#[derive(Debug)]
pub struct SourceFile_attribute {
    pub sourcefile_index: u16,
}

fn read_sourcefile_attribute(buffer: &mut Buffer) -> Result<SourceFile_attribute> {
    let sourcefile_index = buffer.read_u16()?;
    Ok(SourceFile_attribute { sourcefile_index })
}

#[derive(Debug)]
pub struct SourceDebugExtension_attribute {
    pub debug_extension: Vec<u8>,
}

#[derive(Debug)]
pub struct LocalVariableTable_attribute {
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

fn read_local_variable_table(buffer: &mut Buffer) -> Result<LocalVariableTable_attribute> {
    let local_variable_table_length = buffer.read_u16()?;
    let mut local_variable_table = Vec::new();
    for _ in 0..local_variable_table_length {
        let start_pc = buffer.read_u16()?;
        let length = buffer.read_u16()?;
        let name_index = buffer.read_u16()?;
        let descriptor_index = buffer.read_u16()?;
        let index = buffer.read_u16()?;
        local_variable_table.push(LocalVariableTableEntry {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        });
    }
    Ok(LocalVariableTable_attribute {
        local_variable_table_length,
        local_variable_table,
    })
}

#[derive(Debug)]
pub struct LocalVariableTypeTable_attribute {
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

fn read_local_variable_type_table(buffer: &mut Buffer) -> Result<LocalVariableTypeTable_attribute> {
    let local_variable_type_table_length = buffer.read_u16()?;
    let mut local_variable_type_table = Vec::new();
    for _ in 0..local_variable_type_table_length {
        let start_pc = buffer.read_u16()?;
        let length = buffer.read_u16()?;
        let name_index = buffer.read_u16()?;
        let signature_index = buffer.read_u16()?;
        let index = buffer.read_u16()?;
        local_variable_type_table.push(LocalVariableTypeTableEntry {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        });
    }
    Ok(LocalVariableTypeTable_attribute {
        local_variable_type_table_length,
        local_variable_type_table,
    })
}


#[derive(Debug)]
pub struct Deprecated_attribute; 

#[derive(Debug)]
pub struct Synthetic_attribute;

#[derive(Debug)]
pub struct RuntimeVisibleAnnotations_attribute {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

fn read_runtime_visible_annotations_attribute(buffer: &mut Buffer) -> Result<RuntimeVisibleAnnotations_attribute> {
    let num_annotations = buffer.read_u16()?;
    let mut annotations = Vec::new();
    for _ in 0..num_annotations {
        let annotation = Annotation::read(buffer)?;
        annotations.push(annotation);
    }
    Ok(RuntimeVisibleAnnotations_attribute { num_annotations, annotations })
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotations_attribute {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

fn read_runtime_invisible_annotations_attribute(buffer: &mut Buffer) -> Result<RuntimeInvisibleAnnotations_attribute> {
    let num_annotations = buffer.read_u16()?;
    let mut annotations = Vec::new();
    for _ in 0..num_annotations {
        let annotation = Annotation::read(buffer)?;
        annotations.push(annotation);
    }
    Ok(RuntimeInvisibleAnnotations_attribute { num_annotations, annotations })
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotations_attribute {
    pub num_parameters: u16,
    pub parameter_annotations: Vec<Vec<Annotation>>,
}

fn read_runtime_visible_parameter_annotations_attribute(buffer: &mut Buffer) -> Result<RuntimeVisibleParameterAnnotations_attribute> {
    let num_parameters = buffer.read_u8()? as u16;
    let mut parameter_annotations = Vec::new();
    for _ in 0..num_parameters {
        let num_annotations = buffer.read_u16()?;
        let mut annotations = Vec::new();
        for _ in 0..num_annotations {
            let annotation = Annotation::read(buffer)?;
            annotations.push(annotation);
        }
        parameter_annotations.push(annotations);
    }
    Ok(RuntimeVisibleParameterAnnotations_attribute { num_parameters, parameter_annotations })
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotations_attribute {
    pub num_parameters: u16,
    pub parameter_annotations: Vec<Vec<Annotation>>,
}

fn read_runtime_invisible_parameter_annotations_attribute(buffer: &mut Buffer) -> Result<RuntimeInvisibleParameterAnnotations_attribute> {
    let num_parameters = buffer.read_u8()? as u16;
    let mut parameter_annotations = Vec::new();
    for _ in 0..num_parameters {
        let num_annotations = buffer.read_u16()?;
        let mut annotations = Vec::new();
        for _ in 0..num_annotations {
            let annotation = Annotation::read(buffer)?;
            annotations.push(annotation);
        }
        parameter_annotations.push(annotations);
    }
    Ok(RuntimeInvisibleParameterAnnotations_attribute { num_parameters, parameter_annotations })
}

#[derive(Debug)]    
pub struct RuntimeVisibleTypeAnnotations_attribute {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

fn read_runtime_visible_type_annotations_attribute(buffer: &mut Buffer) -> Result<RuntimeVisibleTypeAnnotations_attribute> {
    let num_annotations = buffer.read_u16()?;
    let mut annotations = Vec::new();
    for _ in 0..num_annotations {
        let annotation = Annotation::read(buffer)?;
        annotations.push(annotation);
    }
    Ok(RuntimeVisibleTypeAnnotations_attribute { num_annotations, annotations })
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotations_attribute {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

fn read_runtime_invisible_type_annotations_attribute(buffer: &mut Buffer) -> Result<RuntimeInvisibleTypeAnnotations_attribute> {
    let num_annotations = buffer.read_u16()?;
    let mut annotations = Vec::new();
    for _ in 0..num_annotations {
        let annotation = Annotation::read(buffer)?;
        annotations.push(annotation);
    }
    Ok(RuntimeInvisibleTypeAnnotations_attribute { num_annotations, annotations })
}

#[derive(Debug)]
pub struct AnnotationDefault_attribute {
    pub default_value: Vec<u8>,
}

fn read_annotation_default_attribute(buffer: &mut Buffer, length: u32) -> Result<AnnotationDefault_attribute> {
    let default_value = buffer.read_bytes(length as usize)?;
    Ok(AnnotationDefault_attribute { default_value: default_value.to_vec() })
}

#[derive(Debug)]
pub struct BootstrapMethods_attribute {
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

fn read_bootstrap_methods_attribute(buffer: &mut Buffer) -> Result<BootstrapMethods_attribute> {
    let num_bootstrap_methods = buffer.read_u16()?;
    let mut bootstrap_methods = Vec::new();
    for _ in 0..num_bootstrap_methods {
        let bootstrap_method_ref = buffer.read_u16()?;
        let num_bootstrap_arguments = buffer.read_u16()?;
        let mut bootstrap_arguments = Vec::new();
        for _ in 0..num_bootstrap_arguments {
            let arg = buffer.read_u16()?;
            bootstrap_arguments.push(arg);
        }
        bootstrap_methods.push(BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        });
    }
    Ok(BootstrapMethods_attribute { num_bootstrap_methods, bootstrap_methods })
}

#[derive(Debug)]
pub struct MethodParameters_attribute {
    pub parameters_count: u8,
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16,
}

#[derive(Debug)]
pub struct Module_attribute {
    pub module_name_index: u16,
    pub module_flags: u16,
    pub module_version_index: u16,
    pub requires_count: u16,
    pub requires: Vec<ModuleRequires>,
    pub exports_count: u16,
    pub exports: Vec<ModuleExports>,
    pub opens_count: u16,
    pub opens: Vec<ModuleOpens>,
}

#[derive(Debug)]
pub struct ModuleRequires {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}

#[derive(Debug)]
pub struct ModuleExports {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to: Vec<u16>,
}

#[derive(Debug)]
pub struct ModuleOpens {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to: Vec<u16>,
}

#[derive(Debug)]
pub struct ModuleMainClass_attribute {
    pub main_class_index: u16,
}

#[derive(Debug)]
pub struct NestHost_attribute {
    pub nest_host_index: u16,
}

#[derive(Debug)]
pub struct NestMembers_attribute {
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

#[derive(Debug)]
pub struct ModuleTarget_attribute {
    pub module_target_index: u16,
    pub module_target_flags: u16,
    pub module_target_version_index: u16,
}

#[derive(Debug)]
pub struct ModulePackages_attribute {
    pub number_of_packages: u16,
    pub packages: Vec<u16>,
}

/// LineNumberTable attribute
#[derive(Debug)]
pub struct LineNumberTable_attribute {
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

fn read_line_number_table(buffer: &mut Buffer) -> Result<LineNumberTable_attribute> {
    let line_number_table_length = buffer.read_u16()?;
    let mut line_number_table = Vec::new();
    for _ in 0..line_number_table_length {
        let start_pc = buffer.read_u16()?;
        let line_number = buffer.read_u16()?;
        line_number_table.push(LineNumberTableEntry {
            start_pc,
            line_number,
        });
    }
    Ok(LineNumberTable_attribute {
        line_number_table_length,
        line_number_table,
    })
}


/// Code attribute
/// The Code attribute is used to store the bytecode of a method.
#[derive(Debug)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct Code_attribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<ExceptionTable>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

fn read_code_attribute(buffer: &mut Buffer, classfile: &ClassFile) -> Result<Code_attribute> {
    let max_stack = buffer.read_u16()?;
    let max_locals = buffer.read_u16()?;
    let code_length = buffer.read_u32()?;
    let code = buffer.read_bytes(code_length as usize)?;
    let exception_table_length = buffer.read_u16()?;
    let mut exception_table = Vec::new();
    for _ in 0..exception_table_length {
        let start_pc = buffer.read_u16()?;
        let end_pc = buffer.read_u16()?;
        let handler_pc = buffer.read_u16()?;
        let catch_type = buffer.read_u16()?;
        exception_table.push(ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        });
    }
    let attributes_count = buffer.read_u16()?;
    let mut attributes = Vec::new();
    for _ in 0..attributes_count {
        let attribute = read_raw_attribute(buffer, classfile)?;
        attributes.push(attribute);
    }
    Ok(Code_attribute {
        max_stack,
        max_locals,
        code_length,
        code: code.to_vec(),
        exception_table_length,
        exception_table,
        attributes_count: 0,
        attributes: attributes,
    })
}

/// StackMapTable attribute
/// The StackMapTable attribute is used to store stack map frames for a method.
/// It is used for stack map verification in the Java Virtual Machine.
/// The StackMapTable attribute is a variable-length attribute.
/// The StackMapTable attribute is used to store stack map frames for a method.
/// It is used for stack map verification in the Java Virtual Machine.
/// The StackMapTable attribute is a variable-length attribute.     
#[derive(Debug)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}
#[derive(Debug)]
pub enum StackMapFrame {
    Same { frame_type: u8 },
    SameLocals1StackItem { frame_type: u8, stack: VerificationTypeInfo },
    SameLocals1StackItemExtended { offset_delta: u16, stack: VerificationTypeInfo },
    Chop { frame_type: u8, offset_delta: u16 },
    SameExtended { offset_delta: u16 },
    Append { frame_type: u8, offset_delta: u16, locals: Vec<VerificationTypeInfo> },
    Full { offset_delta: u16, locals: Vec<VerificationTypeInfo>, stack: Vec<VerificationTypeInfo> },
}

#[derive(Debug)]
pub struct  StackMapTable_attribute {
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrame>,
}

fn read_stack_map_table(buffer: &mut Buffer) -> Result<StackMapTable_attribute> {
    let number_of_entries = buffer.read_u16()?;
    let mut entries = Vec::new();
    for _ in 0..number_of_entries {
        let frame_type = buffer.read_u8()?;
        let frame = match frame_type {
            0..=63 => StackMapFrame::Same { frame_type },
            64..=127 => StackMapFrame::SameLocals1StackItem {
                frame_type,
                stack: read_verification_type_info(buffer)?,
            },
            247 => StackMapFrame::SameLocals1StackItemExtended {
                offset_delta: buffer.read_u16()?,
                stack: read_verification_type_info(buffer)?,
            },
            248..=250 => StackMapFrame::Chop {
                frame_type,
                offset_delta: buffer.read_u16()?,
            },
            251 => StackMapFrame::SameExtended {
                offset_delta: buffer.read_u16()?,
            },
            252..=254 => {
                let offset_delta = buffer.read_u16()?;
                let locals_count = (frame_type - 251) as usize;
                let mut locals = Vec::with_capacity(locals_count);
                for _ in 0..locals_count {
                    locals.push(read_verification_type_info(buffer)?);
                }
                StackMapFrame::Append { frame_type, offset_delta, locals }
            }
            255 => {
                let offset_delta = buffer.read_u16()?;
                let num_locals = buffer.read_u16()? as usize;
                let mut locals = Vec::with_capacity(num_locals);
                for _ in 0..num_locals {
                    locals.push(read_verification_type_info(buffer)?);
                }
                let num_stack_items = buffer.read_u16()? as usize;
                let mut stack = Vec::with_capacity(num_stack_items);
                for _ in 0..num_stack_items {
                    stack.push(read_verification_type_info(buffer)?);
                }
                StackMapFrame::Full { offset_delta, locals, stack }
            }
            _ => return Err(JavaAnalyzeError::InvalidClassData("Invalid stack map frame type".to_owned())),
        };
        entries.push(frame);
    }
    Ok(StackMapTable_attribute { number_of_entries, entries })
}

fn read_verification_type_info(buffer: &mut Buffer) -> Result<VerificationTypeInfo> {
    let tag = buffer.read_u8()?;
    Ok(match tag {
        0 => VerificationTypeInfo::Top,
        1 => VerificationTypeInfo::Integer,
        2 => VerificationTypeInfo::Float,
        3 => VerificationTypeInfo::Double,
        4 => VerificationTypeInfo::Long,
        5 => VerificationTypeInfo::Null,
        6 => VerificationTypeInfo::UninitializedThis,
        7 => VerificationTypeInfo::Object { cpool_index: buffer.read_u16()? },
        8 => VerificationTypeInfo::Uninitialized { offset: buffer.read_u16()? },
        _ => return Err(JavaAnalyzeError::InvalidClassData("Invalid verification type info".to_owned())),
    })
}


/// read a raw attribute from the class file
pub(crate) fn read_raw_attribute(buffer:&mut Buffer, classfile: &ClassFile) -> Result<Attribute> {
    let attribute_name_index = buffer.read_u16()?;
    let attribute_name = classfile.constant_pool.get_utf8(attribute_name_index as usize).unwrap();
    let attribute_length = buffer.read_u32()?;
    match attribute_name.as_str() {
        AttributeNames::CODE => {
            let code_attribute = read_code_attribute(buffer, classfile)?;
            return Ok(Attribute::Code(code_attribute));
        }
        AttributeNames::LINE_NUMBER_TABLE => {
            let line_number_table = read_line_number_table(buffer)?;
            return Ok(Attribute::LineNumberTable(line_number_table));
        }
        AttributeNames::STACK_MAP_TABLE => {
            let stack_map_table = read_stack_map_table(buffer)?;
            return Ok(Attribute::StackMapTable(stack_map_table));
        }
        AttributeNames::SOURCE_FILE => {
            let source_file_attribute = read_sourcefile_attribute(buffer)?;
            return Ok(Attribute::SourceFile(source_file_attribute));
        }
        AttributeNames::CONSTANT_VALUE => {
            let constant_value_attribute = read_constant_value_attribute(buffer)?;
            return Ok(Attribute::ConstantValue(constant_value_attribute));
        }
        AttributeNames::LOCAL_VARIABLE_TABLE => {
            let local_variable_table = read_local_variable_table(buffer)?;
            return Ok(Attribute::LocalVariableTable(local_variable_table));
        }
        AttributeNames::DEPRECATED => {
            return Ok(Attribute::Deprecated(Deprecated_attribute));
        }
        AttributeNames::SIGNATURE => {
            let signature_attribute = read_signature_attribute(buffer)?;
            return Ok(Attribute::Signature(signature_attribute));
        }
        AttributeNames::LOCAL_VARIABLE_TYPE_TABLE => {
            let local_variable_type_table = read_local_variable_type_table(buffer)?;
            return Ok(Attribute::LocalVariableTypeTable(local_variable_type_table));
        }
        AttributeNames::INNER_CLASSES => {
            let inner_classes_attribute = read_innter_class_attribute(buffer)?;
            return Ok(Attribute::InnerClasses(inner_classes_attribute));
        }
        AttributeNames::EXCEPTIONS => {
            let exceptions_attribute = read_exceptions_attribute(buffer)?;
            return Ok(Attribute::Exceptions(exceptions_attribute));
        }
        AttributeNames::RUNTIME_VISIBLE_ANNOTATIONS => {
            let runtime_visible_annotations = read_runtime_visible_annotations_attribute(buffer)?;
            return Ok(Attribute::RuntimeVisibleAnnotations(runtime_visible_annotations));
        }
        AttributeNames::ENCLOSING_METHOD => {
            let enclosing_method_attribute = read_enclosing_method_attribute(buffer)?;
            return Ok(Attribute::EnclosingMethod(enclosing_method_attribute));
        }
        AttributeNames::SYNTHETIC => {
            return Ok(Attribute::SYNTHETIC(Synthetic_attribute));
        }
        AttributeNames::RUNTIME_INVISIBLE_ANNOTATIONS => {
            let runtime_invisible_annotations = read_runtime_invisible_annotations_attribute(buffer)?;
            return Ok(Attribute::RuntimeInvisibleAnnotations(runtime_invisible_annotations));
        }
        AttributeNames::RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => {
            let runtime_visible_parameter_annotations = read_runtime_visible_parameter_annotations_attribute(buffer)?;
            return Ok(Attribute::RuntimeVisibleParameterAnnotations(runtime_visible_parameter_annotations));
        }
        AttributeNames::RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
            let runtime_invisible_parameter_annotations = read_runtime_invisible_parameter_annotations_attribute(buffer)?;
            return Ok(Attribute::RuntimeInvisibleParameterAnnotations(runtime_invisible_parameter_annotations));
        }
        AttributeNames::RUNTIME_VISIBLE_TYPE_ANNOTATIONS => {
            let runtime_visible_type_annotations = read_runtime_visible_type_annotations_attribute(buffer)?;
            return Ok(Attribute::RuntimeVisibleTypeAnnotations(runtime_visible_type_annotations));
        }
        AttributeNames::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS => {
            let runtime_invisible_type_annotations = read_runtime_invisible_type_annotations_attribute(buffer)?;
            return Ok(Attribute::RuntimeInvisibleTypeAnnotations(runtime_invisible_type_annotations));
        }
        AttributeNames::ANNOTATION_DEFAULT => {
            let annotation_default = read_annotation_default_attribute(buffer, attribute_length)?;
            return Ok(Attribute::AnnotationDefault(annotation_default));
        }
        AttributeNames::BOOTSTRAP_METHODS => {
            let bootstrap_methods = read_bootstrap_methods_attribute(buffer)?;
            return Ok(Attribute::BootstrapMethods(bootstrap_methods));
        }
        _ => {
            // Handle other attributes or return an error
            return Err(JavaAnalyzeError::InvalidClassData("Unknown attribute type".to_owned()));
        }
    }
}