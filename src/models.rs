use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task{
    pub id: i64,
    pub title: String,
    pub completed: bool
}