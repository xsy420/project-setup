use super::{
    Inner, InnerField, InnerFieldMapping, InnerHandleKeyEventOutput, InnerTipLabel, RadioOption,
    RadioOptionTrait, RadioOptionValue,
};
use crate::common::{Editor, Vcs};
use anyhow::Result;
use heck::ToSnakeCase;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{env, fmt::Debug, fs, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
#[derive(Clone, Copy, Display, EnumIter, FromPrimitive)]
enum CmakeField {
    Name,
    ProjectVersion,
    ProjectType,
    Language,
    LanguageVersion,
    Editor,
    Vcs,
    Path,
}
impl InnerField for CmakeField {
    fn vaildate_string(self, value: &mut str) -> String {
        if value.is_empty() {
            return format!("{} cannot be empty", self.to_string().to_snake_case());
        }
        String::new()
    }
}
#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq)]
enum ProjectType {
    #[default]
    Executable,
    Library,
}
impl RadioOptionValue for ProjectType {
    fn selectable(&self) -> bool {
        true
    }
}
#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq)]
enum Language {
    #[default]
    C,
    Cpp,
}
impl Language {
    fn main_file(self) -> String {
        match self {
            Self::C => "main.c",
            Self::Cpp => "main.cpp",
        }
        .to_string()
    }

    fn standard(self) -> String {
        match self {
            Self::C => "C",
            Self::Cpp => "CXX",
        }
        .to_string()
    }

    fn main_file_content(self) -> String {
        match self {
            Self::C => {
                "\
                #include <stdio.h>\n\
                \n\
                int main() {\n\
                \tprintf(\"Hello, World!\");\n\
                \treturn 0;\n\
                }\n"
            }
            Self::Cpp => {
                "\
                #include <iostream>\n\
                \n\
                int main() {\n\
                \tstd::cout << \"Hello, World!\" << std::endl;\n\
                \treturn 0;\n\
                }\n"
            }
        }
        .to_string()
    }
}
impl RadioOptionValue for Language {
    fn selectable(&self) -> bool {
        true
    }
}
pub(crate) struct CmakeInner {
    name: String,
    cmake_minimum_required: String,
    project_type: RadioOption<ProjectType>,
    language: RadioOption<Language>,
    language_standard_version: String,
    editor: RadioOption<Editor>,
    vcs: RadioOption<Vcs>,
    path: PathBuf,
    focus_index: usize,
    error_messages: Vec<String>,
}
impl CmakeInner {
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            cmake_minimum_required: String::new(),
            project_type: RadioOption::default(),
            language: RadioOption::default(),
            language_standard_version: String::new(),
            editor: RadioOption::default(),
            vcs: RadioOption::default(),
            path: env::current_dir().unwrap(),
            focus_index: 0,
            error_messages: CmakeField::iter().map(|_| String::new()).collect(),
        }
    }
}
impl InnerFieldMapping<CmakeField> for CmakeInner {
    fn get_focus_field_mut(&mut self, field: CmakeField) -> Option<&mut String> {
        match field {
            CmakeField::Name => Some(&mut self.name),
            CmakeField::ProjectVersion => Some(&mut self.cmake_minimum_required),
            CmakeField::LanguageVersion => Some(&mut self.language_standard_version),
            _ => None,
        }
    }

    fn get_field(&self, field: CmakeField) -> &dyn Debug {
        match field {
            CmakeField::Name => &self.name,
            CmakeField::ProjectVersion => &self.cmake_minimum_required,
            CmakeField::ProjectType => &self.project_type.value,
            CmakeField::Language => &self.language.value,
            CmakeField::LanguageVersion => &self.language_standard_version,
            CmakeField::Editor => &self.editor.value,
            CmakeField::Vcs => &self.vcs.value,
            CmakeField::Path => &self.path,
        }
    }

    fn get_radio(&mut self, field: CmakeField) -> Option<&mut dyn RadioOptionTrait> {
        match field {
            CmakeField::ProjectType => Some(&mut self.project_type),
            CmakeField::Language => Some(&mut self.language),
            CmakeField::Editor => Some(&mut self.editor),
            CmakeField::Vcs => Some(&mut self.vcs),
            _ => None,
        }
    }
}
impl InnerTipLabel for CmakeInner {
    fn tips() -> &'static [&'static str] {
        &[
            "Please input the name of this project",
            "Please input the cmake_minimum_required of this project",
            "Use arrow keys to select project_type",
            "Use arrow keys to select language",
            "Use arrow keys to select language_version",
            "Use arrow keys to select editor",
            "Use arrow keys to select vcs tool",
            "Please input the path of this project",
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "name",
            "cmake_minimum_required",
            "project_type",
            "language",
            "language_version",
            "editor",
            "vcs",
            "path",
        ]
    }
}
impl Inner for CmakeInner {
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
                    .border_style(if index == self.focus_index && focus_right_side {
                        Color::Red
                    } else {
                        Color::default()
                    })
                    .border_type(BorderType::Thick);
                if focus_right_side && index == self.focus_index {
                    f.render_widget(
                        Paragraph::new(Self::tips()[index])
                            .style(Color::Blue)
                            .centered(),
                        split_tip_input_error_layout.split(label_input_area[1])[0],
                    );
                }
                let field = CmakeField::from_usize(index).unwrap();
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
                if !self.error_messages[index].is_empty() {
                    f.render_widget(
                        Paragraph::new(self.error_messages[index].clone()).style(Color::Red),
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
        let field_len = CmakeField::iter().count();
        let field = CmakeField::from_usize(self.focus_index).unwrap();
        match key.code {
            KeyCode::Char(c) => {
                if let Some(x) = self.get_focus_field_mut(field) {
                    x.push(c);
                    self.error_messages[self.focus_index] = field.vaildate_string(x);
                }
            }
            KeyCode::Backspace => {
                if let Some(x) = self.get_focus_field_mut(field) {
                    x.pop();
                    self.error_messages[self.focus_index] = field.vaildate_string(x);
                }
            }
            KeyCode::Enter => {
                if self.error_messages.iter().all(String::is_empty) {
                    return InnerHandleKeyEventOutput::default().with_exited();
                }
            }
            KeyCode::Tab => {
                self.focus_index = (self.focus_index + 1) % field_len;
            }
            KeyCode::BackTab => {
                self.focus_index = (self.focus_index + field_len - 1) % field_len;
            }
            KeyCode::Left => {
                self.get_radio(field).map(RadioOptionTrait::prev);
            }
            KeyCode::Right => {
                self.get_radio(field).map(RadioOptionTrait::next);
            }
            _ => {}
        }
        InnerHandleKeyEventOutput::default()
    }

    fn create_and_edit(&self) -> Result<()> {
        let project_path = self.path.join(&self.name);
        fs::create_dir_all(&project_path)?;
        self.vcs.value.init_vcs_repo(&self.name, &self.path)?;
        let cmake_lists = format!(
            "\
                cmake_minimum_required(VERSION {})\n\
                project({})\n\
                \n\
                set(CMAKE_{}_STANDARD {})\n\
                \n\
                add_{}(${{PROJECT_NAME}} {})\n",
            self.cmake_minimum_required,
            self.name,
            self.language.value.standard(),
            self.language_standard_version,
            self.project_type.value.to_string().to_lowercase(),
            self.language.value.main_file()
        );
        fs::write(project_path.join("CMakeLists.txt"), cmake_lists)?;
        fs::write(
            project_path.join(self.language.value.main_file()),
            self.language.value.main_file_content(),
        )?;
        self.editor
            .value
            .run(project_path, self.language.value.main_file())?;
        Ok(())
    }
}
