use super::{CmakeInner, Inner, SpringBootInner, WipInner};
use crate::common::ProjectType;
use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, Paragraph},
};
use strum::IntoEnumIterator;
pub(crate) struct Application {
    selected: ProjectType,
    focus_left_side: bool,
    inners: Vec<Box<dyn Inner>>,
}
impl Application {
    pub(crate) fn new() -> Self {
        let inners: Vec<Box<dyn Inner>> = vec![
            Box::new(SpringBootInner::new()),
            Box::new(CmakeInner::new()),
            Box::new(WipInner {}),
            Box::new(WipInner {}),
        ];
        Self {
            selected: ProjectType::default(),
            focus_left_side: true,
            inners,
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
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
                    .title("Project Type")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if self.focus_left_side {
                        Color::Red
                    } else {
                        Color::LightBlue
                    })),
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
            &mut ListState::default().with_selected(Some(self.selected.num())),
        );
        let right_block = Block::default()
            .title(self.selected.to_string())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.focus_left_side {
                Color::LightBlue
            } else {
                Color::Red
            }));
        frame.render_widget(&right_block, main_layout[1]);
        let inner = self.inners[self.selected.num()].as_mut();
        inner.render(
            frame,
            !self.focus_left_side,
            right_block.inner(main_layout[1]),
        );
        // 底部帮助栏
        let help_bar = Paragraph::new(if self.focus_left_side {
            "j/k: move | Enter: choose | q: quit".to_string()
        } else {
            format!("{}Esc: focus back to left", inner.bottom_help_message())
        })
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
        let bottom_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(frame.area());
        frame.render_widget(help_bar, bottom_layout[1]);
    }

    pub(crate) fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;
            // 处理输入事件
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                if self.focus_left_side {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') => self.selected = self.selected.next(),
                        KeyCode::Char('k') => self.selected = self.selected.prev(),
                        KeyCode::Enter => self.focus_left_side = false,
                        _ => {}
                    }
                } else {
                    let inner = self.inners[self.selected.num()].as_mut();
                    let res = inner.handle_keyevent(key);
                    if !res.esc_handled && key.code == KeyCode::Esc {
                        self.focus_left_side = true;
                    }
                    if res.exit {
                        inner.create_and_edit()?;
                        return Ok(());
                    }
                }
            }
        }
    }
}
