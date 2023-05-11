use axum::{
    routing::{ get, post },
    http::StatusCode,
    Json, Router, extract::State,
};
use serde::{ Deserialize, Serialize };
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn root(State(pool): State<PgPool>) -> Result<Json<User>, StatusCode> {
    Ok(Json(sqlx::query_as!(User, "select * from users as \"users: Json<User>\";")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?))
}

async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>, ) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("insert into users (first_name, last_name) values ($1, $2) returning id, first_name, last_name")
        .bind(payload.first_name)
        .bind(payload.last_name)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
