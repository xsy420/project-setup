use super::{Inner, InnerHandleKeyEventOutput};
use ratatui::{
    crossterm::event::KeyEvent,
    layout::Layout,
    style::{Style, Stylize},
};
use ratatui_macros::constraints;
use tui_big_text::{BigText, PixelSize};
pub(crate) struct WipInner {}
impl Inner for WipInner {
    fn render(&mut self, f: &mut ratatui::Frame, _: bool, area: ratatui::prelude::Rect) {
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
            Layout::vertical(constraints![==15%,==70%,==15%]).split(area)[1],
        );
    }

    fn bottom_help_message(&self) -> String {
        String::new()
    }

    fn handle_keyevent(&mut self, _key: KeyEvent) -> InnerHandleKeyEventOutput {
        InnerHandleKeyEventOutput::default()
    }

    fn create_and_edit(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
