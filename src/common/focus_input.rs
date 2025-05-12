use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::LoopableNumberedEnum;
use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};
use strum_macros::EnumIter;
#[derive(
    Debug, Clone, Copy, EnumIter, FromPrimitive, ToPrimitive, PartialEq, LoopableNumberedEnum,
)]
#[numbered_enum(loop_within = 7)]
pub(crate) enum FocusInput {
    ProjectType,
    ProjectVersion,
    Language,
    LanguageVersion,
    Vcs,
    Editor,
    Name,
    ErrorMessage,
    Bottom,
}

impl FocusInput {
    pub(crate) fn title(self) -> String {
        match self {
            Self::ProjectType => "Project Type",
            Self::ProjectVersion => "Project Version",
            Self::Language => "Language",
            Self::LanguageVersion => "Language Version",
            Self::Vcs => "VCS",
            Self::Editor => "Editor",
            Self::Name => "Project Info",
            _ => "",
        }
        .to_string()
    }

    pub(crate) fn constraint() -> [Constraint; 10] {
        [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ]
    }
    pub(crate) fn focus_border(&self, needed_focus: Self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .border_type(if *self == needed_focus {
                BorderType::Thick
            } else {
                BorderType::default()
            })
            .border_style(if *self == needed_focus {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            })
            .title(needed_focus.title())
    }
}
