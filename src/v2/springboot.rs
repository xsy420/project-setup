use crate::editor::Editor;
use crate::vcs::Vcs;

use super::inner::{Inner, InnerHandleKeyEventOutput};
use super::{Appv2, focus::Focus};
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup::LoopableNumberedEnum;
use ratatui::style::Color;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use reqwest::blocking::Client;
use std::{env, fmt::Debug, fs, io, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use zip::ZipArchive;

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

#[derive(Default, Debug, ToPrimitive, FromPrimitive, LoopableNumberedEnum)]
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
            kotlin_version: "2.1.20".to_string(),
            editor: Editor::default(),
            vcs: Vcs::default(),
            dependencies: vec![String::new()],
            path: env::current_dir().unwrap(),
            focus: Focus::new(),
            focus_index: 0,
        }
    }
    fn get_focus_field_mut(&mut self) -> Result<&mut String, String> {
        match SpringBootField::from_usize(self.focus_index).unwrap() {
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
        let split_input_error_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(1), Constraint::Max(4), Constraint::Max(1)]);
        for i in (0..labels.len()).step_by(2) {
            let line_layout = split_line_layout.split(form_layout[i / 2]);
            for side in 0..2 {
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
                    split_input_error_layout.split(label_input_area[0])[1],
                );
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
                    split_input_error_layout.split(label_input_area[1])[1],
                );
            }
        }
    }
    fn bottom_help_message(&self) -> String {
        "tab: focus next item | shift+tab: focus prev item | Enter: confirm to create project | "
            .to_string()
    }
    fn handle_keyevent(&mut self, key: KeyEvent) -> InnerHandleKeyEventOutput {
        let field_len = SpringBootField::iter().count();
        match key.code {
            KeyCode::Char(c) => match self.get_focus_field_mut() {
                Ok(x) => x.push(c),
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
                self.get_focus_field_mut().ok().unwrap().pop();
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
        let client = Client::new();
        let params = [
            ("type", "maven-project"),
            // ("language", &config.language.to_string().to_lowercase()),
            // ("javaVersion", &config.language_version),
            ("bootVersion", self.boot_version.as_str()),
            ("baseDir", self.name.as_str()),
        ];
        let bytes = client
            .post("https://start.spring.io/starter.zip")
            .form(&params)
            .send()
            .context("Failed to send request to Spring Boot starter")?
            .bytes()
            .context("Failed to read response bytes")?;

        // 直接在内存中解压 ZIP
        let mut archive =
            ZipArchive::new(io::Cursor::new(bytes)).context("Failed to parse ZIP archive")?;

        // 确保目标目录存在
        fs::create_dir_all(&self.path).context("Failed to create project directory")?;

        // 解压所有文件到目标目录
        archive
            .extract(&self.path)
            .context("Failed to extract ZIP archive")?;

        self.editor.run(
            self.path.join(&self.name),
            format!(
                "src/main/java/{}/{}/{}Application.java",
                self.group_id.replace('.', "/"),
                self.artifact_id,
                self.artifact_id[0..1].to_uppercase() + &self.artifact_id[1..]
            ),
        )?;
        Ok(())
    }
}
