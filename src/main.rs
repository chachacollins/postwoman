use std::io;

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;
use App::{CurrentScreen, CurrentlyEditing};
mod App;
mod ui;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Preparing the terminal for capturing user input
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::App::new();
    let _res = run_app(&mut terminal, &mut app).await?;

    //Reverse whatever we have done to the terminal;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App::App,
) -> Result<bool, Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('g') => {
                        app.current_screen = CurrentScreen::Get;
                        app.currently_editing = Some(App::CurrentlyEditing::Value);
                    }
                    KeyCode::Char('p') => {
                        app.current_screen = CurrentScreen::Post;
                        app.currently_editing = Some(App::CurrentlyEditing::Value);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Post if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.post_req().await?;
                                    app.currently_editing = Some(CurrentlyEditing::Key);
                                }
                                CurrentlyEditing::Url => {
                                    app.currently_editing = Some(CurrentlyEditing::Key);
                                }
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.pop();
                                }
                                CurrentlyEditing::Url => {
                                    app.url.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(value);
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.push(value);
                                }
                                CurrentlyEditing::Url => {
                                    app.url.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Get if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.get_req().await?;
                    }
                    KeyCode::Backspace => {
                        app.url.pop();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char(value) => {
                        app.url.push(value);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
