use axum::{extract::Extension, response::IntoResponse, routing::{get, post}, Json, Router};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{MySqlPool, Row};
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

async fn add_user(
    Extension(pool): Extension<MySqlPool>,
    Json(body): Json<User>
) -> impl IntoResponse {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(body.name)
        .bind(body.email)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => (axum::http::StatusCode::CREATED, Json(ApiResponse {
            message: String::from("User created successfully"),
        })).into_response(),
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurado en el archivo .env");
    
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Could not connect to the database");


    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users/add", post(add_user))
        .layer(Extension(pool));

        println!("✅ Server started successfully at 0.0.0.0:3000");

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
}