use super::{Inner, InnerHandleKeyEventOutput, RadioOption, RadioOptionTrait, RadioOptionValue};
use crate::{
    common::{Editor, Vcs},
    features::{download_file, unzip},
};
use anyhow::Result;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup::LoopableNumberedEnum;
use ratatui::style::Color;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{
    env,
    fmt::{Debug, Display},
    fs,
    path::PathBuf,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
#[derive(Debug, Clone, Copy, FromPrimitive, EnumIter)]
enum SpringBootField {
    Name,
    Generator,
    GroupId,
    ArtifactId,
    BootVersion,
    Language,
    JavaVersion,
    Editor,
    Vcs,
    Dependencies,
    Path,
}
impl SpringBootField {
    fn vaildate_string(self, value: &mut str) -> String {
        match self {
            Self::GroupId => {
                if value.ends_with('.') {
                    "group_id cannot end with '.'".to_string()
                } else {
                    String::new()
                }
            }
            Self::ArtifactId => {
                if value.ends_with('.') {
                    "artifact_id cannot end with '.'".to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}
#[derive(
    Clone,
    Copy,
    Default,
    Display,
    Debug,
    ToPrimitive,
    FromPrimitive,
    LoopableNumberedEnum,
    EnumIter,
    PartialEq,
)]
#[numbered_enum(loop_within = 2)]
enum Generator {
    #[default]
    Maven,
    Gradle,
}
impl RadioOptionValue for Generator {
    fn selectable(&self) -> bool {
        true
    }
}
#[derive(Clone, Copy, Default, Display, Debug, EnumIter, PartialEq)]
enum Language {
    #[default]
    Java,
    Kotlin,
}
impl Language {
    fn extension(self) -> String {
        match self {
            Self::Java => "java",
            Self::Kotlin => "kt",
        }
        .to_string()
    }
}
impl RadioOptionValue for Language {
    fn selectable(&self) -> bool {
        true
    }
}
#[derive(Clone, Copy, Default, Debug, EnumIter, PartialEq)]
enum JavaVersion {
    TwentyThree,
    TwentyOne,
    #[default]
    Seventeen,
    Eight,
}
impl Display for JavaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(match self {
            Self::TwentyThree => "23",
            Self::TwentyOne => "21",
            Self::Seventeen => "17",
            Self::Eight => "8",
        })
        .finish()
    }
}
impl RadioOptionValue for JavaVersion {
    fn selectable(&self) -> bool {
        true
    }
}
pub(crate) struct SpringBootInner {
    name: String,
    generator: RadioOption<Generator>,
    group_id: String,
    artifact_id: String,
    boot_version: String,
    language: RadioOption<Language>,
    java_version: RadioOption<JavaVersion>,
    editor: RadioOption<Editor>,
    vcs: RadioOption<Vcs>,
    dependencies: Vec<String>,
    path: PathBuf,
    focus_index: usize,
    tip_messages: Vec<String>,
    error_messages: Vec<String>,
}
impl SpringBootInner {
    pub(crate) fn new() -> Self {
        Self {
            name: "demo".to_string(),
            generator: RadioOption::default(),
            group_id: "com.example".to_string(),
            artifact_id: "demo".to_string(),
            boot_version: "3.3.0".to_string(),
            language: RadioOption::default(),
            java_version: RadioOption::default(),
            editor: RadioOption::default(),
            vcs: RadioOption::default(),
            dependencies: vec![String::new()],
            path: env::current_dir().unwrap(),
            focus_index: 0,
            tip_messages: [
                "Please input the name of this project",
                "Use arrow keys to select generator",
                "Please input the group_id of this project",
                "Please input the artifact_id of this project",
                "Please input the boot_version of this project",
                "Use arrow keys to select language",
                "Use arrow keys to select java_version",
                "Use arrow keys to select editor",
                "Use arrow keys to select vcs tool",
                "Please input the dependencies of this project",
                "Please input the path of this project",
            ]
            .iter()
            .map(|x| (*x).to_string())
            .collect(),
            error_messages: SpringBootField::iter().map(|_| String::new()).collect(),
        }
    }

    fn get_focus_field_mut(&mut self, field: SpringBootField) -> Result<&mut String, String> {
        match field {
            SpringBootField::Name => Ok(&mut self.name),
            SpringBootField::GroupId => Ok(&mut self.group_id),
            SpringBootField::ArtifactId => Ok(&mut self.artifact_id),
            SpringBootField::BootVersion => Ok(&mut self.boot_version),
            _ => Err(String::new()),
        }
    }

    fn get_field(&self, field: SpringBootField) -> &dyn Debug {
        match field {
            SpringBootField::Name => &self.name,
            SpringBootField::Generator => &self.generator.value,
            SpringBootField::GroupId => &self.group_id,
            SpringBootField::ArtifactId => &self.artifact_id,
            SpringBootField::BootVersion => &self.boot_version,
            SpringBootField::Language => &self.language.value,
            SpringBootField::JavaVersion => &self.java_version.value,
            SpringBootField::Editor => &self.editor.value,
            SpringBootField::Vcs => &self.vcs.value,
            SpringBootField::Dependencies => &self.dependencies,
            SpringBootField::Path => &self.path,
        }
    }

    fn get_radio(&mut self, field: SpringBootField) -> Option<&mut dyn RadioOptionTrait> {
        match field {
            SpringBootField::Generator => Some(&mut self.generator),
            SpringBootField::Language => Some(&mut self.language),
            SpringBootField::JavaVersion => Some(&mut self.java_version),
            SpringBootField::Editor => Some(&mut self.editor),
            SpringBootField::Vcs => Some(&mut self.vcs),
            _ => None,
        }
    }

    fn labels() -> &'static [&'static str] {
        &[
            "name",
            "generator",
            "group_id",
            "artifact_id",
            "boot_version",
            "language",
            "java_version",
            "editor",
            "vcs",
            "dependencies",
            "path",
        ]
    }
}
impl Inner for SpringBootInner {
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
                        Paragraph::new(self.tip_messages[index].clone())
                            .style(Color::Blue)
                            .centered(),
                        split_tip_input_error_layout.split(label_input_area[1])[0],
                    );
                }
                let field = SpringBootField::from_usize(index).unwrap();
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
        let field_len = SpringBootField::iter().count();
        let field = SpringBootField::from_usize(self.focus_index).unwrap();
        match key.code {
            KeyCode::Char(c) => {
                if let Ok(x) = self.get_focus_field_mut(field) {
                    x.push(c);
                    self.error_messages[self.focus_index] = field.vaildate_string(x);
                }
            }
            KeyCode::Backspace => {
                self.get_focus_field_mut(field).ok().unwrap().pop();
                self.error_messages[self.focus_index] =
                    field.vaildate_string(self.get_focus_field_mut(field).unwrap());
            }
            KeyCode::Enter => {
                return InnerHandleKeyEventOutput::default().with_exited();
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
        let params = [
            ("groupId", self.group_id.clone()),
            ("artifactId", self.artifact_id.clone()),
            (
                "type",
                format!(
                    "{}-project",
                    self.generator.value.to_string().to_lowercase()
                ),
            ),
            ("name", self.name.clone()),
            ("language", self.language.value.to_string().to_lowercase()),
            ("javaVersion", self.java_version.value.to_string()),
            ("bootVersion", self.boot_version.clone()),
            ("baseDir", self.name.clone()),
            ("dependencies", self.dependencies.join(",")),
        ];
        let temp_zip_file = env::temp_dir().join("starter.zip");
        download_file(
            "https://start.spring.io/starter.zip",
            &params,
            &temp_zip_file,
        )?;
        unzip(&temp_zip_file, &self.path)?;
        fs::remove_file(&temp_zip_file)?;
        self.editor.value.run(
            self.path.join(&self.name),
            format!(
                "src/main/{}/{}/{}/{}Application.{}",
                self.language.value.to_string().to_lowercase(),
                self.group_id.replace('.', "/"),
                self.artifact_id,
                self.artifact_id[0 .. 1].to_uppercase() + &self.artifact_id[1 ..],
                self.language.value.extension()
            ),
        )?;
        Ok(())
    }
}
