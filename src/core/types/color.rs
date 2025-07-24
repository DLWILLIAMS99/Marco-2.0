use serde::{Deserialize, Serialize};

/// RGBA color representation for Marco 2.0
/// 
/// Provides a standardized color type for use throughout the system.
/// All color values are stored as f32 in the range [0.0, 1.0].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorRGBA {
    /// Create a new RGBA color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create a new RGB color with full opacity
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Create a new color from 8-bit values
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Convert to 8-bit values
    pub fn to_u8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

    /// Create a transparent color
    pub fn transparent() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Create common colors
    pub fn white() -> Self { Self::rgb(1.0, 1.0, 1.0) }
    pub fn black() -> Self { Self::rgb(0.0, 0.0, 0.0) }
    pub fn red() -> Self { Self::rgb(1.0, 0.0, 0.0) }
    pub fn green() -> Self { Self::rgb(0.0, 1.0, 0.0) }
    pub fn blue() -> Self { Self::rgb(0.0, 0.0, 1.0) }
}

impl Default for ColorRGBA {
    fn default() -> Self {
        Self::transparent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = ColorRGBA::new(0.5, 0.6, 0.7, 0.8);
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.6);
        assert_eq!(color.b, 0.7);
        assert_eq!(color.a, 0.8);
    }

    #[test]
    fn test_color_clamping() {
        let color = ColorRGBA::new(-0.1, 1.5, 0.5, 2.0);
        assert_eq!(color.r, 0.0);
        assert_eq!(color.g, 1.0);
        assert_eq!(color.b, 0.5);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_u8_conversion() {
        let color = ColorRGBA::from_u8(128, 192, 64, 255);
        let (r, g, b, a) = color.to_u8();
        assert_eq!(r, 128);
        assert_eq!(g, 192);
        assert_eq!(b, 64);
        assert_eq!(a, 255);
    }
}
