pub mod app {
    pub mod application;
    mod cargo;
    mod cmake;
    mod inner;
    mod radio_option;
    mod springboot;
    mod wip;
    pub use application::Application;
    use cargo::CargoInner;
    use cmake::CmakeInner;
    use inner::{
        Inner, InnerField, InnerFieldMapping, InnerHandleKeyEventOutput, InnerTipLabel,
        PrepareInner,
    };
    pub(crate) use radio_option::RadioOptionValue;
    use radio_option::{RadioOption, RadioOptionTrait};
    use springboot::SpringBootInner;
    use wip::WipInner;
}
pub mod args;
pub use args::Args;
pub(crate) mod common {
    mod editor;
    mod loop_number;
    mod project_type;
    mod vcs;
    pub(crate) use editor::Editor;
    pub(crate) use loop_number::LoopNumber;
    pub(crate) use project_type::ProjectType;
    pub(crate) use vcs::Vcs;
}
pub(crate) mod features {
    mod download;
    mod zip;
    pub(crate) use download::{RequestMethod, download_file};
    pub(crate) use zip::unzip;
}
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
pub use project_setup_derive::{LoopableNumberedEnum, RadioOption};
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
