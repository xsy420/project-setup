use ratatui::widgets::ListState;
#[allow(dead_code)]
pub(crate) struct Focus {
    is_focused: bool,
    state: ListState,
}
#[allow(dead_code)]
impl Focus {
    pub(crate) fn new() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            is_focused: false,
        }
    }

    pub(crate) fn toggle(&mut self) {
        self.is_focused = !self.is_focused;
    }
}
