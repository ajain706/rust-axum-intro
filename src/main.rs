use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use axum::{
    Json, Router, extract::{Path, Query}, http::StatusCode, routing::{get, post}
};

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

fn seed_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 1,
            title: "This is task 1".to_string(),
            done: false,
        },
        Task {
            id: 2,
            title: "This is task 2".to_string(),
            done: false,
        },
        Task {
            id: 3,
            title: "This is task 3".to_string(),
            done: false,
        }
    ]
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(check_health))
        .route("/task", get(get_tasks).post(create_tasks))
        .route("/task/{id}", get(get_task));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is running on 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "This is the root"
}

async fn check_health() -> &'static str {
    "Server is working fine"
}

async fn get_tasks(Query(_): Query<HashMap<String, String>>) -> Json<Vec<Task>> {
    // let filter = params
    //     .get("done")
    //     .map_or("Everything".to_string(), |v| format!("Done = {}", v));
    // format!("Tasks filter , {}", filter)

    Json(seed_tasks())
}

async fn create_tasks() -> &'static str {
    "Task added"
}

async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode>{
    let tasks = seed_tasks();
    tasks.into_iter().find(|v| v.id == id).map(Json).ok_or(StatusCode::NOT_FOUND)
}
