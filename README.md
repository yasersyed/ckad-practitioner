# CKAD Practitioner Quiz Application

A terminal-based interactive quiz application for Certified Kubernetes Application Developer (CKAD) exam preparation, built with Rust and following SOLID principles.

## Features

- Interactive terminal UI using Ratatui
- Timed questions with visual countdown
- Progressive hint system
- Automatic answer reveal after time expires
- Multiple CKAD practice questions covering:
  - Pod creation
  - Deployments
  - Resource management
  - ConfigMaps
  - Secrets

## Architecture

This application is designed following **SOLID principles** for maintainability and extensibility:

### SOLID Principles Applied

#### 1. Single Responsibility Principle (SRP)
Each module has one clear responsibility:

- **`models.rs`** - Data structures (Question)
- **`timer.rs`** - Time tracking and expiration logic
- **`quiz_state.rs`** - Quiz domain logic and UI state management
- **`ui.rs`** - Terminal UI rendering
- **`question_repository.rs`** - Question data access
- **`app.rs`** - Application orchestration
- **`main.rs`** - Entry point and terminal setup

#### 2. Open/Closed Principle (OCP)
The `QuestionRepository` trait allows extending question sources without modifying existing code:

```rust
// Current: In-memory questions
let repository = Box::new(InMemoryQuestionRepository);

// Future: Load from file without changing core logic
let repository = Box::new(FileQuestionRepository::new("questions.json"));

// Future: Load from API
let repository = Box::new(ApiQuestionRepository::new("https://api.example.com"));
```

#### 3. Liskov Substitution Principle (LSP)
Any `QuestionRepository` implementation can be substituted without breaking functionality. All implementations must provide `get_questions()` method.

#### 4. Interface Segregation Principle (ISP)
State is split into focused structures:
- `QuizState` - Core quiz logic (questions, navigation, timer)
- `HintState` - Hint display state (separate UI concern)

Components only depend on interfaces they actually use.

#### 5. Dependency Inversion Principle (DIP)
High-level modules depend on abstractions:
- `App` depends on `QuestionRepository` trait, not concrete implementations
- Dependencies are injected via constructor: `App::new(repository)`

## Project Structure

```
src/
├── main.rs                   # Entry point, terminal setup/teardown
├── app.rs                    # Application coordinator with DI
├── models.rs                 # Question data model
├── question_repository.rs    # QuestionRepository trait + implementations
├── quiz_state.rs             # QuizState and HintState
├── timer.rs                  # Timer logic
└── ui.rs                     # Terminal UI rendering
```

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Terminal that supports ANSI colors

### Installation

```bash
git clone <repository-url>
cd ckad-practitioner
cargo build --release
```

### Running

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/ckad-practitioner
```

## Usage

### Controls

- **`h`** - Show next hint (only before time expires)
- **`n`** - Next question (only after time expires)
- **`q`** - Quit application

### Question Flow

1. Question appears with countdown timer
2. Press `h` to reveal hints progressively
3. When time expires, answer is automatically revealed
4. Press `n` to move to next question
5. Quiz completes after all questions

## Extending the Application

### Adding New Question Sources

Create a new implementation of `QuestionRepository`:

```rust
// In src/question_repository.rs or a new file

pub struct JsonQuestionRepository {
    file_path: String,
}

impl JsonQuestionRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl QuestionRepository for JsonQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        // Load from JSON file
        let contents = std::fs::read_to_string(&self.file_path)
            .expect("Failed to read questions file");
        serde_json::from_str(&contents)
            .expect("Failed to parse questions")
    }
}
```

Then use it in `main.rs`:

```rust
let repository = Box::new(JsonQuestionRepository::new("questions.json".to_string()));
let mut app = App::new(repository);
```

### Adding New Question Types

Simply add new `Question` instances to your repository:

```rust
Question {
    id: 6,
    question: "Your new question here".to_string(),
    hints: vec![
        "First hint".to_string(),
        "Second hint".to_string(),
    ],
    answer: "The answer".to_string(),
    time_limit_secs: 120,
}
```

### Customizing the UI

Modify `src/ui.rs` without affecting business logic:
- Change colors in the `Style` definitions
- Adjust layout constraints
- Add new UI sections

### Adding New Features

The modular structure makes it easy to add features:

**Example: Add scoring system**
1. Create `src/scoring.rs` with score tracking logic
2. Add score field to `QuizState`
3. Update `QuizUI::render()` to display score
4. Update `App::handle_next_question()` to calculate score

**Example: Add question categories**
1. Add `category: String` field to `Question` in `models.rs`
2. Update question repositories to include categories
3. Add filtering logic to `QuizState`
4. Update UI to show category

## Testing

### Running Tests

```bash
cargo test
```

### Testing Individual Modules

The SOLID design makes unit testing straightforward:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_expiration() {
        let timer = Timer::new(1);
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(timer.is_expired());
    }
}
```

### Integration Testing

Create mock repositories for testing:

```rust
struct MockQuestionRepository;

impl QuestionRepository for MockQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        vec![/* test questions */]
    }
}

#[test]
fn test_app_with_mock_data() {
    let repo = Box::new(MockQuestionRepository);
    let app = App::new(repo);
    // Test app behavior
}
```

## Development Guidelines

### Code Organization

1. **Keep modules focused** - Each module should have a single, clear responsibility
2. **Use traits for abstraction** - Define interfaces for extensible components
3. **Inject dependencies** - Pass dependencies via constructors, not global state
4. **Separate concerns** - Keep UI, business logic, and data access separate

### Adding Dependencies

Add to `Cargo.toml` and document why the dependency is needed:

```toml
[dependencies]
new-crate = "1.0"  # Brief explanation of what this provides
```

### Performance Considerations

- The app polls events every 100ms and sleeps 50ms per loop iteration
- For better performance, consider using event-driven architecture instead of polling
- Timer updates are lightweight (no allocations)

## Troubleshooting

### Terminal Not Restoring

If the terminal doesn't restore properly after a crash:

```bash
reset
```

Or manually:

```bash
stty sane
```

### Build Errors

Ensure you're using a recent Rust version:

```bash
rustc --version  # Should be 1.70 or higher
rustup update
```

## Contributing

1. Follow the existing SOLID architecture
2. Add tests for new functionality
3. Update documentation for any API changes
4. Run `cargo clippy` and fix warnings before committing
5. Run `cargo fmt` to format code

## License

[Your License Here]

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for terminal UI
- Uses [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal control
- Async runtime provided by [Tokio](https://github.com/tokio-rs/tokio)
