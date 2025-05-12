use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::LoopableNumberedEnum;
use std::fmt::Debug;
use strum::Display;

#[derive(Debug, Clone, Default, Display, FromPrimitive, ToPrimitive, LoopableNumberedEnum)]
#[allow(dead_code)]
#[numbered_enum(loop_within = 7)]
enum ProjectPackaging {
    #[default]
    NotNeed,
    Alpine,
    ArchLinux,
    Fedora,
    Gentoo,
    Nix,
    Ubuntu,
}
