use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAltScreen, LeaveAltScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    id: usize,
    question: String,
    hints: Vec<String>,
    answer: String,
    time_limit_secs: u64,
}

#[derive(Debug)]
struct AppState {
    questions: Vec<Question>,
    current_question_idx: usize,
    time_started: Instant,
    time_limit: Duration,
    show_hints: bool,
    hint_index: usize,
    show_answer: bool,
    user_answer: String,
    show_next_btn: bool,
    correct: Option<bool>,
}

impl AppState {
    fn new(questions: Vec<Question>) -> Self {
        let time_limit = Duration::from_secs(questions[0].time_limit_secs);
        Self {
            questions,
            current_question_idx: 0,
            time_started: Instant::now(),
            time_limit,
            show_hints: false,
            hint_index: 0,
            show_answer: false,
            user_answer: String::new(),
            show_next_btn: false,
            correct: None,
        }
    }

    fn get_elapsed(&self) -> Duration {
        self.time_started.elapsed()
    }

    fn get_remaining(&self) -> Duration {
        self.time_limit.saturating_sub(self.get_elapsed())
    }

    fn is_time_expired(&self) -> bool {
        self.get_elapsed() >= self.time_limit
    }

    fn next_question(&mut self) {
        if self.current_question_idx < self.questions.len() - 1 {
            self.current_question_idx += 1;
            let new_time_limit = Duration::from_secs(
                self.questions[self.current_question_idx].time_limit_secs
            );
            self.reset_question(new_time_limit);
        }
    }

    fn reset_question(&mut self, time_limit: Duration) {
        self.time_started = Instant::now();
        self.time_limit = time_limit;
        self.show_hints = false;
        self.hint_index = 0;
        self.show_answer = false;
        self.user_answer = String::new();
        self.show_next_btn = false;
        self.correct = None;
    }
}

fn create_questions() -> Vec<Question> {
    vec![
        Question {
            id: 1,
            question: "Create a Pod named 'nginx' using the nginx:1.14 image in the default namespace.".to_string(),
            hints: vec![
                "Use: kubectl run <pod-name> --image=<image>".to_string(),
                "Full command: kubectl run nginx --image=nginx:1.14".to_string(),
                "Reference: https://kubernetes.io/docs/reference/kubectl/generated/kubectl-run/".to_string(),
            ],
            answer: "kubectl run nginx --image=nginx:1.14".to_string(),
            time_limit_secs: 60,
        },
        Question {
            id: 2,
            question: "Create a deployment named 'web' with 3 replicas using the httpd:2.4 image and expose port 80.".to_string(),
            hints: vec![
                "Use kubectl create deployment, then kubectl set image, and kubectl expose".to_string(),
                "Or use: kubectl create deployment web --image=httpd:2.4 --replicas=3".to_string(),
                "Then: kubectl expose deployment web --port=80 --type=ClusterIP".to_string(),
            ],
            answer: "kubectl create deployment web --image=httpd:2.4 --replicas=3\nkubectl expose deployment web --port=80 --type=ClusterIP".to_string(),
            time_limit_secs: 120,
        },
        Question {
            id: 3,
            question: "Set resource requests and limits for a pod: request 256Mi memory and 100m CPU, limit 512Mi memory and 200m CPU.".to_string(),
            hints: vec![
                "Use resources.requests and resources.limits in the pod spec".to_string(),
                "Memory is specified in Mi, CPU in m (millicores)".to_string(),
                "Reference: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/".to_string(),
            ],
            answer: "resources:\n  requests:\n    memory: \"256Mi\"\n    cpu: \"100m\"\n  limits:\n    memory: \"512Mi\"\n    cpu: \"200m\"".to_string(),
            time_limit_secs: 90,
        },
        Question {
            id: 4,
            question: "Create a ConfigMap named 'app-config' with key 'database.url' and value 'postgres://db:5432'.".to_string(),
            hints: vec![
                "Use: kubectl create configmap <name> --from-literal=<key>=<value>".to_string(),
                "Full command: kubectl create configmap app-config --from-literal=database.url=postgres://db:5432".to_string(),
                "Reference: https://kubernetes.io/docs/concepts/configuration/configmap/".to_string(),
            ],
            answer: "kubectl create configmap app-config --from-literal=database.url=postgres://db:5432".to_string(),
            time_limit_secs: 60,
        },
        Question {
            id: 5,
            question: "Create a Secret named 'db-secret' with username 'admin' and password 'secret123'.".to_string(),
            hints: vec![
                "Use: kubectl create secret generic <name> --from-literal=<key>=<value>".to_string(),
                "Full command: kubectl create secret generic db-secret --from-literal=username=admin --from-literal=password=secret123".to_string(),
                "Reference: https://kubernetes.io/docs/concepts/configuration/secret/".to_string(),
            ],
            answer: "kubectl create secret generic db-secret --from-literal=username=admin --from-literal=password=secret123".to_string(),
            time_limit_secs: 75,
        },
    ]
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &AppState) {
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

    // Header with timer
    let remaining = app.get_remaining();
    let remaining_text = if app.is_time_expired() {
        "TIME EXPIRED".to_string()
    } else {
        let secs = remaining.as_secs();
        format!("Time remaining: {}:{:02}", secs / 60, secs % 60)
    };

    let color = if remaining.as_secs() < 10 && !app.is_time_expired() {
        Color::Red
    } else {
        Color::Green
    };

    let header = Paragraph::new(remaining_text)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("CKAD Practitioner"));
    f.render_widget(header, chunks[0]);

    // Question
    let question = &app.questions[app.current_question_idx];
    let question_text = format!(
        "Question {} of {}: {}",
        app.current_question_idx + 1,
        app.questions.len(),
        question.question
    );
    let question_widget = Paragraph::new(question_text)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Question"));
    f.render_widget(question_widget, chunks[1]);

    // Hints and Answer
    let mut content_lines = vec![];

    if !app.is_time_expired() {
        let hint_text = if app.show_hints {
            format!(
                "Hint {} (press 'h' for more): {}",
                app.hint_index + 1,
                question.hints.get(app.hint_index).unwrap_or(&"No more hints".to_string())
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
    f.render_widget(content, chunks[2]);

    // Controls
    let controls = if app.is_time_expired() {
        if app.current_question_idx < app.questions.len() - 1 {
            "Press 'n' for next question, 'q' to quit"
        } else {
            "Quiz complete! Press 'q' to quit"
        }
    } else {
        "h: hints | q: quit | (answer revealed after time expires)"
    };

    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls_widget, chunks[3]);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAltScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let questions = create_questions();
    let mut app = AppState::new(questions);

    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAltScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('h') => {
                        if !app.is_time_expired() {
                            app.show_hints = true;
                            if app.hint_index < app.questions[app.current_question_idx].hints.len() - 1
                            {
                                app.hint_index += 1;
                            }
                        }
                    }
                    KeyCode::Char('n') => {
                        if app.is_time_expired() {
                            app.next_question();
                        }
                    }
                    _ => {}
                }
            }
        }

        if app.is_time_expired() {
            app.show_answer = true;
        }

        sleep(Duration::from_millis(50)).await;
    }
}
