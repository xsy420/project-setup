use super::Appv2;
use ratatui::{Frame, layout::Rect};

pub(crate) trait Render {
    fn render(&self, f: &mut Frame, app: &Appv2, area: Rect);
}
