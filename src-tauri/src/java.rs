use std::fs::File;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavaProjectData {
    pub file: Option<File>,
}

