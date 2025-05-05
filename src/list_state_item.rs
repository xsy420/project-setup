// #[allow(dead_code)]
struct ListStateItem<A> {
    state: ListState,
    items: Vec<A>,
}

// #[allow(dead_code)]
impl<A: Clone> ListStateItem<A> {
    fn new(items: Vec<A>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { state, items }
    }
    fn next(&mut self, abc: &mut A) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        *abc = self.items[i].clone();
    }
    fn previous(&mut self, abc: &mut A) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        *abc = self.items[i].clone();
    }
}
