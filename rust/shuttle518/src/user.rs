use crate::AppState;
use actix_web::{
    error, get,
    web::{self},
    Error,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
struct UserNew {
    pub pwd: String,
    pub user_name: String,
}
#[derive(Debug, Serialize, FromRow)]
struct User {
    pub id: i32,
    pub user_name: String,
}

#[get("/addUser")]
pub async fn add_user(
    user_query: web::Query<UserNew>,
    state: web::Data<AppState>,
) -> Result<String, Error> {
    sqlx::query("INSERT INTO users(pwd,user_name) VALUES ($1,$2)")
        .bind(&user_query.pwd)
        .bind(&user_query.user_name)
        .execute(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok("success".to_owned())
}
#[get("/getUser")]
pub async fn get_user(state: web::Data<AppState>) -> Result<web::Json<Vec<User>>, Error> {
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok(web::Json(users))
}
