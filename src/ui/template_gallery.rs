//! Template Gallery and Management System
//! Provides a comprehensive gallery for browsing, previewing, and managing templates
use crate::core::types::error::MarcoError;
use crate::ui::theme::Marco2Theme;
use crate::ui::template_creator::{TemplateCategory, ProjectTemplate};
use glam::Vec2;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, warn, error};

#[derive(Debug, Clone)]
pub struct TemplateGallery {
    pub visible: bool,
    pub current_view: GalleryView,
    pub selected_category: TemplateCategory,
    pub search_query: String,
    pub templates: HashMap<String, TemplateEntry>,
    pub selected_template: Option<String>,
    pub preview_mode: PreviewMode,
    pub sort_order: SortOrder,
    pub filter_options: FilterOptions,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GalleryView {
    Grid,
    List,
    Preview,
    Details,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewMode {
    Thumbnail,
    Interactive,
    Code,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Name,
    DateCreated,
    DateModified,
    Category,
    Popularity,
    Rating,
}

#[derive(Debug, Clone)]
pub struct FilterOptions {
    pub show_builtin: bool,
    pub show_custom: bool,
    pub show_community: bool,
    pub min_rating: f32,
    pub complexity_level: Option<ComplexityLevel>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexityLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone)]
pub struct TemplateEntry {
    pub template: ProjectTemplate,
    pub metadata: TemplateMetadata,
    pub preview_data: PreviewData,
    pub statistics: TemplateStatistics,
}

#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    pub id: String,
    pub author: String,
    pub version: String,
    pub license: String,
    pub created_date: String,
    pub modified_date: String,
    pub tags: Vec<String>,
    pub complexity: ComplexityLevel,
    pub rating: f32,
    pub download_count: u32,
    pub featured: bool,
}

#[derive(Debug, Clone)]
pub struct PreviewData {
    pub thumbnail_path: Option<PathBuf>,
    pub screenshot_paths: Vec<PathBuf>,
    pub demo_project_path: Option<PathBuf>,
    pub preview_html: Option<String>,
    pub code_samples: Vec<CodeSample>,
}

#[derive(Debug, Clone)]
pub struct CodeSample {
    pub filename: String,
    pub language: String,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TemplateStatistics {
    pub usage_count: u32,
    pub success_rate: f32,
    pub average_completion_time: f32,
    pub user_ratings: Vec<f32>,
    pub feedback_comments: Vec<String>,
}

impl TemplateGallery {
    pub fn new() -> Self {
        let mut gallery = Self {
            visible: false,
            current_view: GalleryView::Grid,
            selected_category: TemplateCategory::All,
            search_query: String::new(),
            templates: HashMap::new(),
            selected_template: None,
            preview_mode: PreviewMode::Thumbnail,
            sort_order: SortOrder::Name,
            filter_options: FilterOptions {
                show_builtin: true,
                show_custom: true,
                show_community: false,
                min_rating: 0.0,
                complexity_level: None,
                tags: Vec::new(),
            },
        };
        
        gallery.load_builtin_templates();
        gallery
    }
    
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
        info!("Template gallery: {}", if self.visible { "shown" } else { "hidden" });
    }
    
    pub fn set_view(&mut self, view: GalleryView) {
        self.current_view = view;
        info!("Template gallery view: {:?}", self.current_view);
    }
    
    pub fn set_category(&mut self, category: TemplateCategory) {
        self.selected_category = category;
        self.selected_template = None; // Clear selection when changing category
        info!("Template gallery category: {:?}", self.selected_category);
    }
    
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query.to_lowercase();
        self.selected_template = None; // Clear selection when searching
        info!("Template gallery search: '{}'", self.search_query);
    }
    
    pub fn select_template(&mut self, template_id: String) {
        self.selected_template = Some(template_id.clone());
        info!("Selected template: {}", template_id);
    }
    
    pub fn set_preview_mode(&mut self, mode: PreviewMode) {
        self.preview_mode = mode;
        info!("Preview mode: {:?}", self.preview_mode);
    }
    
    pub fn set_sort_order(&mut self, order: SortOrder) {
        self.sort_order = order;
        info!("Sort order: {:?}", self.sort_order);
    }
    
    pub fn get_filtered_templates(&self) -> Vec<&TemplateEntry> {
        let mut templates: Vec<&TemplateEntry> = self.templates.values()
            .filter(|entry| self.matches_filter(entry))
            .collect();
        
        // Sort templates
        match self.sort_order {
            SortOrder::Name => templates.sort_by(|a, b| a.template.name.cmp(&b.template.name)),
            SortOrder::DateCreated => templates.sort_by(|a, b| a.metadata.created_date.cmp(&b.metadata.created_date)),
            SortOrder::DateModified => templates.sort_by(|a, b| b.metadata.modified_date.cmp(&a.metadata.modified_date)),
            SortOrder::Category => templates.sort_by(|a, b| a.template.category.to_string().cmp(&b.template.category.to_string())),
            SortOrder::Popularity => templates.sort_by(|a, b| b.metadata.download_count.cmp(&a.metadata.download_count)),
            SortOrder::Rating => templates.sort_by(|a, b| b.metadata.rating.partial_cmp(&a.metadata.rating).unwrap_or(std::cmp::Ordering::Equal)),
        }
        
        templates
    }
    
    pub fn get_selected_template(&self) -> Option<&TemplateEntry> {
        self.selected_template.as_ref().and_then(|id| self.templates.get(id))
    }
    
    pub fn create_project_from_template(&self, template_id: &str) -> Result<ProjectTemplate, MarcoError> {
        let entry = self.templates.get(template_id)
            .ok_or_else(|| MarcoError::TemplateNotFound(template_id.to_string()))?;
        
        info!("Creating project from template: {}", template_id);
        Ok(entry.template.clone())
    }
    
    pub fn add_custom_template(&mut self, template: ProjectTemplate) -> Result<(), MarcoError> {
        let entry = TemplateEntry {
            metadata: TemplateMetadata {
                id: template.name.clone(),
                author: "User".to_string(),
                version: "1.0.0".to_string(),
                license: "Custom".to_string(),
                created_date: chrono::Utc::now().to_rfc3339(),
                modified_date: chrono::Utc::now().to_rfc3339(),
                tags: Vec::new(),
                complexity: ComplexityLevel::Beginner,
                rating: 0.0,
                download_count: 0,
                featured: false,
            },
            preview_data: PreviewData {
                thumbnail_path: None,
                screenshot_paths: Vec::new(),
                demo_project_path: None,
                preview_html: None,
                code_samples: Vec::new(),
            },
            statistics: TemplateStatistics {
                usage_count: 0,
                success_rate: 0.0,
                average_completion_time: 0.0,
                user_ratings: Vec::new(),
                feedback_comments: Vec::new(),
            },
            template,
        };
        
        let template_id = entry.metadata.id.clone();
        self.templates.insert(template_id.clone(), entry);
        info!("Added custom template: {}", template_id);
        Ok(())
    }
    
    pub fn remove_template(&mut self, template_id: &str) -> Result<(), MarcoError> {
        // Don't allow removal of builtin templates
        if let Some(entry) = self.templates.get(template_id) {
            if entry.metadata.author == "Marco2" {
                return Err(MarcoError::InvalidOperation("Cannot remove builtin templates".to_string()));
            }
        }
        
        self.templates.remove(template_id)
            .ok_or_else(|| MarcoError::TemplateNotFound(template_id.to_string()))?;
        
        if self.selected_template.as_ref() == Some(&template_id.to_string()) {
            self.selected_template = None;
        }
        
        info!("Removed template: {}", template_id);
        Ok(())
    }
    
    pub fn rate_template(&mut self, template_id: &str, rating: f32) -> Result<(), MarcoError> {
        let entry = self.templates.get_mut(template_id)
            .ok_or_else(|| MarcoError::TemplateNotFound(template_id.to_string()))?;
        
        entry.statistics.user_ratings.push(rating);
        entry.metadata.rating = entry.statistics.user_ratings.iter().sum::<f32>() / entry.statistics.user_ratings.len() as f32;
        
        info!("Rated template '{}' with {:.1} stars (avg: {:.1})", template_id, rating, entry.metadata.rating);
        Ok(())
    }
    
    pub fn render(&self, theme: &Marco2Theme) {
        if !self.visible {
            return;
        }
        
        info!("Rendering template gallery - View: {:?}, Category: {:?}", self.current_view, self.selected_category);
        
        match self.current_view {
            GalleryView::Grid => self.render_grid_view(theme),
            GalleryView::List => self.render_list_view(theme),
            GalleryView::Preview => self.render_preview_view(theme),
            GalleryView::Details => self.render_details_view(theme),
        }
    }
    
    fn render_grid_view(&self, theme: &Marco2Theme) {
        let templates = self.get_filtered_templates();
        info!("Grid view: {} templates", templates.len());
        
        for template in templates {
            info!("Template: {} - {}", template.template.name, template.template.description);
            if template.metadata.featured {
                info!("  ⭐ Featured template");
            }
            info!("  Rating: {:.1}/5.0 ({} downloads)", template.metadata.rating, template.metadata.download_count);
        }
    }
    
    fn render_list_view(&self, theme: &Marco2Theme) {
        let templates = self.get_filtered_templates();
        info!("List view: {} templates", templates.len());
        
        for template in templates {
            info!("📄 {} by {} ({})", template.template.name, template.metadata.author, template.metadata.version);
            info!("   {}", template.template.description);
            info!("   Tags: {:?} | Complexity: {:?}", template.metadata.tags, template.metadata.complexity);
        }
    }
    
    fn render_preview_view(&self, theme: &Marco2Theme) {
        if let Some(template) = self.get_selected_template() {
            info!("Preview: {} - {}", template.template.name, template.template.description);
            
            match self.preview_mode {
                PreviewMode::Thumbnail => {
                    info!("Showing thumbnail preview");
                    if let Some(thumbnail) = &template.preview_data.thumbnail_path {
                        info!("Thumbnail: {:?}", thumbnail);
                    }
                },
                PreviewMode::Interactive => {
                    info!("Showing interactive preview");
                    if let Some(demo) = &template.preview_data.demo_project_path {
                        info!("Demo project: {:?}", demo);
                    }
                },
                PreviewMode::Code => {
                    info!("Showing code preview");
                    for sample in &template.preview_data.code_samples {
                        info!("Code sample: {} ({})", sample.filename, sample.language);
                    }
                },
                PreviewMode::Both => {
                    info!("Showing combined preview");
                },
            }
        } else {
            info!("No template selected for preview");
        }
    }
    
    fn render_details_view(&self, theme: &Marco2Theme) {
        if let Some(template) = self.get_selected_template() {
            info!("Template Details: {}", template.template.name);
            info!("Author: {} | Version: {}", template.metadata.author, template.metadata.version);
            info!("Created: {} | Modified: {}", template.metadata.created_date, template.metadata.modified_date);
            info!("Rating: {:.1}/5.0 ({} ratings)", template.metadata.rating, template.statistics.user_ratings.len());
            info!("Downloads: {} | Usage: {}", template.metadata.download_count, template.statistics.usage_count);
            info!("Description: {}", template.template.description);
            info!("Tags: {:?}", template.metadata.tags);
            info!("Complexity: {:?}", template.metadata.complexity);
            
            if !template.template.gui_elements.is_empty() {
                info!("GUI Elements: {} items", template.template.gui_elements.len());
                for element in &template.template.gui_elements {
                    info!("  - {:?} ({:?})", element.id, element.element_type);
                }
            }
            
            // Note: code_files field removed from ProjectTemplate, commenting out
            // if !template.template.code_files.is_empty() {
            //     info!("Code Files: {} files", template.template.code_files.len());
            //     for (filename, _) in &template.template.code_files {
            //         info!("  - {}", filename);
            //     }
            // }
        } else {
            info!("No template selected for details view");
        }
    }
    
    fn matches_filter(&self, entry: &TemplateEntry) -> bool {
        // Category filter
        if self.selected_category != TemplateCategory::All && entry.template.category != self.selected_category {
            return false;
        }
        
        // Author type filter
        let is_builtin = entry.metadata.author == "Marco2";
        let is_custom = entry.metadata.author == "User";
        let is_community = !is_builtin && !is_custom;
        
        if !self.filter_options.show_builtin && is_builtin {
            return false;
        }
        if !self.filter_options.show_custom && is_custom {
            return false;
        }
        if !self.filter_options.show_community && is_community {
            return false;
        }
        
        // Rating filter
        if entry.metadata.rating < self.filter_options.min_rating {
            return false;
        }
        
        // Complexity filter
        if let Some(ref complexity) = self.filter_options.complexity_level {
            if &entry.metadata.complexity != complexity {
                return false;
            }
        }
        
        // Tag filter
        if !self.filter_options.tags.is_empty() {
            let has_required_tags = self.filter_options.tags.iter()
                .all(|tag| entry.metadata.tags.contains(tag));
            if !has_required_tags {
                return false;
            }
        }
        
        // Search filter
        if !self.search_query.is_empty() {
            let query = &self.search_query;
            if !entry.template.name.to_lowercase().contains(query) &&
               !entry.template.description.to_lowercase().contains(query) &&
               !entry.metadata.author.to_lowercase().contains(query) &&
               !entry.metadata.tags.iter().any(|tag| tag.to_lowercase().contains(query)) {
                return false;
            }
        }
        
        true
    }
    
    fn load_builtin_templates(&mut self) {
        // For now, create minimal template entries for the gallery
        // These would normally be loaded from the actual template system
        
        let calculator_entry = TemplateEntry {
            template: ProjectTemplate {
                id: uuid::Uuid::new_v4(),
                name: "Calculator App".to_string(),
                description: "A simple calculator with basic arithmetic operations".to_string(),
                category: TemplateCategory::Application,
                preview_image: None,
                tags: vec!["calculator".to_string(), "math".to_string()],
                difficulty: crate::ui::template_creator::TemplateDifficulty::Beginner,
                initial_nodes: Vec::new(),
                initial_connections: Vec::new(),
                initial_properties: HashMap::new(),
                canvas_settings: crate::ui::template_creator::CanvasSettings {
                    background_color: [0.1, 0.1, 0.1, 1.0],
                    grid_size: 20.0,
                    snap_to_grid: true,
                    show_grid: true,
                    canvas_size: Vec2::new(400.0, 500.0),
                },
                gui_elements: Vec::new(), // Simplified for now
                gui_layout: crate::ui::template_creator::GuiLayout {
                    layout_type: crate::ui::template_creator::LayoutType::Grid { rows: 5, cols: 4 },
                    constraints: crate::ui::template_creator::LayoutConstraints {
                        min_width: Some(300.0),
                        max_width: Some(400.0),
                        min_height: Some(400.0),
                        max_height: Some(500.0),
                        aspect_ratio: Some(1.0),
                    },
                },
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                version: "1.0.0".to_string(),
                author: "Marco2".to_string(),
            },
            metadata: TemplateMetadata {
                id: "calculator_app".to_string(),
                author: "Marco2".to_string(),
                version: "1.0.0".to_string(),
                license: "MIT".to_string(),
                created_date: "2024-01-01T00:00:00Z".to_string(),
                modified_date: "2024-01-01T00:00:00Z".to_string(),
                tags: vec!["calculator".to_string(), "math".to_string(), "beginner".to_string()],
                complexity: ComplexityLevel::Beginner,
                rating: 4.5,
                download_count: 1250,
                featured: true,
            },
            preview_data: PreviewData {
                thumbnail_path: Some(PathBuf::from("assets/templates/calculator_thumb.png")),
                screenshot_paths: vec![PathBuf::from("assets/templates/calculator_screen1.png")],
                demo_project_path: Some(PathBuf::from("templates/calculator_demo.marco")),
                preview_html: Some("<div>Calculator preview</div>".to_string()),
                code_samples: vec![
                    CodeSample {
                        filename: "main.rs".to_string(),
                        language: "rust".to_string(),
                        code: "fn input_digit(&mut self, digit: u8) {\n    // Add digit to display\n}".to_string(),
                        description: "Main digit input handler".to_string(),
                    },
                ],
            },
            statistics: TemplateStatistics {
                usage_count: 850,
                success_rate: 0.92,
                average_completion_time: 15.5,
                user_ratings: vec![4.0, 5.0, 4.0, 5.0, 4.0],
                feedback_comments: vec![
                    "Great starting template!".to_string(),
                    "Easy to understand and extend".to_string(),
                ],
            },
        };
        
        self.templates.insert("calculator_app".to_string(), calculator_entry);
        
        // Dashboard Template
        let dashboard_entry = TemplateEntry {
            template: ProjectTemplate {
                id: uuid::Uuid::new_v4(),
                name: "Analytics Dashboard".to_string(),
                description: "A data visualization dashboard with charts and metrics".to_string(),
                category: TemplateCategory::Dashboard,
                preview_image: None,
                tags: vec!["dashboard".to_string(), "analytics".to_string()],
                difficulty: crate::ui::template_creator::TemplateDifficulty::Intermediate,
                initial_nodes: Vec::new(),
                initial_connections: Vec::new(),
                initial_properties: HashMap::new(),
                canvas_settings: crate::ui::template_creator::CanvasSettings {
                    background_color: [0.05, 0.05, 0.1, 1.0],
                    grid_size: 25.0,
                    snap_to_grid: true,
                    show_grid: false,
                    canvas_size: Vec2::new(1200.0, 800.0),
                },
                gui_elements: Vec::new(), // Simplified for now
                gui_layout: crate::ui::template_creator::GuiLayout {
                    layout_type: crate::ui::template_creator::LayoutType::Flex { 
                        direction: crate::ui::template_creator::FlexDirection::Column 
                    },
                    constraints: crate::ui::template_creator::LayoutConstraints {
                        min_width: Some(800.0),
                        max_width: None,
                        min_height: Some(600.0),
                        max_height: None,
                        aspect_ratio: None,
                    },
                },
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                version: "1.1.0".to_string(),
                author: "Marco2".to_string(),
            },
            metadata: TemplateMetadata {
                id: "analytics_dashboard".to_string(),
                author: "Marco2".to_string(),
                version: "1.1.0".to_string(),
                license: "MIT".to_string(),
                created_date: "2024-01-15T00:00:00Z".to_string(),
                modified_date: "2024-02-01T00:00:00Z".to_string(),
                tags: vec!["dashboard".to_string(), "analytics".to_string(), "charts".to_string(), "intermediate".to_string()],
                complexity: ComplexityLevel::Intermediate,
                rating: 4.2,
                download_count: 890,
                featured: true,
            },
            preview_data: PreviewData {
                thumbnail_path: Some(PathBuf::from("assets/templates/dashboard_thumb.png")),
                screenshot_paths: vec![
                    PathBuf::from("assets/templates/dashboard_screen1.png"),
                    PathBuf::from("assets/templates/dashboard_screen2.png"),
                ],
                demo_project_path: Some(PathBuf::from("templates/dashboard_demo.marco")),
                preview_html: Some("<div>Dashboard preview with charts</div>".to_string()),
                code_samples: vec![
                    CodeSample {
                        filename: "dashboard.rs".to_string(),
                        language: "rust".to_string(),
                        code: "async fn fetch_from_api(&mut self) -> Result<(), Box<dyn std::error::Error>> {\n    // API integration\n}".to_string(),
                        description: "Async data fetching from API".to_string(),
                    },
                ],
            },
            statistics: TemplateStatistics {
                usage_count: 542,
                success_rate: 0.88,
                average_completion_time: 32.8,
                user_ratings: vec![4.0, 4.0, 5.0, 4.0, 4.0, 3.0],
                feedback_comments: vec![
                    "Great for data visualization projects".to_string(),
                    "Could use more chart types".to_string(),
                    "Very professional looking".to_string(),
                ],
            },
        };
        
        self.templates.insert("analytics_dashboard".to_string(), dashboard_entry);
        
        info!("Loaded {} builtin templates", self.templates.len());
    }
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            show_builtin: true,
            show_custom: true,
            show_community: false,
            min_rating: 0.0,
            complexity_level: None,
            tags: Vec::new(),
        }
    }
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCategory::All => write!(f, "All"),
            TemplateCategory::Application => write!(f, "Application"),
            TemplateCategory::Game => write!(f, "Game"),
            TemplateCategory::Utility => write!(f, "Utility"),
            TemplateCategory::Dashboard => write!(f, "Dashboard"),
            TemplateCategory::Educational => write!(f, "Educational"),
            TemplateCategory::Experimental => write!(f, "Experimental"),
            TemplateCategory::DataProcessing => write!(f, "Data Processing"),
            TemplateCategory::GameLogic => write!(f, "Game Logic"),
            TemplateCategory::WebDevelopment => write!(f, "Web Development"),
            TemplateCategory::MachineLearning => write!(f, "Machine Learning"),
            TemplateCategory::Animation => write!(f, "Animation"),
            TemplateCategory::Audio => write!(f, "Audio"),
            TemplateCategory::Graphics => write!(f, "Graphics"),
            TemplateCategory::Simulation => write!(f, "Simulation"),
            TemplateCategory::Automation => write!(f, "Automation"),
            TemplateCategory::Form => write!(f, "Form"),
            TemplateCategory::Navigation => write!(f, "Navigation"),
            TemplateCategory::Visualization => write!(f, "Visualization"),
            TemplateCategory::UserInterface => write!(f, "User Interface"),
            TemplateCategory::Prototype => write!(f, "Prototype"),
            TemplateCategory::Advanced => write!(f, "Advanced"),
            TemplateCategory::Custom => write!(f, "Custom"),
        }
    }
}
