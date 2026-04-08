/// Dashboard screen — v0.4
///
/// Displays two cards side by side:
///   - Daily Quote card  (person + quote, from DailyQuoteEngine)
///   - Today's Event card (title + description, from GlobalEventsEngine)
///
/// Below the cards the screen shows a quick summary row:
///   Level | Total XP | Tasks done today

use crate::app::Message;
use crate::data::daily_quote_engine::DailyQuote;
use crate::data::global_events_engine::GlobalEvent;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length};

/// Renders the dashboard.
///
/// * `quote`       – today's quote (already fetched from the engine)
/// * `event`       – today's global event (already fetched from the engine)
/// * `tomorrow`    – title of tomorrow's event (for the teaser line)
/// * `level`       – current player level
/// * `total_xp`    – total XP earned
/// * `tasks_done`  – number of tasks completed today
pub fn view<'a>(
    quote: &'a DailyQuote,
    event: &'a GlobalEvent,
    tomorrow: &'a str,
    level: u32,
    total_xp: u32,
    tasks_done: usize,
) -> Element<'a, Message> {
    // ── Quote card ────────────────────────────────────────────────────────────
    let quote_card = container(
        column![
            text("Quote of the Day").size(13).style(iced::widget::text::secondary),
            text("").size(4),
            text(format!("\"{}\"", quote.quote))
                .size(15)
                .style(iced::widget::text::primary),
            text("").size(6),
            text(format!("— {}", quote.person))
                .size(13)
                .style(iced::widget::text::secondary),
        ]
        .spacing(0)
        .padding(16),
    )
    .style(container::rounded_box)
    .width(Length::FillPortion(1));

    // ── Global event card ─────────────────────────────────────────────────────
    let event_card = container(
        column![
            text("Today's Global Event").size(13).style(iced::widget::text::secondary),
            text("").size(4),
            text(event.title.clone())
                .size(17)
                .style(iced::widget::text::primary),
            text("").size(6),
            text(event.description.clone())
                .size(13)
                .style(iced::widget::text::secondary),
            text("").size(10),
            text(format!("Tomorrow: {}", tomorrow))
                .size(12)
                .style(iced::widget::text::secondary),
        ]
        .spacing(0)
        .padding(16),
    )
    .style(container::rounded_box)
    .width(Length::FillPortion(1));

    // ── Stats row — own the strings so there are no dangling references ───────
    let level_str = level.to_string();
    let xp_str = total_xp.to_string();
    let done_str = tasks_done.to_string();

    let stats = container(
        row![
            text("Level ").size(12).style(iced::widget::text::secondary),
            text(level_str).size(14).style(iced::widget::text::primary),
            text("  ·  ").size(14).style(iced::widget::text::secondary),
            text("Total XP ").size(12).style(iced::widget::text::secondary),
            text(xp_str).size(14).style(iced::widget::text::primary),
            text("  ·  ").size(14).style(iced::widget::text::secondary),
            text("Tasks done today ").size(12).style(iced::widget::text::secondary),
            text(done_str).size(14).style(iced::widget::text::primary),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding(iced::Padding::from([12, 16]));

    // ── Full layout ───────────────────────────────────────────────────────────
    let content = column![
        text("Dashboard").size(24),
        text("").size(8),
        row![quote_card, text("  "), event_card].width(Length::Fill),
        text("").size(12),
        stats,
    ]
    .spacing(0)
    .padding(20)
    .width(Length::Fill);

    scrollable(content).into()
}