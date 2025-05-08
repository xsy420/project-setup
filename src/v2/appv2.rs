use crate::focus_input::FocusInput;
use crate::project_type::ProjectType;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, Terminal,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, Paragraph},
};
use strum::IntoEnumIterator;

use super::{render::Render, springboot::SpringBootInner, wip::WipInner};

pub(crate) struct Appv2 {
    pub(crate) selected: ProjectType,
}

impl Appv2 {
    pub(crate) fn new() -> Self {
        Self {
            selected: ProjectType::default(),
        }
    }
}

fn ui(frame: &mut Frame, app: &Appv2) {
    // 主布局 - 水平分割为左右两部分
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(10)])
        .split(frame.area());
    let items: Vec<String> = ProjectType::iter().map(|x| x.to_string()).collect();

    // 左侧列表
    let list = List::new(items)
        .block(
            Block::default()
                .title(FocusInput::ProjectType.title())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("» ");

    frame.render_stateful_widget(
        list,
        main_layout[0],
        &mut ListState::default().with_selected(Some(app.selected.num())),
    );
    let render: &dyn Render = match app.selected {
        ProjectType::SpringBoot => &SpringBootInner::new(),
        _ => &WipInner {},
    };
    render.render(frame, main_layout[1]);

    // 底部帮助栏
    let help_bar = Paragraph::new("j/k: move | Enter: choose | q: quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    let bottom_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    frame.render_widget(help_bar, bottom_layout[1]);
}

pub(crate) fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Appv2) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        // 处理输入事件
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => app.selected = app.selected.next(),
                KeyCode::Char('k') => app.selected = app.selected.prev(),
                // KeyCode::Enter => {
                //     // 这里可以添加选择确认的逻辑
                // }
                _ => {}
            }
        }
    }
}
