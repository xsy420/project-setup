use anyhow::Result;
use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};
#[derive(Default)]
pub(super) struct InnerHandleKeyEventOutput {
    pub(crate) esc_handled: bool,
    pub(crate) exit: bool,
}
impl InnerHandleKeyEventOutput {
    pub(super) fn with_exited(mut self) -> Self {
        self.exit = true;
        self
    }

    #[allow(dead_code)]
    pub(super) fn with_esc_handled(mut self) -> Self {
        self.esc_handled = true;
        self
    }
}
pub(super) trait Inner {
    fn render(&mut self, f: &mut Frame, focus_right_side: bool, area: Rect);
    fn bottom_help_message(&self) -> String;
    fn handle_keyevent(&mut self, key: KeyEvent) -> InnerHandleKeyEventOutput;
    fn create_and_edit(&self) -> Result<()>;
}
