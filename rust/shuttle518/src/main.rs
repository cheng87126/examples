use actix_files::{Files, NamedFile};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error, get, middleware,
    web::{self, ServiceConfig},
    Responder,
};
use shuttle_actix_web::ShuttleActixWeb;

use sqlx::PgPool;

use shuttle518::AppState;

const FIVE_MINUTES: Duration = Duration::minutes(50);

mod funds;
mod login;
mod url;
mod user;

#[get("/")]
async fn index_page() -> impl Responder {
    NamedFile::open_async("assets/index.html").await
}
#[get("/login")]
async fn login_page() -> impl Responder {
    NamedFile::open_async("assets/login.html").await
}
#[get("/")]
async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://user-shuttle518:{secrets.PASSWORD}@db.shuttle.rs:5432/db-shuttle518"
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
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
    let state = web::Data::new(AppState { pool });
    let secret_key = Key::generate();

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .service(index)
                .service(login::login)
                .service(login::logout)
                .service(user::add_user)
                .service(user::get_user)
                .service(url::add_url)
                .service(url::get_urls)
                .service(funds::add_fund)
                .service(funds::get_funds)
                .service(funds::update_fund)
                .service(funds::get_fund)
                .service(funds::get_fund_name)
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
        )
        .service(Files::new("/static", "./assets/static/"))
        .service(web::scope("").service(index_page).service(login_page));
    };

    Ok(config.into())
}
