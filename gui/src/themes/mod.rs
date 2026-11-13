
pub struct Theme {
    pub background: u32,
    pub surface: u32,
    pub foreground: u32,
    pub accent: u32,
}

pub const DARK: Theme = Theme {
    background: 0xFF08090D,
    surface: 0xFF17191F,
    foreground: 0xFFF4F5F7,
    accent: 0xFFB7FF36,
};
