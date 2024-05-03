use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Form, Json, Router,
};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", post(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewTodo {
    description: String,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY ID")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<Redirect> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<String> {
    sqlx::query!("DELETE FROM todos WHERE id = (?)", id)
        .execute(&pool)
        .await?;
    Ok(format!("Succesfully deleted todo!"))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    sqlx::query!(
        "UPDATE todos SET description = (?), done = (?)  WHERE id = (?)",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}
