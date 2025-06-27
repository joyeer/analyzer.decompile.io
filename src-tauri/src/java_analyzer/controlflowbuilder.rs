use std::io::Result;
use crate::java_analyzer::{classfile::ClassFile, method::JvmMethod};
use crate::java_analyzer::opcode::*;
use crate::java_analyzer::controlflow::ControlFlowGraph;

#[derive(Clone)]
enum FlowType {
    Goto,
    GotoTenaryOperator,
    Throw,
    Return,
    ReturnValue,
    Conditional,
    Switch,
    Jsr,
    Ret,
    Unknown,
}

pub trait ControlFlowGraphBuilder {
    fn build(&self, method:&JvmMethod, classfile: &ClassFile) -> Result<ControlFlowGraph>;
} 

impl ControlFlowGraphBuilder for ControlFlowGraph {
    fn build(&self, method:&JvmMethod, classfile:&ClassFile) -> Result<ControlFlowGraph> {

        let constantpool = &classfile.constant_pool;
        let mut blocks = vec![];
        
        let instructions = &method.code;
        let length = instructions.len();
        let mut flow_types: Vec<FlowType> = vec![FlowType::Unknown; length];

        let mut index = 0;
        let mut lastStatementOffset = 0;

        while index < length {
            let opcode = &instructions[index];
            
            match opcode.opcode {
                OP_ISTORE | OP_LSTORE | OP_FSTORE | OP_DSTORE | OP_ASTORE |
                OP_ISTORE_0 | OP_ISTORE_1 | OP_ISTORE_2 | OP_ISTORE_3 |
                OP_LSTORE_0 | OP_LSTORE_1 | OP_LSTORE_2 | OP_LSTORE_3 |
                OP_FSTORE_0 | OP_FSTORE_1 | OP_FSTORE_2 | OP_FSTORE_3 |
                OP_DSTORE_0 | OP_DSTORE_1 | OP_DSTORE_2 | OP_DSTORE_3 |
                OP_ASTORE_0 | OP_ASTORE_1 | OP_ASTORE_2 | OP_ASTORE_3 |
                OP_IASTORE | OP_LASTORE | OP_FASTORE | OP_DASTORE | OP_AASTORE | OP_BASTORE | OP_CASTORE | OP_SASTORE => {
                    lastStatementOffset = opcode.offset;
                }
                OP_RET => {
                    panic!("RET instruction not supported");
                }
                OP_PUTSTATIC  | OP_PUTFIELD => {
                    lastStatementOffset = opcode.offset;
                }
                OP_INVOKEVIRTUAL | OP_INVOKESPECIAL | OP_INVOKESTATIC | OP_INVOKEINTERFACE | OP_INVOKEDYNAMIC => {
                    let (_, name_and_type_index) = constantpool.get_method_ref(opcode.value as usize).unwrap();
                    let (_, descriptor_index) = constantpool.get_name_and_type(name_and_type_index as usize).unwrap();
                    let descriptor = constantpool.get_utf8(descriptor_index as usize).unwrap();
                    if descriptor.ends_with("V") {
                        lastStatementOffset  = opcode.offset;
                    }
                }
                OP_IINC => {
                    panic!("IINC instruction not supported");
                }
                OP_GOTO | OP_GOTO_W => {
                    panic!("GOTO instruction not supported");
                }
                _ => {
                   // panic!("Unknown opcode: {}", opcode.opcode);
                }
            }
            index += 1;
        }

        let mut last_index = 0;
        
        
        Ok(ControlFlowGraph {
            blocks,
            edges: vec![],
        })
    }

    
}
