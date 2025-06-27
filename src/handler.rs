use std::fs;
use axum::{extract::{Path, State, Form}, Json};
use sqlx::PgPool;
use serde_json::{json, Value};
use crate::models::Task;
use std::sync::Arc;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use serde::Deserialize;
use include_dir::{include_dir, Dir};
#[derive(Debug, Deserialize)]
pub struct TaskForm {
    pub title: String,
    pub completed: Option<bool>,
}

//get all task
pub async fn get_tasks(State(pool): State<Arc<PgPool>>) -> Json<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks")
        .fetch_all(&*pool)
        .await
        .unwrap();

    Json(tasks)
}

//create task
pub async fn create_task(State(pool): State<Arc<PgPool>>, Form(form): Form<TaskForm>) -> Result<Redirect, (StatusCode, String)> {
    if form.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title cannot be empty".into()));
    }
    let completed = form.completed.unwrap_or(false);
    match sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title,completed) VALUES ($1, $2) RETURNING id, title, completed")
        .bind(&form.title)
        .bind(completed)
        .fetch_one(&*pool)
        .await
    {
        Ok(task)=> Ok(Redirect::to(&format!("/tasks/{}", task.id))),
        Err(e)=>{
            eprintln!("Failed to create task: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task". into()))
        }
    }
}
//get task

pub async fn get_task(Path(id): Path<i64>, State(pool): State<Arc<PgPool>>) -> Json<Task> {
    let task = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks WHERE id = $1").bind(id)
        .fetch_one(&*pool)
        .await
        .unwrap();
    Json(task)
}

//update task
pub async fn update_task(Path(id): Path<i64>, State(pool): State<Arc<PgPool>>, Json(task): Json<Task>) -> Json<Task> {
    let _id = id;
    sqlx::query("UPDATE tasks SET title = {task.title}, completed = {task.completed} WHERE id = {_id}")
        .execute(&*pool)
        .await
        .unwrap();
    Json(task)
}

//delete task
pub async fn delete_task(Path(id): Path<i64>, State(pool): State<Arc<PgPool>>) -> Json<Value> {
    sqlx::query("DELETE FROM tasks WHERE id = {id}")
        .execute(&*pool)
        .await
        .unwrap();
    Json(json!({"message": format!("Task {} deleted", id)}))
}

pub async fn create_task_page() -> impl IntoResponse {
    match fs::read_to_string("templates/create-task.html"){
        Ok(html)=>Html(html),
        Err(e)=> {
            eprintln!("Failed to read template: {}", e);
            Html(format!(
                r#"<html><body style="color: red">
                <h1>Internal Server Error</h1>
                <p>Template not found: {}</p>
                </body></html>"#,
                e
            ))
        }
    }
}
// pub async fn root_handler() ->Box<dyn IntoResponse> {
//     match fs::read_to_string("templates/index.html") {
//         Ok(html) => Box::new(Html(html)),
//         Err(e) => {
//             eprintln!("Failed to read template: {}", e);
//             Box::new((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Internal Server Error - Template not found"
//             ).into_response())
//         }
//     }
// }

pub async fn root_handler() -> impl IntoResponse {
    match fs::read_to_string("templates/index.html") {
        Ok(html) => Html(html),
        Err(e) => {
            eprintln!("Failed to read template: {}", e);
            Html(format!(
                r#"<html><body style="color: red">
                <h1>Internal Server Error</h1>
                <p>Template not found: {}</p>
                </body></html>"#,
                e
            ))
        }
    }
}
pub async fn task_detail_page(Path(id): Path<i32>, State(pool):State<Arc<PgPool>>)->impl IntoResponse{
    //получаем задачу
    let task_result = sqlx::query_as::<_, Task>(
        "SELECT id, title, completed FROM tasks WHERE id = $1"
    )
        .bind(id)
        .fetch_one(&*pool)
        .await;
    match task_result{
        Ok(task)=>{
            let html = fs::read_to_string("templates/task.html")
                .unwrap()
                .replace("{id}", &task.id.to_string())
                .replace("{title}", &task.title)
                .replace("{completed_class}", if task.completed{"completed"}else{""})
                .replace("{status_text}", if task.completed{"Done"}else{"Not done"});
            Html(html)
        }
        Err(e) => {
            eprintln!("Failed to read template: {}", e);
            Html(format!(
                r#"<html><body style="color: red">
                <h1>Internal Server Error</h1>
                <p>Template not found: {}</p>
                </body></html>"#,
                e
            ))
        }
    }

}
