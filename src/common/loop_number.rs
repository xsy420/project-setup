#[derive(Clone)]
pub(crate) struct LoopNumber {
    pub(crate) value:  usize,
    pub(crate) length: usize,
}
impl LoopNumber {
    pub(crate) fn new(length: usize) -> Self {
        Self { value: 0, length }
    }

    pub(crate) fn next(&self) -> Self {
        Self {
            value:  (self.value + 1) % self.length,
            length: self.length,
        }
    }

    pub(crate) fn prev(&self) -> Self {
        Self {
            value:  (self.value + self.length - 1) % self.length,
            length: self.length,
        }
    }
}
