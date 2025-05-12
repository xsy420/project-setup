#[derive(Clone, Copy)]
pub(crate) enum AppDirection {
    Next,
    Prev,
}
impl AppDirection {
    pub(crate) fn get_counter(self, i: Option<usize>, len: usize) -> usize {
        match i {
            Some(i) => match self {
                Self::Next => {
                    if i >= len - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                Self::Prev => {
                    if i == 0 {
                        len - 1
                    } else {
                        i - 1
                    }
                }
            },
            None => 0,
        }
    }
}
