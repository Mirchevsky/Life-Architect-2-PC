// Analytics screen — v0.3 placeholder.
// Full chart implementation will be added in v0.4 once the data layer is stable.
// This screen is intentionally minimal and delegates all data display to app.rs view().
use crate::app::Message;
use iced::Element;
use iced::widget::{column, text};

#[allow(dead_code)]
pub fn view(
    total_xp: u32,
    level: u32,
    completed: usize,
    pending: usize,
    goals_count: usize,
) -> Element<'static, Message> {
    column![
        text("Analytics").size(24),
        text("").size(8),
        text(format!("Total XP earned: {}", total_xp)).size(16),
        text(format!("Current level: {}", level)).size(16),
        text(format!("Tasks completed: {}", completed)).size(16),
        text(format!("Tasks pending: {}", pending)).size(16),
        text(format!("Goals created: {}", goals_count)).size(16),
    ]
    .spacing(8)
    .padding(20)
    .into()
}