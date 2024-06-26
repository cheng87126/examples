use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key}, error, get, http::StatusCode, middleware, post, web::{self, Json, ServiceConfig}, Error, HttpMessage as _, HttpRequest, Responder
};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool};
use serde::{Deserialize, Serialize};

const FIVE_MINUTES: Duration = Duration::minutes(5);

#[get("/")]
async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

#[get("/login")]
async fn login(req: HttpRequest) -> impl Responder {
    // some kind of authentication should happen here

    // attach a verified user identity to the active session
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
#[get("/addUser")]
async fn add_user(user_query: web::Query<UserNew>, state: web::Data<AppState>) -> Result<String,Error>{
    sqlx::query("INSERT INTO users(pwd,user) VALUES ($1,$2)")
        .bind(&user_query.pwd)
        .bind(&user_query.user)
        .execute(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok("success".to_owned())
}
#[get("/getUser")]
async fn get_user(state: web::Data<AppState>) -> Result<Json<Vec<User>>,Error>{
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok(Json(users))
}

#[post("")]
async fn add(todo: web::Json<TodoNew>, state: web::Data<AppState>) -> Result<Json<Todo>,Error> {
    let todo = sqlx::query_as("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind(&todo.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}
#[derive(Serialize, Deserialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}
#[derive(Debug,Deserialize)]
struct UserNew {
    pub pwd: String,
    pub user: String,
}
#[derive(Serialize, Deserialize, FromRow)]
struct User {
    pub id: i32,
    pub user: String
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Generate a random secret key. Note that it is important to use a unique
    // secret key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    //
    // When deployed the secret key should be read from deployment secrets.
    //
    // For example, a secure random key (in base64 format) can be generated with the OpenSSL CLI:
    // ```
    // openssl rand -base64 64
    // ```
    //
    // Then decoded and converted to a Key:
    // ```
    // let secret_key = Key::from(base64::decode(&private_key_base64).unwrap());
    // ```
    let secret_key = Key::generate();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(index)
                .service(login)
                .service(logout)
                .service(add)
                .service(add_user)
                .service(get_user)
                .app_data(state)
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .cookie_name("auth-example".to_owned())
                        .cookie_secure(false)
                        .session_lifecycle(PersistentSession::default().session_ttl(FIVE_MINUTES))
                        .build(),
                )
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default()),
        );
    };

    Ok(config.into())
}
