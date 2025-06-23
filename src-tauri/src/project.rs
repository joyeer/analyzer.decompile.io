use uuid;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 项目文件需要支持类型，比如Hex文件，Java反编译项目，Android反编译项目
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectType {
    Hex,
    Java,
    Android,
}

pub struct Project {
    pub project_type: ProjectType,
    pub id: String,
    pub name: String,
    pub path: String,
}

pub static PROJECTS: Lazy<Mutex<HashMap<String, Project>>> = Lazy::new(|| Mutex::new(HashMap::new()));

impl Project {
    pub fn new(project_type: ProjectType, name: String, path: String) -> String {
        let project = Project {
            project_type: project_type,
            id: uuid::Uuid::new_v4().to_string(),
            name,
            path
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
        } else if path.ends_with(".java") {
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
