use std::path::Path;
use std::collections::HashMap;
use std::io::{Read, Seek, Cursor};
use zip::ZipArchive;
use crate::android_analyzer::error::{AndroidAnalyzeError, Result};
use crate::android_analyzer::dex_analyzer::DexAnalyzer;
use crate::android_analyzer::axml_analyzer::AXMLAnalyzer;
use crate::android::AndroidProjectData;

/// APK analyzer for parsing Android APK files
pub struct ApkAnalyzer {
    apk_path: String,
    zip_archive: Option<ZipArchive<std::fs::File>>,
}

impl ApkAnalyzer {
    /// Create a new APK analyzer
    pub fn new(apk_path: String) -> Self {
        Self {
            apk_path,
            zip_archive: None,
        }
    }

    /// Open and analyze the APK file
    pub fn analyze(&mut self) -> Result<AndroidProjectData> {
        let file = std::fs::File::open(&self.apk_path)
            .map_err(|e| AndroidAnalyzeError::InvalidApkFile(format!("Cannot open APK file: {}", e)))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AndroidAnalyzeError::InvalidApkFile(format!("Invalid ZIP archive: {}", e)))?;

        let mut project_data = AndroidProjectData::new();
        
        // Step 1: Analyze AndroidManifest.xml
        if let Ok(mut manifest_file) = archive.by_name("AndroidManifest.xml") {
            let mut manifest_data = Vec::new();
            manifest_file.read_to_end(&mut manifest_data)?;
            
            let mut axml_analyzer = AXMLAnalyzer::new(Cursor::new(manifest_data));
            if let Ok(manifest) = axml_analyzer.analyze() {
                project_data.manifest = Some(manifest);
            }
        }
        
        // Step 2: Analyze resources.arsc
        if let Ok(mut arsc_file) = archive.by_name("resources.arsc") {
            let mut arsc_data = Vec::new();
            arsc_file.read_to_end(&mut arsc_data)?;
            
            let mut arsc_analyzer = crate::android_analyzer::arsc_analyzer::ARSCAnalyzer::new(Cursor::new(arsc_data));
            if let Ok(resource_table) = arsc_analyzer.analyze() {
                project_data.resource_table = Some(resource_table);
            }
        }

        // Step 3: Find and analyze DEX files
        let mut dex_files = Vec::new();
        let mut dex_index = 0;
        
        // Look for classes.dex, classes2.dex, etc.
        loop {
            let dex_name = if dex_index == 0 {
                "classes.dex".to_string()
            } else {
                format!("classes{}.dex", dex_index + 1)
            };
            
            match archive.by_name(&dex_name) {
                Ok(mut dex_file) => {
                    let mut dex_data = Vec::new();
                    dex_file.read_to_end(&mut dex_data)?;
                    
                    let mut dex_analyzer = DexAnalyzer::new(Cursor::new(dex_data));
                    let dex_file_struct = dex_analyzer.analyze()?;
                    dex_files.push(dex_file_struct);
                    
                    dex_index += 1;
                }
                Err(_) => break,
            }
        }

        if dex_files.is_empty() {
            return Err(AndroidAnalyzeError::InvalidApkFile("No DEX files found in APK".to_string()));
        }

        // Analyze AndroidManifest.xml
        if let Ok(mut manifest_file) = archive.by_name("AndroidManifest.xml") {
            let mut manifest_data = Vec::new();
            manifest_file.read_to_end(&mut manifest_data)?;
            
            let mut axml_analyzer = AXMLAnalyzer::new(Cursor::new(manifest_data));
            if let Ok(manifest) = axml_analyzer.analyze_manifest() {
                project_data.manifest = Some(manifest);
            }
        }

        // Analyze resources.arsc
        if let Ok(mut resources_file) = archive.by_name("resources.arsc") {
            let mut resources_data = Vec::new();
            resources_file.read_to_end(&mut resources_data)?;
            
            let mut arsc_analyzer = crate::android_analyzer::arsc_analyzer::ARSCAnalyzer::new(Cursor::new(resources_data));
            if let Ok(resource_table) = arsc_analyzer.analyze() {
                project_data.resource_table = Some(resource_table);
            }
        }

        // Store the archive for later use
        self.zip_archive = Some(archive);
        
        // Update project data
        project_data.apk_path = self.apk_path.clone();
        project_data.dex_files = dex_files;

        Ok(project_data)
    }

    /// Get the content of a specific file from the APK
    pub fn get_file_content(&mut self, file_path: &str) -> Result<Vec<u8>> {
        if let Some(ref mut archive) = self.zip_archive {
            let mut file = archive.by_name(file_path)
                .map_err(|e| AndroidAnalyzeError::InvalidApkFile(format!("File not found: {}", e)))?;
            
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;
            Ok(content)
        } else {
            Err(AndroidAnalyzeError::InvalidApkFile("APK not opened".to_string()))
        }
    }

    /// List all files in the APK
    pub fn list_files(&mut self) -> Result<Vec<String>> {
        if let Some(ref mut archive) = self.zip_archive {
            let mut files = Vec::new();
            for i in 0..archive.len() {
                let file = archive.by_index(i)?;
                files.push(file.name().to_string());
            }
            Ok(files)
        } else {
            Err(AndroidAnalyzeError::InvalidApkFile("APK not opened".to_string()))
        }
    }
}

/// Content types for different file types in APK
#[derive(Debug, Clone)]
pub enum ContentType {
    Code,
    Image,
    Binary,
    Xml,
}

/// File node representation for APK contents
#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: String,
    pub content_type: ContentType,
    pub size: u64,
    pub payload: Option<Vec<u8>>,
}
