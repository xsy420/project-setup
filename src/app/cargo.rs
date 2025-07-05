use super::{
    Inner, InnerCommonState, InnerField, InnerFieldMapping, InnerHandleKeyEventOutput,
    InnerTipLabel, PrepareInner, RadioOption, handle_inner_keyevent,
};
use crate::{
    InnerState, RadioOption,
    common::{Editor, Vcs},
};
use anyhow::Result;
use heck::ToSnakeCase;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{env, fmt::Debug, fs, path::PathBuf, process::Command};
use strum_macros::{Display, EnumIter};
#[derive(Clone, Copy, Display, EnumIter, FromPrimitive, ToPrimitive)]
enum CargoField {
    Name,
    ProjectType,
    Edition,
    Editor,
    Vcs,
    Path,
}
impl InnerField for CargoField {
    fn vaildate_string(self, value: &mut str) -> String {
        if value.is_empty() {
            return format!("{} cannot be empty", self.to_string().to_snake_case());
        }
        String::new()
    }
}
#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq, RadioOption)]
enum ProjectType {
    #[default]
    Executable,
    Library,
}
impl ProjectType {
    fn args(self) -> String {
        match self {
            Self::Executable => "--name".to_string(),
            Self::Library => "--lib".to_string(),
        }
    }

    fn main_file(self) -> String {
        match self {
            Self::Executable => "src/main.rs".to_string(),
            Self::Library => "src/lib.rs".to_string(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq, RadioOption)]
enum Edition {
    #[strum(to_string = "2015")]
    Fifteen,
    #[strum(to_string = "2018")]
    Eighteen,
    #[strum(to_string = "2021")]
    TwentyOne,
    #[default]
    #[strum(to_string = "2024")]
    TwentyFour,
}
#[derive(InnerState, Clone)]
pub(super) struct CargoInner {
    name: String,
    project_type: RadioOption<ProjectType>,
    edition: RadioOption<Edition>,
    editor: RadioOption<Editor>,
    vcs: RadioOption<Vcs>,
    path: PathBuf,
    common_state: InnerCommonState,
}
impl CargoInner {
    pub(super) fn new() -> Self {
        Self {
            name: String::new(),
            project_type: RadioOption::default(),
            edition: RadioOption::default(),
            editor: RadioOption::default(),
            vcs: RadioOption::default(),
            path: env::current_dir().unwrap(),
            common_state: InnerCommonState::new::<CargoField>(),
        }
    }
}
impl InnerFieldMapping<CargoField> for CargoInner {
    fn get_focus_field_mut(&mut self, field: CargoField) -> Option<&mut String> {
        match field {
            CargoField::Name => Some(&mut self.name),
            _ => None,
        }
    }

    fn get_field(&self, field: CargoField) -> &dyn std::fmt::Debug {
        match field {
            CargoField::Name => &self.name,
            CargoField::ProjectType => &self.project_type.value,
            CargoField::Edition => &self.edition.value,
            CargoField::Editor => &self.editor.value,
            CargoField::Vcs => &self.vcs.value,
            CargoField::Path => &self.path,
        }
    }

    fn get_radio(&mut self, field: CargoField) -> Option<&mut dyn super::RadioOptionTrait> {
        match field {
            CargoField::ProjectType => Some(&mut self.project_type),
            CargoField::Edition => Some(&mut self.edition),
            CargoField::Editor => Some(&mut self.editor),
            CargoField::Vcs => Some(&mut self.vcs),
            _ => None,
        }
    }
}
impl InnerTipLabel for CargoInner {
    fn tips() -> &'static [&'static str] {
        &[
            "Please input the name of this project",
            "Use arrow keys to select project_type",
            "Use arrow keys to select edition",
            "Use arrow keys to select editor",
            "Use arrow keys to select vcs tool",
            "Please input the path of this project",
        ]
    }

    fn labels() -> &'static [&'static str] {
        &["name", "project_type", "edition", "editor", "vcs", "path"]
    }
}
impl PrepareInner for CargoInner {
    async fn prepare(_tx: tokio::sync::mpsc::Sender<u16>) {}

    fn is_prepared() -> bool {
        true
    }
}
impl Inner for CargoInner {
    fn render(&mut self, f: &mut Frame, focus_right_side: bool, area: Rect) {
        let labels = Self::labels();
        // 表单布局 - 垂直排列输入框
        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5); 6])
            .split(area);
        let split_line_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2); 2]);
        let split_label_input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(20), Constraint::Min(0)]);
        let split_tip_input_error_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(1), Constraint::Max(3), Constraint::Max(1)]);
        for i in (0 .. labels.len()).step_by(2) {
            let line_layout = split_line_layout.split(form_layout[i / 2]);
            for side in 0 .. 2 {
                let index = i + side;
                if index == labels.len() {
                    break;
                }
                let side_line_layout = line_layout[side];
                let label_input_area = split_label_input_layout.split(side_line_layout);
                f.render_widget(
                    Paragraph::new(labels[index]).centered().block(
                        Block::default()
                            .borders(Borders::all())
                            .border_type(BorderType::Thick),
                    ),
                    split_tip_input_error_layout.split(label_input_area[0])[1],
                );
                let focus_block = Block::new()
                    .borders(Borders::ALL)
                    .border_style(
                        if index == self.common_state.focus_index.value && focus_right_side {
                            Color::Red
                        } else {
                            Color::default()
                        },
                    )
                    .border_type(BorderType::Thick);
                if focus_right_side && index == self.common_state.focus_index.value {
                    f.render_widget(
                        Paragraph::new(Self::tips()[index])
                            .style(Color::Blue)
                            .centered(),
                        split_tip_input_error_layout.split(label_input_area[1])[0],
                    );
                }
                let field = CargoField::from_usize(index).unwrap();
                if let Some(r) = self.get_radio(field) {
                    f.render_widget(
                        Paragraph::new(
                            (0 .. r.length())
                                .map(|curr| r.get_symbol(curr))
                                .collect::<Vec<String>>()
                                .join("    "),
                        )
                        .centered()
                        .block(focus_block),
                        split_tip_input_error_layout.split(label_input_area[1])[1],
                    );
                    continue;
                }
                let field_value = self.get_field(field);
                let field_string_value = format!("{field_value:?}").replace('"', "");
                f.render_widget(
                    Paragraph::new(if field_string_value.is_empty() {
                        format!("Please input {}", labels[index])
                    } else {
                        field_string_value.clone()
                    })
                    .centered()
                    .style(if field_string_value.is_empty() {
                        Color::Gray
                    } else {
                        Color::default()
                    })
                    .block(focus_block),
                    split_tip_input_error_layout.split(label_input_area[1])[1],
                );
                if !self.common_state.error_messages[index].is_empty() {
                    f.render_widget(
                        Paragraph::new(self.common_state.error_messages[index].clone())
                            .style(Color::Red),
                        split_tip_input_error_layout.split(label_input_area[1])[2],
                    );
                }
            }
        }
    }

    fn bottom_help_message(&self) -> String {
        "tab: focus next item | shift+tab: focus prev item | Enter: confirm to create project | "
            .to_string()
    }

    fn handle_keyevent(&mut self, key: KeyEvent) -> InnerHandleKeyEventOutput {
        handle_inner_keyevent(self, key)
    }

    fn create_and_edit(&self) -> Result<()> {
        let project_path = self.path.join(&self.name);
        fs::create_dir_all(&project_path)?;
        self.vcs.value.init_vcs_repo(&self.name, &self.path)?;
        Command::new("cargo")
            .arg("init")
            .arg(self.project_type.value.args())
            .arg(&self.name)
            .arg("--edition")
            .arg(format!("{}", self.edition.value))
            .current_dir(&project_path)
            .status()?;
        self.editor
            .value
            .run(project_path, self.project_type.value.main_file())?;
        Ok(())
    }
}
