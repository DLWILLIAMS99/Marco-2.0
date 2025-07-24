//! Centralized event handling for Marco 2.0 UI
use crate::ui::theme::Marco2Theme;

#[derive(Debug, Clone)]
pub enum UIEvent {
    MouseClick { x: f32, y: f32, button: MouseButton },
    MouseMove { x: f32, y: f32 },
    KeyPress { key: String },
    MenuAction(crate::ui::menu::MenuAction),
    MenuItemClicked(String),
    Error(crate::core::types::error::MarcoError),
    ButtonClicked { id: String },
    SliderChanged { id: String, value: f32 },
    NodeDragged { node_id: String, position: (f32, f32) },
    ErrorOccurred { error: String },
    ContextMenuRequested { x: f32, y: f32 },
}

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub fn handle_event(event: UIEvent) {
    match event {
        UIEvent::MouseClick { x, y, button } => {
            println!("Mouse click at ({}, {}) with {:?}", x, y, button);
        }
        UIEvent::MouseMove { x, y } => {
            println!("Mouse move to ({}, {})", x, y);
        }
        UIEvent::KeyPress { key } => {
            println!("Key pressed: {}", key);
        }
        UIEvent::MenuAction(action) => {
            println!("Menu action: {:?}", action);
        }
        UIEvent::MenuItemClicked(item) => {
            println!("Menu item clicked: {}", item);
        }
        UIEvent::Error(err) => {
            println!("Error event: {}", err);
        }
        UIEvent::ButtonClicked { id } => {
            println!("Button clicked: {}", id);
        }
        UIEvent::SliderChanged { id, value } => {
            println!("Slider changed: {} = {}", id, value);
        }
        UIEvent::NodeDragged { node_id, position } => {
            println!("Node dragged: {} to {:?}", node_id, position);
        }
        UIEvent::ErrorOccurred { error } => {
            println!("Error occurred: {}", error);
        }
        UIEvent::ContextMenuRequested { x, y } => {
            println!("Context menu requested at ({}, {})", x, y);
        }
    }
}

pub fn handle_event_with_theme(event: UIEvent, theme: &Marco2Theme) {
    match event {
        UIEvent::MouseClick { x, y, button } => {
            println!("Mouse click at ({}, {}) with {:?} (highlight: {:?})", x, y, button, theme.accent_color);
        }
        UIEvent::MouseMove { x, y } => {
            println!("Mouse move to ({}, {}) (hover animation speed: {}s)", x, y, theme.animation_speed);
        }
        UIEvent::KeyPress { key } => {
            println!("Key pressed: {}", key);
        }
        UIEvent::MenuAction(action) => {
            println!("Menu action: {:?}", action);
        }
        UIEvent::MenuItemClicked(item) => {
            println!("Menu item clicked: {} (accent: {:?})", item, theme.accent_color);
        }
        UIEvent::Error(err) => {
            println!("Error event: {} (error color: {:?})", err, theme.error_color);
        }
        UIEvent::ButtonClicked { id } => {
            println!("Button clicked: {} (theme: accent {:?})", id, theme.accent_color);
        }
        UIEvent::SliderChanged { id, value } => {
            println!("Slider changed: {} = {} (theme: highlight {:?})", id, value, theme.accent_color);
        }
        UIEvent::NodeDragged { node_id, position } => {
            println!("Node dragged: {} to {:?} (drag color: {:?})", node_id, position, theme.accent_color);
        }
        UIEvent::ErrorOccurred { error } => {
            println!("Error occurred: {} (error color: {:?})", error, theme.error_color);
        }
        UIEvent::ContextMenuRequested { x, y } => {
            println!("Context menu requested at ({}, {}) (border: {:?})", x, y, theme.border_color);
        }
    }
}
