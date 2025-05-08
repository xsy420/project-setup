pub(crate) struct Focus {
    focus: bool,
}

#[allow(dead_code)]
impl Focus {
    pub(crate) fn new() -> Self {
        Self { focus: false }
    }
    pub(crate) fn toggle(&mut self) {
        self.focus = !self.focus;
    }
}
