use crate::app::RadioOptionValue;
use crate::common::ExecutableEnumTrait;
use anyhow::Result;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::{ExecutableEnum, LoopableNumberedEnum};
use std::{
    io::Error,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use strum_macros::{Display, EnumIter};
#[derive(
    Copy,
    Debug,
    Default,
    LoopableNumberedEnum,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Display,
    EnumIter,
    PartialEq,
    ExecutableEnum,
)]
#[numbered_enum(loop_within = 6)]
pub(crate) enum Editor {
    #[default]
    #[exe("")]
    NotNeed,
    #[exe("vim")]
    Vim,
    #[exe("code")]
    VSCode,
    #[exe("nvim")]
    Neovim,
    #[exe("idea")]
    Idea,
    #[exe("clion")]
    Clion,
    #[exe("rustrover")]
    Rustrover,
}
impl RadioOptionValue for Editor {
    fn selectable(&self) -> bool {
        *super::EXECUTABLE_ENUM_CACHE
            .lock()
            .unwrap()
            .get(&self.exe())
            .unwrap()
    }
}
impl Editor {
    pub(crate) fn run(self, project_path: PathBuf, main: String) -> Result<ExitStatus, Error> {
        match self {
            Self::NotNeed => Ok(ExitStatus::default()),
            _ => Command::new(self.exe())
                .arg(main)
                .current_dir(project_path)
                .spawn()?
                .wait(),
        }
    }
}
