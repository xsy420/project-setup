use crate::editor::Editor;
use crate::vcs::Vcs;

use super::inner::{Inner, InnerHandleKeyEventOutput};
use super::{Appv2, focus::Focus};
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use reqwest::blocking::Client;
use std::path::PathBuf;
use std::{env, fs, io};
use zip::ZipArchive;

#[derive(Default)]
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
    kotlin_version: Option<String>,
    editor: Editor,
    vcs: Vcs,
    dependencies: Vec<String>,
    path: PathBuf,
    focus: Focus,
}

impl SpringBootInner {
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            generator: Generator::default(),
            group_id: String::new(),
            artifact_id: String::new(),
            boot_version: String::new(),
            java_version: String::new(),
            kotlin_version: None,
            editor: Editor::default(),
            vcs: Vcs::default(),
            dependencies: vec![String::new()],
            path: env::current_dir().unwrap(),
            focus: Focus::new(),
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
        for i in (0..labels.len()).step_by(2) {
            let (half, left, right) = (i / 2, i, i + 1);
            let line_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2); 2])
                .split(form_layout[half]);
            let (left_full_layout, right_full_layout) = (line_layout[0], line_layout[1]);
            // 标签
            let left_label_input_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(20), Constraint::Min(0)])
                .split(left_full_layout);
            f.render_widget(
                Paragraph::new(labels[left]).centered().block(
                    Block::default()
                        .borders(Borders::all())
                        .border_type(BorderType::Thick),
                ),
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(1), Constraint::Max(4), Constraint::Max(1)])
                    .split(left_label_input_area[0])[1],
            );
            f.render_widget(
                Paragraph::new(if left == 0 {
                    if self.name.is_empty() {
                        format!("Please input {}", labels[left])
                    } else {
                        self.name.to_string()
                    }
                } else {
                    format!("Please input {}", labels[left])
                })
                .style(if left == 0 && self.name.is_empty() {
                    Color::Gray
                } else {
                    Color::default()
                })
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(if left == 0 && !app.focus_left_side {
                            Color::Red
                        } else {
                            Color::default()
                        })
                        .border_type(BorderType::Thick),
                ),
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(1), Constraint::Max(4), Constraint::Max(1)])
                    .split(left_label_input_area[1])[1],
            );

            if right != labels.len() {
                let right_label_input_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Length(20), Constraint::Min(0)])
                    .split(right_full_layout);
                f.render_widget(
                    Paragraph::new(labels[right]).centered().block(
                        Block::default()
                            .borders(Borders::all())
                            .border_type(BorderType::Thick),
                    ),
                    Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Max(1), Constraint::Max(4), Constraint::Max(1)])
                        .split(right_label_input_area[0])[1],
                );

                f.render_widget(
                    Paragraph::new(format!("Please input {}", labels[right])).block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick),
                    ),
                    Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Max(1), Constraint::Max(4), Constraint::Max(1)])
                        .split(right_label_input_area[1])[1],
                );
            }
        }
    }
    fn bottom_help_message(&self) -> String {
        String::new()
    }
    fn handle_keyevent(&mut self, key: KeyEvent) -> InnerHandleKeyEventOutput {
        match key.code {
            KeyCode::Char(c) => {
                self.name.push(c);
            }
            KeyCode::Backspace => {
                self.name.pop();
            }
            KeyCode::Enter => {
                return InnerHandleKeyEventOutput::default().with_exited();
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

        println!("create {}", self.name);
        self.editor.run(
            self.path.join(&self.name),
            "src/main/java/com/example/demo/DemoApplication.java".to_string(),
        )?;
        Ok(())
    }
}
