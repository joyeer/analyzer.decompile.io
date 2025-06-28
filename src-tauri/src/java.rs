// Removed unused import of File, since File does not implement Clone, PartialEq, or Eq.

use crate::{java_analyzer::jar::{JarEntry, JarReader}, project::Project}; // Add this import if ZipEntry comes from the 'zip' crate

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

#[tauri::command]
pub fn java_project_list_files(project_id: String) -> Vec<String> {
    Project::with_project(&project_id, |project: &Project| {
        if let crate::project::ProjectData::Java(java_data) = &project.data {
            Ok(java_data.class_files.iter().map(|entry| entry.name.clone()).collect())
        } else {
            Err("Not a Java project".to_string())
        }
    })
    .unwrap_or_else(|_| Vec::new())
    
}
