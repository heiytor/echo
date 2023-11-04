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
    /// The alpha component of the color.
    a: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    fn set(&mut self, r: u8, g: u8,  b: u8, a: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
    }

    fn get(&self) -> [u8; 4] {
        return [
            self.r,
            self.g,
            self.b,
            self.a,
        ];
    }
}

pub struct Theme {
    bg: Color,
    fg: Color,
    cs: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::new(0, 0, 0, 255),
            fg: Color::new(255, 255, 255, 255),
            cs: Color::new(255, 255, 255, 255),
        }
    }
}

impl Theme {
    /// Sets the background color using a hexadecimal string.
    pub fn set_hex_bg(&mut self, hex: &str) -> Result<(), String> {
        let color = Srgb::<u8>::from_str(hex).map_err(|e| e.to_string())?;
        Ok(self.bg.set(color.red, color.green, color.blue, 255))
    }

    /// Returns the background color as an array of 4 floats in the format \[R, G, B].
    /// Each component is normalized to the range 0~1.
    pub fn bg(&self) -> [f32; 4] {
        self.bg.get().map(|c| c as f32 / 255.0)
    }
    
    /// Sets the foreground color using a hexadecimal string.
    pub fn set_hex_fg(&mut self, hex: &str) -> Result<(), String> {
        let color = Srgb::<u8>::from_str(hex).map_err(|e| e.to_string())?;
        Ok(self.fg.set(color.red, color.green, color.blue, 255))
    }

    /// Returns the foreground color as an array of 3 floats in the format \[R, G, B].
    /// Each component is normalized to the range 0~1.
    pub fn fg(&self) -> [f32; 4] {
        self.fg.get().map(|c| c as f32 / 255.0)
    }

    pub fn set_hex_cs(&mut self, hex: &str, alpha: u8) -> Result<(), String> {
        let color = Srgb::<u8>::from_str(hex).map_err(|e| e.to_string())?;
        Ok(self.cs.set(color.red, color.green, color.blue, alpha))
    }

    pub fn cs(&self) -> [f32; 4] {
        self.cs.get().map(|c| c as f32 / 255.0)
    }
}
