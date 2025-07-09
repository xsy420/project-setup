use crate::app::RadioOptionValue;
use anyhow::Result;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::{EnumFunc, LoopableNumberedEnum};
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
    EnumFunc,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Display,
    EnumIter,
    PartialEq,
)]
#[numbered_enum(loop_within = 6)]
pub(crate) enum Editor {
    #[default]
    #[enum_func(exe(""))]
    NotNeed,
    #[enum_func(exe("vim"))]
    Vim,
    #[enum_func(exe("code"))]
    VSCode,
    #[enum_func(exe("nvim"))]
    Neovim,
    #[enum_func(exe("idea"))]
    Idea,
    #[enum_func(exe("clion"))]
    Clion,
    #[enum_func(exe("rustrover"))]
    Rustrover,
}
impl RadioOptionValue for Editor {
    fn selectable(&self) -> bool {
        match self {
            Self::NotNeed => true,
            _ => Command::new(self.exe())
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
        }
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
