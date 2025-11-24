use crate::models::Question;
use crate::timer::Timer;

/// Manages the core quiz domain logic (Single Responsibility & Interface Segregation)
/// This is separated from UI concerns
#[derive(Debug)]
pub struct QuizState {
    questions: Vec<Question>,
    current_index: usize,
    timer: Timer,
}

impl QuizState {
    pub fn new(questions: Vec<Question>) -> Self {
        let timer = Timer::new(questions[0].time_limit_secs);
        Self {
            questions,
            current_index: 0,
            timer,
        }
    }

    pub fn current_question(&self) -> &Question {
        &self.questions[self.current_index]
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn total_questions(&self) -> usize {
        self.questions.len()
    }

    pub fn timer(&self) -> &Timer {
        &self.timer
    }

    pub fn is_last_question(&self) -> bool {
        self.current_index >= self.questions.len() - 1
    }

    pub fn next_question(&mut self) {
        if !self.is_last_question() {
            self.current_index += 1;
            let new_limit = self.questions[self.current_index].time_limit_secs;
            self.timer.reset(new_limit);
        }
    }
}

/// Manages UI-specific state (Interface Segregation Principle)
/// Separated from domain logic to follow ISP
#[derive(Debug)]
pub struct HintState {
    show_hints: bool,
    hint_index: usize,
}

impl HintState {
    pub fn new() -> Self {
        Self {
            show_hints: false,
            hint_index: 0,
        }
    }

    pub fn show_hints(&self) -> bool {
        self.show_hints
    }

    pub fn hint_index(&self) -> usize {
        self.hint_index
    }

    pub fn enable_hints(&mut self) {
        self.show_hints = true;
    }

    pub fn next_hint(&mut self, max_hints: usize) {
        if self.hint_index < max_hints.saturating_sub(1) {
            self.hint_index += 1;
        }
    }

    pub fn reset(&mut self) {
        self.show_hints = false;
        self.hint_index = 0;
    }
}
