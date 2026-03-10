mod app;
mod config;
mod editor;
mod input;
mod syntax;
mod theme;
mod tree;
mod ui;

use std::env;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::App;

fn main() -> io::Result<()> {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let app = App::new(root);
    let mouse_enabled = app.config.general.mouse_enabled;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    if mouse_enabled {
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    } else {
        execute!(stdout, EnterAlternateScreen)?;
    }
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let mut app = app;
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    if mouse_enabled {
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
    } else {
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    }
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    while app.running {
        terminal.draw(|frame| {
            app.terminal_size = frame.area();
            ui::render(frame, app);
        })?;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    input::handler::handle_key_event(app, key);
                }
                Event::Resize(_, _) => {
                    // Terminal resized - will redraw on next loop
                }
                _ => {}
            }
        }
    }

    Ok(())
}
