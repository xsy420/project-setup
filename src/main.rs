use std::{
    env::{self},
    fmt::Debug,
    fs::{self},
    io::{self},
    path::PathBuf,
    rc::Rc,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    prelude::*,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};
use reqwest::blocking::Client;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use zip::ZipArchive;

#[derive(Debug, Clone, Default, EnumIter)]
enum ProjectType {
    #[default]
    SpringBoot,
    CMake,
    Maven,
    Python,
    Cargo,
}

impl ProjectType {
    fn desc(&self) -> String {
        match self {
            Self::SpringBoot => "Spring Boot",
            Self::CMake => "CMake",
            Self::Python => "Python",
            Self::Maven => "Maven",
            Self::Cargo => "Cargo",
        }
        .to_string()
    }
    fn versions(&self) -> Vec<String> {
        match self {
            Self::SpringBoot => vec!["3.3.0", "3.4.0", "3.5.0"],
            Self::CMake => vec!["3.25", "3.26", "4.0", "4.1"],
            Self::Python => vec!["3.13", "3.12", "3.11", "3.10", "3.9", "3.8"],
            Self::Maven => vec!["3.9.9"],
            Self::Cargo => vec!["1.86.0"],
        }
        .iter()
        .map(ToString::to_string)
        .collect()
    }
    fn languages(&self) -> Vec<String> {
        match self {
            Self::CMake => vec!["C", "CPP"],
            Self::Python => vec!["Python"],
            Self::Maven | Self::SpringBoot => vec!["Java", "Kotlin"],
            Self::Cargo => vec!["Rust"],
        }
        .iter()
        .map(ToString::to_string)
        .collect()
    }
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
enum ProjectPackaging {
    #[default]
    NotNeed,
    Alpine,
    ArchLinux,
    Fedora,
    Gentoo,
    Nix,
    Ubuntu,
}

#[allow(dead_code)]
impl ProjectPackaging {
    fn desc(&self) -> &str {
        match self {
            Self::Alpine => "Alpine",
            Self::ArchLinux => "ArchLinux",
            Self::Fedora => "Fedora",
            Self::Gentoo => "Gentoo",
            Self::Nix => "Nix",
            Self::Ubuntu => "Ubuntu",
            Self::NotNeed => "NotNeed",
        }
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
enum Vcs {
    #[default]
    NotNeed,
    Git,
    Svn,
}

#[allow(dead_code)]
impl Vcs {
    fn init(&self) -> String {
        match self {
            Self::NotNeed => "",
            Self::Git => "git init",
            Self::Svn => "svn create {}",
        }
        .to_string()
    }

    fn is_available(&self) -> bool {
        match self {
            Self::NotNeed => true,
            Self::Git | Self::Svn => false,
        }
    }

    fn url(&self) -> String {
        match self {
            Self::NotNeed => "",
            Self::Git => "git init",
            Self::Svn => "svn create {}",
        }
        .to_string()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProjectConfig {
    name: String,
    version: String,
    path: PathBuf,
    language: String,
    project_type: ProjectType,
    packaging: ProjectPackaging,
    vcs: Vcs,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            language: String::new(),
            version: String::new(),
            path: env::current_dir().unwrap(),
            project_type: ProjectType::default(),
            packaging: ProjectPackaging::default(),
            vcs: Vcs::default(),
        }
    }
}

#[derive(Clone, Copy, EnumIter)]
enum FocusInput {
    ProjectType,
    Version,
    Language,
    Name,
    ErrorMessage,
    Bottom,
}

impl FocusInput {
    fn from(i: usize) -> Self {
        match i {
            0 => Some(Self::ProjectType),
            1 => Some(Self::Version),
            2 => Some(Self::Language),
            3 => Some(Self::Name),
            4 => Some(Self::ErrorMessage),
            5 => Some(Self::Bottom),
            _ => None,
        }
        .unwrap()
    }
    fn num(&self) -> usize {
        match self {
            Self::ProjectType => 0,
            Self::Version => 1,
            Self::Language => 2,
            Self::Name => 3,
            Self::ErrorMessage => 4,
            Self::Bottom => 5,
        }
    }

    fn is(&self, o: Self) -> bool {
        self.num() == o.num()
    }

    fn next(&self) -> Self {
        FocusInput::from((self.num() + 1) % Self::ErrorMessage.num())
    }

    fn prev(&self) -> Self {
        FocusInput::from((self.num() - 1) % Self::ErrorMessage.num())
    }

    fn title(&self) -> String {
        match self {
            Self::ProjectType => "Project Type",
            Self::Version => "Version",
            Self::Language => "Language",
            Self::Name => "Project Info",
            _ => "",
        }
        .to_string()
    }

    fn constraint() -> [Constraint; 7] {
        [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ]
    }
}

struct ProjectSetupApp {
    type_state: ListState,
    version_state: ListState,
    language_state: ListState,
    config: ProjectConfig,
    input_mode: InputMode,
    // [0]: project_type
    // [1]: version
    // [2]: language
    // [3]: name
    show: [bool; 4],
    msg: String,
    focus: FocusInput,
}

enum InputMode {
    Normal,
    Editing,
}

#[allow(dead_code)]
struct ListStateItem<A> {
    state: ListState,
    items: Vec<A>,
}

#[allow(dead_code)]
impl<A: Clone> ListStateItem<A> {
    fn new(items: Vec<A>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { state, items }
    }
    fn next(&mut self, abc: &mut A) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        *abc = self.items[i].clone();
    }
    fn previous(&mut self, abc: &mut A) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        *abc = self.items[i].clone();
    }
}

#[derive(Clone, Copy)]
enum AppDirection {
    Next,
    Prev,
}

impl AppDirection {
    fn get_counter(&self, i: Option<usize>, len: usize) -> usize {
        match i {
            Some(i) => match self {
                Self::Next => {
                    if i >= len - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                Self::Prev => {
                    if i == 0 {
                        len - 1
                    } else {
                        i - 1
                    }
                }
            },
            None => 0,
        }
    }
}

type SwitchItemInListState<T> = dyn Fn(AppDirection, &mut ListState, Vec<T>) -> T;

impl ProjectSetupApp {
    fn new() -> Self {
        let mut state1 = ListState::default();
        state1.select(Some(0));
        let mut state2 = ListState::default();
        state2.select(Some(0));
        let mut state3 = ListState::default();
        state3.select(Some(0));

        Self {
            type_state: state1,
            version_state: state2,
            language_state: state3,
            config: ProjectConfig::default(),
            input_mode: InputMode::Normal,
            show: [true, false, false, false],
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
                    &mut self.type_state,
                    ProjectType::iter().collect(),
                );
            }
            FocusInput::Version => {
                self.config.version = Self::generic_nav_fn::<String>()(
                    ad,
                    &mut self.version_state,
                    self.config.project_type.versions(),
                );
            }
            FocusInput::Language => {
                self.config.language = Self::generic_nav_fn::<String>()(
                    ad,
                    &mut self.language_state,
                    self.config.project_type.languages(),
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

fn main() -> Result<()> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建应用
    let mut app = ProjectSetupApp::new();
    let res = run_app(&mut terminal, &mut app);

    // 清理终端
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut ProjectSetupApp) -> Result<()> {
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
                                FocusInput::Version => {
                                    app.version_state.select(Some(0));
                                    app.config.version =
                                        app.config.project_type.versions()[0].to_string();
                                }
                                FocusInput::Language => {
                                    app.language_state.select(Some(0));
                                    app.config.language =
                                        app.config.project_type.languages()[0].to_string();
                                }
                                _ => {}
                            }
                            if app.focus.is(FocusInput::Name) {
                                app.input_mode = InputMode::Editing;
                                if app.config.name.is_empty() {
                                    app.msg = "Empty Project Name".to_string();
                                }
                            }
                        }
                        KeyCode::Esc => {
                            if app.focus.is(FocusInput::ProjectType) {
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
                                create_project(&app.config)?;
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
    focus_list_item_ui(f, app, FocusInput::ProjectType, chunks.clone());
    focus_list_item_ui(f, app, FocusInput::Version, chunks.clone());
    focus_list_item_ui(f, app, FocusInput::Language, chunks.clone());

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
            app.config.project_type.desc(),
            app.config.version,
            app.config.name
        );
        let paragraph = Paragraph::new(info)
            .block(focus_border(app, FocusInput::Name).style(input_style))
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
        InputMode::Normal => "NORMAL MODE: q=quit, j/k=move, Enter=edit",
        InputMode::Editing => "EDIT MODE: Esc=exit edit, Enter=create project",
    };
    let mode_indicator = Paragraph::new(mode_text).block(Block::default());
    f.render_widget(mode_indicator, chunks[FocusInput::Bottom.num()]);
}

fn focus_border(app: &ProjectSetupApp, focus: FocusInput) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_type(if app.focus.is(focus) {
            BorderType::Thick
        } else {
            BorderType::default()
        })
        .border_style(if app.focus.is(focus) {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        })
        .title(focus.title())
}

fn focus_list_item_ui(f: &mut Frame, app: &ProjectSetupApp, focus: FocusInput, chunks: Rc<[Rect]>) {
    if app.show[focus.num()] {
        // 项目类型选择

        let items: Vec<ListItem> = match focus {
            FocusInput::ProjectType => Some(ProjectType::iter().map(|x| x.desc()).collect()),
            FocusInput::Version => Some(app.config.project_type.versions()),
            FocusInput::Language => Some(app.config.project_type.languages()),
            _ => None,
        }
        .unwrap()
        .iter()
        .map(|i| ListItem::new(i.to_string()).style(Style::default()))
        .collect();

        let list = List::new(items)
            .block(focus_border(app, focus))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        f.render_stateful_widget(
            list,
            chunks[focus.num()],
            &mut match focus {
                FocusInput::ProjectType => Some(app.type_state.clone()),
                FocusInput::Version => Some(app.version_state.clone()),
                FocusInput::Language => Some(app.language_state.clone()),
                _ => None,
            }
            .unwrap(),
        );
    }
}

fn create_project(config: &ProjectConfig) -> Result<()> {
    let project_path = config.path.join(&config.name);
    fs::create_dir_all(&project_path)?;

    match config.project_type {
        ProjectType::SpringBoot => {
            let client = Client::new();
            let params = [
                ("type", "maven-project"),
                ("bootVersion", &config.version),
                ("baseDir", &config.name),
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
            fs::create_dir_all(&config.path).context("Failed to create project directory")?;

            // 解压所有文件到目标目录
            archive
                .extract(&config.path)
                .context("Failed to extract ZIP archive")?;
        }
        ProjectType::CMake => {
            let cmake_lists = format!(
                "\
                cmake_minimum_required(VERSION {})\n\
                project({})\n\
                \n\
                set(CMAKE_C_STANDARD 11)\n\
                \n\
                add_executable(${{PROJECT_NAME}} {})\n",
                config.version,
                config.name,
                if config.language == "C" {
                    "main.c"
                } else {
                    "main.cpp"
                }
            );

            let main_c = "\
                #include <stdio.h>\n\
                \n\
                int main() {\n\
                \tprintf(\"Hello, World!\");\n\
                \treturn 0;\n\
                }\n";

            let main_cpp = "\
                #include <iostream>\n\
                \n\
                int main() {\n\
                \tstd::cout << \"Hello, World!\" << std::endl;\n\
                \treturn 0;\n\
                }\n";

            fs::write(project_path.join("CMakeLists.txt"), cmake_lists)?;
            if config.language == "C" {
                fs::write(project_path.join("main.c"), main_c)?;
            } else {
                fs::write(project_path.join("main.cpp"), main_cpp)?;
            }
        }
        _ => {
            println!(
                "Created {} project directory at {:?}",
                config.project_type.desc(),
                project_path
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod hello {
    #[test]
    fn world() {
        println!("hello world");
    }
}
