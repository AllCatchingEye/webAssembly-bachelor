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
        .route("/dht11_values", post(dht11_values))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum_server::Server::bind(address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Serialize, Deserialize)]
struct Dht11SensorValues {
    temperature: i64,
    humidity: i64,
}

#[debug_handler]
async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Dht11SensorValues>>, AppError> {
    println!("/ was called");
    let todos = sqlx::query_as!(Dht11SensorValues, "SELECT temperature, humidity FROM dht11")
        .fetch_all(&pool)
        .await?;

    Ok(Json(todos))
}

async fn hello_world() -> Result<Json<String>, AppError> {
    println!("/hello_world was called");
    Ok(Json(format!("Hello, world!")))
}

async fn dht11_values(
    State(pool): State<SqlitePool>,
    Form(dht11_sensor_values): Form<Dht11SensorValues>,
) -> Result<String, AppError> {
    println!("/dht11_values was called");
    sqlx::query!(
        "INSERT INTO dht11 (temperature, humidity) VALUES (?, ?)",
        dht11_sensor_values.temperature,
        dht11_sensor_values.humidity,
    )
    .execute(&pool)
    .await?;

    let entries = sqlx::query_as!(Dht11SensorValues, "SELECT temperature, humidity FROM dht11")
        .fetch_all(&pool)
        .await?;

    // Construct the success message along with the current entries
    let mut success_message = format!("Successfully inserted sensor values!\nCurrent entries:\n");
    for entry in entries {
        success_message.push_str(&format!(
            "Temperature: {}, Humidity: {}\n",
            entry.temperature, entry.humidity
        ));
    }

    Ok(format!("Successfully inserted sensor values!\n"))
}
