use iced::Color;

// --- Color Palette (mirrors the Android app's dark theme) ---

#[allow(dead_code)]
pub const BACKGROUND: Color = Color::from_rgb(0.10, 0.10, 0.12);
#[allow(dead_code)]
pub const SURFACE: Color = Color::from_rgb(0.15, 0.15, 0.18);
#[allow(dead_code)]
pub const ACCENT_AMBER: Color = Color::from_rgb(1.0, 0.76, 0.03);   // Pinned tasks
#[allow(dead_code)]
pub const ACCENT_RED: Color = Color::from_rgb(0.90, 0.22, 0.21);    // Urgent tasks
#[allow(dead_code)]
pub const ACCENT_GREEN: Color = Color::from_rgb(0.30, 0.69, 0.31);  // XP bar fill
#[allow(dead_code)]
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.95);
#[allow(dead_code)]
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.60, 0.60, 0.65);
#[allow(dead_code)]
pub const TEXT_COMPLETED: Color = Color::from_rgb(0.40, 0.40, 0.45);
