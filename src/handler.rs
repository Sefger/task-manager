use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use serde_json::{json, Value};
use crate::models::Task;
use std::sync::Arc;

//get all task
pub async fn get_tasks(State(pool):State<Arc<PgPool>>)->Json<Vec<Task>>{
    let tasks = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks")
        .fetch_all(&*pool)
        .await
        .unwrap();

    Json(tasks)
}

//create task
pub async fn create_task(State(pool):State<Arc<PgPool>>, Json(task): Json<Task>)->Json<Task>{
     sqlx::query(
        "INSERT INTO tasks (title, completed) VALUES ({task.title}, {task.completed})",

    )
        .execute(&*pool)
        .await
        .unwrap();
    Json(task)
}

//get task

pub async fn get_task(Path(id): Path<i64>, State(pool): State<Arc<PgPool>>)-> Json<Task>{
    let task = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks WHERE id = {}").bind(id)
        .fetch_one(&*pool)
        .await
        .unwrap();
    Json(task)
}

//update task
pub async fn update_task(Path(id): Path<i64>,State(pool):State<Arc<PgPool>>, Json(task):Json<Task>)->Json<Task>{
    let _id = id;
    sqlx::query("UPDATE tasks SET title = {task.title}, completed = {task.completed} WHERE id = {_id}" )
        .execute(&*pool)
        .await
        .unwrap();
    Json(task)
}

//delete task
pub async fn delete_task(Path(id): Path<i64>, State(pool): State<Arc<PgPool>>)->Json<Value>{
    sqlx::query("DELETE FROM tasks WHERE id = {id}")
        .execute(&*pool)
        .await
        .unwrap();
    Json(json!({"message": format!("Task {} deleted", id)}))
}