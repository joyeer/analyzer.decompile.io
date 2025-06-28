// Removed unused import of File, since File does not implement Clone, PartialEq, or Eq.

use crate::java_analyzer::jar::{JarEntry, JarReader}; // Add this import if ZipEntry comes from the 'zip' crate

pub struct JavaProjectData {
    pub jar_reader:JarReader,
    pub class_files: Vec<JarEntry>,
}

impl JavaProjectData {
    pub fn new(jar_path: String, class_files: Vec<JarEntry>) -> Self {
        JavaProjectData {
            jar_reader: JarReader::new(&jar_path),
            class_files,
        }
    }
}


pub fn hex_project_list_files(project: &JavaProjectData) -> Vec<String> {
    project.class_files.iter().map(|entry| entry.name.clone()).collect()
}   