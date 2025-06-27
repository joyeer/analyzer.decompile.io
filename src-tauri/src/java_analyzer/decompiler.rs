use crate::java_analyzer::{
    classfile::ClassFile,
    controlflow::ControlFlowGraph,
    controlflowbuilder::ControlFlowGraphBuilder,
    error::{Result, JavaAnalyzeError},
};

pub struct Decompiler<'a> {
    classfile: &'a ClassFile
}

impl<'a> Decompiler<'a> {
    // Constructor for the Decompiler struct
    pub fn new(classfile: &'a ClassFile) -> Self {
        Decompiler {
            classfile,
        }
    }

    pub fn decompile_method(&self, method: &str) -> Result<String> {
        self.classfile.methods.iter()
            .find(|m| m.name == method)
            .map(|m| {

                let controlflowbuilder = ControlFlowGraph::new();
                controlflowbuilder.build(m, self.classfile).unwrap();
                let mut result = String::new();
                result.push_str(&format!("Method: {}\n", m.name));
                result.push_str(&format!("Descriptor: {}\n", m.descriptor));
                result.push_str(&format!("Access Flags: {}\n", m.access_flags));
                result
            })
            .ok_or_else(|| JavaAnalyzeError::InvalidClassData(format!("Method {} not found", method)))
    }
}