use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use std::net::UdpSocket;
use tower_http::cors::CorsLayer;

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Get environment variables
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    // Create router for server
    let app = Router::new()
        .route("/", get(list))
        .route("/hello_world", get(hello_world))
        .route("/create", get(create))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    // Spawn the HTTP server task
    let http_server_task = tokio::spawn(async move {
        if let Err(err) = axum_server::Server::bind(address)
            .serve(app.into_make_service())
            .await
        {
            eprintln!("HTTP server error: {}", err);
        }
    });

    // Spawn the UDP server task
    let udp_server_task = tokio::spawn(udp_server());

    // Wait for both servers to finish
    http_server_task.await?;
    udp_server_task.await?;

    Ok(())
}

async fn udp_server() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Convert the received bytes to a slice and print it as UTF-8 string
        let received_data = &buf[..amt];
        if let Ok(s) = std::str::from_utf8(received_data) {
            println!("Received: {}", s);
        } else {
            println!("Received data is not valid UTF-8.");
        }
    } // the socket is closed here
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[debug_handler]
async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;

    Ok(Json(todos))
}

async fn hello_world() -> Result<Json<String>, AppError> {
    Ok(Json(format!("Hello, world!")))
}

async fn create(
    State(pool): State<SqlitePool>,
    Form(todo): Form<Todo>,
) -> Result<String, AppError> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&pool)
    .await?;

    Ok(format!("Successfully inserted todo!"))
}
