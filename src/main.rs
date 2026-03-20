mod app;
mod models;
mod storage;
mod ui;

use app::{App, InputMode, NamingMode};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    io::stdout().execute(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;

    let mut app = App::new();
    app.films = storage::load();

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Up => app.move_selection_up(),
                            KeyCode::Down => app.move_selection_down(),
                            KeyCode::Char('w') => app.toggle_watched(),
                            KeyCode::Char('d') => app.delete_film(),
                            KeyCode::Char('s') => storage::try_save(&app.films),
                            // KeyCode::Char('a') => app.rename(String::from("Yolanda")),
                            KeyCode::Char('a') => {
                                app.input_mode = InputMode::Naming(NamingMode::Creating);
                                app.input_value.clear();
                            }
                            KeyCode::Char('n') => {
                                app.input_mode = InputMode::Naming(NamingMode::Modifying);
                                app.input_value.clear();
                            }
                            KeyCode::Char('r') => {
                                app.input_mode = InputMode::Rating;
                                app.input_value.clear();
                            }
                            KeyCode::Char('y') => {
                                app.input_mode = InputMode::Director;
                                app.input_value.clear();
                            }
                            KeyCode::Char('f') => {
                                app.filter = match app.filter {
                                    models::Filter::All => models::Filter::Unwatched,
                                    models::Filter::Unwatched => models::Filter::Watched,
                                    models::Filter::Watched => models::Filter::All,
                                };
                                app.selected_index = 0;
                            }
                            KeyCode::Enter => {
                                app.input_mode = InputMode::Breakdown;
                            }
                            _ => {}
                        }
                    }
                    InputMode::Breakdown => {
                        if matches!(key.code, KeyCode::Enter) | matches!(key.code, KeyCode::Esc) | matches!(key.code, KeyCode::Char('q')) {
                            app.input_mode = InputMode::Normal;
                        }
                    }
                    _ => match key.code {
                        KeyCode::Enter => {
                            if !app.input_value.is_empty() {
                                match app.input_mode {
                                    InputMode::Naming(NamingMode::Creating) => {
                                        app.add_film(app.input_value.clone())
                                    }
                                    InputMode::Naming(NamingMode::Modifying) => {
                                        app.rename(app.input_value.clone())
                                    }
                                    InputMode::Director => app.director(app.input_value.clone()),
                                    InputMode::Rating => {
                                        if let Ok(val) = app.input_value.parse::<u8>() {
                                            if !app.input_value.is_empty() && 0 < val && val <= 10 {
                                                app.rate(val)
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                app.input_mode = InputMode::Normal;
                            }
                        }
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        KeyCode::Char(c) => app.input_value.push(c),
                        KeyCode::Backspace => {
                            app.input_value.pop();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
    io::stdout().execute(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;

    disable_raw_mode()?;
    Ok(())
}
