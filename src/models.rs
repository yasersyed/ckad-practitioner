use serde::{Deserialize, Serialize};

/// Represents a single quiz question with hints and answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub question: String,
    pub hints: Vec<String>,
    pub answer: String,
    pub time_limit_secs: u64,
}
