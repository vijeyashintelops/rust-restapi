use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct QuestionId(pub String);
