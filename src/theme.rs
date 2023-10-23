use palette::Srgb;
use std::str::FromStr;

/// Represents a color in the RGBA format.
struct Color {
    /// The red component of the color.
    r: u8,
    /// The green component of the color.
    g: u8,
    /// The blue component of the color.
    b: u8, 
    /// The alpha (transparency) component of the color.
    a: u8,
}

impl Default for Color {
    /// Provides a default `Color`, which is fully black.
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 1,
        }
    }
}

pub struct Theme {
    bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::default(),
        }
    }
}

impl Theme {
    /// Sets the background color using a hexadecimal string.
    pub fn set_hex_bg(&mut self, hex: &str) -> Result<(), String> {
        let color = Srgb::<u8>::from_str(hex).map_err(|e| e.to_string())?;

        self.bg.r = color.red;
        self.bg.b = color.blue;
        self.bg.g = color.green;
        self.bg.a = 255;

        Ok(())
    }

    /// Returns the background color as an array of 4 floats in the format \[R, G, B, A].
    /// Each component is normalized to the range 0..1.
    pub fn bg(&self) -> [f32; 4] {
        return [
            self.bg.r as f32 / 255.0,
            self.bg.g as f32 / 255.0,
            self.bg.b as f32 / 255.0,
            self.bg.a as f32 / 255.0,
        ];
    }
}

