use crate::common::{
    AppDirection, Editor, FocusInput, Language, ProjectConfig, ProjectType, Vcs, create_project,
};
use anyhow::Result;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::{Block, List, ListItem, ListState, Paragraph},
};
use std::rc::Rc;
use strum::IntoEnumIterator;
type SwitchItemInListState<T> = dyn Fn(AppDirection, &mut ListState, Vec<T>) -> T;
enum InputMode {
    Normal,
    Editing,
}
pub(crate) struct ProjectSetupApp {
    project_type_state: ListState,
    project_version_state: ListState,
    language_state: ListState,
    language_version_state: ListState,
    vcs_state: ListState,
    editor_state: ListState,
    config: ProjectConfig,
    input_mode: InputMode,
    // [0]: project_type
    // [1]: version
    // [2]: language
    // [3]: language_version
    // [4]: vcs
    // [5]: editor
    // [6]: name
    show: [bool; 7],
    msg: String,
    focus: FocusInput,
}
impl ProjectSetupApp {
    pub(crate) fn new() -> Self {
        Self {
            project_type_state: ListState::default().with_selected(Some(0)),
            project_version_state: ListState::default().with_selected(Some(0)),
            language_state: ListState::default().with_selected(Some(0)),
            language_version_state: ListState::default().with_selected(Some(0)),
            vcs_state: ListState::default().with_selected(Some(0)),
            editor_state: ListState::default().with_selected(Some(0)),
            config: ProjectConfig::default(),
            input_mode: InputMode::Normal,
            show: [true, false, false, false, false, false, false],
            msg: String::new(),
            focus: FocusInput::ProjectType,
        }
    }

    fn select_next(&mut self) {
        self.nop(AppDirection::Next);
    }

    fn select_prev(&mut self) {
        self.nop(AppDirection::Prev);
    }

    fn nop(&mut self, ad: AppDirection) {
        match self.focus {
            FocusInput::ProjectType => {
                self.config.project_type = Self::generic_nav_fn::<ProjectType>()(
                    ad,
                    &mut self.project_type_state,
                    ProjectType::iter().collect(),
                );
            }
            FocusInput::ProjectVersion => {
                self.config.project_version = Self::generic_nav_fn::<String>()(
                    ad,
                    &mut self.project_version_state,
                    self.config.project_type.versions(),
                );
            }
            FocusInput::Language => {
                self.config.language = Self::generic_nav_fn::<Language>()(
                    ad,
                    &mut self.language_state,
                    self.config.project_type.languages(),
                );
            }
            FocusInput::LanguageVersion => {
                self.config.language_version = Self::generic_nav_fn::<String>()(
                    ad,
                    &mut self.language_version_state,
                    self.config.language.versions(),
                );
            }
            FocusInput::Vcs => {
                self.config.vcs =
                    Self::generic_nav_fn::<Vcs>()(ad, &mut self.vcs_state, Vcs::iter().collect());
            }
            FocusInput::Editor => {
                self.config.editor = Self::generic_nav_fn::<Editor>()(
                    ad,
                    &mut self.editor_state,
                    Editor::iter().collect(),
                );
            }
            _ => {}
        }
    }

    fn generic_nav_fn<T: Clone>() -> Box<SwitchItemInListState<T>> {
        Box::new(|x, y, z| {
            let i = x.get_counter(y.selected(), z.len());
            y.select(Some(i));
            z[i].clone()
        })
    }

    fn focus_next(&mut self) {
        self.focus = self.focus.next();
        self.show[self.focus.num()] = true;
    }

    fn focus_prev(&mut self) {
        self.show[self.focus.num()] = false;
        self.focus = self.focus.prev();
    }
}
pub(crate) fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut ProjectSetupApp,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') => app.select_next(),
                        KeyCode::Char('k') => app.select_prev(),
                        KeyCode::Enter => {
                            app.focus_next();
                            match app.focus {
                                FocusInput::ProjectVersion => {
                                    app.project_version_state.select(Some(0));
                                    app.config.project_version =
                                        app.config.project_type.versions()[0].to_string();
                                }
                                FocusInput::Language => {
                                    app.language_state.select(Some(0));
                                    app.config.language = app.config.project_type.languages()[0];
                                }
                                FocusInput::LanguageVersion => {
                                    app.language_version_state.select(Some(0));
                                    app.config
                                        .language_version
                                        .clone_from(&app.config.language.versions()[0]);
                                }
                                FocusInput::Vcs => {
                                    app.vcs_state.select(Some(0));
                                    app.config.vcs = Vcs::default();
                                }
                                FocusInput::Editor => {
                                    app.editor_state.select(Some(0));
                                    app.config.editor = Editor::default();
                                }
                                _ => {}
                            }
                            if app.focus == FocusInput::Name {
                                app.input_mode = InputMode::Editing;
                                if app.config.name.is_empty() {
                                    app.msg = "Empty Project Name".to_string();
                                }
                            }
                        }
                        KeyCode::Esc => {
                            if app.focus == FocusInput::ProjectType {
                                return Ok(());
                            }
                            app.focus_prev();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.msg.clear();
                            app.focus_prev();
                        }
                        KeyCode::Enter => {
                            // 在这里执行项目创建逻辑
                            if app.config.name.is_empty() {
                                app.msg = "Empty Project Name".to_string();
                            } else {
                                let project_path = app.config.path.join(&app.config.name);
                                let main_file = create_project(&app.config)?;
                                app.config.editor.run(project_path, main_file)?;
                                return Ok(());
                            }
                        }
                        KeyCode::Char(c) => {
                            app.config.name.push(c);
                            app.msg.clear();
                        }
                        KeyCode::Backspace => {
                            app.config.name.pop();
                            if app.config.name.is_empty() {
                                app.msg = "Empty Project Name".to_string();
                            }
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
fn ui(f: &mut Frame, app: &ProjectSetupApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(u16::try_from(FocusInput::iter().count()).unwrap())
        .constraints(FocusInput::constraint())
        .split(f.area());
    focus_list_item_ui(f, app, FocusInput::ProjectType, &chunks);
    focus_list_item_ui(f, app, FocusInput::ProjectVersion, &chunks);
    focus_list_item_ui(f, app, FocusInput::Language, &chunks);
    focus_list_item_ui(f, app, FocusInput::LanguageVersion, &chunks);
    focus_list_item_ui(f, app, FocusInput::Vcs, &chunks);
    focus_list_item_ui(f, app, FocusInput::Editor, &chunks);
    if app.show[FocusInput::Name.num()] {
        // 输入框样式
        let input_style = if app.config.name.is_empty() {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        };
        // 项目信息显示
        let info = format!(
            "{} {} project: {}",
            app.config.project_type, app.config.project_version, app.config.name
        );
        let paragraph = Paragraph::new(info)
            .block(app.focus.focus_border(FocusInput::Name).style(input_style))
            .style(input_style);
        f.render_widget(paragraph, chunks[FocusInput::Name.num()]);
    }
    if !app.msg.is_empty() {
        let error_text =
            Text::from(Line::from(app.msg.as_str())).style(Style::default().fg(Color::Red));
        let error_paragraph = Paragraph::new(error_text).alignment(Alignment::Center);
        f.render_widget(error_paragraph, chunks[FocusInput::ErrorMessage.num()]);
    }
    // 输入模式指示器
    let mode_text = match app.input_mode {
        InputMode::Normal => {
            format!(
                "NORMAL MODE: q=quit, j/k=move, Esc={}, Enter={}",
                if app.focus.num() > 0 {
                    "focus previous one"
                } else {
                    "quit"
                },
                if app.focus == FocusInput::Name {
                    "edit"
                } else {
                    "focus next one"
                }
            )
        }
        InputMode::Editing => "EDIT MODE: Esc=exit edit, Enter=create project".to_string(),
    };
    let mode_indicator = Paragraph::new(mode_text).block(Block::default());
    f.render_widget(mode_indicator, chunks[FocusInput::Bottom.num()]);
}
fn focus_list_item_ui(
    f: &mut Frame,
    app: &ProjectSetupApp,
    focus: FocusInput,
    chunks: &Rc<[Rect]>,
) {
    if app.show[focus.num()] {
        let items: Vec<ListItem> = match focus {
            FocusInput::ProjectType => Some(ProjectType::iter().map(|x| x.to_string()).collect()),
            FocusInput::ProjectVersion => Some(app.config.project_type.versions()),
            FocusInput::Language => Some(
                app.config
                    .project_type
                    .languages()
                    .iter()
                    .map(Language::to_string)
                    .collect(),
            ),
            FocusInput::LanguageVersion => Some(app.config.language.versions()),
            FocusInput::Vcs => Some(
                Vcs::iter()
                    .filter(|x| x.is_available())
                    .map(|x| x.to_string())
                    .collect(),
            ),
            FocusInput::Editor => Some(
                Editor::iter()
                    .filter(Editor::is_available)
                    .map(|x| x.to_string())
                    .collect(),
            ),
            _ => None,
        }
        .unwrap()
        .iter()
        .map(|i| ListItem::new(i.to_string()).style(Style::default()))
        .collect();
        let list = List::new(items)
            .block(app.focus.focus_border(focus))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        f.render_stateful_widget(
            list,
            chunks[focus.num()],
            &mut match focus {
                FocusInput::ProjectType => Some(app.project_type_state.clone()),
                FocusInput::ProjectVersion => Some(app.project_version_state.clone()),
                FocusInput::Language => Some(app.language_state.clone()),
                FocusInput::LanguageVersion => Some(app.language_version_state.clone()),
                FocusInput::Vcs => Some(app.vcs_state.clone()),
                FocusInput::Editor => Some(app.editor_state.clone()),
                _ => None,
            }
            .unwrap(),
        );
    }
}
