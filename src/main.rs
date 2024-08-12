use std::io;

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{
            self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use App::CurrentScreen;

mod App;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Preparing the terminal for capturing user input
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::App::new();
    let res = run_app(&mut terminal, &mut app).await?;

    //Reverse whatever we have done to the terminal;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Ok(pw_thanks) = res {
        if pw_thanks {
            println!("Thanks for using postwoman :)");
        } else if let Err(err) = res {
            println!("{:?}", err);
        }
    }
    Ok(())
}
async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App::App) -> io::Result<bool> {
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
            }
        }
    }
}
