use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
pub use project_setup_derive::LoopableNumberedEnum;
pub trait LoopableNumberedEnum {
    fn num(&self) -> usize;
    fn next_index(&self) -> usize;
    fn prev_index(&self) -> usize;
    #[must_use]
    fn next(&self) -> Self;
    #[must_use]
    fn prev(&self) -> Self;
}
#[allow(dead_code)]
#[derive(Debug, Clone, LoopableNumberedEnum, FromPrimitive, ToPrimitive, PartialEq)]
#[numbered_enum(loop_within = 3)]
enum LoopableNumbered {
    One,
    Two,
    Three,
}
#[cfg(test)]
mod project_setup_derive_test {
    use crate::LoopableNumbered;
    #[test]
    fn test_numbered_enum_extra_method() {
        assert_eq!(LoopableNumbered::One.next(), LoopableNumbered::Two);
        assert_eq!(LoopableNumbered::One.prev(), LoopableNumbered::Three);
    }
}
