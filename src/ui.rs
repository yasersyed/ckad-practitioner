use crate::quiz_state::{HintState, QuizState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Handles all UI rendering logic (Single Responsibility Principle)
/// This module is responsible only for presentation, not business logic
pub struct QuizUI;

impl QuizUI {
    pub fn render(f: &mut Frame, quiz_state: &QuizState, hint_state: &HintState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Min(5),
                Constraint::Length(4),
            ])
            .split(f.size());

        Self::render_header(f, quiz_state, chunks[0]);
        Self::render_question(f, quiz_state, chunks[1]);
        Self::render_content(f, quiz_state, hint_state, chunks[2]);
        Self::render_controls(f, quiz_state, chunks[3]);
    }

    fn render_header(f: &mut Frame, quiz_state: &QuizState, area: ratatui::layout::Rect) {
        let timer = quiz_state.timer();
        let remaining_text = if timer.is_expired() {
            "TIME EXPIRED".to_string()
        } else {
            let remaining = timer.remaining();
            let secs = remaining.as_secs();
            format!("Time remaining: {}:{:02}", secs / 60, secs % 60)
        };

        let color = if timer.remaining().as_secs() < 10 && !timer.is_expired() {
            Color::Red
        } else {
            Color::Green
        };

        let header = Paragraph::new(remaining_text)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("CKAD Practitioner"));

        f.render_widget(header, area);
    }

    fn render_question(f: &mut Frame, quiz_state: &QuizState, area: ratatui::layout::Rect) {
        let question = quiz_state.current_question();
        let question_text = format!(
            "Question {} of {}: {}",
            quiz_state.current_index() + 1,
            quiz_state.total_questions(),
            question.question
        );

        let question_widget = Paragraph::new(question_text)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Question"));

        f.render_widget(question_widget, area);
    }

    fn render_content(
        f: &mut Frame,
        quiz_state: &QuizState,
        hint_state: &HintState,
        area: ratatui::layout::Rect,
    ) {
        let mut content_lines = vec![];
        let question = quiz_state.current_question();
        let timer = quiz_state.timer();

        if !timer.is_expired() {
            let hint_text = if hint_state.show_hints() {
                let hint_idx = hint_state.hint_index();
                format!(
                    "Hint {} (press 'h' for more): {}",
                    hint_idx + 1,
                    question.hints.get(hint_idx).unwrap_or(&"No more hints".to_string())
                )
            } else {
                "Press 'h' for hints".to_string()
            };
            content_lines.push(Line::from(Span::styled(
                hint_text,
                Style::default().fg(Color::Yellow),
            )));
        } else {
            content_lines.push(Line::from(Span::styled(
                "Answer:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));
            for line in question.answer.lines() {
                content_lines.push(Line::from(Span::raw(line)));
            }
        }

        let content = Paragraph::new(content_lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Content"));

        f.render_widget(content, area);
    }

    fn render_controls(f: &mut Frame, quiz_state: &QuizState, area: ratatui::layout::Rect) {
        let timer = quiz_state.timer();

        let controls = if timer.is_expired() {
            if quiz_state.is_last_question() {
                "Quiz complete! Press 'q' to quit"
            } else {
                "Press 'n' for next question, 'q' to quit"
            }
        } else {
            "h: hints | q: quit | (answer revealed after time expires)"
        };

        let controls_widget = Paragraph::new(controls)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(controls_widget, area);
    }
}
