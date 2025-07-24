//! ErrorPanel: UI component for displaying global errors
use crate::devtools::error_log::ErrorLog;
use crate::core::types::error::MarcoError;
use crate::ui::theme::Marco2Theme;

#[derive(Debug, Clone)]
pub struct ErrorPanel {
    pub error_log: ErrorLog,
}

impl ErrorPanel {
    pub fn new(error_log: ErrorLog) -> Self {
        Self { error_log }
    }

    /// Render the error panel UI
    pub fn render(&self) {
        let errors = self.error_log.get_all();
        if errors.is_empty() {
            println!("No errors.");
        } else {
            println!("--- Error Log ---");
            for (i, err) in errors.iter().enumerate() {
                println!("{i}: {err}");
            }
        }
    }

    /// Render the error panel UI with theme colors and animation
    pub fn render_with_theme(&self, theme: &Marco2Theme) {
        let errors = self.error_log.get_all();
        if errors.is_empty() {
            println!("No errors.");
        } else {
            println!("--- Error Log (animated, color: {:?}, speed: {}s) ---", theme.error_color, theme.animation_speed);
            for (i, err) in errors.iter().enumerate() {
                println!("{i}: {err}");
            }
        }
    }

    /// Clear all errors from the log
    pub fn clear_errors(&self) {
        self.error_log.clear();
    }
}
