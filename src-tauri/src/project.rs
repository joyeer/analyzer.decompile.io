

pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub description: String,
    pub version: String,
}

impl Project {
    pub fn new(name: String, path: String, description: String, version: String) -> Self {
        Project {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            path,
            description,
            version,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }
}

pub fn project_create(name: String, path: String, description: String, version: String) -> Project {
    let project  = Project::new(name, path, description, version);
    PROJECTS.lock().unwrap().insert(project.id, project);
}

pub static PROJECTS: Lazy<Mutex<HashMap<String, Project>>> = Lazy::new(|| Mutex::new(HashMap::new()));
