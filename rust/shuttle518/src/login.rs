use crate::AppState;
use actix_identity::Identity;
use actix_web::{
    error, get,
    http::StatusCode,
    post,
    web::{self},
    Error, HttpMessage as _, HttpRequest, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
struct LoginFormData {
    pub pwd: String,
    pub user_name: String,
}
#[derive(Debug, Serialize, FromRow)]
struct LoginUser {
    pub id: i32,
    pub pwd: String,
    pub user_name: String,
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    form: web::Form<LoginFormData>,
    state: web::Data<AppState>,
) -> actix_web::Result<impl Responder, Error> {
    // some kind of authentication should happen here
    let user = sqlx::query_as::<_, LoginUser>("SELECT * FROM users WHERE user_name = $1")
        .bind(&form.user_name)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    let res = if user.pwd == form.pwd {
        "/".to_owned()
    } else {
        "/login".to_owned()
    };
    // attach a verified user identity to the active session
    // into()
    Identity::login(&req.extensions(), user.id.to_string().to_owned()).unwrap();

    Ok(web::Redirect::to(res).using_status_code(StatusCode::FOUND))
}

#[get("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
