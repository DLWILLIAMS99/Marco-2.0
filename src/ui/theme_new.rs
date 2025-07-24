//! Marco 2.0 Theme System
//! 
//! Consistent visual styling and theming for the IDE.
//! Updated for WGPU rendering system

/// RGBA color representation for WGPU
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }
    
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// Marco 2.0 visual theme configuration
#[derive(Debug, Clone)]
pub struct Marco2Theme {
    // Color palette
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub background_color: Color,
    pub surface_color: Color,
    pub text_color: Color,
    pub secondary_text_color: Color,
    pub error_color: Color,
    pub warning_color: Color,
    pub success_color: Color,
    
    // Node colors
    pub node_add_color: Color,
    pub node_multiply_color: Color,
    pub node_constant_color: Color,
    pub node_logic_color: Color,
    pub node_conditional_color: Color,
    pub node_button_color: Color,
    pub node_slider_color: Color,
    
    // UI element styling
    pub border_color: Color,
    pub hover_color: Color,
    pub selected_color: Color,
    pub button_background: Color,
    pub card_background: Color,
    pub heading_color: Color,
    
    // Layout and sizing
    pub window_rounding: f32,
    pub button_rounding: f32,
    pub card_rounding: f32,
    pub card_padding: f32,
    pub button_padding: f32,
    
    // Typography
    pub heading_font_size: f32,
    pub subheading_font_size: f32,
    pub body_font_size: f32,
    pub small_font_size: f32,
    
    // Animation
    pub animation_speed: f32,
    pub hover_animation_speed: f32,
    
    // Grid and canvas
    pub grid_color: Color,
    pub grid_size: f32,
    pub canvas_background: Color,
}

impl Default for Marco2Theme {
    fn default() -> Self {
        Self {
            // Color palette
            primary_color: Color::rgb(100, 150, 255),
            secondary_color: Color::rgb(150, 100, 255),
            accent_color: Color::rgb(255, 150, 100),
            background_color: Color::rgb(25, 25, 30),
            surface_color: Color::rgb(35, 35, 40),
            text_color: Color::rgb(220, 220, 220),
            secondary_text_color: Color::rgb(160, 160, 160),
            error_color: Color::rgb(255, 100, 100),
            warning_color: Color::rgb(255, 200, 100),
            success_color: Color::rgb(100, 255, 150),
            
            // Node colors
            node_add_color: Color::rgb(100, 180, 100),
            node_multiply_color: Color::rgb(180, 100, 100),
            node_constant_color: Color::rgb(100, 100, 180),
            node_logic_color: Color::rgb(180, 180, 100),
            node_conditional_color: Color::rgb(180, 100, 180),
            node_button_color: Color::rgb(120, 160, 200),
            node_slider_color: Color::rgb(200, 160, 120),
            
            // UI element styling
            border_color: Color::rgb(60, 60, 70),
            hover_color: Color::rgb(45, 45, 55),
            selected_color: Color::rgb(100, 150, 255),
            button_background: Color::rgb(50, 50, 60),
            card_background: Color::rgb(30, 30, 35),
            heading_color: Color::rgb(255, 255, 255),
            
            // Layout and sizing
            window_rounding: 8.0,
            button_rounding: 4.0,
            card_rounding: 6.0,
            card_padding: 12.0,
            button_padding: 8.0,
            
            // Typography
            heading_font_size: 18.0,
            subheading_font_size: 16.0,
            body_font_size: 14.0,
            small_font_size: 12.0,
            
            // Animation
            animation_speed: 0.15,
            hover_animation_speed: 0.1,
            
            // Grid and canvas
            grid_color: Color::rgb(40, 40, 50),
            grid_size: 20.0,
            canvas_background: Color::rgb(20, 20, 25),
        }
    }
}

impl Marco2Theme {
    /// Create a light theme variant
    pub fn light() -> Self {
        Self {
            background_color: Color::rgb(250, 250, 255),
            surface_color: Color::rgb(240, 240, 245),
            text_color: Color::rgb(30, 30, 30),
            secondary_text_color: Color::rgb(90, 90, 90),
            border_color: Color::rgb(200, 200, 210),
            hover_color: Color::rgb(230, 230, 240),
            button_background: Color::rgb(220, 220, 230),
            card_background: Color::rgb(245, 245, 250),
            heading_color: Color::rgb(0, 0, 0),
            grid_color: Color::rgb(220, 220, 230),
            canvas_background: Color::rgb(245, 245, 250),
            ..Self::default()
        }
    }
    
    /// Create a high contrast theme variant
    pub fn high_contrast() -> Self {
        Self {
            background_color: Color::rgb(0, 0, 0),
            surface_color: Color::rgb(20, 20, 20),
            text_color: Color::rgb(255, 255, 255),
            secondary_text_color: Color::rgb(200, 200, 200),
            border_color: Color::rgb(100, 100, 100),
            hover_color: Color::rgb(40, 40, 40),
            button_background: Color::rgb(60, 60, 60),
            card_background: Color::rgb(30, 30, 30),
            grid_color: Color::rgb(80, 80, 80),
            canvas_background: Color::rgb(10, 10, 10),
            ..Self::default()
        }
    }
    
    /// Get color for specific node type
    pub fn node_color(&self, node_type: &str) -> Color {
        match node_type {
            "add" => self.node_add_color,
            "multiply" => self.node_multiply_color,
            "constant" => self.node_constant_color,
            "branch" | "conditional" => self.node_conditional_color,
            "button" => self.node_button_color,
            "slider" => self.node_slider_color,
            _ => self.node_logic_color,
        }
    }
    
    /// Apply hover effect to a color
    pub fn hover_color(&self, base_color: Color) -> Color {
        Color {
            r: (base_color.r * 1.1).min(1.0),
            g: (base_color.g * 1.1).min(1.0),
            b: (base_color.b * 1.1).min(1.0),
            a: base_color.a,
        }
    }
    
    /// Apply selection effect to a color
    pub fn selected_color(&self, base_color: Color) -> Color {
        Color {
            r: (base_color.r + self.selected_color.r) * 0.5,
            g: (base_color.g + self.selected_color.g) * 0.5,
            b: (base_color.b + self.selected_color.b) * 0.5,
            a: base_color.a,
        }
    }
}
