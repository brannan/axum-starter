use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
