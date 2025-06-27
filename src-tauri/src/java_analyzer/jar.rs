use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
pub struct JarReader {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct JarEntry {
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
    pub is_class_file: bool,
}

impl JarReader {
    /// 创建新的 JAR 读取器
    pub fn new(jar_path: &str) -> Self {
        JarReader {
            path: jar_path.to_string(),
        }
    }

    /// 列出 JAR 文件中的所有条目（只获取文件名和基本信息）
    pub fn list_entries(&self) -> Result<Vec<JarEntry>, String> {
        let file = File::open(&self.path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        let mut entries = Vec::new();
        
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = file.name().to_string();
            let size = file.size();
            let is_directory = name.ends_with('/');
            let is_class_file = name.ends_with(".class");
            
            entries.push(JarEntry {
                name,
                size,
                is_directory,
                is_class_file,
            });
        }
        
        Ok(entries)
    }

    /// 只列出 class 文件
    pub fn list_class_files(&self) -> Result<Vec<String>, String> {
        let entries = self.list_entries()?;
        Ok(entries
            .into_iter()
            .filter(|entry| entry.is_class_file)
            .map(|entry| entry.name)
            .collect())
    }

    /// 根据文件名读取指定文件的内容
    pub fn read_file(&self, file_name: &str) -> Result<Vec<u8>, String> {
        let file = File::open(&self.path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        let mut zip_file = archive.by_name(file_name).map_err(|e| e.to_string())?;
        let mut contents = Vec::new();
        zip_file.read_to_end(&mut contents).map_err(|e| e.to_string())?;
        
        Ok(contents)
    }

    /// 读取指定文件的内容为字符串（适用于文本文件）
    pub fn read_file_as_string(&self, file_name: &str) -> Result<String, String> {
        let bytes = self.read_file(file_name)?;
        String::from_utf8(bytes).map_err(|e| e.to_string())
    }

    /// 检查文件是否存在
    pub fn contains_file(&self, file_name: &str) -> Result<bool, String> {
        let file = File::open(&self.path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        let result = archive.by_name(file_name).is_ok();
        Ok(result)
    }

    /// 获取 META-INF/MANIFEST.MF 内容
    pub fn get_manifest(&self) -> Result<String, String> {
        self.read_file_as_string("META-INF/MANIFEST.MF")
    }

    /// 列出指定目录下的文件（不递归）
    pub fn list_directory(&self, dir_path: &str) -> Result<Vec<String>, String> {
        let entries = self.list_entries()?;
        let normalized_dir = if dir_path.is_empty() {
            String::new()
        } else if dir_path.ends_with('/') {
            dir_path.to_string()
        } else {
            format!("{}/", dir_path)
        };
        
        let result = entries
            .into_iter()
            .filter_map(|entry| {
                if entry.name.starts_with(&normalized_dir) && !entry.is_directory {
                    let relative_path = &entry.name[normalized_dir.len()..];
                    if !relative_path.contains('/') {
                        Some(entry.name)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        Ok(result)
    }

    /// 获取文件信息（不读取内容）
    pub fn get_file_info(&self, file_name: &str) -> Result<JarEntry, String> {
        let file = File::open(&self.path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        let zip_file = archive.by_name(file_name).map_err(|e| e.to_string())?;
        let name = zip_file.name().to_string();
        let size = zip_file.size();
        let is_directory = name.ends_with('/');
        let is_class_file = name.ends_with(".class");
        
        Ok(JarEntry {
            name,
            size,
            is_directory,
            is_class_file,
        })
    }
}


