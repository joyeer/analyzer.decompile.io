use std::collections::HashMap;
use crate::android_analyzer::error::{AndroidAnalyzeError, Result};

/// Dalvik opcode analyzer for parsing and analyzing bytecode
pub struct DalvikOpcodeAnalyzer {
    opcode_map: HashMap<u8, DalvikOpcode>,
}

impl DalvikOpcodeAnalyzer {
    pub fn new() -> Self {
        let mut opcode_map = HashMap::new();
        
        // Initialize opcode map with Dalvik opcodes
        opcode_map.insert(0x00, DalvikOpcode::Nop);
        opcode_map.insert(0x01, DalvikOpcode::Move);
        opcode_map.insert(0x02, DalvikOpcode::MoveFrom16);
        opcode_map.insert(0x03, DalvikOpcode::Move16);
        opcode_map.insert(0x04, DalvikOpcode::MoveWide);
        opcode_map.insert(0x05, DalvikOpcode::MoveWideFrom16);
        opcode_map.insert(0x06, DalvikOpcode::MoveWide16);
        opcode_map.insert(0x07, DalvikOpcode::MoveObject);
        opcode_map.insert(0x08, DalvikOpcode::MoveObjectFrom16);
        opcode_map.insert(0x09, DalvikOpcode::MoveObject16);
        opcode_map.insert(0x0a, DalvikOpcode::MoveResult);
        opcode_map.insert(0x0b, DalvikOpcode::MoveResultWide);
        opcode_map.insert(0x0c, DalvikOpcode::MoveResultObject);
        opcode_map.insert(0x0d, DalvikOpcode::MoveException);
        opcode_map.insert(0x0e, DalvikOpcode::ReturnVoid);
        opcode_map.insert(0x0f, DalvikOpcode::Return);
        opcode_map.insert(0x10, DalvikOpcode::ReturnWide);
        opcode_map.insert(0x11, DalvikOpcode::ReturnObject);
        opcode_map.insert(0x12, DalvikOpcode::Const4);
        opcode_map.insert(0x13, DalvikOpcode::Const16);
        opcode_map.insert(0x14, DalvikOpcode::Const);
        opcode_map.insert(0x15, DalvikOpcode::ConstHigh16);
        opcode_map.insert(0x16, DalvikOpcode::ConstWide16);
        opcode_map.insert(0x17, DalvikOpcode::ConstWide32);
        opcode_map.insert(0x18, DalvikOpcode::ConstWide);
        opcode_map.insert(0x19, DalvikOpcode::ConstWideHigh16);
        opcode_map.insert(0x1a, DalvikOpcode::ConstString);
        opcode_map.insert(0x1b, DalvikOpcode::ConstStringJumbo);
        opcode_map.insert(0x1c, DalvikOpcode::ConstClass);
        opcode_map.insert(0x1d, DalvikOpcode::MonitorEnter);
        opcode_map.insert(0x1e, DalvikOpcode::MonitorExit);
        opcode_map.insert(0x1f, DalvikOpcode::CheckCast);
        opcode_map.insert(0x20, DalvikOpcode::InstanceOf);
        opcode_map.insert(0x21, DalvikOpcode::ArrayLength);
        opcode_map.insert(0x22, DalvikOpcode::NewInstance);
        opcode_map.insert(0x23, DalvikOpcode::NewArray);
        opcode_map.insert(0x24, DalvikOpcode::FilledNewArray);
        opcode_map.insert(0x25, DalvikOpcode::FilledNewArrayRange);
        opcode_map.insert(0x26, DalvikOpcode::FillArrayData);
        opcode_map.insert(0x27, DalvikOpcode::Throw);
        opcode_map.insert(0x28, DalvikOpcode::Goto);
        opcode_map.insert(0x29, DalvikOpcode::Goto16);
        opcode_map.insert(0x2a, DalvikOpcode::Goto32);
        opcode_map.insert(0x2b, DalvikOpcode::PackedSwitch);
        opcode_map.insert(0x2c, DalvikOpcode::SparseSwitch);
        
        // Comparison opcodes
        opcode_map.insert(0x2d, DalvikOpcode::CmplFloat);
        opcode_map.insert(0x2e, DalvikOpcode::CmpgFloat);
        opcode_map.insert(0x2f, DalvikOpcode::CmplDouble);
        opcode_map.insert(0x30, DalvikOpcode::CmpgDouble);
        opcode_map.insert(0x31, DalvikOpcode::CmpLong);
        
        // Conditional branches
        opcode_map.insert(0x32, DalvikOpcode::IfEq);
        opcode_map.insert(0x33, DalvikOpcode::IfNe);
        opcode_map.insert(0x34, DalvikOpcode::IfLt);
        opcode_map.insert(0x35, DalvikOpcode::IfGe);
        opcode_map.insert(0x36, DalvikOpcode::IfGt);
        opcode_map.insert(0x37, DalvikOpcode::IfLe);
        opcode_map.insert(0x38, DalvikOpcode::IfEqz);
        opcode_map.insert(0x39, DalvikOpcode::IfNez);
        opcode_map.insert(0x3a, DalvikOpcode::IfLtz);
        opcode_map.insert(0x3b, DalvikOpcode::IfGez);
        opcode_map.insert(0x3c, DalvikOpcode::IfGtz);
        opcode_map.insert(0x3d, DalvikOpcode::IfLez);
        
        // Array operations
        opcode_map.insert(0x44, DalvikOpcode::Aget);
        opcode_map.insert(0x45, DalvikOpcode::AgetWide);
        opcode_map.insert(0x46, DalvikOpcode::AgetObject);
        opcode_map.insert(0x47, DalvikOpcode::AgetBoolean);
        opcode_map.insert(0x48, DalvikOpcode::AgetByte);
        opcode_map.insert(0x49, DalvikOpcode::AgetChar);
        opcode_map.insert(0x4a, DalvikOpcode::AgetShort);
        opcode_map.insert(0x4b, DalvikOpcode::Aput);
        opcode_map.insert(0x4c, DalvikOpcode::AputWide);
        opcode_map.insert(0x4d, DalvikOpcode::AputObject);
        opcode_map.insert(0x4e, DalvikOpcode::AputBoolean);
        opcode_map.insert(0x4f, DalvikOpcode::AputByte);
        opcode_map.insert(0x50, DalvikOpcode::AputChar);
        opcode_map.insert(0x51, DalvikOpcode::AputShort);
        
        // Instance field operations
        opcode_map.insert(0x52, DalvikOpcode::Iget);
        opcode_map.insert(0x53, DalvikOpcode::IgetWide);
        opcode_map.insert(0x54, DalvikOpcode::IgetObject);
        opcode_map.insert(0x55, DalvikOpcode::IgetBoolean);
        opcode_map.insert(0x56, DalvikOpcode::IgetByte);
        opcode_map.insert(0x57, DalvikOpcode::IgetChar);
        opcode_map.insert(0x58, DalvikOpcode::IgetShort);
        opcode_map.insert(0x59, DalvikOpcode::Iput);
        opcode_map.insert(0x5a, DalvikOpcode::IputWide);
        opcode_map.insert(0x5b, DalvikOpcode::IputObject);
        opcode_map.insert(0x5c, DalvikOpcode::IputBoolean);
        opcode_map.insert(0x5d, DalvikOpcode::IputByte);
        opcode_map.insert(0x5e, DalvikOpcode::IputChar);
        opcode_map.insert(0x5f, DalvikOpcode::IputShort);
        
        // Static field operations
        opcode_map.insert(0x60, DalvikOpcode::Sget);
        opcode_map.insert(0x61, DalvikOpcode::SgetWide);
        opcode_map.insert(0x62, DalvikOpcode::SgetObject);
        opcode_map.insert(0x63, DalvikOpcode::SgetBoolean);
        opcode_map.insert(0x64, DalvikOpcode::SgetByte);
        opcode_map.insert(0x65, DalvikOpcode::SgetChar);
        opcode_map.insert(0x66, DalvikOpcode::SgetShort);
        opcode_map.insert(0x67, DalvikOpcode::Sput);
        opcode_map.insert(0x68, DalvikOpcode::SputWide);
        opcode_map.insert(0x69, DalvikOpcode::SputObject);
        opcode_map.insert(0x6a, DalvikOpcode::SputBoolean);
        opcode_map.insert(0x6b, DalvikOpcode::SputByte);
        opcode_map.insert(0x6c, DalvikOpcode::SputChar);
        opcode_map.insert(0x6d, DalvikOpcode::SputShort);
        
        // Method invocation
        opcode_map.insert(0x6e, DalvikOpcode::InvokeVirtual);
        opcode_map.insert(0x6f, DalvikOpcode::InvokeSuper);
        opcode_map.insert(0x70, DalvikOpcode::InvokeDirect);
        opcode_map.insert(0x71, DalvikOpcode::InvokeStatic);
        opcode_map.insert(0x72, DalvikOpcode::InvokeInterface);
        opcode_map.insert(0x74, DalvikOpcode::InvokeVirtualRange);
        opcode_map.insert(0x75, DalvikOpcode::InvokeSuperRange);
        opcode_map.insert(0x76, DalvikOpcode::InvokeDirectRange);
        opcode_map.insert(0x77, DalvikOpcode::InvokeStaticRange);
        opcode_map.insert(0x78, DalvikOpcode::InvokeInterfaceRange);
        
        // Arithmetic operations
        opcode_map.insert(0x7b, DalvikOpcode::NegInt);
        opcode_map.insert(0x7c, DalvikOpcode::NotInt);
        opcode_map.insert(0x7d, DalvikOpcode::NegLong);
        opcode_map.insert(0x7e, DalvikOpcode::NotLong);
        opcode_map.insert(0x7f, DalvikOpcode::NegFloat);
        opcode_map.insert(0x80, DalvikOpcode::NegDouble);
        opcode_map.insert(0x81, DalvikOpcode::IntToLong);
        opcode_map.insert(0x82, DalvikOpcode::IntToFloat);
        opcode_map.insert(0x83, DalvikOpcode::IntToDouble);
        opcode_map.insert(0x84, DalvikOpcode::LongToInt);
        opcode_map.insert(0x85, DalvikOpcode::LongToFloat);
        opcode_map.insert(0x86, DalvikOpcode::LongToDouble);
        opcode_map.insert(0x87, DalvikOpcode::FloatToInt);
        opcode_map.insert(0x88, DalvikOpcode::FloatToLong);
        opcode_map.insert(0x89, DalvikOpcode::FloatToDouble);
        opcode_map.insert(0x8a, DalvikOpcode::DoubleToInt);
        opcode_map.insert(0x8b, DalvikOpcode::DoubleToLong);
        opcode_map.insert(0x8c, DalvikOpcode::DoubleToFloat);
        opcode_map.insert(0x8d, DalvikOpcode::IntToByte);
        opcode_map.insert(0x8e, DalvikOpcode::IntToChar);
        opcode_map.insert(0x8f, DalvikOpcode::IntToShort);
        
        // Binary operations
        opcode_map.insert(0x90, DalvikOpcode::AddInt);
        opcode_map.insert(0x91, DalvikOpcode::SubInt);
        opcode_map.insert(0x92, DalvikOpcode::MulInt);
        opcode_map.insert(0x93, DalvikOpcode::DivInt);
        opcode_map.insert(0x94, DalvikOpcode::RemInt);
        opcode_map.insert(0x95, DalvikOpcode::AndInt);
        opcode_map.insert(0x96, DalvikOpcode::OrInt);
        opcode_map.insert(0x97, DalvikOpcode::XorInt);
        opcode_map.insert(0x98, DalvikOpcode::ShlInt);
        opcode_map.insert(0x99, DalvikOpcode::ShrInt);
        opcode_map.insert(0x9a, DalvikOpcode::UshrInt);
        
        Self { opcode_map }
    }

    /// Decode a single instruction
    pub fn decode_instruction(&self, bytecode: &[u8], offset: usize) -> Result<DalvikInstruction> {
        if offset >= bytecode.len() {
            return Err(AndroidAnalyzeError::ParseError("Bytecode offset out of bounds".to_string()));
        }
        
        let opcode_byte = bytecode[offset];
        let opcode = self.opcode_map.get(&opcode_byte)
            .ok_or_else(|| AndroidAnalyzeError::ParseError(format!("Unknown opcode: 0x{:02x}", opcode_byte)))?;
        
        let instruction = match opcode {
            DalvikOpcode::Nop => DalvikInstruction {
                opcode: opcode.clone(),
                operands: vec![],
                size: 1,
            },
            DalvikOpcode::Move => {
                if offset + 1 >= bytecode.len() {
                    return Err(AndroidAnalyzeError::ParseError("Incomplete instruction".to_string()));
                }
                let operand = bytecode[offset + 1];
                DalvikInstruction {
                    opcode: opcode.clone(),
                    operands: vec![operand as u32],
                    size: 2,
                }
            }
            DalvikOpcode::ReturnVoid => DalvikInstruction {
                opcode: opcode.clone(),
                operands: vec![],
                size: 1,
            },
            DalvikOpcode::InvokeVirtual | DalvikOpcode::InvokeSuper | DalvikOpcode::InvokeDirect | 
            DalvikOpcode::InvokeStatic | DalvikOpcode::InvokeInterface => {
                if offset + 5 >= bytecode.len() {
                    return Err(AndroidAnalyzeError::ParseError("Incomplete invoke instruction".to_string()));
                }
                let arg_count = (bytecode[offset + 1] >> 4) & 0xf;
                let method_idx = ((bytecode[offset + 3] as u16) << 8) | (bytecode[offset + 2] as u16);
                let args = bytecode[offset + 1] & 0xf;
                
                DalvikInstruction {
                    opcode: opcode.clone(),
                    operands: vec![arg_count as u32, method_idx as u32, args as u32],
                    size: 6,
                }
            }
            _ => {
                // Default case for other opcodes
                DalvikInstruction {
                    opcode: opcode.clone(),
                    operands: vec![],
                    size: 2,
                }
            }
        };
        
        Ok(instruction)
    }

    /// Analyze a method's bytecode
    pub fn analyze_method(&self, bytecode: &[u8]) -> Result<Vec<DalvikInstruction>> {
        let mut instructions = Vec::new();
        let mut offset = 0;
        
        while offset < bytecode.len() {
            let instruction = self.decode_instruction(bytecode, offset)?;
            offset += instruction.size;
            instructions.push(instruction);
        }
        
        Ok(instructions)
    }
}

/// Dalvik opcodes
#[derive(Debug, Clone, PartialEq)]
pub enum DalvikOpcode {
    Nop,
    Move,
    MoveFrom16,
    Move16,
    MoveWide,
    MoveWideFrom16,
    MoveWide16,
    MoveObject,
    MoveObjectFrom16,
    MoveObject16,
    MoveResult,
    MoveResultWide,
    MoveResultObject,
    MoveException,
    ReturnVoid,
    Return,
    ReturnWide,
    ReturnObject,
    Const4,
    Const16,
    Const,
    ConstHigh16,
    ConstWide16,
    ConstWide32,
    ConstWide,
    ConstWideHigh16,
    ConstString,
    ConstStringJumbo,
    ConstClass,
    MonitorEnter,
    MonitorExit,
    CheckCast,
    InstanceOf,
    ArrayLength,
    NewInstance,
    NewArray,
    FilledNewArray,
    FilledNewArrayRange,
    FillArrayData,
    Throw,
    Goto,
    Goto16,
    Goto32,
    PackedSwitch,
    SparseSwitch,
    CmplFloat,
    CmpgFloat,
    CmplDouble,
    CmpgDouble,
    CmpLong,
    IfEq,
    IfNe,
    IfLt,
    IfGe,
    IfGt,
    IfLe,
    IfEqz,
    IfNez,
    IfLtz,
    IfGez,
    IfGtz,
    IfLez,
    Aget,
    AgetWide,
    AgetObject,
    AgetBoolean,
    AgetByte,
    AgetChar,
    AgetShort,
    Aput,
    AputWide,
    AputObject,
    AputBoolean,
    AputByte,
    AputChar,
    AputShort,
    Iget,
    IgetWide,
    IgetObject,
    IgetBoolean,
    IgetByte,
    IgetChar,
    IgetShort,
    Iput,
    IputWide,
    IputObject,
    IputBoolean,
    IputByte,
    IputChar,
    IputShort,
    Sget,
    SgetWide,
    SgetObject,
    SgetBoolean,
    SgetByte,
    SgetChar,
    SgetShort,
    Sput,
    SputWide,
    SputObject,
    SputBoolean,
    SputByte,
    SputChar,
    SputShort,
    InvokeVirtual,
    InvokeSuper,
    InvokeDirect,
    InvokeStatic,
    InvokeInterface,
    InvokeVirtualRange,
    InvokeSuperRange,
    InvokeDirectRange,
    InvokeStaticRange,
    InvokeInterfaceRange,
    NegInt,
    NotInt,
    NegLong,
    NotLong,
    NegFloat,
    NegDouble,
    IntToLong,
    IntToFloat,
    IntToDouble,
    LongToInt,
    LongToFloat,
    LongToDouble,
    FloatToInt,
    FloatToLong,
    FloatToDouble,
    DoubleToInt,
    DoubleToLong,
    DoubleToFloat,
    IntToByte,
    IntToChar,
    IntToShort,
    AddInt,
    SubInt,
    MulInt,
    DivInt,
    RemInt,
    AndInt,
    OrInt,
    XorInt,
    ShlInt,
    ShrInt,
    UshrInt,
}

/// Dalvik instruction
#[derive(Debug, Clone)]
pub struct DalvikInstruction {
    pub opcode: DalvikOpcode,
    pub operands: Vec<u32>,
    pub size: usize,
}

impl DalvikInstruction {
    /// Get a human-readable representation of the instruction
    pub fn to_string(&self) -> String {
        format!("{:?} {:?}", self.opcode, self.operands)
    }
}
