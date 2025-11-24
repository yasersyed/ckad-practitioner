use crate::question_repository::QuestionRepository;
use crate::quiz_state::{HintState, QuizState};
use crate::ui::QuizUI;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::io;
use std::time::Duration;
use tokio::time::sleep;

/// Application coordinator that orchestrates quiz logic (Dependency Inversion Principle)
/// Depends on the QuestionRepository abstraction, not concrete implementations
pub struct App {
    quiz_state: QuizState,
    hint_state: HintState,
}

impl App {
    /// Creates a new App instance using dependency injection
    /// This follows the Dependency Inversion Principle - we depend on the
    /// QuestionRepository trait (abstraction) rather than concrete implementations
    pub fn new(repository: Box<dyn QuestionRepository>) -> Self {
        let questions = repository.get_questions();
        Self {
            quiz_state: QuizState::new(questions),
            hint_state: HintState::new(),
        }
    }

    /// Main event loop for the application
    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| QuizUI::render(f, &self.quiz_state, &self.hint_state))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('h') => self.handle_hint_request(),
                        KeyCode::Char('n') => self.handle_next_question(),
                        _ => {}
                    }
                }
            }

            sleep(Duration::from_millis(50)).await;
        }
    }

    fn handle_hint_request(&mut self) {
        if !self.quiz_state.timer().is_expired() {
            self.hint_state.enable_hints();
            let max_hints = self.quiz_state.current_question().hints.len();
            self.hint_state.next_hint(max_hints);
        }
    }

    fn handle_next_question(&mut self) {
        if self.quiz_state.timer().is_expired() {
            self.quiz_state.next_question();
            self.hint_state.reset();
        }
    }
}
