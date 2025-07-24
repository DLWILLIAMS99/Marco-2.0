//! Menu system for Marco 2.0
use crate::ui::theme::Marco2Theme;

#[derive(Debug, Clone)]
pub struct MenuBar {
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
}

impl MenuItem {
    pub fn new(label: &str, action: MenuAction) -> Self {
        Self {
            label: label.to_string(),
            action,
        }
    }

    pub fn separator() -> Self {
        Self {
            label: "---".to_string(),
            action: MenuAction::Custom("separator".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    NewProject,
    OpenProject,
    SaveProject,
    SaveAs,
    Exit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Preferences,
    ShowErrorPanel,
    ToggleNodeLibrary,
    TogglePropertiesPanel,
    ToggleErrorPanel,
    ToggleDebugTools,
    RunTests,
    ClearErrors,
    ExportProject,
    Custom(String),
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            items: vec![
                MenuItem { label: "File".into(), action: MenuAction::Custom("file_menu".into()) },
                MenuItem { label: "Edit".into(), action: MenuAction::Custom("edit_menu".into()) },
                MenuItem { label: "View".into(), action: MenuAction::Custom("view_menu".into()) },
                MenuItem { label: "Project".into(), action: MenuAction::Custom("project_menu".into()) },
                MenuItem { label: "Help".into(), action: MenuAction::Custom("help_menu".into()) },
            ],
        }
    }
    
    pub fn add_menu(&mut self, label: String, items: Vec<MenuItem>) {
        // For now, just add the main menu item
        // In a full implementation, this would handle sub-menus
        self.items.push(MenuItem {
            label,
            action: MenuAction::Custom("custom_menu".to_string()),
        });
    }
    
    pub fn trigger(&self, action: &MenuAction) {
        // Placeholder for menu action handling
        println!("Menu action triggered: {:?}", action);
    }
    pub fn open_menu(&self, theme: &Marco2Theme) {
        println!("Opening menu with transition speed: {}s", theme.animation_speed);
        // Simulate fade/slide animation
    }
    pub fn close_menu(&self, theme: &Marco2Theme) {
        println!("Closing menu with transition speed: {}s", theme.animation_speed);
        // Simulate fade/slide animation
    }
}
