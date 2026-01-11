use crate::common::LoopNumber;
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
    id:               LoopNumber,
}
impl<V: RadioOptionValue> Default for RadioOption<V> {
    fn default() -> Self {
        let value = V::default();
        let mut id = LoopNumber::new(V::iter().filter(RadioOptionValue::selectable).count());
        for v in V::iter().filter(RadioOptionValue::selectable) {
            if v == value {
                break;
            }
            id = id.next();
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
        self.id = self.id.next();
        self.value = V::iter()
            .filter(RadioOptionValue::selectable)
            .collect::<Vec<V>>()[self.id.value];
    }

    fn prev(&mut self) {
        self.id = self.id.prev();
        self.value = V::iter()
            .filter(RadioOptionValue::selectable)
            .collect::<Vec<V>>()[self.id.value];
    }

    fn get_symbol(&self, curr: usize) -> String {
        format!(
            "{} {}",
            if self.id.value == curr { "◉" } else { "○" },
            V::iter()
                .filter(RadioOptionValue::selectable)
                .collect::<Vec<V>>()[curr]
        )
    }

    fn length(&self) -> usize {
        self.id.length
    }
}
