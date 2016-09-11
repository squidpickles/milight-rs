//! Represents available colors
use ::std::fmt;

/// A single color that the bridge can understand.
/// # Examples
///
/// ```
/// # use milight::colors::*;
/// # fn main() {
/// let yellowish = Color(0x7A);
/// assert_eq!("0x7A", format!("{:#X}", yellowish));
/// # }
/// ```
#[derive(Debug)]
pub struct Color(pub u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        let Color(color) = self;
        color
    }
}

/// Violet (`0x00`)
pub const VIOLET: Color = Color(0x00);
/// Royal Blue (`0x10`)
pub const ROYAL_BLUE: Color = Color(0x10);
/// Baby Blue (`0x20`)
pub const BABY_BLUE: Color = Color(0x20);
/// Aqua (`0x30`)
pub const AQUA: Color = Color(0x30);
/// Mint (`0x40`)
pub const MINT: Color = Color(0x40);
/// Seafoam Green (`0x50`)
pub const SEAFOAM_GREEN: Color = Color(0x50);
/// Green (`0x60`)
pub const GREEN: Color = Color(0x60);
/// Lime Green (`0x70`)
pub const LIME_GREEN: Color = Color(0x70);
/// Yellow (`0x80`)
pub const YELLOW: Color = Color(0x80);
/// Yellow-Orange (`0x90`)
pub const YELLOW_ORANGE: Color = Color(0x90);
/// Orange (`0xA0`)
pub const ORANGE: Color = Color(0xA0);
/// Red (`0xB0`)
pub const RED: Color = Color(0xB0);
/// Pink (`0xC0`)
pub const PINK: Color = Color(0xC0);
/// Fuchsia (`0xD0`)
pub const FUCHSIA: Color = Color(0xD0);
/// Lilac (`0xE0`)
pub const LILAC: Color = Color(0xE0);
/// Lavender (`0xF0`)
pub const LAVENDER: Color = Color(0xF0);
