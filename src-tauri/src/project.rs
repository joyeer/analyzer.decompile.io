use uuid;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::hex::HexProjectData;
use crate::java::JavaProjectData;
use crate::android::AndroidProjectData;

// 项目文件需要支持类型，比如Hex文件，Java反编译项目，Android反编译项目
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectType {
    Hex,
    Java,
    Android,
}

pub enum ProjectData {
    Hex(HexProjectData),
    Java(JavaProjectData),
    Android(AndroidProjectData),
}

pub struct Project {
    pub project_type: ProjectType,
    pub id: String,
    pub name: String,
    pub path: String,
    pub data: ProjectData,
}

pub static PROJECTS: Lazy<Mutex<HashMap<String, Project>>> = Lazy::new(|| Mutex::new(HashMap::new()));

impl Project {
    pub fn new(project_type: ProjectType, name: String, path: String) -> String {
        let data = match project_type {
            ProjectType::Hex => {
                let file = File::open(&path).ok();
                ProjectData::Hex(HexProjectData {
                    file
                })
            },
            ProjectType::Java => {
                ProjectData::Java(JavaProjectData::new(path.clone()))
            },
            ProjectType::Android => {
                ProjectData::Android(AndroidProjectData {
                    apk_path: path.clone(),
                    // 其他 Android 专属字段
                })
            }
        };

        let project = Project {
            project_type,
            id: uuid::Uuid::new_v4().to_string(),
            name,
            path,
            data,
        };
        let id = project.id.clone();
        let mut projects = PROJECTS.lock().unwrap();
        projects.insert(id.clone(), project);
        id
    }

    /// 给定一个文件或目录路径，自动判断类型并创建 Project
    pub fn create_project_from_path(path: &str) -> String {
        let project_type = if path.ends_with(".apk") {
            ProjectType::Android
        } else if path.ends_with(".jar") || path.ends_with(".class") {
            ProjectType::Java
        } else {
            ProjectType::Hex // 默认类型为 Hex
        };

        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unnamed")
            .to_string();
        return Project::new(project_type, name, path.to_string())
    }

    pub fn query_type(project_id: &str) -> Option<ProjectType> {
        let projects = PROJECTS.lock().unwrap();
        projects.get(project_id).map(|p| p.project_type.clone())
    }   

    /// 执行需要访问项目的操作（不可变）
    pub fn with_project<T, F>(project_id: &str, f: F) -> Result<T, String>
    where
        F: FnOnce(&Project) -> Result<T, String>,
    {
        let projects = PROJECTS.lock().map_err(|_| "Failed to lock projects")?;
        let project = projects.get(project_id).ok_or("Project not found")?;
        f(project)
    }

    /// 执行需要访问项目的操作（可变）
    pub fn with_project_mut<T, F>(project_id: &str, f: F) -> Result<T, String>
    where
        F: FnOnce(&mut Project) -> Result<T, String>,
    {
        let mut projects = PROJECTS.lock().map_err(|_| "Failed to lock projects")?;
        let project = projects.get_mut(project_id).ok_or("Project not found")?;
        f(project)
    }

}

#[tauri::command]
pub fn create_project(path: &str) -> String {
    let project_id = Project::create_project_from_path(path);
    project_id
}

#[tauri::command]
pub fn project_get_type(project_id: &str) -> String {
    match Project::query_type(project_id) {
        Some(project_type) => format!("{:?}", project_type),
        None => "Unknown".to_string(),
    }
}