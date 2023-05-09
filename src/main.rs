use axum::{
    routing::{ get, post },
    http::StatusCode,
    Json, Router,
};
use serde::{ Deserialize, Serialize };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hello world!"
}

async fn create_user(Json(payload): Json<CreateUser>, ) -> (StatusCode, Json<User>) {
    let user = User {
        id: 0,
        first_name: payload.first_name,
        last_name: payload.last_name,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}