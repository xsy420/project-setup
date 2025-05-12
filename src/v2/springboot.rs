use super::{Appv2, Focus, Inner, InnerHandleKeyEventOutput};
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
use std::{env, fmt::Debug, fs, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
#[derive(Debug, Clone, Copy, FromPrimitive, EnumIter)]
enum SpringBootField {
    Name,
    Generator,
    GroupId,
    ArtifactId,
    BootVersion,
    JavaVersion,
    KotlinVersion,
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
#[derive(Default, Display, Debug, ToPrimitive, FromPrimitive, LoopableNumberedEnum)]
#[numbered_enum(loop_within = 2)]
#[allow(dead_code)]
enum Generator {
    #[default]
    Maven,
    Gradle,
}
#[allow(dead_code)]
pub(crate) struct SpringBootInner {
    name: String,
    generator: Generator,
    group_id: String,
    artifact_id: String,
    boot_version: String,
    java_version: String,
    kotlin_version: String,
    editor: Editor,
    vcs: Vcs,
    dependencies: Vec<String>,
    path: PathBuf,
    focus: Focus,
    focus_index: usize,
    tip_messages: Vec<String>,
    error_messages: Vec<String>,
}
impl SpringBootInner {
    pub(crate) fn new() -> Self {
        Self {
            name: "demo".to_string(),
            generator: Generator::default(),
            group_id: "com.example".to_string(),
            artifact_id: "demo".to_string(),
            boot_version: "3.3.0".to_string(),
            java_version: "17".to_string(),
            kotlin_version: String::new(),
            editor: Editor::default(),
            vcs: Vcs::default(),
            dependencies: vec![String::new()],
            path: env::current_dir().unwrap(),
            focus: Focus::new(),
            focus_index: 0,
            tip_messages: [
                "Please input the name of this project",
                "Use j/k to scroll between Maven or Gradle",
                "Please input the group_id of this project",
                "Please input the artifact_id of this project",
                "Please input the boot_version of this project",
                "Please input the java_version of this project",
                "Please input the kotlin_version of this project",
                "Use j/k to scroll editor",
                "Use j/k to scroll Vcs tool",
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
            SpringBootField::JavaVersion => Ok(&mut self.java_version),
            SpringBootField::KotlinVersion => Ok(&mut self.kotlin_version),
            _ => Err(String::new()),
        }
    }

    fn get_field(&self, field: SpringBootField) -> &dyn Debug {
        match field {
            SpringBootField::Name => &self.name,
            SpringBootField::Generator => &self.generator,
            SpringBootField::GroupId => &self.group_id,
            SpringBootField::ArtifactId => &self.artifact_id,
            SpringBootField::BootVersion => &self.boot_version,
            SpringBootField::JavaVersion => &self.java_version,
            SpringBootField::KotlinVersion => &self.kotlin_version,
            SpringBootField::Editor => &self.editor,
            SpringBootField::Vcs => &self.vcs,
            SpringBootField::Dependencies => &self.dependencies,
            SpringBootField::Path => &self.path,
        }
    }
}
impl Inner for SpringBootInner {
    fn render(&self, f: &mut Frame, app: &Appv2, area: Rect) {
        let labels = [
            "name",
            "generator",
            "group_id",
            "artifact_id",
            "boot_version",
            "java_version",
            "kotlin_version",
            "editor",
            "vcs",
            "dependencies",
            "path",
        ];
        // 表单布局 - 垂直排列输入框
        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6); 6])
            .split(area);
        let split_line_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2); 2]);
        let split_label_input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(20), Constraint::Min(0)]);
        let split_tip_input_error_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(1), Constraint::Max(4), Constraint::Max(1)]);
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
                if !app.focus_left_side && index == self.focus_index {
                    f.render_widget(
                        Paragraph::new(self.tip_messages[index].clone())
                            .style(Color::Blue)
                            .centered(),
                        split_tip_input_error_layout.split(label_input_area[1])[0],
                    );
                }
                let field_value = self.get_field(SpringBootField::from_usize(index).unwrap());
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
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_style(if index == self.focus_index && !app.focus_left_side {
                                Color::Red
                            } else {
                                Color::default()
                            })
                            .border_type(BorderType::Thick),
                    ),
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
            KeyCode::Char(c) => match self.get_focus_field_mut(field) {
                Ok(x) => {
                    x.push(c);
                    self.error_messages[self.focus_index] = field.vaildate_string(x);
                }
                Err(_) => match self.focus_index {
                    1 => match c {
                        'j' => self.generator = self.generator.next(),
                        'k' => self.generator = self.generator.prev(),
                        _ => {}
                    },
                    7 => match c {
                        'j' => self.editor = self.editor.next(),
                        'k' => self.editor = self.editor.prev(),
                        _ => {}
                    },
                    8 => match c {
                        'j' => self.vcs = self.vcs.next(),
                        'k' => self.vcs = self.vcs.prev(),
                        _ => {}
                    },
                    _ => unreachable!(),
                },
            },
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
            _ => {}
        }
        InnerHandleKeyEventOutput::default()
    }

    fn create_and_edit(&self) -> Result<()> {
        let project_path = self.path.join(&self.name);
        fs::create_dir_all(&project_path)?;
        self.vcs.init_vcs_repo(&self.name, &self.path)?;
        let language = if self.kotlin_version.is_empty() {
            "java"
        } else {
            "kotlin"
        };
        let extension = if self.kotlin_version.is_empty() {
            "kt"
        } else {
            "java"
        };
        let params = [
            ("groupId", self.group_id.as_str()),
            ("artifactId", self.artifact_id.as_str()),
            (
                "type",
                &format!("{}-project", self.generator.to_string().to_lowercase()),
            ),
            ("name", self.name.as_str()),
            ("language", language),
            ("javaVersion", self.java_version.as_str()),
            ("bootVersion", self.boot_version.as_str()),
            ("baseDir", self.name.as_str()),
            ("dependencies", &self.dependencies.join(",")),
        ];
        let temp_zip_file = env::temp_dir().join("starter.zip");
        download_file(
            "https://start.spring.io/starter.zip",
            &params,
            &temp_zip_file,
        )?;
        unzip(&temp_zip_file, &self.path)?;
        fs::remove_file(&temp_zip_file)?;
        self.editor.run(
            self.path.join(&self.name),
            format!(
                "src/main/{}/{}/{}/{}Application.{}",
                language,
                self.group_id.replace('.', "/"),
                self.artifact_id,
                self.artifact_id[0 .. 1].to_uppercase() + &self.artifact_id[1 ..],
                extension
            ),
        )?;
        Ok(())
    }
}
