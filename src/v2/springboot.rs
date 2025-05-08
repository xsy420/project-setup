use super::render::Render;
use super::{Appv2, focus::Focus};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

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
    dependencies: Vec<String>,
    path: String,
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
            dependencies: vec![String::new()],
            path: String::new(),
            focus: Focus::new(),
        }
    }
}

impl Render for SpringBootInner {
    fn render(&self, f: &mut Frame, _app: &Appv2, area: Rect) {
        let labels = [
            "name",
            "generator",
            "group_id",
            "artifact_id",
            "boot_version",
            "java_version",
            "kotlin_version",
            "dependencies",
            "path",
        ];
        // 表单布局 - 垂直排列输入框
        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6); 5])
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
                Paragraph::new(format!("Please input {}", labels[left])).block(
                    Block::new()
                        .borders(Borders::ALL)
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
}
