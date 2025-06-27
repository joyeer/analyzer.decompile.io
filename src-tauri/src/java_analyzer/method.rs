use crate::java_analyzer::{classfile::ClassFile, error::JavaAnalyzeError, opcode::*};
use crate::java_analyzer::io::Buffer;
use crate::java_analyzer::attributes::{read_raw_attribute, Attribute};
use crate::java_analyzer::error::Result;

pub struct JvmMethod {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
    pub code: Vec<Instruction>,
}

pub fn read_jvm_method(buffer:&mut Buffer, classfile:&ClassFile) -> Result<JvmMethod> {
    let access_flags = buffer.read_u16()?;
    let name_index = buffer.read_u16()?;
    let descriptor_index = buffer.read_u16()?;
    let attributes_count = buffer.read_u16()?;
    let mut attributes = vec![];
    let name =classfile.constant_pool.get_utf8(name_index as usize).unwrap();
    let descriptor = classfile.constant_pool.get_utf8(descriptor_index as usize).unwrap();

    let mut code: Vec<Instruction> = vec![];
    for _ in 0..attributes_count {
        let raw_atrribute = read_raw_attribute(buffer, &classfile)?;

        match raw_atrribute {
            Attribute::Code(ref code_attr) => {
                let mut code_reader: JvmCodeReader<'_> = JvmCodeReader::new(&code_attr.code);
                let opcodes = code_reader.read()?;
                code = opcodes;
            }
            _ => {}
        }
        attributes.push(raw_atrribute);
    }

    Ok(JvmMethod {
        access_flags,
        name: name.to_string(), 
        descriptor: descriptor.to_string(),
        code,
        attributes,
    })
}


struct JvmCodeReader<'a> {
    code: &'a Vec<u8>,
    position: u32,
}

impl<'a> JvmCodeReader<'a> {
    pub fn new(code: &'a Vec<u8>) -> Self {
        JvmCodeReader {
            code,
            position: 0,
        }
    }

    pub fn read(&mut self) -> Result<Vec<Instruction>> {
        let mut opcodes = vec![];
        while self.position < self.code.len() as u32 {
            let opcode = self.decode()?;
            opcodes.push(opcode);
        }
        Ok(opcodes)
    }

    fn decode(&mut self) -> Result<Instruction> {
        let opcode = self.read_u8()?;
        let offset = self.position - 1;
        match opcode {
            OP_AALOAD => Ok(Instruction::new(opcode, offset)),
            OP_AASTORE => Ok(Instruction::new(opcode, offset)),
            OP_ACONST_NULL => Ok(Instruction::new(opcode, offset)),
            OP_ALOAD => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_ALOAD_0 => Ok(Instruction::new(opcode, offset)),
            OP_ALOAD_1 => Ok(Instruction::new(opcode, offset)),
            OP_ALOAD_2 => Ok(Instruction::new(opcode, offset)),
            OP_ALOAD_3 => Ok(Instruction::new(opcode, offset)),
            OP_ANEWARRAY => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_ATHROW => Ok(Instruction::new(opcode, offset)),
            OP_ARRAYLENGTH => Ok(Instruction::new(opcode, offset)),
            OP_ARETURN => Ok(Instruction::new(opcode, offset)),
            OP_ASTORE => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_ASTORE_0 => Ok(Instruction::new(opcode, offset)),
            OP_ASTORE_1 => Ok(Instruction::new(opcode, offset)),
            OP_ASTORE_2 => Ok(Instruction::new(opcode, offset)),
            OP_ASTORE_3 => Ok(Instruction::new(opcode, offset)),
            OP_BALOAD => Ok(Instruction::new(opcode, offset)),
            OP_BASTORE => Ok(Instruction::new(opcode, offset)),
            OP_BIPUSH => Ok(Instruction::new2(opcode, offset, self.read_i8()? as i32)),
            OP_BREAKPOINT => Ok(Instruction::new(opcode, offset)),
            OP_CALOAD => Ok(Instruction::new(opcode, offset)),
            OP_CASTORE => Ok(Instruction::new(opcode, offset)),
            OP_CHECKCAST => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_D2F => Ok(Instruction::new(opcode, offset)),
            OP_D2I => Ok(Instruction::new(opcode, offset)),
            OP_D2L => Ok(Instruction::new(opcode, offset)),
            OP_DADD => Ok(Instruction::new(opcode, offset)),
            OP_DALOAD => Ok(Instruction::new(opcode, offset)),
            OP_DASTORE => Ok(Instruction::new(opcode, offset)),
            OP_DCMPG => Ok(Instruction::new(opcode, offset)),
            OP_DCMPL => Ok(Instruction::new(opcode, offset)),
            OP_DCONST_0 => Ok(Instruction::new(opcode, offset)),
            OP_DCONST_1 => Ok(Instruction::new(opcode, offset)),
            OP_DLOAD => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_DLOAD_0 => Ok(Instruction::new(opcode, offset)),
            OP_DLOAD_1 => Ok(Instruction::new(opcode, offset)),
            OP_DLOAD_2 => Ok(Instruction::new(opcode, offset)),
            OP_DLOAD_3 => Ok(Instruction::new(opcode, offset)),
            OP_DMUL => Ok(Instruction::new(opcode, offset)),
            OP_DNEG => Ok(Instruction::new(opcode, offset)),
            OP_DREM => Ok(Instruction::new(opcode, offset)),
            OP_DRETURN => Ok(Instruction::new(opcode, offset)),
            OP_DSTORE => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_DSTORE_0 => Ok(Instruction::new(opcode, offset)),
            OP_DSTORE_1 => Ok(Instruction::new(opcode, offset)),
            OP_DSTORE_2 => Ok(Instruction::new(opcode, offset)),
            OP_DSTORE_3 => Ok(Instruction::new(opcode, offset)),
            OP_DSUB => Ok(Instruction::new(opcode, offset)),
            OP_DUP => Ok(Instruction::new(opcode, offset)),
            OP_DUP_X1 => Ok(Instruction::new(opcode, offset)),
            OP_DUP_X2 => Ok(Instruction::new(opcode, offset)),
            OP_DUP2 => Ok(Instruction::new(opcode, offset)),
            OP_DUP2_X1 => Ok(Instruction::new(opcode, offset)),
            OP_DUP2_X2 => Ok(Instruction::new(opcode, offset)),
            OP_F2D => Ok(Instruction::new(opcode, offset)),
            OP_F2I => Ok(Instruction::new(opcode, offset)),
            OP_F2L => Ok(Instruction::new(opcode, offset)),
            OP_FADD => Ok(Instruction::new(opcode, offset)),
            OP_FALOAD => Ok(Instruction::new(opcode, offset)),
            OP_FASTORE => Ok(Instruction::new(opcode, offset)),
            OP_FCMPL => Ok(Instruction::new(opcode, offset)),
            OP_FCMPG => Ok(Instruction::new(opcode, offset)),
            OP_FCONST_0 => Ok(Instruction::new(opcode, offset)),
            OP_FCONST_1 => Ok(Instruction::new(opcode, offset)),
            OP_FCONST_2 => Ok(Instruction::new(opcode, offset)),
            OP_FLOAD => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_FLOAD_0 => Ok(Instruction::new(opcode, offset)),
            OP_FLOAD_1 => Ok(Instruction::new(opcode, offset)),
            OP_FLOAD_2 => Ok(Instruction::new(opcode, offset)),
            OP_FLOAD_3 => Ok(Instruction::new(opcode, offset)),
            OP_FMUL => Ok(Instruction::new(opcode, offset)),
            OP_FNEG => Ok(Instruction::new(opcode, offset)),
            OP_FREM => Ok(Instruction::new(opcode, offset)),
            OP_FRETURN => Ok(Instruction::new(opcode, offset)),
            OP_FSTORE => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_FSTORE_0 => Ok(Instruction::new(opcode, offset)),
            OP_FSTORE_1 => Ok(Instruction::new(opcode, offset)),
            OP_FSTORE_2 => Ok(Instruction::new(opcode, offset)),
            OP_FSTORE_3 => Ok(Instruction::new(opcode, offset)),
            OP_FSUB => Ok(Instruction::new(opcode, offset)),
            OP_GETFIELD => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_GETSTATIC => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_GOTO => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_GOTO_W => Ok(Instruction::new2(opcode, offset, self.read_u32()? as i32)),
            OP_I2B => Ok(Instruction::new(opcode, offset)),
            OP_I2C => Ok(Instruction::new(opcode, offset)),
            OP_I2D => Ok(Instruction::new(opcode, offset)),
            OP_I2F => Ok(Instruction::new(opcode, offset)),
            OP_I2L => Ok(Instruction::new(opcode, offset)),
            OP_I2S => Ok(Instruction::new(opcode, offset)),
            OP_IADD => Ok(Instruction::new(opcode, offset)),
            OP_IALOAD => Ok(Instruction::new(opcode, offset)),
            OP_IAND => Ok(Instruction::new(opcode, offset)),
            OP_IASTORE => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_M1 => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_0 => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_1 => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_2 => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_3 => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_4 => Ok(Instruction::new(opcode, offset)),
            OP_ICONST_5 => Ok(Instruction::new(opcode, offset)),
            OP_IDIV => Ok(Instruction::new(opcode, offset)),
            OP_LDIV => Ok(Instruction::new(opcode, offset)),
            OP_FDIV => Ok(Instruction::new(opcode, offset)),
            OP_DDIV => Ok(Instruction::new(opcode, offset)),
            OP_IF_ACMPEQ => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ACMPNE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ICMPEQ => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ICMPNE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ICMPLT => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ICMPGE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ICMPGT => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IF_ICMPLE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFEQ => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFNE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFLT => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFGE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFGT => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFLE => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFNULL => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IFNONNULL => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IINC => {
                let index = self.read_u8()?;
                let increment = self.read_u8()?;
                Ok(Instruction::new3(opcode, offset, index as i32, increment as i32))
            }
            OP_ILOAD => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_ILOAD_0 => Ok(Instruction::new(opcode, offset)),
            OP_ILOAD_1 => Ok(Instruction::new(opcode, offset)),
            OP_ILOAD_2 => Ok(Instruction::new(opcode, offset)),
            OP_ILOAD_3 => Ok(Instruction::new(opcode, offset)),
            OP_IMUL => Ok(Instruction::new(opcode, offset)),
            OP_INEG => Ok(Instruction::new(opcode, offset)),
            OP_INSTANCEOF => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_INVOKEDYNAMIC => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_INVOKEINTERFACE => Ok(Instruction::new3(opcode, offset, self.read_u16()? as i32, self.read_u8()? as i32)),
            OP_INVOKESPECIAL => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_INVOKESTATIC => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_INVOKEVIRTUAL => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_IOR => Ok(Instruction::new(opcode, offset)),
            OP_IREM => Ok(Instruction::new(opcode, offset)),
            OP_IRETURN => Ok(Instruction::new(opcode, offset)),
            OP_ISHL => Ok(Instruction::new(opcode, offset)),
            OP_ISHR => Ok(Instruction::new(opcode, offset)),
            OP_ISTORE => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_ISTORE_0 => Ok(Instruction::new(opcode, offset)),
            OP_ISTORE_1 => Ok(Instruction::new(opcode, offset)),
            OP_ISTORE_2 => Ok(Instruction::new(opcode, offset)),
            OP_ISTORE_3 => Ok(Instruction::new(opcode, offset)),
            OP_ISUB => Ok(Instruction::new(opcode, offset)),
            OP_IUSHR => Ok(Instruction::new(opcode, offset)),
            OP_IXOR => Ok(Instruction::new(opcode, offset)),
            OP_JSR => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_JSR_W => Ok(Instruction::new2(opcode, offset, self.read_u32()? as i32)),
            OP_L2D => Ok(Instruction::new(opcode, offset)),
            OP_L2F => Ok(Instruction::new(opcode, offset)),
            OP_L2I => Ok(Instruction::new(opcode, offset)),
            OP_LADD => Ok(Instruction::new(opcode, offset)),
            OP_LALOAD => Ok(Instruction::new(opcode, offset)),
            OP_LAND => Ok(Instruction::new(opcode, offset)),
            OP_LASTORE => Ok(Instruction::new(opcode, offset)),
            OP_LCMP => Ok(Instruction::new(opcode, offset)),
            OP_LCONST_0 => Ok(Instruction::new(opcode, offset)),
            OP_LCONST_1 => Ok(Instruction::new(opcode, offset)),
            OP_LDC => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_LDC_W => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_LDC2_W => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_LLOAD => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_LLOAD_0 => Ok(Instruction::new(opcode, offset)),
            OP_LLOAD_1 => Ok(Instruction::new(opcode, offset)),
            OP_LLOAD_2 => Ok(Instruction::new(opcode, offset)),
            OP_LLOAD_3 => Ok(Instruction::new(opcode, offset)),
            OP_LMUL => Ok(Instruction::new(opcode, offset)),
            OP_LNEG => Ok(Instruction::new(opcode, offset)),
            OP_LOR => Ok(Instruction::new(opcode, offset)),
            OP_LREM => Ok(Instruction::new(opcode, offset)),
            OP_LRETURN => Ok(Instruction::new(opcode, offset)),
            OP_LSHL => Ok(Instruction::new(opcode, offset)),
            OP_LSHR => Ok(Instruction::new(opcode, offset)),
            OP_LSTORE => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_LSTORE_0 => Ok(Instruction::new(opcode, offset)),
            OP_LSTORE_1 => Ok(Instruction::new(opcode, offset)),
            OP_LSTORE_2 => Ok(Instruction::new(opcode, offset)),
            OP_LSTORE_3 => Ok(Instruction::new(opcode, offset)),
            OP_LSUB => Ok(Instruction::new(opcode, offset)),
            OP_LUSHR => Ok(Instruction::new(opcode, offset)),
            OP_LXOR => Ok(Instruction::new(opcode, offset)),
            OP_MONITORENTER => Ok(Instruction::new(opcode, offset)),
            OP_MONITOREXIT => Ok(Instruction::new(opcode, offset)),
            OP_MULTIANEWARRAY => Ok(Instruction::new3(opcode, offset, self.read_u16()? as i32, self.read_u8()? as i32)),
            OP_NEW => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_NEWARRAY => {
                let array_type = self.read_u8()?;
                let t = match array_type {
                    4 => ArrayType::Boolean, // T_BOOLEAN
                    5 => ArrayType::Char,    // T_CHAR
                    6 => ArrayType::Float,   // T_FLOAT
                    7 => ArrayType::Double,  // T_DOUBLE
                    8 => ArrayType::Byte,    // T_BYTE
                    9 => ArrayType::Short,   // T_SHORT
                    10 => ArrayType::Int,    // T_INT
                    11 => ArrayType::Long,   // T_LONG
                    _ => {
                        return Err(JavaAnalyzeError::InvalidClassData(format!("Invalid array type: {array_type}")));
                    }
                };
                Ok(Instruction::new2(opcode, offset, t as i32))
            }
            OP_NOP => Ok(Instruction::new(opcode, offset)),
            OP_POP => Ok(Instruction::new(opcode, offset)),
            OP_POP2 => Ok(Instruction::new(opcode, offset)),
            OP_PUTFIELD => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_PUTSTATIC => Ok(Instruction::new2(opcode, offset, self.read_u16()? as i32)),
            OP_RET => Ok(Instruction::new2(opcode, offset, self.read_u8()? as i32)),
            OP_RETURN => Ok(Instruction::new(opcode, offset)),
            OP_SALOAD => Ok(Instruction::new(opcode, offset)),
            OP_SASTORE => Ok(Instruction::new(opcode, offset)),
            OP_SIPUSH => Ok(Instruction::new2(opcode, offset, self.read_i16()? as i32)),
            OP_SWAP => Ok(Instruction::new(opcode, offset)),
            _ => Err(JavaAnalyzeError::InvalidClassData(format!("Invalid opcode: {opcode}")))
        }
    }

    fn read_u32(&mut self) -> Result<u32> {
        let byte1 = self.read_u8()?;
        let byte2 = self.read_u8()?;
        let byte3 = self.read_u8()?;
        let byte4 = self.read_u8()?;
        Ok(u32::from_be_bytes([byte1, byte2, byte3, byte4]))
    }

    fn read_u16(&mut self, ) -> Result<u16> {
        let byte1 = self.read_u8()?;
        let byte2 = self.read_u8()?;
        Ok(u16::from_be_bytes([byte1, byte2]))
    }

    fn read_i16(&mut self, ) -> Result<i16> {
        let value = self.read_u16()?;
        Ok(unsafe { std::mem::transmute::<u16, i16>(value) })
    }
    
    fn read_i8(&mut self, ) -> Result<i8> {
        let value = self.read_u8()?;
        Ok(unsafe { std::mem::transmute::<u8, i8>(value) })
    }
    
    fn read_u8(&mut self) -> Result<u8> {
        let byte = self.code.get(self.position as usize)
            .ok_or(JavaAnalyzeError::InvalidClassData(format!("Invalid bytecode data postion: {0}", self.position)))?;
        self.position += 1;
        Ok(*byte)
    }
}
