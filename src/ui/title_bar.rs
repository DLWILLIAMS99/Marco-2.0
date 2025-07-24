//! Title bar for Marco 2.0
use crate::ui::theme::Marco2Theme;

#[derive(Debug, Clone)]
pub struct TitleBar {
    pub title: String,
    pub menu_bar: crate::ui::menu::MenuBar,
}

impl TitleBar {
    pub fn new(title: &str, menu_bar: crate::ui::menu::MenuBar) -> Self {
        Self { title: title.into(), menu_bar }
    }
    pub fn render(&self) {
        println!("=== {} ===", self.title);
        // Render menu bar
        for item in &self.menu_bar.items {
            print!("[{}] ", item.label);
        }
        println!("");
    }
    pub fn render_with_theme(&self, theme: &Marco2Theme, active_menu: Option<&str>) {
        println!("=== {} ===", self.title);
        for item in &self.menu_bar.items {
            if Some(item.label.as_str()) == active_menu {
                print!("[{}*] (accent: {:?}) ", item.label, theme.accent_color);
            } else {
                print!("[{}] ", item.label);
            }
        }
        println!("");
        println!("(Title bar transition speed: {}s)", theme.animation_speed);
    }
}
