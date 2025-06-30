// Removed unused import of File, since File does not implement Clone, PartialEq, or Eq.

use crate::{java_analyzer::jar::{JarEntry, JarReader}, project::Project}; // Add this import if ZipEntry comes from the 'zip' crate
use crate::java_analyzer::disassembler::disassemble_classfile;

pub struct JavaProjectData {
    pub jar_reader:JarReader,
    pub class_files: Vec<JarEntry>,
}

impl JavaProjectData {
    pub fn new(jar_path: String) -> Self {
        // 创建 JAR 读取器并扫描文件
        let jar_reader = crate::java_analyzer::jar::JarReader::new(&jar_path);
        let class_files = jar_reader.list_entries().unwrap_or_default();

        JavaProjectData {
            jar_reader,
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

#[tauri::command]
pub fn java_project_read_file_content(project_id: String, file_name: String) -> Result<String, String> {
    Project::with_project(&project_id, |project| {
        if let crate::project::ProjectData::Java(java_data) = &project.data {
            if file_name.ends_with(".class") {
                // 如果是 class 文件，进行反汇编
                let bytes = java_data.jar_reader.read_file(&file_name)?;
                disassemble_classfile(bytes)
            } else {
                // 其他文件直接读取为字符串
                java_data.jar_reader.read_file_as_string(&file_name)
            }
        } else {
            Err("Not a Java project".to_string())
        }
    })
}

