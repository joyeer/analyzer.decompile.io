use crate::java_analyzer::classfile::{ClassFile, ClassFileReader};
use crate::java_analyzer::constantpool::ConstantPoolEntry;
use crate::java_analyzer::attributes::Attribute;
use crate::java_analyzer::opcode::*;
use std::result::Result;

pub struct ClassFileDisassembler {
    class_file: ClassFile,
}

impl ClassFileDisassembler {
    pub fn new(data: Vec<u8>) -> Result<Self, String> {
        let reader = ClassFileReader::new(&data);
        match reader.read() {
            Ok(class_file) => Ok(Self { class_file }),
            Err(e) => Err(format!("Failed to parse class file: {:?}", e)),
        }
    }

    pub fn disassemble(&self) -> Result<String, String> {
        let mut output = String::new();
        
        // 输出基本信息
        output.push_str(&format!("// Class file version: {}.{}\n", 
            self.class_file.major_version, self.class_file.minor_version));
        output.push_str(&format!("// Magic: 0x{:08X}\n\n", self.class_file.magic));
        
        // 输出常量池
        self.format_constant_pool(&mut output);
        
        // 输出访问标志
        output.push_str(&format!("\nAccess flags: 0x{:04X} [{}]\n", 
            self.class_file.access_flags, 
            Self::format_access_flags(self.class_file.access_flags)));
        
        // 输出类信息
        self.format_class_info(&mut output);
        
        // 输出接口
        self.format_interfaces(&mut output);
        
        // 输出字段
        self.format_fields(&mut output);
        
        // 输出方法
        self.format_methods(&mut output);
        
        // 输出类属性
        self.format_class_attributes(&mut output);
        
        Ok(output)
    }

    fn format_constant_pool(&self, output: &mut String) {
        output.push_str(&format!("Constant pool ({}):\n", 
            self.class_file.constant_pool.constant_pool.len()));
        
        for (index, entry) in self.class_file.constant_pool.constant_pool.iter().enumerate() {
            let formatted = self.format_constant_pool_entry(entry, index + 1);
            output.push_str(&format!("  #{}: {}\n", index + 1, formatted));
        }
    }

    fn format_constant_pool_entry(&self, entry: &ConstantPoolEntry, _index: usize) -> String {
        match entry {
            ConstantPoolEntry::Utf8(s) => format!("Utf8 [{}]", s),
            ConstantPoolEntry::Integer(i) => format!("Integer [{}]", i),
            ConstantPoolEntry::Float(f) => format!("Float [{}]", f),
            ConstantPoolEntry::Long(l) => format!("Long [{}]", l),
            ConstantPoolEntry::Double(d) => format!("Double [{}]", d),
            ConstantPoolEntry::ClassRef(index) => {
                if let Some(name) = self.class_file.constant_pool.get_utf8(*index as usize) {
                    format!("Class [#{} = {}]", index, name)
                } else {
                    format!("Class [#{}]", index)
                }
            },
            ConstantPoolEntry::StringRef(index) => {
                if let Some(string) = self.class_file.constant_pool.get_utf8(*index as usize) {
                    format!("String [#{} = \"{}\"]", index, string)
                } else {
                    format!("String [#{}]", index)
                }
            },
            ConstantPoolEntry::FieldRef(class_index, name_and_type_index) => {
                format!("Fieldref [#{}.#{}]", class_index, name_and_type_index)
            },
            ConstantPoolEntry::MethodRef(class_index, name_and_type_index) => {
                format!("Methodref [#{}.#{}]", class_index, name_and_type_index)
            },
            ConstantPoolEntry::InterfaceMethodRef(class_index, name_and_type_index) => {
                format!("InterfaceMethodref [#{}.#{}]", class_index, name_and_type_index)
            },
            ConstantPoolEntry::NameAndTypeRef(name_index, descriptor_index) => {
                format!("NameAndType [#{}:#{}]", name_index, descriptor_index)
            },
            ConstantPoolEntry::MethodHandleRef(kind, index) => {
                format!("MethodHandle [{}:#{}]", kind, index)
            },
            ConstantPoolEntry::MethodTypeRef(index) => {
                format!("MethodType [#{}]", index)
            },
            ConstantPoolEntry::InvokeDynamicRef(bootstrap_index, name_and_type_index) => {
                format!("InvokeDynamic [#{}:#{}]", bootstrap_index, name_and_type_index)
            },
            ConstantPoolEntry::Module(index) => {
                format!("Module [#{}]", index)
            },
            ConstantPoolEntry::Package(index) => {
                format!("Package [#{}]", index)
            },
            ConstantPoolEntry::Dynamic(bootstrap_index, name_and_type_index) => {
                format!("Dynamic [#{}:#{}]", bootstrap_index, name_and_type_index)
            },
        }
    }

    fn format_class_info(&self, output: &mut String) {
        let this_class_name = if let Some(name) = self.get_class_name(self.class_file.this_class) {
            format!("#{} = {}", self.class_file.this_class, name)
        } else {
            format!("#{}", self.class_file.this_class)
        };

        let super_class_name = if self.class_file.super_class != 0 {
            if let Some(name) = self.get_class_name(self.class_file.super_class) {
                format!("#{} = {}", self.class_file.super_class, name)
            } else {
                format!("#{}", self.class_file.super_class)
            }
        } else {
            "0".to_string()
        };

        output.push_str(&format!("This class: {}\n", this_class_name));
        output.push_str(&format!("Super class: {}\n", super_class_name));
    }

    fn format_interfaces(&self, output: &mut String) {
        if !self.class_file.interfaces.is_empty() {
            output.push_str(&format!("\nInterfaces ({}):\n", self.class_file.interfaces.len()));
            for (i, interface_index) in self.class_file.interfaces.iter().enumerate() {
                let interface_name = if let Some(name) = self.get_class_name(*interface_index) {
                    format!("#{} = {}", interface_index, name)
                } else {
                    format!("#{}", interface_index)
                };
                output.push_str(&format!("  #{}: {}\n", i, interface_name));
            }
        }
    }

    fn format_fields(&self, output: &mut String) {
        if !self.class_file.fields.is_empty() {
            output.push_str(&format!("\nFields ({}):\n", self.class_file.fields.len()));
            for (i, field) in self.class_file.fields.iter().enumerate() {
                output.push_str(&format!("  Field #{}: {} {} {}\n", 
                    i,
                    Self::format_access_flags(field.access_flags),
                    field.name,
                    field.descriptor
                ));
                
                // 输出字段属性
                for attribute in &field.attributes {
                    self.format_attribute(output, attribute, "    ");
                }
            }
        }
    }

    fn format_methods(&self, output: &mut String) {
        if !self.class_file.methods.is_empty() {
            output.push_str(&format!("\nMethods ({}):\n", self.class_file.methods.len()));
            for (i, method) in self.class_file.methods.iter().enumerate() {
                output.push_str(&format!("  Method #{}: {} {} {}\n", 
                    i,
                    Self::format_access_flags(method.access_flags),
                    method.name,
                    method.descriptor
                ));
                
                // 输出方法属性（包括代码）
                for attribute in &method.attributes {
                    self.format_attribute(output, attribute, "    ");
                }

                // 输出字节码指令
                if !method.code.is_empty() {
                    output.push_str("    Bytecode:\n");
                    for instruction in &method.code {
                        let formatted = self.format_instruction(instruction);
                        output.push_str(&format!("      {}: {}\n", instruction.offset, formatted));
                    }
                }
            }
        }
    }

    fn format_class_attributes(&self, output: &mut String) {
        if !self.class_file.attributes.is_empty() {
            output.push_str(&format!("\nClass attributes ({}):\n", self.class_file.attributes.len()));
            for attribute in &self.class_file.attributes {
                self.format_attribute(output, attribute, "  ");
            }
        }
    }

    fn format_attribute(&self, output: &mut String, attribute: &Attribute, indent: &str) {
        match attribute {
            Attribute::SourceFile(source_file) => {
                if let Some(filename) = self.class_file.constant_pool.get_utf8(source_file.sourcefile_index as usize) {
                    output.push_str(&format!("{}SourceFile: \"{}\"\n", indent, filename));
                }
            },
            Attribute::Code(code_attr) => {
                output.push_str(&format!("{}Code:\n", indent));
                output.push_str(&format!("{}  stack={}, locals={}, args_size=?\n", 
                    indent, code_attr.max_stack, code_attr.max_locals));
                
                // 输出字节码会在 format_methods 中单独处理
            },
            Attribute::LineNumberTable(line_table) => {
                output.push_str(&format!("{}LineNumberTable:\n", indent));
                // 输出行号表详情
            },
            Attribute::RuntimeVisibleAnnotations(annotations) => {
                output.push_str(&format!("{}RuntimeVisibleAnnotations: {} annotations\n", 
                    indent, annotations.num_annotations));
            },
            _ => {
                output.push_str(&format!("{}Attribute: {:?}\n", indent, attribute));
            }
        }
    }

    fn format_instruction(&self, instruction: &crate::java_analyzer::opcode::Instruction) -> String {
        match instruction.opcode {
            OP_NOP => "nop".to_string(),
            OP_ACONST_NULL => "aconst_null".to_string(),
            OP_ICONST_M1 => "iconst_m1".to_string(),
            OP_ICONST_0 => "iconst_0".to_string(),
            OP_ICONST_1 => "iconst_1".to_string(),
            OP_ICONST_2 => "iconst_2".to_string(),
            OP_ICONST_3 => "iconst_3".to_string(),
            OP_ICONST_4 => "iconst_4".to_string(),
            OP_ICONST_5 => "iconst_5".to_string(),
            OP_LCONST_0 => "lconst_0".to_string(),
            OP_LCONST_1 => "lconst_1".to_string(),
            OP_FCONST_0 => "fconst_0".to_string(),
            OP_FCONST_1 => "fconst_1".to_string(),
            OP_FCONST_2 => "fconst_2".to_string(),
            OP_DCONST_0 => "dconst_0".to_string(),
            OP_DCONST_1 => "dconst_1".to_string(),
            OP_BIPUSH => format!("bipush {}", instruction.value),
            OP_SIPUSH => format!("sipush {}", instruction.value),
            OP_LDC => {
                if let Some(constant) = self.get_constant_info(instruction.value as u16) {
                    format!("ldc #{} // {}", instruction.value, constant)
                } else {
                    format!("ldc #{}", instruction.value)
                }
            },
            OP_LDC_W => {
                if let Some(constant) = self.get_constant_info(instruction.value as u16) {
                    format!("ldc_w #{} // {}", instruction.value, constant)
                } else {
                    format!("ldc_w #{}", instruction.value)
                }
            },
            OP_LDC2_W => {
                if let Some(constant) = self.get_constant_info(instruction.value as u16) {
                    format!("ldc2_w #{} // {}", instruction.value, constant)
                } else {
                    format!("ldc2_w #{}", instruction.value)
                }
            },
            OP_ILOAD => format!("iload {}", instruction.value),
            OP_LLOAD => format!("lload {}", instruction.value),
            OP_FLOAD => format!("fload {}", instruction.value),
            OP_DLOAD => format!("dload {}", instruction.value),
            OP_ALOAD => format!("aload {}", instruction.value),
            OP_ILOAD_0 => "iload_0".to_string(),
            OP_ILOAD_1 => "iload_1".to_string(),
            OP_ILOAD_2 => "iload_2".to_string(),
            OP_ILOAD_3 => "iload_3".to_string(),
            OP_LLOAD_0 => "lload_0".to_string(),
            OP_LLOAD_1 => "lload_1".to_string(),
            OP_LLOAD_2 => "lload_2".to_string(),
            OP_LLOAD_3 => "lload_3".to_string(),
            OP_FLOAD_0 => "fload_0".to_string(),
            OP_FLOAD_1 => "fload_1".to_string(),
            OP_FLOAD_2 => "fload_2".to_string(),
            OP_FLOAD_3 => "fload_3".to_string(),
            OP_DLOAD_0 => "dload_0".to_string(),
            OP_DLOAD_1 => "dload_1".to_string(),
            OP_DLOAD_2 => "dload_2".to_string(),
            OP_DLOAD_3 => "dload_3".to_string(),
            OP_ALOAD_0 => "aload_0".to_string(),
            OP_ALOAD_1 => "aload_1".to_string(),
            OP_ALOAD_2 => "aload_2".to_string(),
            OP_ALOAD_3 => "aload_3".to_string(),
            OP_IALOAD => "iaload".to_string(),
            OP_LALOAD => "laload".to_string(),
            OP_FALOAD => "faload".to_string(),
            OP_DALOAD => "daload".to_string(),
            OP_AALOAD => "aaload".to_string(),
            OP_BALOAD => "baload".to_string(),
            OP_CALOAD => "caload".to_string(),
            OP_SALOAD => "saload".to_string(),
            OP_ISTORE => format!("istore {}", instruction.value),
            OP_LSTORE => format!("lstore {}", instruction.value),
            OP_FSTORE => format!("fstore {}", instruction.value),
            OP_DSTORE => format!("dstore {}", instruction.value),
            OP_ASTORE => format!("astore {}", instruction.value),
            OP_ISTORE_0 => "istore_0".to_string(),
            OP_ISTORE_1 => "istore_1".to_string(),
            OP_ISTORE_2 => "istore_2".to_string(),
            OP_ISTORE_3 => "istore_3".to_string(),
            OP_LSTORE_0 => "lstore_0".to_string(),
            OP_LSTORE_1 => "lstore_1".to_string(),
            OP_LSTORE_2 => "lstore_2".to_string(),
            OP_LSTORE_3 => "lstore_3".to_string(),
            OP_FSTORE_0 => "fstore_0".to_string(),
            OP_FSTORE_1 => "fstore_1".to_string(),
            OP_FSTORE_2 => "fstore_2".to_string(),
            OP_FSTORE_3 => "fstore_3".to_string(),
            OP_DSTORE_0 => "dstore_0".to_string(),
            OP_DSTORE_1 => "dstore_1".to_string(),
            OP_DSTORE_2 => "dstore_2".to_string(),
            OP_DSTORE_3 => "dstore_3".to_string(),
            OP_ASTORE_0 => "astore_0".to_string(),
            OP_ASTORE_1 => "astore_1".to_string(),
            OP_ASTORE_2 => "astore_2".to_string(),
            OP_ASTORE_3 => "astore_3".to_string(),
            OP_IASTORE => "iastore".to_string(),
            OP_LASTORE => "lastore".to_string(),
            OP_FASTORE => "fastore".to_string(),
            OP_DASTORE => "dastore".to_string(),
            OP_AASTORE => "aastore".to_string(),
            OP_BASTORE => "bastore".to_string(),
            OP_CASTORE => "castore".to_string(),
            OP_SASTORE => "sastore".to_string(),
            OP_POP => "pop".to_string(),
            OP_POP2 => "pop2".to_string(),
            OP_DUP => "dup".to_string(),
            OP_DUP_X1 => "dup_x1".to_string(),
            OP_DUP_X2 => "dup_x2".to_string(),
            OP_DUP2 => "dup2".to_string(),
            OP_DUP2_X1 => "dup2_x1".to_string(),
            OP_DUP2_X2 => "dup2_x2".to_string(),
            OP_SWAP => "swap".to_string(),
            OP_IADD => "iadd".to_string(),
            OP_LADD => "ladd".to_string(),
            OP_FADD => "fadd".to_string(),
            OP_DADD => "dadd".to_string(),
            OP_ISUB => "isub".to_string(),
            OP_LSUB => "lsub".to_string(),
            OP_FSUB => "fsub".to_string(),
            OP_DSUB => "dsub".to_string(),
            OP_IMUL => "imul".to_string(),
            OP_LMUL => "lmul".to_string(),
            OP_FMUL => "fmul".to_string(),
            OP_DMUL => "dmul".to_string(),
            OP_IDIV => "idiv".to_string(),
            OP_LDIV => "ldiv".to_string(),
            OP_FDIV => "fdiv".to_string(),
            OP_DDIV => "ddiv".to_string(),
            OP_IREM => "irem".to_string(),
            OP_LREM => "lrem".to_string(),
            OP_FREM => "frem".to_string(),
            OP_DREM => "drem".to_string(),
            OP_INEG => "ineg".to_string(),
            OP_LNEG => "lneg".to_string(),
            OP_FNEG => "fneg".to_string(),
            OP_DNEG => "dneg".to_string(),
            OP_ISHL => "ishl".to_string(),
            OP_LSHL => "lshl".to_string(),
            OP_ISHR => "ishr".to_string(),
            OP_LSHR => "lshr".to_string(),
            OP_IUSHR => "iushr".to_string(),
            OP_LUSHR => "lushr".to_string(),
            OP_IAND => "iand".to_string(),
            OP_LAND => "land".to_string(),
            OP_IOR => "ior".to_string(),
            OP_LOR => "lor".to_string(),
            OP_IXOR => "ixor".to_string(),
            OP_LXOR => "lxor".to_string(),
            OP_IINC => format!("iinc {}, {}", instruction.value, instruction.value2),
            OP_I2L => "i2l".to_string(),
            OP_I2F => "i2f".to_string(),
            OP_I2D => "i2d".to_string(),
            OP_L2I => "l2i".to_string(),
            OP_L2F => "l2f".to_string(),
            OP_L2D => "l2d".to_string(),
            OP_F2I => "f2i".to_string(),
            OP_F2L => "f2l".to_string(),
            OP_F2D => "f2d".to_string(),
            OP_D2I => "d2i".to_string(),
            OP_D2L => "d2l".to_string(),
            OP_D2F => "d2f".to_string(),
            OP_I2B => "i2b".to_string(),
            OP_I2C => "i2c".to_string(),
            OP_I2S => "i2s".to_string(),
            OP_LCMP => "lcmp".to_string(),
            OP_FCMPL => "fcmpl".to_string(),
            OP_FCMPG => "fcmpg".to_string(),
            OP_DCMPL => "dcmpl".to_string(),
            OP_DCMPG => "dcmpg".to_string(),
            OP_IFEQ => format!("ifeq {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IFNE => format!("ifne {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IFLT => format!("iflt {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IFGE => format!("ifge {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IFGT => format!("ifgt {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IFLE => format!("ifle {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ICMPEQ => format!("if_icmpeq {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ICMPNE => format!("if_icmpne {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ICMPLT => format!("if_icmplt {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ICMPGE => format!("if_icmpge {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ICMPGT => format!("if_icmpgt {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ICMPLE => format!("if_icmple {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ACMPEQ => format!("if_acmpeq {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IF_ACMPNE => format!("if_acmpne {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_GOTO => format!("goto {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_JSR => format!("jsr {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_RET => format!("ret {}", instruction.value),
            OP_IRETURN => "ireturn".to_string(),
            OP_LRETURN => "lreturn".to_string(),
            OP_FRETURN => "freturn".to_string(),
            OP_DRETURN => "dreturn".to_string(),
            OP_ARETURN => "areturn".to_string(),
            OP_RETURN => "return".to_string(),
            OP_GETSTATIC => {
                if let Some(field_info) = self.get_field_ref_info(instruction.value as u16) {
                    format!("getstatic #{} // {}", instruction.value, field_info)
                } else {
                    format!("getstatic #{}", instruction.value)
                }
            },
            OP_PUTSTATIC => {
                if let Some(field_info) = self.get_field_ref_info(instruction.value as u16) {
                    format!("putstatic #{} // {}", instruction.value, field_info)
                } else {
                    format!("putstatic #{}", instruction.value)
                }
            },
            OP_GETFIELD => {
                if let Some(field_info) = self.get_field_ref_info(instruction.value as u16) {
                    format!("getfield #{} // {}", instruction.value, field_info)
                } else {
                    format!("getfield #{}", instruction.value)
                }
            },
            OP_PUTFIELD => {
                if let Some(field_info) = self.get_field_ref_info(instruction.value as u16) {
                    format!("putfield #{} // {}", instruction.value, field_info)
                } else {
                    format!("putfield #{}", instruction.value)
                }
            },
            OP_INVOKEVIRTUAL => {
                if let Some(method_info) = self.get_method_ref_info(instruction.value as u16) {
                    format!("invokevirtual #{} // {}", instruction.value, method_info)
                } else {
                    format!("invokevirtual #{}", instruction.value)
                }
            },
            OP_INVOKESPECIAL => {
                if let Some(method_info) = self.get_method_ref_info(instruction.value as u16) {
                    format!("invokespecial #{} // {}", instruction.value, method_info)
                } else {
                    format!("invokespecial #{}", instruction.value)
                }
            },
            OP_INVOKESTATIC => {
                if let Some(method_info) = self.get_method_ref_info(instruction.value as u16) {
                    format!("invokestatic #{} // {}", instruction.value, method_info)
                } else {
                    format!("invokestatic #{}", instruction.value)
                }
            },
            OP_INVOKEINTERFACE => {
                if let Some(method_info) = self.get_interface_method_ref_info(instruction.value as u16) {
                    format!("invokeinterface #{}, {} // {}", instruction.value, instruction.value2, method_info)
                } else {
                    format!("invokeinterface #{}, {}", instruction.value, instruction.value2)
                }
            },
            OP_INVOKEDYNAMIC => {
                format!("invokedynamic #{}", instruction.value)
            },
            OP_NEW => {
                if let Some(class_name) = self.get_class_name(instruction.value as u16) {
                    format!("new #{} // {}", instruction.value, class_name)
                } else {
                    format!("new #{}", instruction.value)
                }
            },
            OP_NEWARRAY => {
                let type_name = match instruction.value {
                    4 => "boolean",
                    5 => "char", 
                    6 => "float",
                    7 => "double",
                    8 => "byte",
                    9 => "short",
                    10 => "int",
                    11 => "long",
                    _ => "unknown",
                };
                format!("newarray {}", type_name)
            },
            OP_ANEWARRAY => {
                if let Some(class_name) = self.get_class_name(instruction.value as u16) {
                    format!("anewarray #{} // {}", instruction.value, class_name)
                } else {
                    format!("anewarray #{}", instruction.value)
                }
            },
            OP_ARRAYLENGTH => "arraylength".to_string(),
            OP_ATHROW => "athrow".to_string(),
            OP_CHECKCAST => {
                if let Some(class_name) = self.get_class_name(instruction.value as u16) {
                    format!("checkcast #{} // {}", instruction.value, class_name)
                } else {
                    format!("checkcast #{}", instruction.value)
                }
            },
            OP_INSTANCEOF => {
                if let Some(class_name) = self.get_class_name(instruction.value as u16) {
                    format!("instanceof #{} // {}", instruction.value, class_name)
                } else {
                    format!("instanceof #{}", instruction.value)
                }
            },
            OP_MONITORENTER => "monitorenter".to_string(),
            OP_MONITOREXIT => "monitorexit".to_string(),
            OP_IFNULL => format!("ifnull {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_IFNONNULL => format!("ifnonnull {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_GOTO_W => format!("goto_w {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_JSR_W => format!("jsr_w {} // +{}", instruction.offset as i32 + instruction.value, instruction.value),
            OP_MULTIANEWARRAY => {
                if let Some(class_name) = self.get_class_name(instruction.value as u16) {
                    format!("multianewarray #{}, {} // {}", instruction.value, instruction.value2, class_name)
                } else {
                    format!("multianewarray #{}, {}", instruction.value, instruction.value2)
                }
            },
            OP_TABLESWITCH => {
                let mut result = format!("tableswitch {{\n");
                result.push_str(&format!("         default: {} // +{}\n", instruction.offset as i32 + instruction.value, instruction.value));
                
                for (i, (_, target)) in instruction.pairs.iter().enumerate() {
                    let case_value = instruction.value2 + i as i32;
                    let target_addr = instruction.offset as i32 + (*target as i32);
                    result.push_str(&format!("         {}: {} // +{}\n", case_value, target_addr, *target as i32));
                }
                result.push_str("     }");
                result
            },
            OP_LOOKUPSWITCH => {
                let mut result = format!("lookupswitch {{\n");
                result.push_str(&format!("         default: {} // +{}\n", instruction.offset as i32 + instruction.value, instruction.value));
                
                for (key, target) in &instruction.pairs {
                    let target_addr = instruction.offset as i32 + (*target as i32);
                    result.push_str(&format!("         {}: {} // +{}\n", *key as i32, target_addr, *target as i32));
                }
                result.push_str("     }");
                result
            },
            OP_WIDE => {
                // Wide instruction modifies the next instruction
                // The value field contains the modified opcode and value2 contains the wide index
                match instruction.value as u8 {
                    OP_ILOAD => format!("wide iload {}", instruction.value2),
                    OP_LLOAD => format!("wide lload {}", instruction.value2),
                    OP_FLOAD => format!("wide fload {}", instruction.value2),
                    OP_DLOAD => format!("wide dload {}", instruction.value2),
                    OP_ALOAD => format!("wide aload {}", instruction.value2),
                    OP_ISTORE => format!("wide istore {}", instruction.value2),
                    OP_LSTORE => format!("wide lstore {}", instruction.value2),
                    OP_FSTORE => format!("wide fstore {}", instruction.value2),
                    OP_DSTORE => format!("wide dstore {}", instruction.value2),
                    OP_ASTORE => format!("wide astore {}", instruction.value2),
                    OP_IINC => {
                        // For wide iinc, value2 is the index and pairs[0] contains the increment value
                        let increment = instruction.pairs.get(0).map(|(_, v)| *v as i16).unwrap_or(0);
                        format!("wide iinc {}, {}", instruction.value2, increment)
                    },
                    OP_RET => format!("wide ret {}", instruction.value2),
                    _ => format!("wide unknown_opcode_0x{:02X} {}", instruction.value as u8, instruction.value2),
                }
            },
            OP_BREAKPOINT => "breakpoint".to_string(),
            OP_IMPDEP1 => "impdep1".to_string(),
            OP_IMPDEP2 => "impdep2".to_string(),
            _ => format!("unknown_opcode_0x{:02X}", instruction.opcode)
        }
    }

    fn format_access_flags(flags: u16) -> String {
        let mut parts = Vec::new();
        if flags & 0x0001 != 0 { parts.push("public"); }
        if flags & 0x0002 != 0 { parts.push("private"); }
        if flags & 0x0004 != 0 { parts.push("protected"); }
        if flags & 0x0008 != 0 { parts.push("static"); }
        if flags & 0x0010 != 0 { parts.push("final"); }
        if flags & 0x0020 != 0 { parts.push("super"); }
        if flags & 0x0040 != 0 { parts.push("volatile"); }
        if flags & 0x0080 != 0 { parts.push("transient"); }
        if flags & 0x0100 != 0 { parts.push("native"); }
        if flags & 0x0200 != 0 { parts.push("interface"); }
        if flags & 0x0400 != 0 { parts.push("abstract"); }
        if flags & 0x0800 != 0 { parts.push("strict"); }
        if flags & 0x1000 != 0 { parts.push("synthetic"); }
        if flags & 0x2000 != 0 { parts.push("annotation"); }
        if flags & 0x4000 != 0 { parts.push("enum"); }
        parts.join(" ")
    }

    fn get_class_name(&self, index: u16) -> Option<String> {
        if let Some(ConstantPoolEntry::ClassRef(name_index)) = 
            self.class_file.constant_pool.constant_pool.get((index - 1) as usize) {
            self.class_file.constant_pool.get_utf8(*name_index as usize).map(|s| s.clone())
        } else {
            None
        }
    }

    fn get_field_ref_info(&self, index: u16) -> Option<String> {
        if let Some(ConstantPoolEntry::FieldRef(class_index, name_and_type_index)) = 
            self.class_file.constant_pool.constant_pool.get((index - 1) as usize) {
            let class_name = self.get_class_name(*class_index)?;
            let (name, descriptor) = self.get_name_and_type_info(*name_and_type_index)?;
            Some(format!("{}.{}:{}", class_name, name, descriptor))
        } else {
            None
        }
    }

    fn get_method_ref_info(&self, index: u16) -> Option<String> {
        if let Some(ConstantPoolEntry::MethodRef(class_index, name_and_type_index)) = 
            self.class_file.constant_pool.constant_pool.get((index - 1) as usize) {
            let class_name = self.get_class_name(*class_index)?;
            let (name, descriptor) = self.get_name_and_type_info(*name_and_type_index)?;
            Some(format!("{}.{}:{}", class_name, name, descriptor))
        } else {
            None
        }
    }

    fn get_interface_method_ref_info(&self, index: u16) -> Option<String> {
        if let Some(ConstantPoolEntry::InterfaceMethodRef(class_index, name_and_type_index)) = 
            self.class_file.constant_pool.constant_pool.get((index - 1) as usize) {
            let class_name = self.get_class_name(*class_index)?;
            let (name, descriptor) = self.get_name_and_type_info(*name_and_type_index)?;
            Some(format!("{}.{}:{}", class_name, name, descriptor))
        } else {
            None
        }
    }

    fn get_name_and_type_info(&self, index: u16) -> Option<(String, String)> {
        if let Some(ConstantPoolEntry::NameAndTypeRef(name_index, descriptor_index)) = 
            self.class_file.constant_pool.constant_pool.get((index - 1) as usize) {
            let name = self.class_file.constant_pool.get_utf8(*name_index as usize)?.clone();
            let descriptor = self.class_file.constant_pool.get_utf8(*descriptor_index as usize)?.clone();
            Some((name, descriptor))
        } else {
            None
        }
    }

    fn get_constant_info(&self, index: u16) -> Option<String> {
        if let Some(entry) = self.class_file.constant_pool.constant_pool.get((index - 1) as usize) {
            match entry {
                ConstantPoolEntry::Integer(i) => Some(format!("int {}", i)),
                ConstantPoolEntry::Float(f) => Some(format!("float {}", f)),
                ConstantPoolEntry::Long(l) => Some(format!("long {}", l)),
                ConstantPoolEntry::Double(d) => Some(format!("double {}", d)),
                ConstantPoolEntry::StringRef(string_index) => {
                    if let Some(string) = self.class_file.constant_pool.get_utf8(*string_index as usize) {
                        Some(format!("String \"{}\"", string))
                    } else {
                        Some(format!("String #{}",  string_index))
                    }
                },
                ConstantPoolEntry::ClassRef(name_index) => {
                    if let Some(class_name) = self.class_file.constant_pool.get_utf8(*name_index as usize) {
                        Some(format!("Class {}", class_name))
                    } else {
                        Some(format!("Class #{}", name_index))
                    }
                },
                ConstantPoolEntry::MethodHandleRef(kind, ref_index) => {
                    Some(format!("MethodHandle {}:#{}", kind, ref_index))
                },
                ConstantPoolEntry::MethodTypeRef(descriptor_index) => {
                    if let Some(descriptor) = self.class_file.constant_pool.get_utf8(*descriptor_index as usize) {
                        Some(format!("MethodType {}", descriptor))
                    } else {
                        Some(format!("MethodType #{}", descriptor_index))
                    }
                },
                ConstantPoolEntry::InvokeDynamicRef(bootstrap_method_attr_index, name_and_type_index) => {
                    Some(format!("InvokeDynamic #{}:#{}", bootstrap_method_attr_index, name_and_type_index))
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

pub fn disassemble_classfile(class_data: Vec<u8>) -> Result<String, String> {
    let disassembler = ClassFileDisassembler::new(class_data)?;
    disassembler.disassemble()
}