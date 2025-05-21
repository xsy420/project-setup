use clap::ValueEnum;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::LoopableNumberedEnum;
use strum_macros::{Display, EnumIter};
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    EnumIter,
    Display,
    PartialEq,
    FromPrimitive,
    ToPrimitive,
    LoopableNumberedEnum,
    ValueEnum,
)]
#[numbered_enum(loop_within = 4)]
pub(crate) enum ProjectType {
    #[default]
    SpringBoot,
    Cmake,
    Maven,
    Cargo,
}
