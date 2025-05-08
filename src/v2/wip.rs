use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
};
use tui_big_text::{BigText, PixelSize};

use super::render::Render;

pub(crate) struct WipInner {}
impl Render for WipInner {
    fn render(&self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        f.render_widget(
            BigText::builder()
                .pixel_size(PixelSize::Full)
                .centered()
                .style(Style::new().blue())
                .lines(vec![
                    "Work".yellow().into(),
                    "In".yellow().into(),
                    "Progress".yellow().into(),
                ])
                .build(),
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                ])
                .split(area)[1],
        );
    }
}
