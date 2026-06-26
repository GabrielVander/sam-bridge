use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};
use sam_site::infrastructure::sam_site_adapter::{AdapterError, SamSiteAdapter};
use std::{error::Error, io, rc::Rc, time::Duration};
use student_management::domain::entities::{Student, StudentPosition};
use tokio::sync::mpsc;

enum AppEvent {
    Input(CEvent),
    Tick,
    DataFetched(Vec<Student>),
    Error(String),
}

#[derive(PartialEq)]
enum AppState {
    Login,
    Loading,
    List,
    ErrorScreen,
}

#[derive(PartialEq)]
enum Focus {
    Username,
    Password,
}

struct App {
    state: AppState,
    focus: Focus,
    username_input: String,
    password_input: String,
    error_msg: Option<String>,
    students: Vec<Student>,
    table_state: TableState,
    base_url: String,
}

impl App {
    fn new(base_url: String) -> Self {
        Self {
            state: AppState::Login,
            focus: Focus::Username,
            username_input: String::new(),
            password_input: String::new(),
            error_msg: None,
            students: Vec::new(),
            table_state: TableState::default(),
            base_url,
        }
    }

    pub fn next_row(&mut self) {
        let i: usize = match self.table_state.selected() {
            Some(i) => {
                if i >= self.students.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let i: usize = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.students.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    // Create App and Event Channel
    let mut app: App = App::new("https://musical.congregacao.org.br".to_string());
    let (tx, mut rx): (mpsc::Sender<AppEvent>, mpsc::Receiver<AppEvent>) = mpsc::channel(100);

    // Event loop thread
    let tick_rate: Duration = Duration::from_millis(250);
    let event_tx: mpsc::Sender<AppEvent> = tx.clone();

    tokio::spawn(async move {
        loop {
            match event::poll(tick_rate) {
                Ok(true) => {
                    match event::read() {
                        Ok(CEvent::Key(key)) => {
                            // Send fails only if the receiver is dropped (app shutting down)
                            let _ = event_tx.send(AppEvent::Input(CEvent::Key(key))).await;
                        }
                        Ok(_) => {}      // Ignore mouse/resize events
                        Err(_) => break, // IO error on read, break loop
                    }
                }
                Ok(false) => {}  // No event this tick, proceed
                Err(_) => break, // IO error on poll, break loop
            }

            // Always send a tick to drive UI updates
            if event_tx.send(AppEvent::Tick).await.is_err() {
                break; // Receiver dropped, exit loop
            }
        }
    });

    // Main UI Loop
    loop {
        terminal.draw(|f: &mut Frame| draw_ui(f, &mut app))?;

        let incoming_event: Option<AppEvent> = rx.recv().await;
        if let Some(event) = incoming_event {
            match event {
                AppEvent::Input(CEvent::Key(key)) => {
                    if key.code == KeyCode::Esc {
                        break; // Exit app
                    }

                    match app.state {
                        AppState::Login => handle_login_input(&mut app, key, tx.clone()),
                        AppState::List => match key.code {
                            KeyCode::Down | KeyCode::Char('j') => app.next_row(),
                            KeyCode::Up | KeyCode::Char('k') => app.previous_row(),
                            _ => {}
                        },
                        AppState::ErrorScreen if key.code == KeyCode::Enter => {
                            app.state = AppState::Login; // Reset on error acknowledgment
                        }
                        _ => {}
                    }
                }
                AppEvent::DataFetched(students) => {
                    app.students = students;
                    if !app.students.is_empty() {
                        app.table_state.select(Some(0));
                    }
                    app.state = AppState::List;
                }
                AppEvent::Error(err) => {
                    app.error_msg = Some(err);
                    app.state = AppState::ErrorScreen;
                }
                _ => {}
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// Handles keystrokes when the user is on the Login screen
fn handle_login_input(app: &mut App, key: KeyEvent, tx: mpsc::Sender<AppEvent>) {
    match key.code {
        KeyCode::Char(c) => {
            if app.focus == Focus::Username {
                app.username_input.push(c);
            } else {
                app.password_input.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.focus == Focus::Username {
                app.username_input.pop();
            } else {
                app.password_input.pop();
            }
        }
        KeyCode::Tab | KeyCode::BackTab | KeyCode::Up | KeyCode::Down => {
            app.focus = if app.focus == Focus::Username {
                Focus::Password
            } else {
                Focus::Username
            };
        }
        KeyCode::Enter => {
            // Trigger authentication
            app.state = AppState::Loading;
            let user: String = app.username_input.clone();
            let pass: String = app.password_input.clone();
            let base_url: String = app.base_url.clone();

            tokio::spawn(async move {
                let fetch_result: Result<Vec<Student>, AdapterError> =
                    fetch_data(base_url, user, pass).await;
                match fetch_result {
                    Ok(students) => {
                        let _ = tx.send(AppEvent::DataFetched(students)).await;
                    }
                    Err(e) => {
                        let _ = tx.send(AppEvent::Error(e.to_string())).await;
                    }
                }
            });
        }
        _ => {}
    }
}

async fn fetch_data(
    base_url: String,
    user: String,
    pass: String,
) -> Result<Vec<Student>, AdapterError> {
    let mut adapter: SamSiteAdapter = SamSiteAdapter::new(&base_url)?;
    let _session_id: String = adapter.login(&user, &pass).await?;
    let students: Vec<Student> = adapter.get_students().await?;
    Ok(students)
}

// -----------------------------------------------------------------------------
// UI RENDERING LOGIC
// -----------------------------------------------------------------------------

fn draw_ui(f: &mut Frame, app: &mut App) {
    let chunks: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    match app.state {
        AppState::Login => draw_login(f, app, chunks[0]),
        AppState::Loading => draw_loading(f, chunks[0]),
        AppState::List => draw_table(f, app, chunks[0]),
        AppState::ErrorScreen => draw_error(f, app, chunks[0]),
    }

    // Global Footer
    let help_msg: &str = match app.state {
        AppState::Login => " <Tab> Switch Field | <Enter> Login | <Esc> Quit ",
        AppState::List => " <Up/Down> Scroll | <Esc> Quit ",
        AppState::Loading => " Please wait... | <Esc> Quit ",
        AppState::ErrorScreen => " <Enter> Go Back | <Esc> Quit ",
    };

    let footer: Paragraph = Paragraph::new(help_msg)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[1]);
}

fn draw_login(f: &mut Frame, app: &App, area: Rect) {
    let block: Block = Block::default()
        .title(" Login to SamSite ")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    // Center the login form
    let vertical_chunks: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(3), // Username
            Constraint::Length(3), // Password
            Constraint::Percentage(40),
        ])
        .split(area);

    let horizontal_chunks: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40), // Actual input width
            Constraint::Percentage(30),
        ])
        .split(vertical_chunks[1]); // Apply to username row

    let pass_horizontal_chunks: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(vertical_chunks[2]);

    // Username Input
    let user_style: Style = if app.focus == Focus::Username {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let user_input: Paragraph = Paragraph::new(app.username_input.as_str())
        .style(user_style)
        .block(Block::default().borders(Borders::ALL).title(" Username "));

    f.render_widget(user_input, horizontal_chunks[1]);

    // Password Input (Masked)
    let pass_style: Style = if app.focus == Focus::Password {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let masked_pass: String = "*".repeat(app.password_input.len());
    let pass_input: Paragraph = Paragraph::new(masked_pass.as_str())
        .style(pass_style)
        .block(Block::default().borders(Borders::ALL).title(" Password "));

    f.render_widget(pass_input, pass_horizontal_chunks[1]);
}

fn draw_loading(f: &mut Frame, area: Rect) {
    let p: Paragraph = Paragraph::new("Authenticating & Fetching Data...")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let layout: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(3),
            Constraint::Percentage(45),
        ])
        .split(area);

    f.render_widget(p, layout[1]);
}

fn draw_error(f: &mut Frame, app: &App, area: Rect) {
    let msg: &str = app.error_msg.as_deref().unwrap_or("Unknown Error");
    let p: Paragraph = Paragraph::new(format!(
        "ERROR: {}\n\nPress <Enter> to return to login.",
        msg
    ))
    .style(Style::default().fg(Color::Red))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title(" Error "));

    let layout: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(6),
            Constraint::Percentage(40),
        ])
        .split(area);

    f.render_widget(p, layout[1]);
}

fn draw_table(f: &mut Frame, app: &mut App, area: Rect) {
    let titles: [&str; 4] = ["ID", "Name", "Location", "Position"];
    let header_cells: Vec<Cell> = titles
        .iter()
        .map(|h: &&str| Cell::from(*h).style(Style::default().fg(Color::Cyan)))
        .collect();

    let header: Row = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1)
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .students
        .iter()
        .map(|student: &Student| {
            // Quick formatting for position enum
            let pos_str: &str = match &student.position {
                StudentPosition::Musician { .. } => "Musician",
                StudentPosition::Organist { .. } => "Organist",
                StudentPosition::Unknown(u) => u.as_str(),
            };

            let cells: Vec<Cell> = vec![
                Cell::from(student.id.clone()),
                Cell::from(student.name.clone()),
                Cell::from(student.location.clone()),
                Cell::from(pos_str),
            ];
            Row::new(cells).height(1)
        })
        .collect();

    let t: Table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Students "))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");

    f.render_stateful_widget(t, area, &mut app.table_state);
}
