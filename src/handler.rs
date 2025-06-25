use axum::{extract::{Path, State}, Json};
use sqlx::SqlitePool;
use serde_json::{json, Value};
use crate::models::Task;

//get all task
pub async fn get_tasks(State(pool):State<SqlitePool>)->Json<Vec<Task>>{
    let tasks = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(tasks)
}

//create task
pub async fn create_task(State(pool):State<SqlitePool>, Json(task): Json<Task>)->Json<Task>{
    let _ = sqlx::query!(
        "INSERT INTO tasks (title, completed) VALUES (?, ?)",
        task.title, task.compited
    )
        .excute(&pool)
        .await
        .unwrap();
    Json(task)
}

//get task

pub async fn get_task(Path(id): Path<i64>, State(pool): State<SqlitePool>)-> Json<Task>{
    let task = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks WHERE id = ?").bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(task)
}

//update task
pub async fn update_task(Path(id): Path<i64>,State(pool):State<SqlitePool>, Json(task):Json<Task>)->Json<Task>{
    let _ = sqlx::query!("UPDATE tasks SET title = ?, completed = ?, WHERE id = ?", task.title, task.completed, id)
        .execute(&pool)
        .await
        .unwrap();
    Json(task)
}

//delete task
pub async fn delete_task(Path(id): Path<i64>, State(pool): State<SqlitePool>)->Json<Value>{
    let _ = sqlx::query!("Delete FROM tasks WHERE id = ?", id)
        .execute(&pool)
        .await
        .unwrap();
    Json(json!({"message": format!("Task {} deleted", id)}))
}