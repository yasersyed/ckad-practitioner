mod app;
mod models;
mod question_repository;
mod quiz_state;
mod timer;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use question_repository::InMemoryQuestionRepository;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Main entry point demonstrating Dependency Inversion Principle
/// The App is created with a QuestionRepository abstraction, making it
/// easy to swap implementations without changing the core application logic
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Dependency Injection: Create app with a concrete repository implementation
    // This could easily be swapped with FileQuestionRepository or any other implementation
    let repository = Box::new(InMemoryQuestionRepository);
    let mut app = App::new(repository);

    // Run the application
    let res = app.run(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
