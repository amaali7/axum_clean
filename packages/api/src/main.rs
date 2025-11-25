use axum::{
    routing::post,
    extract::State,
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::net::SocketAddr;

use tokio::net::TcpListener;
use axum::serve;

use application::CreateUserUseCase;
use infrastructure::database::InMemoryUserRepository;

#[derive(Clone)]
struct AppState {
    use_case: Arc<CreateUserUseCase<InMemoryUserRepository>>,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: String,
}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (axum::http::StatusCode, String)> {
    let result = state.use_case.execute(payload.name).await;

    match result {
        Ok(id) => Ok(Json(CreateUserResponse { id: id.to_string() })),
        Err(e) => Err((axum::http::StatusCode::BAD_REQUEST, e.to_string())),
    }
}

// (Optional) Get user endpoint – commented out for minimal runnable code
// async fn get_user_handler(...) {}

#[tokio::main]
async fn main() {
    // Dependencies
    let repo = InMemoryUserRepository::new();
    let use_case = CreateUserUseCase::new(repo);

    let state = AppState {
        use_case: Arc::new(use_case),
    };

    // Build Axum app
    let app = Router::new()
        .route("/users", post(create_user_handler))
        .with_state(state);

    // Axum 0.7 Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
