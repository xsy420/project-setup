use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};
use strum_macros::EnumIter;
#[derive(Debug, Clone, Copy, EnumIter, FromPrimitive, ToPrimitive, PartialEq)]
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

impl FocusInput {
    pub(crate) fn num(self) -> usize {
        Self::to_usize(&self).unwrap()
    }

    pub(crate) fn next(self) -> Self {
        Self::from_usize((self.num() + 1) % Self::ErrorMessage.num()).unwrap()
    }

    pub(crate) fn prev(self) -> Self {
        Self::from_usize((self.num() - 1) % Self::ErrorMessage.num()).unwrap()
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
            .title(needed_focus.title())
    }
}

#[cfg(test)]
mod force_input {
    use super::FocusInput;

    #[test]
    fn num() {
        assert_eq!(FocusInput::ProjectType.num(), 0);
    }

    #[test]
    fn next() {
        assert_eq!(FocusInput::ProjectType.next(), FocusInput::ProjectVersion);
        assert_eq!(FocusInput::Name.next(), FocusInput::ProjectType);
    }

    #[test]
    fn prev() {
        assert_eq!(FocusInput::ProjectVersion.prev(), FocusInput::ProjectType);
    }
}
