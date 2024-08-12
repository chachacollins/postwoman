use std::io;

use ratatui::{
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{
            self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
    prelude::{Backend, CrosstermBackend},
    Terminal,
};

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
    todo!()
}
