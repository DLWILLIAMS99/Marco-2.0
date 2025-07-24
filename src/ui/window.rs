//! Window component for Marco 2.0
use crate::ui::theme::Marco2Theme;

#[derive(Debug, Clone)]
pub struct Window {
    pub title_bar: crate::ui::title_bar::TitleBar,
}

impl Window {
    pub fn new(title_bar: crate::ui::title_bar::TitleBar) -> Self {
        Self { title_bar }
    }
    pub fn render(&self) {
        self.title_bar.render();
        // Placeholder for window content rendering
        println!("[Window content here]");
    }
    pub fn render_with_theme(&self, theme: &Marco2Theme) {
        println!("Rendering window with border radius: {} and shadow strength: {}", theme.border_radius, theme.shadow_strength);
        self.title_bar.render();
        println!("[Window content here]");
    }
}
