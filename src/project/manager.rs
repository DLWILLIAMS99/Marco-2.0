//! Project persistence and file management
use crate::core::types::error::MarcoError;
// Template imports temporarily disabled for build compatibility
// use crate::project::template::TemplateDefinition;
use rfd::FileDialog;
use serde_json;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, warn};

pub struct ProjectManager {
    current_project_path: Option<PathBuf>,
    autosave_enabled: bool,
    recovery_path: PathBuf,
}

impl ProjectManager {
    pub fn new() -> Self {
        let recovery_path = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("marco2")
            .join("recovery");
        
        // Ensure recovery directory exists
        if let Err(e) = fs::create_dir_all(&recovery_path) {
            warn!("Failed to create recovery directory: {}", e);
        }
        
        Self {
            current_project_path: None,
            autosave_enabled: true,
            recovery_path,
        }
    }
    
    // Project creation temporarily simplified for build compatibility
    pub fn new_project(&mut self, template_name: &str) -> Result<(), MarcoError> {
        info!("Creating new project from template: {}", template_name);
        
        let folder = FileDialog::new()
            .set_title("Choose Project Location")
            .pick_folder()
            .ok_or_else(|| MarcoError::Persistence("No folder selected".to_string()))?;
        
        let project_path = folder.join(template_name);
        fs::create_dir_all(&project_path)
            .map_err(|e| MarcoError::Persistence(format!("Failed to create project directory: {}", e)))?;
        
        // Create project structure
        self.create_project_structure(&project_path, template_name)?;
        
        self.current_project_path = Some(project_path.clone());
        info!("Project created at: {:?}", project_path);
        
        Ok(())
    }
    
    pub fn open_project(&mut self) -> Result<(), MarcoError> {
        let file_path = FileDialog::new()
            .set_title("Open Marco 2.0 Project")
            .add_filter("Marco 2.0 Project", &["marco2"])
            .pick_file()
            .ok_or_else(|| MarcoError::Persistence("No file selected".to_string()))?;
        
        self.load_project(&file_path)?;
        self.current_project_path = Some(file_path.parent().unwrap().to_path_buf());
        
        Ok(())
    }
    
    pub fn save_project(&self) -> Result<(), MarcoError> {
        if let Some(ref path) = self.current_project_path {
            self.save_to_path(path)?;
            info!("Project saved to: {:?}", path);
        } else {
            return self.save_project_as();
        }
        Ok(())
    }
    
    pub fn save_project_as(&self) -> Result<(), MarcoError> {
        let file_path = FileDialog::new()
            .set_title("Save Marco 2.0 Project")
            .add_filter("Marco 2.0 Project", &["marco2"])
            .save_file()
            .ok_or_else(|| MarcoError::Persistence("No file selected".to_string()))?;
        
        self.save_to_path(&file_path)?;
        info!("Project saved as: {:?}", file_path);
        
        Ok(())
    }
    
    fn create_project_structure(&self, project_path: &Path, template_name: &str) -> Result<(), MarcoError> {
        // Create main project file  
        let project_file = project_path.join("project.marco2");
        let project_data = format!(r#"{{
    "name": "{}",
    "version": "1.0.0",
    "created": "{}",
    "template": "{}"
}}"#, 
            project_path.file_name().unwrap().to_string_lossy(),
            chrono::Utc::now().to_rfc3339(),
            template_name
        );
        
        fs::write(&project_file, project_data)
            .map_err(|e| MarcoError::Persistence(format!("Failed to write project file: {}", e)))?;
        
        // Create assets directory
        fs::create_dir_all(project_path.join("assets"))
            .map_err(|e| MarcoError::Persistence(format!("Failed to create assets directory: {}", e)))?;
        
        // Create nodes directory for custom nodes
        fs::create_dir_all(project_path.join("nodes"))
            .map_err(|e| MarcoError::Persistence(format!("Failed to create nodes directory: {}", e)))?;
        
        // Create exports directory for output
        fs::create_dir_all(project_path.join("exports"))
            .map_err(|e| MarcoError::Persistence(format!("Failed to create exports directory: {}", e)))?;
        
        Ok(())
    }
    
    fn load_project(&self, file_path: &Path) -> Result<(), MarcoError> {
        info!("Loading project from: {:?}", file_path);
        
        let contents = fs::read_to_string(file_path)
            .map_err(|e| MarcoError::Persistence(format!("Failed to read project file: {}", e)))?;
        
        let _project_data: serde_json::Value = serde_json::from_str(&contents)
            .map_err(|e| MarcoError::Persistence(format!("Failed to parse project file: {}", e)))?;
        
        // TODO: Load project data into registry and UI state
        info!("Project loaded successfully");
        
        Ok(())
    }
    
    fn save_to_path(&self, file_path: &Path) -> Result<(), MarcoError> {
        // TODO: Serialize current project state
        let project_data = serde_json::json!({
            "version": "2.0",
            "created": chrono::Utc::now(),
            "registry": {},
            "graph": {},
            "ui_state": {}
        });
        
        let contents = serde_json::to_string_pretty(&project_data)
            .map_err(|e| MarcoError::Persistence(format!("Failed to serialize project: {}", e)))?;
        
        fs::write(file_path, contents)
            .map_err(|e| MarcoError::Persistence(format!("Failed to write project file: {}", e)))?;
        
        Ok(())
    }
    
    pub fn enable_autosave(&mut self, enabled: bool) {
        self.autosave_enabled = enabled;
        info!("Autosave {}", if enabled { "enabled" } else { "disabled" });
    }
    
    pub fn create_recovery_snapshot(&self) -> Result<(), MarcoError> {
        if !self.autosave_enabled {
            return Ok(());
        }
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let recovery_file = self.recovery_path.join(format!("recovery_{}.marco2", timestamp));
        
        self.save_to_path(&recovery_file)?;
        info!("Recovery snapshot created: {:?}", recovery_file);
        
        // Clean up old recovery files (keep last 10)
        self.cleanup_old_recovery_files()?;
        
        Ok(())
    }
    
    fn cleanup_old_recovery_files(&self) -> Result<(), MarcoError> {
        let mut recovery_files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.recovery_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("recovery_") && name.ends_with(".marco2") {
                        recovery_files.push(entry.path());
                    }
                }
            }
        }
        
        // Sort by modification time (newest first)
        recovery_files.sort_by(|a, b| {
            let a_time = fs::metadata(a).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let b_time = fs::metadata(b).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            b_time.cmp(&a_time)
        });
        
        // Remove files beyond the first 10
        for file_to_remove in recovery_files.iter().skip(10) {
            if let Err(e) = fs::remove_file(file_to_remove) {
                warn!("Failed to remove old recovery file {:?}: {}", file_to_remove, e);
            }
        }
        
        Ok(())
    }
    
    pub fn list_recovery_files(&self) -> Vec<PathBuf> {
        let mut recovery_files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.recovery_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("recovery_") && name.ends_with(".marco2") {
                        recovery_files.push(entry.path());
                    }
                }
            }
        }
        
        recovery_files.sort_by(|a, b| {
            let a_time = fs::metadata(a).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let b_time = fs::metadata(b).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            b_time.cmp(&a_time)
        });
        
        recovery_files
    }
    
    pub fn current_project_path(&self) -> Option<&Path> {
        self.current_project_path.as_deref()
    }
}
