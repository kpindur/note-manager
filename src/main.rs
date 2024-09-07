#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod app;
mod note;

use app::{App, InputMode};

use ratatui::{
    backend::CrosstermBackend, Terminal
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App::new("sample.json".to_string())?;
    app.load_notes()?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(),LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res { println!("{:?}", err); }

    return Ok(());
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Enter => {},
                    _ => {},
                },
            }
        }
    }
}

pub fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::LightBlue);
    let header_cells = ["Id", "Contents", "Tags", "Links"]
        .iter()
        .map(|h| return Cell::from(*h).style(Style::default().fg(Color::LightGreen)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.notes.iter().map(|(id, note)| {
        let id = Cell::from(id.clone());
        let contents = Cell::from(note.contents().chars().take(30).collect::<String>() + "...");
        let tags = Cell::from(note.tags().join(", "));
        let links = Cell::from(format!("{}", note.links().len()));
        return Row::new(vec![id, contents, tags, links]);
    });

    let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];
    let t = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ");
    
    f.render_widget(t, chunks[0]);
}
