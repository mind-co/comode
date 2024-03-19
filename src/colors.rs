// Color pack contains current colors
use colored::{Colorize, CustomColor};

pub struct ComindColors {
    primary: CustomColor,
    secondary: CustomColor,
    tertiary: CustomColor,
}

// Extractor methods
impl ComindColors {
    pub fn primary(&self) -> CustomColor {
        self.primary
    }

    pub fn secondary(&self) -> CustomColor {
        self.secondary
    }

    pub fn tertiary(&self) -> CustomColor {
        self.tertiary
    }
}

// Default color pack
impl Default for ComindColors {
    fn default() -> Self {
        // Make structs with custom colors
        ComindColors {
            primary: CustomColor::new(0, 137, 200),
            secondary: CustomColor::new(207, 94, 74),
            tertiary: CustomColor::new(0, 152, 119),
        }
    }
}
