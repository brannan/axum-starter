use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    {Extension, Json},
};
use sqlx::types::Uuid;
use sqlx::PgPool;
// use serde_json::{json, Value};

use crate::{errors::ApiError, models::user::User};

pub async fn all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM users ORDER BY email";

    let users = sqlx::query_as::<_, User>(sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(users))
}

/// Retrieve single user by id or NotFound error.
pub async fn user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<User>, ApiError> {
    let sql = "SELECT * FROM users WHERE id = $1";

    let user = sqlx::query_as::<_, User>(sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| ApiError::NotFound)?;

    Ok(Json(user))
}
