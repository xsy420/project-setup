use super::RadioOptionTrait;
use anyhow::Result;
use num_traits::{FromPrimitive, ToPrimitive};
use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};
use std::fmt::{Debug, Display};
use strum::IntoEnumIterator;
use tokio::sync::mpsc;
#[derive(Default)]
pub(crate) struct InnerHandleKeyEventOutput {
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
pub(super) trait InnerField:
    Clone + Copy + Display + IntoEnumIterator + FromPrimitive + ToPrimitive
{
    fn vaildate_string(self, value: &mut str) -> String;
}
pub(super) trait InnerFieldMapping<F: InnerField> {
    fn get_focus_field_mut(&mut self, field: F) -> Option<&mut String>;
    fn get_field(&self, field: F) -> &dyn Debug;
    fn get_radio(&mut self, field: F) -> Option<&mut dyn RadioOptionTrait>;
}
pub(super) trait InnerTipLabel {
    fn tips() -> &'static [&'static str];
    fn labels() -> &'static [&'static str];
}
pub(crate) trait PrepareInner {
    async fn prepare(tx: mpsc::Sender<u16>);
    fn is_prepared() -> bool;
}
pub(crate) trait Inner: Sync {
    fn render(&mut self, f: &mut Frame, focus_right_side: bool, area: Rect);
    fn bottom_help_message(&self) -> String;
    fn handle_keyevent(&mut self, key: KeyEvent) -> InnerHandleKeyEventOutput;
    fn create_and_edit(&self) -> Result<()>;
}
