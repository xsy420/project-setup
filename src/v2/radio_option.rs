use crate::common::AppDirection;
use std::fmt::Display;
use strum::IntoEnumIterator;
pub(crate) trait RadioOptionValue:
    Display + Default + Copy + IntoEnumIterator + PartialEq
{
    fn selectable(&self) -> bool;
}
#[derive(Clone)]
pub(super) struct RadioOption<V>
where
    V: RadioOptionValue,
{
    pub(super) value: V,
    id: usize,
}
impl<V: RadioOptionValue> Default for RadioOption<V> {
    fn default() -> Self {
        let value = V::default();
        let mut id = 0;
        for v in V::iter().filter(RadioOptionValue::selectable) {
            if v == value {
                break;
            }
            id += 1;
        }
        Self { value, id }
    }
}
pub(super) trait RadioOptionTrait {
    fn next(&mut self);
    fn prev(&mut self);
    fn get_symbol(&self, curr: usize) -> String;
    fn length(&self) -> usize;
}
impl<V: RadioOptionValue> RadioOptionTrait for RadioOption<V> {
    fn next(&mut self) {
        self.id = AppDirection::Next.get_counter(Some(self.id), self.length());
        self.value = V::iter()
            .filter(RadioOptionValue::selectable)
            .collect::<Vec<V>>()[self.id];
    }

    fn prev(&mut self) {
        self.id = AppDirection::Prev.get_counter(Some(self.id), self.length());
        self.value = V::iter()
            .filter(RadioOptionValue::selectable)
            .collect::<Vec<V>>()[self.id];
    }

    fn get_symbol(&self, curr: usize) -> String {
        format!(
            "{} {}",
            if self.id == curr { "◉" } else { "○" },
            V::iter()
                .filter(RadioOptionValue::selectable)
                .collect::<Vec<V>>()[curr]
        )
    }

    fn length(&self) -> usize {
        V::iter().filter(RadioOptionValue::selectable).count()
    }
}
