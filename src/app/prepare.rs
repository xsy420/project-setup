use super::SpringBootInner;
use crate::common::Executable;
use anyhow::{Error, Result};
use num_traits::ToPrimitive;
use ratatui::{
    Terminal,
    crossterm::event::{self, KeyCode},
    layout::Layout,
    prelude::*,
    style::{Style, palette::tailwind},
    widgets::{Block, LineGauge, Paragraph},
};
use ratatui_macros::constraints;
use tokio::sync::mpsc;
#[derive(Clone, Copy)]
pub(crate) struct PrepareRecv {
    step: usize,
    offset: f64,
}
impl PrepareRecv {
    pub(crate) fn new(offset: f64) -> Self {
        Self {
            step: usize::from(offset != 0.),
            offset,
        }
    }

    fn step(&self) -> f64 {
        (self.step + 1).to_f64().unwrap()
    }

    fn len(&self) -> usize {
        PrepareTexts::descs()[self.offset.trunc().to_usize().unwrap()].len()
    }

    fn next_step(&mut self) -> &mut Self {
        self.step += usize::from(self.step != self.len());
        self
    }

    pub(crate) fn send_done(&mut self, permit: &mut PreparePermit<'_>) {
        self.step = self.len();
        self.send(permit, None);
    }

    pub(crate) fn send_ok(&mut self, permit: &mut PreparePermit<'_>) {
        self.send(permit, None);
    }

    pub(crate) fn send_error(&mut self, permit: &mut PreparePermit<'_>, error: Error) {
        self.send(permit, Some(error));
    }

    fn send(&mut self, permit: &mut PreparePermit<'_>, error: Option<Error>) {
        if let Some(error) = error {
            permit.next().unwrap().send(Err(error));
        } else {
            permit.next().unwrap().send(Ok(*self));
            self.next_step();
        }
    }
}
pub(crate) trait PrepareTrait {
    fn prepare(permit: &mut PreparePermit<'_>, offset: f64) -> impl Future<Output = bool> + Send;
    fn descs() -> Vec<String>;
}
pub(crate) type PreparePermit<'a> = mpsc::PermitIterator<'a, Result<PrepareRecv>>;
#[derive(Default)]
struct PrepareProgress {
    progress: f64,
    err_msg: String,
}
impl PrepareProgress {
    fn recv(&mut self, recv: Result<PrepareRecv>) -> &mut Self {
        match recv {
            Ok(recv) => {
                self.progress = (recv.step() / PrepareTexts::f64_descs_len(self.header_offset())
                    + recv.offset)
                    / PrepareTexts::f64_headers_len();
                self
            }
            Err(error) => {
                self.err_msg = error.to_string();
                println!("{error}");
                self
            }
        }
    }

    fn desc(&self) -> String {
        let index = self.remove_header_offset()
            * PrepareTexts::f64_descs_len(self.header_offset())
            * PrepareTexts::f64_headers_len();
        PrepareTexts::descs()[self.header_offset()][if index == 0. {
            0
        } else {
            index.to_usize().unwrap() - 1
        }]
        .clone()
    }

    fn preparing(&self) -> bool {
        (self.progress - 1.).abs() != 0.
    }

    fn remove_header_offset(&self) -> f64 {
        (self.progress * PrepareTexts::f64_headers_len()).fract() / PrepareTexts::f64_headers_len()
    }

    fn header_offset(&self) -> usize {
        (self.progress * PrepareTexts::f64_headers_len())
            .trunc()
            .to_usize()
            .unwrap()
    }

    fn header_text(&self) -> &'static str {
        let header_index = (self.progress * PrepareTexts::f64_headers_len())
            .trunc()
            .to_usize()
            .unwrap();
        if header_index == PrepareTexts::headers().len() {
            "Done"
        } else {
            PrepareTexts::headers()[header_index]
        }
    }

    fn percent(&mut self) -> String {
        format!("{:.2}%", self.progress * 100.)
    }
}
struct PrepareTexts {}
impl PrepareTexts {
    fn headers() -> &'static [&'static str] {
        &["Environment checking", "Inner preparing"]
    }

    fn descs() -> Vec<Vec<String>> {
        vec![Executable::descs(), SpringBootInner::descs()]
    }

    fn f64_headers_len() -> f64 {
        Self::headers().len().to_f64().unwrap()
    }

    fn f64_descs_len(offset: usize) -> f64 {
        Self::descs()[offset].len().to_f64().unwrap()
    }
}
#[derive(Default, PartialEq)]
enum PrepareStatus {
    #[default]
    Start,
    Stop,
    Quit,
}
impl PrepareStatus {
    fn running(&self) -> bool {
        self != &Self::Quit
    }

    fn stopped(&self) -> bool {
        self == &Self::Stop
    }
}
#[derive(Default)]
pub struct PrepareApplication {
    progress: PrepareProgress,
    status: PrepareStatus,
}
impl PrepareApplication {
    /// # Errors
    /// # Panics
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()>
    where
        <B as Backend>::Error: Sync,
        <B as Backend>::Error: Send,
        <B as Backend>::Error: 'static,
    {
        let buffer = PrepareTexts::descs().iter().map(Vec::len).sum();
        let (tx, mut rx) = mpsc::channel(buffer);
        tokio::spawn(async move {
            let mut permit = tx.reserve_many(buffer).await.unwrap();
            // Environment checking spawn
            let preparing = Executable::prepare(&mut permit, 0.).await;
            // Inner preparing spawn
            if preparing {
                SpringBootInner::prepare(&mut permit, 1.).await;
            }
        });
        while self.status.running() && self.progress.preparing() {
            terminal.draw(|f| self.ui(f))?;
            self.handle_event()?;
            self.update(&mut rx);
        }
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        let prepare_h_area = Layout::horizontal(constraints![*=1,==25%,*=1]).split(frame.area())[1];
        let prepare_area = Layout::vertical(constraints![*=1,==20%,*=1]).split(prepare_h_area)[1];
        frame.render_widget(
            Paragraph::default().centered().block(Block::bordered()),
            prepare_area,
        );
        let prepare_split_area =
            Layout::vertical(constraints![==1/3;3]).split(prepare_area.inner(Margin::new(1, 1)));
        frame.render_widget(
            Paragraph::new(self.progress.header_text()).centered(),
            prepare_split_area[0],
        );
        frame.render_widget(
            Paragraph::new(self.progress.desc()).centered(),
            prepare_split_area[1],
        );
        frame.render_widget(
            LineGauge::default()
                .filled_symbol("⣿")
                .unfilled_symbol("⣿")
                .filled_style(Style::default().fg(tailwind::CYAN.c400))
                .unfilled_style(Style::default().fg(tailwind::CYAN.c800))
                .label(self.progress.percent())
                .ratio(self.progress.progress),
            prepare_split_area[2],
        );
    }

    fn handle_event(&mut self) -> Result<()> {
        if self.status.stopped()
            && let Some(key) = event::read()?.as_key_press_event()
            && let KeyCode::Char(c) = key.code
        {
            match c {
                'q' => {
                    std::process::exit(0);
                }
                'r' => {
                    // retry download
                    todo!()
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn update(&mut self, rx: &mut mpsc::Receiver<Result<PrepareRecv>>) {
        if self.progress.err_msg.is_empty() {
            if let Ok(recv) = rx.try_recv() {
                self.progress.recv(recv);
            }
        } else {
            self.status = PrepareStatus::Stop;
        }
    }
}
