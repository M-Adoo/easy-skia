#[derive(Debug)]
pub struct Color(pub(crate) u32);

impl Color {
    pub fn from_u32(c: u32) -> Color {
        Color { 0: c }
    }

    pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Color {
        Color {
            0: ((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32)),
        }
    }

    pub fn alpha(&self) -> u8 {
        (self.0 >> 24 & 0xFF) as u8
    }

    pub fn red(&self) -> u8 {
        (self.0 >> 16 & 0xFF) as u8
    }

    pub fn sk_color_get_g(&self) -> u8 {
        (self.0 >> 8 & 0xFF) as u8
    }

    pub fn sk_color_get_b(&self) -> u8 {
        (self.0 >> 0 & 0xFF) as u8
    }
}
