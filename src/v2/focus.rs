use ratatui::widgets::ListState;
#[allow(dead_code)]
pub(super) struct Focus {
    is_focused: bool,
    state: ListState,
}
#[allow(dead_code)]
impl Focus {
    pub(super) fn new() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            is_focused: false,
        }
    }

    pub(super) fn toggle(&mut self) {
        self.is_focused = !self.is_focused;
    }
}
