use crate::java_analyzer::opcode::*;

pub struct ClassFileDisassembler {
    data: Vec<u8>,
    offset: usize,
}

impl ClassFileDisassembler {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, offset: 0 }
    }

    pub fn disassemble(&mut self) -> Result<String, String> {
        let mut output = String::new();
        
        // 解析 classfile 头部
        let magic = self.read_u32()?;
        if magic != 0xCAFEBABE {
            return Err("Invalid class file magic number".to_string());
        }
        
        let minor_version = self.read_u16()?;
        let major_version = self.read_u16()?;
        
        output.push_str(&format!("// Class file version: {}.{}\n", major_version, minor_version));
        output.push_str(&format!("// Magic: 0x{:08X}\n\n", magic));
        
        // 解析常量池
        let constant_pool_count = self.read_u16()?;
        output.push_str(&format!("Constant pool ({}): \n", constant_pool_count - 1));
        
        let mut constant_pool = Vec::new();
        for i in 1..constant_pool_count {
            let tag = self.read_u8()?;
            let cp_info = self.parse_constant_pool_entry(tag)?;
            output.push_str(&format!("  #{}: {}\n", i, cp_info));
            constant_pool.push(cp_info);
        }
        
        // 解析访问标志
        let access_flags = self.read_u16()?;
        output.push_str(&format!("\nAccess flags: 0x{:04X} [", access_flags));
        output.push_str(&Self::format_access_flags(access_flags));
        output.push_str("]\n");
        
        // 解析类、父类、接口
        let this_class = self.read_u16()?;
        let super_class = self.read_u16()?;
        let interfaces_count = self.read_u16()?;
        
        output.push_str(&format!("This class: #{}\n", this_class));
        output.push_str(&format!("Super class: #{}\n", super_class));
        
        // 跳过接口
        for _ in 0..interfaces_count {
            self.read_u16()?;
        }
        
        // 解析字段
        let fields_count = self.read_u16()?;
        output.push_str(&format!("\nFields ({}): \n", fields_count));
        for i in 0..fields_count {
            let field_info = self.parse_field_or_method()?;
            output.push_str(&format!("  Field #{}: {}\n", i, field_info));
        }
        
        // 解析方法
        let methods_count = self.read_u16()?;
        output.push_str(&format!("\nMethods ({}): \n", methods_count));
        for i in 0..methods_count {
            let method_info = self.parse_method_with_code()?;
            output.push_str(&format!("  Method #{}: {}\n", i, method_info));
        }
        
        Ok(output)
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        if self.offset >= self.data.len() {
            return Err("Unexpected end of file".to_string());
        }
        let value = self.data[self.offset];
        self.offset += 1;
        Ok(value)
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        let high = self.read_u8()? as u16;
        let low = self.read_u8()? as u16;
        Ok((high << 8) | low)
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let high = self.read_u16()? as u32;
        let low = self.read_u16()? as u32;
        Ok((high << 16) | low)
    }

    fn parse_constant_pool_entry(&mut self, tag: u8) -> Result<String, String> {
        match tag {
            1 => { // UTF8
                let length = self.read_u16()?;
                let mut bytes = vec![0u8; length as usize];
                for i in 0..length {
                    bytes[i as usize] = self.read_u8()?;
                }
                let string = String::from_utf8_lossy(&bytes);
                Ok(format!("Utf8 [{}]", string))
            }
            3 => { // Integer
                let value = self.read_u32()?;
                Ok(format!("Integer [{}]", value as i32))
            }
            7 => { // Class
                let name_index = self.read_u16()?;
                Ok(format!("Class [#{}]", name_index))
            }
            8 => { // String
                let string_index = self.read_u16()?;
                Ok(format!("String [#{}]", string_index))
            }
            9 => { // Fieldref
                let class_index = self.read_u16()?;
                let name_and_type_index = self.read_u16()?;
                Ok(format!("Fieldref [#{}.#{}]", class_index, name_and_type_index))
            }
            10 => { // Methodref
                let class_index = self.read_u16()?;
                let name_and_type_index = self.read_u16()?;
                Ok(format!("Methodref [#{}.#{}]", class_index, name_and_type_index))
            }
            12 => { // NameAndType
                let name_index = self.read_u16()?;
                let descriptor_index = self.read_u16()?;
                Ok(format!("NameAndType [#{}:#{}]", name_index, descriptor_index))
            }
            _ => {
                Err(format!("Unsupported constant pool tag: {}", tag))
            }
        
        }
    }

    fn format_access_flags(flags: u16) -> String {
        let mut parts = Vec::new();
        if flags & 0x0001 != 0 { parts.push("public"); }
        if flags & 0x0010 != 0 { parts.push("final"); }
        if flags & 0x0020 != 0 { parts.push("super"); }
        if flags & 0x0200 != 0 { parts.push("interface"); }
        if flags & 0x0400 != 0 { parts.push("abstract"); }
        if flags & 0x1000 != 0 { parts.push("synthetic"); }
        if flags & 0x2000 != 0 { parts.push("annotation"); }
        if flags & 0x4000 != 0 { parts.push("enum"); }
        parts.join(" ")
    }

    fn parse_field_or_method(&mut self) -> Result<String, String> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        let attributes_count = self.read_u16()?;
        
        // 跳过属性
        for _ in 0..attributes_count {
            let _attribute_name_index = self.read_u16()?;
            let attribute_length = self.read_u32()?;
            for _ in 0..attribute_length {
                self.read_u8()?;
            }
        }
        
        Ok(format!("access_flags: 0x{:04X}, name: #{}, descriptor: #{}", 
                  access_flags, name_index, descriptor_index))
    }

    fn parse_method_with_code(&mut self) -> Result<String, String> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        let attributes_count = self.read_u16()?;
        
        let mut result = format!("access_flags: 0x{:04X}, name: #{}, descriptor: #{}\n", 
                                access_flags, name_index, descriptor_index);
        
        // 解析属性（重点是 Code 属性）
        for _ in 0..attributes_count {
            let attribute_name_index = self.read_u16()?;
            let attribute_length = self.read_u32()?;
            
            // 如果是 Code 属性，解析字节码
            if self.is_code_attribute(attribute_name_index) {
                result.push_str(&self.parse_code_attribute()?);
            } else {
                // 跳过其他属性
                for _ in 0..attribute_length {
                    self.read_u8()?;
                }
            }
        }
        
        Ok(result)
    }

    fn is_code_attribute(&self, _name_index: u16) -> bool {
        // 简化实现：假设是 Code 属性
        // 实际应该检查常量池中的字符串是否为 "Code"
        true
    }

    fn parse_code_attribute(&mut self) -> Result<String, String> {
        let max_stack = self.read_u16()?;
        let max_locals = self.read_u16()?;
        let code_length = self.read_u32()?;
        
        let mut result = format!("    Code:\n");
        result.push_str(&format!("      stack={}, locals={}, args_size=?\n", max_stack, max_locals));
        
        // 解析字节码指令
        for i in 0..code_length {
            let opcode = self.read_u8()?;
            let instruction = self.disassemble_instruction(opcode, i)?;
            result.push_str(&format!("        {}: {}\n", i, instruction));
        }
        
        // 跳过异常表和其他属性
        let exception_table_length = self.read_u16()?;
        for _ in 0..exception_table_length {
            self.read_u16()?; // start_pc
            self.read_u16()?; // end_pc
            self.read_u16()?; // handler_pc
            self.read_u16()?; // catch_type
        }
        
        let attributes_count = self.read_u16()?;
        for _ in 0..attributes_count {
            let _attribute_name_index = self.read_u16()?;
            let attribute_length = self.read_u32()?;
            for _ in 0..attribute_length {
                self.read_u8()?;
            }
        }
        
        Ok(result)
    }

    fn disassemble_instruction(&mut self, opcode: u8, _pc: u32) -> Result<String, String> {
        match opcode {
            // 常量指令
            OP_NOP => Ok("nop".to_string()),
            OP_ACONST_NULL => Ok("aconst_null".to_string()),
            OP_ICONST_M1 => Ok("iconst_m1".to_string()),
            OP_ICONST_0 => Ok("iconst_0".to_string()),
            OP_ICONST_1 => Ok("iconst_1".to_string()),
            OP_ICONST_2 => Ok("iconst_2".to_string()),
            OP_ICONST_3 => Ok("iconst_3".to_string()),
            OP_ICONST_4 => Ok("iconst_4".to_string()),
            OP_ICONST_5 => Ok("iconst_5".to_string()),
            OP_LCONST_0 => Ok("lconst_0".to_string()),
            OP_LCONST_1 => Ok("lconst_1".to_string()),
            OP_FCONST_0 => Ok("fconst_0".to_string()),
            OP_FCONST_1 => Ok("fconst_1".to_string()),
            OP_FCONST_2 => Ok("fconst_2".to_string()),
            OP_DCONST_0 => Ok("dconst_0".to_string()),
            OP_DCONST_1 => Ok("dconst_1".to_string()),

            // 压栈指令
            OP_BIPUSH => {
                let value = self.read_u8()? as i8;
                Ok(format!("bipush {}", value))
            }
            OP_SIPUSH => {
                let value = self.read_u16()? as i16;
                Ok(format!("sipush {}", value))
            }
            OP_LDC => {
                let index = self.read_u8()?;
                Ok(format!("ldc #{}", index))
            }
            OP_LDC_W => {
                let index = self.read_u16()?;
                Ok(format!("ldc_w #{}", index))
            }
            OP_LDC2_W => {
                let index = self.read_u16()?;
                Ok(format!("ldc2_w #{}", index))
            }

            // 局部变量加载指令
            OP_ILOAD => {
                let index = self.read_u8()?;
                Ok(format!("iload {}", index))
            }
            OP_LLOAD => {
                let index = self.read_u8()?;
                Ok(format!("lload {}", index))
            }
            OP_FLOAD => {
                let index = self.read_u8()?;
                Ok(format!("fload {}", index))
            }
            OP_DLOAD => {
                let index = self.read_u8()?;
                Ok(format!("dload {}", index))
            }
            OP_ALOAD => {
                let index = self.read_u8()?;
                Ok(format!("aload {}", index))
            }
            OP_ILOAD_0 => Ok("iload_0".to_string()),
            OP_ILOAD_1 => Ok("iload_1".to_string()),
            OP_ILOAD_2 => Ok("iload_2".to_string()),
            OP_ILOAD_3 => Ok("iload_3".to_string()),
            OP_LLOAD_0 => Ok("lload_0".to_string()),
            OP_LLOAD_1 => Ok("lload_1".to_string()),
            OP_LLOAD_2 => Ok("lload_2".to_string()),
            OP_LLOAD_3 => Ok("lload_3".to_string()),
            OP_FLOAD_0 => Ok("fload_0".to_string()),
            OP_FLOAD_1 => Ok("fload_1".to_string()),
            OP_FLOAD_2 => Ok("fload_2".to_string()),
            OP_FLOAD_3 => Ok("fload_3".to_string()),
            OP_DLOAD_0 => Ok("dload_0".to_string()),
            OP_DLOAD_1 => Ok("dload_1".to_string()),
            OP_DLOAD_2 => Ok("dload_2".to_string()),
            OP_DLOAD_3 => Ok("dload_3".to_string()),
            OP_ALOAD_0 => Ok("aload_0".to_string()),
            OP_ALOAD_1 => Ok("aload_1".to_string()),
            OP_ALOAD_2 => Ok("aload_2".to_string()),
            OP_ALOAD_3 => Ok("aload_3".to_string()),

            // 数组加载指令
            OP_IALOAD => Ok("iaload".to_string()),
            OP_LALOAD => Ok("laload".to_string()),
            OP_FALOAD => Ok("faload".to_string()),
            OP_DALOAD => Ok("daload".to_string()),
            OP_AALOAD => Ok("aaload".to_string()),
            OP_BALOAD => Ok("baload".to_string()),
            OP_CALOAD => Ok("caload".to_string()),
            OP_SALOAD => Ok("saload".to_string()),

            // 局部变量存储指令
            OP_ISTORE => {
                let index = self.read_u8()?;
                Ok(format!("istore {}", index))
            }
            OP_LSTORE => {
                let index = self.read_u8()?;
                Ok(format!("lstore {}", index))
            }
            OP_FSTORE => {
                let index = self.read_u8()?;
                Ok(format!("fstore {}", index))
            }
            OP_DSTORE => {
                let index = self.read_u8()?;
                Ok(format!("dstore {}", index))
            }
            OP_ASTORE => {
                let index = self.read_u8()?;
                Ok(format!("astore {}", index))
            }
            OP_ISTORE_0 => Ok("istore_0".to_string()),
            OP_ISTORE_1 => Ok("istore_1".to_string()),
            OP_ISTORE_2 => Ok("istore_2".to_string()),
            OP_ISTORE_3 => Ok("istore_3".to_string()),
            OP_LSTORE_0 => Ok("lstore_0".to_string()),
            OP_LSTORE_1 => Ok("lstore_1".to_string()),
            OP_LSTORE_2 => Ok("lstore_2".to_string()),
            OP_LSTORE_3 => Ok("lstore_3".to_string()),
            OP_FSTORE_0 => Ok("fstore_0".to_string()),
            OP_FSTORE_1 => Ok("fstore_1".to_string()),
            OP_FSTORE_2 => Ok("fstore_2".to_string()),
            OP_FSTORE_3 => Ok("fstore_3".to_string()),
            OP_DSTORE_0 => Ok("dstore_0".to_string()),
            OP_DSTORE_1 => Ok("dstore_1".to_string()),
            OP_DSTORE_2 => Ok("dstore_2".to_string()),
            OP_DSTORE_3 => Ok("dstore_3".to_string()),
            OP_ASTORE_0 => Ok("astore_0".to_string()),
            OP_ASTORE_1 => Ok("astore_1".to_string()),
            OP_ASTORE_2 => Ok("astore_2".to_string()),
            OP_ASTORE_3 => Ok("astore_3".to_string()),

            // 数组存储指令
            OP_IASTORE => Ok("iastore".to_string()),
            OP_LASTORE => Ok("lastore".to_string()),
            OP_FASTORE => Ok("fastore".to_string()),
            OP_DASTORE => Ok("dastore".to_string()),
            OP_AASTORE => Ok("aastore".to_string()),
            OP_BASTORE => Ok("bastore".to_string()),
            OP_CASTORE => Ok("castore".to_string()),
            OP_SASTORE => Ok("sastore".to_string()),

            // 栈操作指令
            OP_POP => Ok("pop".to_string()),
            OP_POP2 => Ok("pop2".to_string()),
            OP_DUP => Ok("dup".to_string()),
            OP_DUP_X1 => Ok("dup_x1".to_string()),
            OP_DUP_X2 => Ok("dup_x2".to_string()),
            OP_DUP2 => Ok("dup2".to_string()),
            OP_DUP2_X1 => Ok("dup2_x1".to_string()),
            OP_DUP2_X2 => Ok("dup2_x2".to_string()),
            OP_SWAP => Ok("swap".to_string()),

            // 数学运算指令
            OP_IADD => Ok("iadd".to_string()),
            OP_LADD => Ok("ladd".to_string()),
            OP_FADD => Ok("fadd".to_string()),
            OP_DADD => Ok("dadd".to_string()),
            OP_ISUB => Ok("isub".to_string()),
            OP_LSUB => Ok("lsub".to_string()),
            OP_FSUB => Ok("fsub".to_string()),
            OP_DSUB => Ok("dsub".to_string()),
            OP_IMUL => Ok("imul".to_string()),
            OP_LMUL => Ok("lmul".to_string()),
            OP_FMUL => Ok("fmul".to_string()),
            OP_DMUL => Ok("dmul".to_string()),
            OP_IDIV => Ok("idiv".to_string()),
            OP_LDIV => Ok("ldiv".to_string()),
            OP_FDIV => Ok("fdiv".to_string()),
            OP_DDIV => Ok("ddiv".to_string()),
            OP_IREM => Ok("irem".to_string()),
            OP_LREM => Ok("lrem".to_string()),
            OP_FREM => Ok("frem".to_string()),
            OP_DREM => Ok("drem".to_string()),
            OP_INEG => Ok("ineg".to_string()),
            OP_LNEG => Ok("lneg".to_string()),
            OP_FNEG => Ok("fneg".to_string()),
            OP_DNEG => Ok("dneg".to_string()),

            // 位运算指令
            OP_ISHL => Ok("ishl".to_string()),
            OP_LSHL => Ok("lshl".to_string()),
            OP_ISHR => Ok("ishr".to_string()),
            OP_LSHR => Ok("lshr".to_string()),
            OP_IUSHR => Ok("iushr".to_string()),
            OP_LUSHR => Ok("lushr".to_string()),
            OP_IAND => Ok("iand".to_string()),
            OP_LAND => Ok("land".to_string()),
            OP_IOR => Ok("ior".to_string()),
            OP_LOR => Ok("lor".to_string()),
            OP_IXOR => Ok("ixor".to_string()),
            OP_LXOR => Ok("lxor".to_string()),
            OP_IINC => {
                let index = self.read_u8()?;
                let const_val = self.read_u8()? as i8;
                Ok(format!("iinc {}, {}", index, const_val))
            }

            // 类型转换指令
            OP_I2L => Ok("i2l".to_string()),
            OP_I2F => Ok("i2f".to_string()),
            OP_I2D => Ok("i2d".to_string()),
            OP_L2I => Ok("l2i".to_string()),
            OP_L2F => Ok("l2f".to_string()),
            OP_L2D => Ok("l2d".to_string()),
            OP_F2I => Ok("f2i".to_string()),
            OP_F2L => Ok("f2l".to_string()),
            OP_F2D => Ok("f2d".to_string()),
            OP_D2I => Ok("d2i".to_string()),
            OP_D2L => Ok("d2l".to_string()),
            OP_D2F => Ok("d2f".to_string()),
            OP_I2B => Ok("i2b".to_string()),
            OP_I2C => Ok("i2c".to_string()),
            OP_I2S => Ok("i2s".to_string()),

            // 比较指令
            OP_LCMP => Ok("lcmp".to_string()),
            OP_FCMPL => Ok("fcmpl".to_string()),
            OP_FCMPG => Ok("fcmpg".to_string()),
            OP_DCMPL => Ok("dcmpl".to_string()),
            OP_DCMPG => Ok("dcmpg".to_string()),

            // 条件跳转指令
            OP_IFEQ => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifeq {}", branch))
            }
            OP_IFNE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifne {}", branch))
            }
            OP_IFLT => {
                let branch = self.read_u16()? as i16;
                Ok(format!("iflt {}", branch))
            }
            OP_IFGE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifge {}", branch))
            }
            OP_IFGT => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifgt {}", branch))
            }
            OP_IFLE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifle {}", branch))
            }
            OP_IF_ICMPEQ => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_icmpeq {}", branch))
            }
            OP_IF_ICMPNE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_icmpne {}", branch))
            }
            OP_IF_ICMPLT => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_icmplt {}", branch))
            }
            OP_IF_ICMPGE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_icmpge {}", branch))
            }
            OP_IF_ICMPGT => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_icmpgt {}", branch))
            }
            OP_IF_ICMPLE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_icmple {}", branch))
            }
            OP_IF_ACMPEQ => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_acmpeq {}", branch))
            }
            OP_IF_ACMPNE => {
                let branch = self.read_u16()? as i16;
                Ok(format!("if_acmpne {}", branch))
            }

            // 无条件跳转指令
            OP_GOTO => {
                let branch = self.read_u16()? as i16;
                Ok(format!("goto {}", branch))
            }
            OP_JSR => {
                let branch = self.read_u16()? as i16;
                Ok(format!("jsr {}", branch))
            }
            OP_RET => {
                let index = self.read_u8()?;
                Ok(format!("ret {}", index))
            }

            // 返回指令
            OP_IRETURN => Ok("ireturn".to_string()),
            OP_LRETURN => Ok("lreturn".to_string()),
            OP_FRETURN => Ok("freturn".to_string()),
            OP_DRETURN => Ok("dreturn".to_string()),
            OP_ARETURN => Ok("areturn".to_string()),
            OP_RETURN => Ok("return".to_string()),

            // 字段访问指令
            OP_GETSTATIC => {
                let index = self.read_u16()?;
                Ok(format!("getstatic #{}", index))
            }
            OP_PUTSTATIC => {
                let index = self.read_u16()?;
                Ok(format!("putstatic #{}", index))
            }
            OP_GETFIELD => {
                let index = self.read_u16()?;
                Ok(format!("getfield #{}", index))
            }
            OP_PUTFIELD => {
                let index = self.read_u16()?;
                Ok(format!("putfield #{}", index))
            }

            // 方法调用指令
            OP_INVOKEVIRTUAL => {
                let index = self.read_u16()?;
                Ok(format!("invokevirtual #{}", index))
            }
            OP_INVOKESPECIAL => {
                let index = self.read_u16()?;
                Ok(format!("invokespecial #{}", index))
            }
            OP_INVOKESTATIC => {
                let index = self.read_u16()?;
                Ok(format!("invokestatic #{}", index))
            }
            OP_INVOKEINTERFACE => {
                let index = self.read_u16()?;
                let count = self.read_u8()?;
                let _zero = self.read_u8()?; // 必须为0
                Ok(format!("invokeinterface #{}, {}", index, count))
            }
            OP_INVOKEDYNAMIC => {
                let index = self.read_u16()?;
                let _zero1 = self.read_u8()?; // 必须为0
                let _zero2 = self.read_u8()?; // 必须为0
                Ok(format!("invokedynamic #{}", index))
            }

            // 对象和数组操作指令
            OP_NEW => {
                let index = self.read_u16()?;
                Ok(format!("new #{}", index))
            }
            OP_NEWARRAY => {
                let atype = self.read_u8()?;
                let type_name = match atype {
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
                Ok(format!("newarray {}", type_name))
            }
            OP_ANEWARRAY => {
                let index = self.read_u16()?;
                Ok(format!("anewarray #{}", index))
            }
            OP_ARRAYLENGTH => Ok("arraylength".to_string()),
            OP_ATHROW => Ok("athrow".to_string()),
            OP_CHECKCAST => {
                let index = self.read_u16()?;
                Ok(format!("checkcast #{}", index))
            }
            OP_INSTANCEOF => {
                let index = self.read_u16()?;
                Ok(format!("instanceof #{}", index))
            }
            OP_MONITORENTER => Ok("monitorenter".to_string()),
            OP_MONITOREXIT => Ok("monitorexit".to_string()),

            // 其他指令
            OP_IFNULL => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifnull {}", branch))
            }
            OP_IFNONNULL => {
                let branch = self.read_u16()? as i16;
                Ok(format!("ifnonnull {}", branch))
            }
            OP_GOTO_W => {
                let branch = self.read_u32()? as i32;
                Ok(format!("goto_w {}", branch))
            }
            OP_JSR_W => {
                let branch = self.read_u32()? as i32;
                Ok(format!("jsr_w {}", branch))
            }

            // 复杂指令（需要特殊处理）
            OP_TABLESWITCH => {
                // 跳过填充字节
                while (self.offset % 4) != 0 {
                    self.read_u8()?;
                }
                let default = self.read_u32()? as i32;
                let low = self.read_u32()? as i32;
                let high = self.read_u32()? as i32;
                let count = (high - low + 1) as usize;
                for _ in 0..count {
                    self.read_u32()?; // 跳过跳转表
                }
                Ok(format!("tableswitch (default: {}, low: {}, high: {})", default, low, high))
            }
            OP_LOOKUPSWITCH => {
                // 跳过填充字节
                while (self.offset % 4) != 0 {
                    self.read_u8()?;
                }
                let default = self.read_u32()? as i32;
                let npairs = self.read_u32()?;
                for _ in 0..npairs {
                    self.read_u32()?; // match
                    self.read_u32()?; // offset
                }
                Ok(format!("lookupswitch (default: {}, npairs: {})", default, npairs))
            }

            OP_WIDE => {
                let wide_opcode = self.read_u8()?;
                match wide_opcode {
                    OP_ILOAD | OP_FLOAD | OP_ALOAD | OP_LLOAD | OP_DLOAD |
                    OP_ISTORE | OP_FSTORE | OP_ASTORE | OP_LSTORE | OP_DSTORE | OP_RET => {
                        let index = self.read_u16()?;
                        Ok(format!("wide {} {}", self.opcode_name(wide_opcode), index))
                    }
                    OP_IINC => {
                        let index = self.read_u16()?;
                        let const_val = self.read_u16()? as i16;
                        Ok(format!("wide iinc {}, {}", index, const_val))
                    }
                    _ => Ok(format!("wide unknown_opcode_0x{:02X}", wide_opcode))
                }
            }

            OP_MULTIANEWARRAY => {
                let index = self.read_u16()?;
                let dimensions = self.read_u8()?;
                Ok(format!("multianewarray #{}, {}", index, dimensions))
            }

            _ => Ok(format!("unknown_opcode_0x{:02X}", opcode))
        }
    }

    fn opcode_name(&self, opcode: u8) -> &'static str {
        match opcode {
            OP_ILOAD => "iload",
            OP_LLOAD => "lload",
            OP_FLOAD => "fload",
            OP_DLOAD => "dload",
            OP_ALOAD => "aload",
            OP_ISTORE => "istore",
            OP_LSTORE => "lstore",
            OP_FSTORE => "fstore",
            OP_DSTORE => "dstore",
            OP_ASTORE => "astore",
            OP_RET => "ret",
            _ => "unknown"
        }
    }
}

pub fn disassemble_classfile(class_data: Vec<u8>) -> Result<String, String> {
    let mut disassembler = ClassFileDisassembler::new(class_data);
    disassembler.disassemble()
}