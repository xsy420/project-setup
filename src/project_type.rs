use crate::language::Language;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::LoopableNumberedEnum;
use strum_macros::{Display, EnumIter};

#[derive(
    Debug,
    Clone,
    Default,
    EnumIter,
    Display,
    PartialEq,
    FromPrimitive,
    ToPrimitive,
    LoopableNumberedEnum,
)]
#[numbered_enum(loop_within = 4)]
pub(crate) enum ProjectType {
    #[default]
    SpringBoot,
    CMake,
    Maven,
    Cargo,
}

impl ProjectType {
    pub(crate) fn versions(&self) -> Vec<String> {
        match self {
            Self::SpringBoot => vec!["3.3.0", "3.4.0", "3.5.0"],
            Self::CMake => vec!["3.25", "3.26", "4.0"],
            Self::Maven => vec!["3.9.9"],
            Self::Cargo => vec!["1.86.0"],
        }
        .iter()
        .map(ToString::to_string)
        .collect()
    }
    pub(crate) fn languages(&self) -> Vec<Language> {
        match self {
            Self::CMake => vec![Language::C, Language::Cpp],
            Self::Maven | Self::SpringBoot => vec![Language::Java, Language::Kotlin],
            Self::Cargo => vec![Language::Rust],
        }
    }
}
