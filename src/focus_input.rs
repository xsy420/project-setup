use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};
use strum_macros::EnumIter;
#[derive(Clone, Copy, EnumIter)]
pub(crate) enum FocusInput {
    ProjectType,
    ProjectVersion,
    Language,
    LanguageVersion,
    Vcs,
    Name,
    ErrorMessage,
    Bottom,
}

impl PartialEq for FocusInput {
    fn eq(&self, other: &Self) -> bool {
        self.num() == other.num()
    }
}

impl FocusInput {
    fn from(i: usize) -> Self {
        match i {
            0 => Some(Self::ProjectType),
            1 => Some(Self::ProjectVersion),
            2 => Some(Self::Language),
            3 => Some(Self::LanguageVersion),
            4 => Some(Self::Vcs),
            5 => Some(Self::Name),
            6 => Some(Self::ErrorMessage),
            7 => Some(Self::Bottom),
            _ => None,
        }
        .unwrap()
    }
    pub(crate) fn num(self) -> usize {
        match self {
            Self::ProjectType => 0,
            Self::ProjectVersion => 1,
            Self::Language => 2,
            Self::LanguageVersion => 3,
            Self::Vcs => 4,
            Self::Name => 5,
            Self::ErrorMessage => 6,
            Self::Bottom => 7,
        }
    }

    pub(crate) fn next(self) -> Self {
        FocusInput::from((self.num() + 1) % Self::ErrorMessage.num())
    }

    pub(crate) fn prev(self) -> Self {
        FocusInput::from((self.num() - 1) % Self::ErrorMessage.num())
    }

    pub(crate) fn title(self) -> String {
        match self {
            Self::ProjectType => "Project Type",
            Self::ProjectVersion => "Project Version",
            Self::Language => "Language",
            Self::LanguageVersion => "Language Version",
            Self::Vcs => "VCS",
            Self::Name => "Project Info",
            _ => "",
        }
        .to_string()
    }

    pub(crate) fn constraint() -> [Constraint; 9] {
        [
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
            .title(self.title())
    }
}
