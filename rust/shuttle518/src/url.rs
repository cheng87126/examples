use crate::AppState;
use actix_identity::Identity;
use actix_web::{
    error, get, post,
    web::{self},
    Error, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
struct Url {
    pub id: i32,
    pub content: String,
    pub remark: String,
}
#[derive(Debug, Deserialize)]
struct AddUrl {
    pub content: String,
    pub remark: String,
}

#[post("/addUrl")]
pub async fn add_url(
    state: web::Data<AppState>,
    identity: Option<Identity>,
    json: web::Json<AddUrl>,
) -> impl Responder {
    if let Some(user) = identity {
        let id = user.id().unwrap();
        sqlx::query("INSERT INTO urls(content,remark,user_id) VALUES ($1,$2,$3)")
            .bind(&json.content)
            .bind(&json.remark)
            .bind(id.parse::<i32>().unwrap())
            .execute(&state.pool)
            .await
            .unwrap();
        format!("success {}", user.id().unwrap())
    } else {
        "fail".to_owned()
    }
}
#[get("/getUrls")]
pub async fn get_urls(
    state: web::Data<AppState>,
    identity: Option<Identity>,
) -> Result<web::Json<Vec<Url>>, Error> {
    if let Some(user) = identity {
        let id = user.id().unwrap();
        let urls = sqlx::query_as("SELECT * FROM urls WHERE user_id = $1")
            .bind(id.parse::<i32>().unwrap())
            .fetch_all(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        Ok(web::Json(urls))
    } else {
        let v: Vec<Url> = Vec::new();
        Ok(web::Json(v))
    }
}
