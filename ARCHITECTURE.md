# Architecture Documentation

This document provides detailed architectural information about the CKAD Practitioner application for maintainers and contributors.

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Module Breakdown](#module-breakdown)
3. [Data Flow](#data-flow)
4. [SOLID Principles in Practice](#solid-principles-in-practice)
5. [Extension Points](#extension-points)
6. [Testing Strategy](#testing-strategy)
7. [Performance Considerations](#performance-considerations)

## Design Philosophy

This application follows **SOLID principles** and **Clean Architecture** concepts to ensure:

- **Maintainability**: Easy to understand and modify
- **Testability**: Components can be tested in isolation
- **Extensibility**: New features don't require rewriting existing code
- **Flexibility**: Swap implementations without breaking changes

### Key Design Decisions

1. **Trait-based abstractions** for extensibility (QuestionRepository)
2. **State separation** to prevent coupling between UI and domain logic
3. **Dependency injection** to enable testing and flexibility
4. **Single-purpose modules** to reduce cognitive load

## Module Breakdown

### `models.rs` - Domain Models

**Purpose**: Define core data structures

**Contents**:
- `Question` struct with serialization support

**Dependencies**: Only `serde` for serialization

**Key Points**:
- Pure data structure, no business logic
- Serializable for future JSON/file support
- No dependencies on other application modules

### `timer.rs` - Time Management

**Purpose**: Handle all timing logic

**Responsibilities**:
- Track elapsed time
- Calculate remaining time
- Determine expiration status
- Reset timer for new questions

**Key Methods**:
```rust
pub fn new(limit_secs: u64) -> Self
pub fn elapsed(&self) -> Duration
pub fn remaining(&self) -> Duration
pub fn is_expired(&self) -> bool
pub fn reset(&mut self, limit_secs: u64)
```

**Design Notes**:
- Stateful but focused on single responsibility
- Uses `saturating_sub` to prevent underflow
- Instant-based for accuracy

### `question_repository.rs` - Data Access Layer

**Purpose**: Abstract question data sources

**Pattern**: Repository Pattern + Strategy Pattern

**Structure**:
```rust
trait QuestionRepository {
    fn get_questions(&self) -> Vec<Question>;
}

struct InMemoryQuestionRepository;
struct FileQuestionRepository { file_path: String }
```

**Why This Design**:
- **Open/Closed**: Add new sources without modifying existing code
- **Dependency Inversion**: App depends on trait, not concrete type
- **Testability**: Easy to create mock repositories

**Extension Examples**:
- `ApiQuestionRepository` - Fetch from REST API
- `DatabaseQuestionRepository` - Load from SQLite/PostgreSQL
- `CachedQuestionRepository` - Wrapper adding caching layer

### `quiz_state.rs` - Domain State Management

**Purpose**: Manage quiz business logic

**Contains Two State Structures**:

#### `QuizState` - Core Domain Logic
```rust
pub struct QuizState {
    questions: Vec<Question>,
    current_index: usize,
    timer: Timer,
}
```

**Responsibilities**:
- Track current question
- Manage question navigation
- Own the timer
- Provide question metadata

**Key Methods**:
- `current_question()` - Get active question
- `next_question()` - Navigate forward
- `is_last_question()` - Check if quiz is complete
- `timer()` - Access timing information

#### `HintState` - UI Presentation State
```rust
pub struct HintState {
    show_hints: bool,
    hint_index: usize,
}
```

**Why Separate**:
- **Interface Segregation**: UI concerns separate from domain logic
- **Testability**: Can test hint display logic independently
- **Clarity**: Clear separation between "what" (quiz) and "how" (presentation)

### `ui.rs` - Presentation Layer

**Purpose**: Handle all terminal UI rendering

**Pattern**: Immediate Mode UI (Ratatui pattern)

**Structure**:
```rust
pub struct QuizUI;

impl QuizUI {
    pub fn render(f: &mut Frame, quiz_state: &QuizState, hint_state: &HintState)
    fn render_header(...)
    fn render_question(...)
    fn render_content(...)
    fn render_controls(...)
}
```

**Design Decisions**:
- **Stateless rendering**: Pure function of state
- **Modular rendering**: Each section rendered separately
- **No business logic**: Only presentation and styling
- **Color coding**: Red for urgency, green for time, yellow for hints

**Layout Structure**:
```
┌─────────────────────────────┐
│  Header (Timer)       │ 3 lines
├─────────────────────────────┤
│  Question             │ 8 lines
├─────────────────────────────┤
│  Content (Hints/Answer) │ Min 5 lines
├─────────────────────────────┤
│  Controls             │ 4 lines
└─────────────────────────────┘
```

### `app.rs` - Application Coordinator

**Purpose**: Orchestrate application flow

**Pattern**: Dependency Injection + Mediator

**Key Design**:
```rust
pub struct App {
    quiz_state: QuizState,
    hint_state: HintState,
}

impl App {
    pub fn new(repository: Box<dyn QuestionRepository>) -> Self
    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>
}
```

**Responsibilities**:
- Accept injected dependencies
- Coordinate state updates
- Handle input events
- Trigger UI rendering

**Event Handling**:
- `handle_hint_request()` - Process 'h' key
- `handle_next_question()` - Process 'n' key
- Delegates to appropriate state objects

**Why This Structure**:
- **Dependency Inversion**: Depends on `QuestionRepository` trait
- **Single Responsibility**: Only coordinates, doesn't implement logic
- **Testability**: Can inject mock repository and test without UI

### `main.rs` - Entry Point

**Purpose**: Bootstrap application and manage terminal lifecycle

**Responsibilities**:
1. Terminal setup (raw mode, alternate screen)
2. Dependency injection
3. Application execution
4. Terminal cleanup (even on error)

**Critical Pattern**:
```rust
// Setup terminal
enable_raw_mode()?;
// ... terminal initialization ...

// Create and run app with injected dependency
let repository = Box::new(InMemoryQuestionRepository);
let mut app = App::new(repository);
let res = app.run(&mut terminal).await;

// ALWAYS cleanup, even on error
disable_raw_mode()?;
// ... terminal restoration ...
```

## Data Flow

### Application Startup

```
main()
  ↓
Create QuestionRepository ──→ InMemoryQuestionRepository::get_questions()
  ↓                                      ↓
App::new(repository) ←─────── Returns Vec<Question>
  ↓
Creates QuizState & HintState
  ↓
QuizState::new(questions) ──→ Creates Timer
  ↓
App::run(terminal) ──→ Event Loop
```

### Event Loop (per iteration)

```
terminal.draw() ──→ QuizUI::render(quiz_state, hint_state)
  ↓                          ↓
event::poll(100ms)   Renders all UI sections
  ↓
Match key event:
  'h' → App::handle_hint_request()
          ↓
        HintState::enable_hints()
        HintState::next_hint()

  'n' → App::handle_next_question()
          ↓
        QuizState::next_question()
        HintState::reset()

  'q' → Return (exit loop)
  ↓
sleep(50ms)
  ↓
Loop continues...
```

### State Updates

```
User Input ('h')
  ↓
App::handle_hint_request()
  ↓
HintState::enable_hints() ──→ show_hints = true
HintState::next_hint() ───→ hint_index += 1
  ↓
Next render cycle shows updated hints
```

## SOLID Principles in Practice

### Single Responsibility Examples

**Before** (Monolithic):
- `main.rs` had UI rendering, state management, event handling, and question data

**After** (SRP):
- `Timer` only tracks time
- `QuizState` only manages quiz logic
- `HintState` only manages hint display
- `QuizUI` only renders UI
- `App` only coordinates

### Open/Closed Examples

**Closed for modification**:
- `QuestionRepository` trait doesn't change
- `App::new()` signature doesn't change

**Open for extension**:
```rust
// Add new repository without touching existing code
pub struct WebQuestionRepository {
    api_url: String,
}

impl QuestionRepository for WebQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        // Fetch from API
    }
}

// Use it immediately
let repo = Box::new(WebQuestionRepository { api_url: "..." });
let app = App::new(repo);
```

### Liskov Substitution Examples

All `QuestionRepository` implementations are interchangeable:

```rust
fn create_app(use_file: bool) -> App {
    let repo: Box<dyn QuestionRepository> = if use_file {
        Box::new(FileQuestionRepository::new("q.json".into()))
    } else {
        Box::new(InMemoryQuestionRepository)
    };

    App::new(repo)  // Works with either implementation
}
```

### Interface Segregation Examples

**Before**: Fat `AppState` interface:
```rust
struct AppState {
    // Domain concerns
    questions: Vec<Question>,
    current_question_idx: usize,

    // Timing concerns
    time_started: Instant,
    time_limit: Duration,

    // UI concerns
    show_hints: bool,
    hint_index: usize,
    show_answer: bool,
}
```

**After**: Segregated interfaces:
```rust
QuizState  // Domain needs only: questions, current_index, timer
HintState  // UI needs only: show_hints, hint_index
Timer      // Timing needs only: started, limit
```

### Dependency Inversion Examples

**High-level** (App) depends on **abstraction** (QuestionRepository):
```rust
impl App {
    // Depends on trait, not concrete type
    pub fn new(repository: Box<dyn QuestionRepository>) -> Self
}
```

Not on **low-level** implementation details:
```rust
// BAD: Direct dependency
impl App {
    pub fn new() -> Self {
        let questions = vec![/* hardcoded */];
        // ...
    }
}
```

## Extension Points

### Adding New Question Sources

1. Implement `QuestionRepository` trait
2. Inject in `main.rs`
3. No other changes needed

**Example - Database Repository**:
```rust
// In src/question_repository.rs or new file
pub struct DatabaseQuestionRepository {
    connection_string: String,
}

impl QuestionRepository for DatabaseQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        // Connect to database
        // Query questions table
        // Map to Question structs
        // Return Vec<Question>
    }
}

// In main.rs
let repo = Box::new(DatabaseQuestionRepository {
    connection_string: "postgres://...".into()
});
```

### Adding Scoring System

1. **Create `src/scoring.rs`**:
```rust
pub struct Score {
    correct: usize,
    total: usize,
    time_remaining: Vec<Duration>,
}

impl Score {
    pub fn add_result(&mut self, correct: bool, time: Duration) { }
    pub fn calculate_percentage(&self) -> f64 { }
}
```

2. **Update `QuizState`**:
```rust
pub struct QuizState {
    // existing fields...
    score: Score,
}
```

3. **Update `App`**:
```rust
fn handle_answer_submission(&mut self, user_answer: String) {
    let correct = self.quiz_state.check_answer(user_answer);
    let time = self.quiz_state.timer().remaining();
    self.quiz_state.record_score(correct, time);
}
```

4. **Update `QuizUI`**:
```rust
fn render_score(f: &mut Frame, score: &Score, area: Rect) {
    // Render score display
}
```

### Adding Question Difficulty Levels

1. **Update `models.rs`**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Question {
    // existing fields...
    pub difficulty: Difficulty,
}
```

2. **Add filtering to `QuestionRepository`**:
```rust
pub trait QuestionRepository {
    fn get_questions(&self) -> Vec<Question>;
    fn get_questions_by_difficulty(&self, difficulty: Difficulty) -> Vec<Question>;
}
```

3. **Update UI to show difficulty**

### Adding Answer Validation

1. **Create `src/validator.rs`**:
```rust
pub trait AnswerValidator {
    fn validate(&self, user_answer: &str, correct_answer: &str) -> bool;
}

pub struct ExactMatchValidator;
pub struct FuzzyMatchValidator;
pub struct CommandValidator;  // Kubernetes command validation
```

2. **Inject into `App`**:
```rust
pub struct App {
    // existing fields...
    validator: Box<dyn AnswerValidator>,
}
```

## Testing Strategy

### Unit Testing

Each module can be tested independently:

**Timer Tests**:
```rust
#[test]
fn timer_expires_after_limit() {
    let timer = Timer::new(1);
    std::thread::sleep(Duration::from_secs(2));
    assert!(timer.is_expired());
}
```

**QuizState Tests**:
```rust
#[test]
fn quiz_navigates_to_next_question() {
    let questions = vec![/* test questions */];
    let mut quiz = QuizState::new(questions);
    assert_eq!(quiz.current_index(), 0);
    quiz.next_question();
    assert_eq!(quiz.current_index(), 1);
}
```

**Repository Tests**:
```rust
#[test]
fn repository_returns_questions() {
    let repo = InMemoryQuestionRepository;
    let questions = repo.get_questions();
    assert!(!questions.is_empty());
}
```

### Integration Testing

Test components working together using mock repositories:

```rust
struct TestQuestionRepository {
    questions: Vec<Question>,
}

impl QuestionRepository for TestQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        self.questions.clone()
    }
}

#[test]
fn app_processes_questions_from_repository() {
    let test_questions = vec![
        Question { id: 1, /* ... */ },
    ];
    let repo = Box::new(TestQuestionRepository {
        questions: test_questions
    });
    let app = App::new(repo);
    // Test app behavior
}
```

### UI Testing

UI is harder to test directly, but can validate rendering logic:

```rust
#[test]
fn ui_shows_different_content_when_expired() {
    // Create quiz state with expired timer
    // Create hint state
    // Capture rendered output
    // Assert answer is shown, not hints
}
```

## Performance Considerations

### Current Performance Characteristics

- **Event polling**: 100ms intervals
- **Render rate**: ~20 FPS (50ms sleep)
- **Memory**: Minimal allocations after startup
- **Allocations per frame**: Only for formatted strings in UI

### Optimization Opportunities

1. **Event-driven instead of polling**:
```rust
// Instead of poll(100ms) + sleep(50ms)
// Use blocking event::read() with timeout
```

2. **Lazy rendering**:
```rust
// Only redraw when state changes
if state_changed {
    terminal.draw(|f| QuizUI::render(f, &quiz_state, &hint_state))?;
}
```

3. **String interning**:
```rust
// Cache formatted strings instead of recreating each frame
struct UICache {
    timer_text: String,
    question_text: String,
}
```

### Memory Usage

Current memory footprint is minimal:
- Questions loaded once at startup
- State objects are small (< 1KB total)
- UI rendering allocates temporary strings (deallocated immediately)

### Scalability Considerations

The current design scales well for:
- **Hundreds of questions**: O(1) access by index
- **Long hint lists**: Only current hint is formatted
- **Multiple quiz sessions**: Each App instance is independent

Not optimized for:
- **Thousands of questions**: All loaded in memory
- **Solution**: Implement lazy-loading repository
- **Very fast question changes**: 50ms sleep adds latency
- **Solution**: Reduce sleep or use event-driven architecture

## Maintenance Guidelines

### Adding New Features

1. **Identify the appropriate module** based on SRP
2. **Create new abstractions** if crossing module boundaries
3. **Use dependency injection** for new dependencies
4. **Update documentation** for API changes
5. **Add tests** for new functionality

### Refactoring Checklist

- [ ] Does this change maintain SOLID principles?
- [ ] Are responsibilities clearly separated?
- [ ] Can this be extended without modification?
- [ ] Are dependencies injected, not hardcoded?
- [ ] Is the change testable in isolation?
- [ ] Is documentation updated?

### Code Review Focus Areas

1. **Separation of concerns**: Is logic in the right module?
2. **Dependency direction**: Do abstractions depend on concretions?
3. **Testability**: Can this be tested without the full app?
4. **Extensibility**: Would future changes require modification?
5. **Clarity**: Is the purpose immediately clear?

## Conclusion

This architecture prioritizes:
- **Long-term maintainability** over quick hacks
- **Explicit dependencies** over implicit coupling
- **Testability** over convenience
- **Extensibility** over simplicity

The SOLID foundation enables confident changes and feature additions without fear of breaking existing functionality.
