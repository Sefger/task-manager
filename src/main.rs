mod db;
mod handler;
mod models;

use axum::{Router, routing::{get}, extract::State};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
#[tokio::main]
async fn main() {
    // Подключение к SQLite
    let pool = db::init_db().await.expect("Don`t init db");

    // Оборачиваем pool в Arc для реализации Clone
    let shared_pool = Arc::new(pool);

    // Роутинг
    let app = Router::new()
        .route("/", get(handler::root_handler))
        .nest_service("/static", ServeDir::new("static"))
        .route("/create-task", get(handler::create_task_page))
        .route("/tasks",
               get({
                   let pool = Arc::clone(&shared_pool);
                   move || handler::get_tasks(State(pool))
               })
                   .post({
                       let pool = Arc::clone(&shared_pool);
                       move |form| handler::create_task(State(pool), form)
                   }))
        .route("/tasks/{id}",
               get({
                   let pool = Arc::clone(&shared_pool);
                   move |path| handler::task_detail_page(path, State(pool))
               })
                   .put({
                       let pool = Arc::clone(&shared_pool);
                       move |path, json| handler::update_task(path, State(pool), json)
                   })
                   .delete({
                       let pool = Arc::clone(&shared_pool);
                       move |path| handler::delete_task(path, State(pool))
                   }));

    // Запуск сервера
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(), app
    ).await.unwrap()
}