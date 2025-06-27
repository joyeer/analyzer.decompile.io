use crate::java_analyzer::classfile::ClassFile;


pub struct Disassembler<'a> {
    classfile: &'a ClassFile,
}

impl<'a> Disassembler<'a> {
    pub fn new(classfile: &'a ClassFile) -> Self {
        
        return Disassembler {
            classfile,
        }
    }
}