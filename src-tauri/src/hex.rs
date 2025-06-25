use crate::project::{PROJECTS, ProjectData};
use std::{fs::File, io::{Read, Seek, SeekFrom}};

#[derive(Debug)]
pub struct HexProjectData {
    pub file: Option<File>,
}


#[tauri::command]
pub fn hex_project_get_file_size(project_id: &str) -> Result<u64, String> {
    let projects = PROJECTS.lock().unwrap();
    let project = projects.get(project_id).ok_or("Project not found")?;
    
    match &project.data {
        ProjectData::Hex(hex_data) => {
            if let Some(file) = &hex_data.file {
                let size = file.metadata().map_err(|e| e.to_string())?.len();
                Ok(size)
            } else {
                Err("File not opened".to_string())
            }
        }
        _ => Err("Not a hex project".to_string())
    }
}

#[tauri::command]
pub fn hex_project_get_total_pages(project_id: &str, page_size: usize) -> Result<usize, String> {
    let file_size = hex_project_get_file_size(project_id)?;
    let total_pages = ((file_size as usize) + page_size - 1) / page_size;
    Ok(total_pages)
}

#[tauri::command]
pub fn hex_project_read_page(project_id: &str, page: usize, page_size: usize) -> Result<Vec<u8>, String> {
    let mut projects: std::sync::MutexGuard<'_, std::collections::HashMap<String, crate::project::Project>> = PROJECTS.lock().unwrap();
    let project = projects.get_mut(project_id).ok_or("Project not found")?;
    
    match &mut project.data {
        ProjectData::Hex(hex_data) => {
            if let Some(file) = &mut hex_data.file {
                let offset = (page * page_size) as u64;
                file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;
                
                let mut buffer = vec![0u8; page_size];
                let bytes_read = file.read(&mut buffer).map_err(|e| e.to_string())?;
                buffer.truncate(bytes_read);
                
                Ok(buffer)
            } else {
                Err("File not opened".to_string())
            }
        }
        _ => Err("Not a hex project".to_string())
    }
}