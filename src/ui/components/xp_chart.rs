use crate::data::analytics_dao::DailyXpPoint;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Font, Length, Point, Rectangle};
use iced::alignment::{Horizontal, Vertical};

/// A pure-Rust XP line chart rendered via Iced's Canvas widget.
/// Displays daily XP gain as a filled area chart on a dark background.
pub struct XpChart {
    pub data: Vec<DailyXpPoint>,
}

impl XpChart {
    pub fn new(data: Vec<DailyXpPoint>) -> Self {
        Self { data }
    }

    /// Consumes self and returns a static Element so callers have no lifetime issues.
    pub fn view<Message: 'static>(self) -> Element<'static, Message> {
        Canvas::new(XpChartProgram { data: self.data })
            .width(Length::Fill)
            .height(Length::Fixed(220.0))
            .into()
    }
}

struct XpChartProgram {
    data: Vec<DailyXpPoint>,
}

impl<Message> canvas::Program<Message> for XpChartProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let pad_left: f32 = 48.0;
        let pad_right: f32 = 16.0;
        let pad_top: f32 = 16.0;
        let pad_bottom: f32 = 36.0;

        let chart_w = bounds.width - pad_left - pad_right;
        let chart_h = bounds.height - pad_top - pad_bottom;

        // --- Background ---
        let bg_color = Color::from_rgb(0.15, 0.15, 0.18); // SURFACE
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), bg_color);

        if self.data.is_empty() {
            return vec![frame.into_geometry()];
        }

        let max_xp = self.data.iter().map(|p| p.xp).max().unwrap_or(1).max(1);
        let n = self.data.len();

        // --- Grid lines ---
        let grid_color = Color::from_rgba(0.95, 0.95, 0.95, 0.07);
        let grid_lines = 4u32;
        for i in 0..=grid_lines {
            let y = pad_top + chart_h * (1.0 - i as f32 / grid_lines as f32);
            let line = Path::line(
                Point::new(pad_left, y),
                Point::new(pad_left + chart_w, y),
            );
            frame.stroke(
                &line,
                Stroke::default()
                    .with_color(grid_color)
                    .with_width(1.0),
            );

            // Y-axis label
            let xp_label = (max_xp as f32 * i as f32 / grid_lines as f32) as u32;
            frame.fill_text(Text {
                content: format!("{}", xp_label),
                position: Point::new(0.0, y - 7.0),
                color: Color::from_rgb(0.60, 0.60, 0.65),
                size: iced::Pixels(10.0),
                font: Font::DEFAULT,
                align_x: Horizontal::Left.into(),
                align_y: Vertical::Top,
                line_height: iced::widget::text::LineHeight::default(),
                shaping: iced::widget::text::Shaping::Basic,
                max_width: pad_left,
            });
        }

        // --- Compute data points ---
        let points: Vec<Point> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, dp)| {
                let x = pad_left + (i as f32 / (n - 1).max(1) as f32) * chart_w;
                let y = pad_top + chart_h * (1.0 - dp.xp as f32 / max_xp as f32);
                Point::new(x, y)
            })
            .collect();

        // --- Filled area under the line ---
        let fill_color = Color::from_rgba(0.30, 0.69, 0.31, 0.18); // ACCENT_GREEN translucent
        let mut fill_path = canvas::path::Builder::new();
        fill_path.move_to(Point::new(points[0].x, pad_top + chart_h));
        for &pt in &points {
            fill_path.line_to(pt);
        }
        fill_path.line_to(Point::new(points[n - 1].x, pad_top + chart_h));
        fill_path.close();
        frame.fill(&fill_path.build(), fill_color);

        // --- Line ---
        let line_color = Color::from_rgb(0.30, 0.69, 0.31); // ACCENT_GREEN
        let mut line_path = canvas::path::Builder::new();
        line_path.move_to(points[0]);
        for &pt in points.iter().skip(1) {
            line_path.line_to(pt);
        }
        frame.stroke(
            &line_path.build(),
            Stroke::default()
                .with_color(line_color)
                .with_width(2.0),
        );

        // --- Dots at each data point ---
        for &pt in &points {
            let dot = Path::circle(pt, 3.5);
            frame.fill(&dot, line_color);
        }

        // --- X-axis labels ---
        // Show every label if ≤ 7 points, otherwise every 5th
        let step = if n <= 7 { 1 } else { 5 };
        for (i, dp) in self.data.iter().enumerate() {
            if i % step == 0 || i == n - 1 {
                let x = pad_left + (i as f32 / (n - 1).max(1) as f32) * chart_w;
                frame.fill_text(Text {
                    content: dp.label.clone(),
                    position: Point::new(x, pad_top + chart_h + 6.0),
                    color: Color::from_rgb(0.60, 0.60, 0.65),
                    size: iced::Pixels(10.0),
                    font: Font::DEFAULT,
                    align_x: Horizontal::Center.into(),
                    align_y: Vertical::Top,
                    line_height: iced::widget::text::LineHeight::default(),
                    shaping: iced::widget::text::Shaping::Basic,
                    max_width: f32::INFINITY,
                });
            }
        }

        vec![frame.into_geometry()]
    }
}