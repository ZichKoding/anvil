mod app;
mod ui;
mod input;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::App;

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    while app.running {
        terminal.draw(|frame| {
            app.terminal_size = frame.area();
            ui::render(frame, app);
        })?;

        // Poll at 50ms (20fps max, near-zero CPU when idle)
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                input::handler::handle_key_event(app, key);
            }
        }
    }

    Ok(())
}
