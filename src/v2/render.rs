use ratatui::{Frame, layout::Rect};

pub(crate) trait Render {
    fn render(&self, f: &mut Frame, area: Rect);
}
