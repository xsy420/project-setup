use super::{
    CargoInner, CmakeInner, SpringBootInner, WipInner,
    inner::{Inner, PrepareInner},
};
use crate::{Args, common::ProjectType};
use anyhow::Result;
use clap::Parser;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Layout,
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListState, Paragraph},
};
use ratatui_macros::constraints;
use strum::IntoEnumIterator;
use tokio::sync::mpsc;
pub struct Application {
    selected: ProjectType,
    focus_left_side: bool,
    inners: Vec<Box<dyn Inner>>,
    default_inner: Option<ProjectType>,
}
impl Application {
    #[must_use]
    pub fn new(args: Args) -> Self {
        let inners: Vec<Box<dyn Inner>> = vec![
            Box::new(SpringBootInner::new()),
            Box::new(CmakeInner::new()),
            Box::new(WipInner {}),
            Box::new(CargoInner::new()),
        ];
        Self {
            selected: ProjectType::default(),
            focus_left_side: args.project_type.is_none(),
            inners,
            default_inner: args.project_type,
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        if let Some(default_inner) = self.default_inner {
            let inner = self.inners[default_inner.num()].as_mut();
            inner.render(frame, true, frame.area());
            // 底部帮助栏
            let help_bar = Paragraph::new(if self.focus_left_side {
                "j/k: move | Enter: choose | q: quit".to_string()
            } else {
                format!("{}q: quit", inner.bottom_help_message())
            })
            .style(Style::default().fg(Color::Gray))
            .centered();
            let bottom_layout = Layout::vertical(constraints![>=0,==1]).split(frame.area());
            frame.render_widget(help_bar, bottom_layout[1]);
        } else {
            // 主布局 - 水平分割为左右两部分
            let main_layout = Layout::horizontal(constraints![==20,>=10]).split(frame.area());
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
            .centered();
            let bottom_layout = Layout::vertical(constraints![>=0,==1]).split(frame.area());
            frame.render_widget(help_bar, bottom_layout[1]);
        }
    }

    fn prepare_inner<B: Backend, F: Fn() -> bool, P>(
        terminal: &mut Terminal<B>,
        pt: ProjectType,
        is_prepared: F,
        prepare: P,
    ) -> Result<()>
    where
        P: FnOnce(
                mpsc::Sender<u16>,
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + 'static,
        <B as Backend>::Error: Sync,
        <B as Backend>::Error: Send,
        <B as Backend>::Error: 'static,
    {
        if let Some(p) = Args::parse().project_type
            && p != pt
        {
            return Ok(());
        }
        let (tx, mut rx) = mpsc::channel(100);
        tokio::spawn(async move {
            prepare(tx).await;
        });
        let mut progress = 0;
        while !is_prepared() {
            terminal.draw(|f| {
                let gauge = Gauge::default()
                    .block(Block::default().title("下载进度").borders(Borders::ALL))
                    .gauge_style(Style::default().fg(Color::Yellow))
                    .percent(progress);
                f.render_widget(gauge, f.area());
                // 检查通道中的进度更新
                if let Ok(new_progress) = rx.try_recv() {
                    progress = new_progress;
                }
            })?;
        }
        Ok(())
    }

    /// # Errors
    pub fn prepare_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()>
    where
        <B as Backend>::Error: Sync,
        <B as Backend>::Error: Send,
        <B as Backend>::Error: 'static,
    {
        Self::prepare_inner(
            terminal,
            ProjectType::SpringBoot,
            SpringBootInner::is_prepared,
            |tx| Box::pin(SpringBootInner::prepare(tx)),
        )
    }

    /// # Errors
    /// # Panics
    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<()>
    where
        <B as Backend>::Error: Send,
        <B as Backend>::Error: Sync,
        <B as Backend>::Error: 'static,
    {
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
                        KeyCode::Char('j') => {
                            if self.default_inner.is_none() {
                                self.selected = self.selected.next();
                            }
                        }
                        KeyCode::Char('k') => {
                            if self.default_inner.is_none() {
                                self.selected = self.selected.prev();
                            }
                        }
                        KeyCode::Enter => self.focus_left_side = false,
                        _ => {}
                    }
                } else {
                    let inner = self.inners[if let Some(default_inner) = self.default_inner {
                        default_inner
                    } else {
                        self.selected
                    }
                    .num()]
                    .as_mut();
                    let res = inner.handle_keyevent(key);
                    if !res.esc_handled && self.default_inner.is_none() && key.code == KeyCode::Esc
                    {
                        self.focus_left_side = true;
                    }
                    if res.exit {
                        inner.create_and_edit()?;
                        return Ok(());
                    }
                    if self.default_inner.is_some() && key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
        }
    }
}
