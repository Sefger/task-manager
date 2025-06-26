use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task{
    pub id: i32,
    pub title: String,
    pub completed: bool
}