use super::RadioOptionTrait;
use crate::common::LoopNumber;
use anyhow::Result;
use num_traits::{FromPrimitive, ToPrimitive};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
};
use std::fmt::{Debug, Display};
use strum::IntoEnumIterator;
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
    fn validate_string(self, value: &mut str) -> String;
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
pub(super) trait Inner: Sync {
    fn render(&mut self, f: &mut Frame, focus_right_side: bool, area: Rect);
    fn bottom_help_message(&self) -> String;
    fn handle_keyevent(&mut self, key: KeyEvent) -> InnerHandleKeyEventOutput;
    fn create_and_edit(&self) -> Result<()>;
}
#[derive(Clone)]
pub(super) struct InnerCommonState {
    pub(super) focus_index: LoopNumber,
    pub(super) error_messages: Vec<String>,
}
impl InnerCommonState {
    pub(super) fn new<F>() -> Self
    where
        F: InnerField,
    {
        Self {
            focus_index: LoopNumber::new(F::iter().count()),
            error_messages: F::iter().map(|_| String::new()).collect(),
        }
    }
}
pub(super) trait InnerState {
    fn state(self) -> InnerCommonState;
    fn with_state(&mut self, state: InnerCommonState) -> &mut Self;
}
pub(super) fn handle_inner_keyevent<T, F>(inner: &mut T, key: KeyEvent) -> InnerHandleKeyEventOutput
where
    T: InnerFieldMapping<F> + Inner + InnerState + Clone,
    F: InnerField,
{
    let mut state = inner.clone().state();
    let field = F::from_usize(state.focus_index.value).unwrap();
    match key.code {
        KeyCode::Char(c) => {
            if let Some(x) = inner.get_focus_field_mut(field) {
                x.push(c);
                state.error_messages[state.focus_index.value] = field.validate_string(x);
            }
        }
        KeyCode::Backspace => {
            if let Some(x) = inner.get_focus_field_mut(field) {
                x.pop();
                state.error_messages[state.focus_index.value] = field.validate_string(x);
            }
        }
        KeyCode::Enter => {
            F::iter().for_each(|field| {
                if let Some(x) = inner.get_focus_field_mut(field) {
                    state.error_messages[field.to_usize().unwrap()] = field.validate_string(x);
                }
            });
            if state.error_messages.iter().all(String::is_empty) {
                return InnerHandleKeyEventOutput::default().with_exited();
            }
        }
        KeyCode::Tab => {
            state.focus_index = state.focus_index.next();
        }
        KeyCode::BackTab => {
            state.focus_index = state.focus_index.prev();
        }
        KeyCode::Left => {
            inner.get_radio(field).map(RadioOptionTrait::prev);
        }
        KeyCode::Right => {
            inner.get_radio(field).map(RadioOptionTrait::next);
        }
        _ => {}
    }
    inner.with_state(state);
    InnerHandleKeyEventOutput::default()
}
