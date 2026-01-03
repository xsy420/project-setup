use super::SpringBootInner;
use crate::common::ExecutableEnum;
use anyhow::Result;
use num_traits::ToPrimitive;
use ratatui::{
    Terminal,
    layout::Layout,
    prelude::*,
    style::{Style, palette::tailwind},
    widgets::{Block, LineGauge, Paragraph},
};
use ratatui_macros::constraints;
use tokio::sync::mpsc;
#[derive(Clone, Copy)]
pub struct PrepareRecv {
    step: f64,
    offset: f64,
}
impl PrepareRecv {
    #[must_use]
    pub fn new(offset: f64) -> Self {
        Self { step: 0., offset }
    }

    fn step(&self) -> f64 {
        self.step + 1.
    }

    pub fn next_step(&mut self) -> &mut Self {
        self.step += 1.;
        self
    }

    /// # Panics
    pub fn send(self, permit: &mut mpsc::PermitIterator<'_, Self>) {
        permit.next().unwrap().send(self);
    }
}
pub trait PrepareTrait {
    fn prepare(
        permit: &mut mpsc::PermitIterator<'_, PrepareRecv>,
        offset: f64,
    ) -> impl Future<Output = ()> + Send;
    fn descs() -> Vec<String>;
}
#[derive(Default)]
struct PrepareProgress {
    progress: f64,
}
impl PrepareProgress {
    fn recv(&mut self, recv: &PrepareRecv) -> &mut Self {
        self.progress = (recv.step() / self.descs().len().to_f64().unwrap() + recv.offset)
            / PrepareHeader::f64_len();
        self
    }

    fn descs(&self) -> Vec<String> {
        [ExecutableEnum::descs(), SpringBootInner::descs()]
            .get(self.header_offset())
            .unwrap()
            .clone()
    }

    fn desc(&self) -> String {
        let index = self.remove_header_offset()
            * self.descs().len().to_f64().unwrap()
            * PrepareHeader::f64_len();
        self.descs()
            .get(if index == 0. {
                0
            } else {
                index.to_usize().unwrap() - 1
            })
            .unwrap()
            .clone()
    }

    fn preparing(&self) -> bool {
        (self.progress - 1.).abs() != 0.
    }

    fn remove_header_offset(&self) -> f64 {
        (self.progress * PrepareHeader::f64_len()).fract() / PrepareHeader::f64_len()
    }

    fn header_offset(&self) -> usize {
        (self.progress * PrepareHeader::f64_len())
            .trunc()
            .to_usize()
            .unwrap()
    }

    fn header_text(&self) -> &'static str {
        let header_index = (self.progress * PrepareHeader::f64_len())
            .trunc()
            .to_usize()
            .unwrap();
        if header_index == PrepareHeader::lists().len() {
            "Done"
        } else {
            PrepareHeader::lists()[header_index]
        }
    }

    fn percent(&mut self) -> String {
        format!("{:.2}%", self.progress * 100.)
    }
}
struct PrepareHeader {}
impl PrepareHeader {
    fn lists() -> &'static [&'static str] {
        &["Environment checking", "Inner preparing"]
    }

    fn f64_len() -> f64 {
        Self::lists().len().to_f64().unwrap()
    }
}
#[derive(Default)]
pub struct PrepareApplication {
    progress: PrepareProgress,
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
        let buffer = ExecutableEnum::descs().len() + SpringBootInner::descs().len();
        let (tx, mut rx) = mpsc::channel(buffer);
        tokio::spawn(async move {
            let mut permit = tx.reserve_many(buffer).await.unwrap();
            // Environment checking spawn
            ExecutableEnum::prepare(&mut permit, 0.).await;
            // Inner preparing spawn
            SpringBootInner::prepare(&mut permit, 1.).await;
        });
        while self.progress.preparing() {
            terminal.draw(|f| self.ui(f))?;
            if let Ok(progress) = rx.try_recv() {
                self.progress.recv(&progress);
            }
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
}
