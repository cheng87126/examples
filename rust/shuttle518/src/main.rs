use actix_files::NamedFile;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error, get, post,
    http::StatusCode,
    middleware,
    web::{self, ServiceConfig},
    Error,
    HttpMessage as _, HttpRequest, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use chrono::naive::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

const FIVE_MINUTES: Duration = Duration::minutes(50);

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

#[post("/login")]
async fn login(req: HttpRequest,form: web::Form<LoginFormData>,state: web::Data<AppState>) -> actix_web::Result<impl Responder,Error> {
    // some kind of authentication should happen here
    let user = sqlx::query_as::<_,LoginUser>("SELECT * FROM users WHERE user_name = $1")
        .bind(&form.user_name)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    let res = if user.pwd == form.pwd {
        "/".to_owned()
    }else{
        "/login".to_owned()
    };
    // attach a verified user identity to the active session
    // into()
    Identity::login(&req.extensions(), user.id.to_string().to_owned()).unwrap();
    
    Ok(web::Redirect::to(res).using_status_code(StatusCode::FOUND))
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
#[get("/addUser")]
async fn add_user(user_query: web::Query<UserNew>, state: web::Data<AppState>) -> Result<String,Error>{
    sqlx::query("INSERT INTO users(pwd,user_name) VALUES ($1,$2)")
        .bind(&user_query.pwd)
        .bind(&user_query.user_name)
        .execute(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok("success".to_owned())
}
#[get("/getUser")]
async fn get_user(state: web::Data<AppState>) -> Result<web::Json<Vec<User>>,Error>{
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok(web::Json(users))
}
#[post("/addUrl")]
async fn add_url(state: web::Data<AppState>,identity: Option<Identity>,json:web::Json<AddUrl>) -> impl Responder{
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
    }else{
        "fail".to_owned()
    }
}
#[get("/getUrls")]
async fn get_urls(state: web::Data<AppState>,identity: Option<Identity>) -> Result<web::Json<Vec<Url>>,Error>{
    if let Some(user) = identity{
        let id = user.id().unwrap();
        let urls = sqlx::query_as("SELECT * FROM urls WHERE user_id = $1")
            .bind(id.parse::<i32>().unwrap())
            .fetch_all(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        Ok(web::Json(urls))
    }else {
        let v: Vec<Url> = Vec::new();
        Ok(web::Json(v))
    }
}
#[post("/addFund")]
async fn add_fund(state: web::Data<AppState>,identity: Option<Identity>,json:web::Json<AddFund>) -> impl Responder{
    if let Some(user) = identity {
        let id = user.id().unwrap();
        sqlx::query("INSERT INTO funds(code,buy_date,price,amount,tranche,user_id) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(&json.code)
            .bind(&json.buy_date)
            .bind(&json.price)
            .bind(&json.amount)
            .bind(&json.tranche)
            .bind(id.parse::<i32>().unwrap())
            .execute(&state.pool)
            .await
            .unwrap();
        format!("success {}", user.id().unwrap())
    }else{
        "fail".to_owned()
    }
} 
#[get("/getFunds")]
async fn get_funds(state: web::Data<AppState>,identity: Option<Identity>) -> Result<web::Json<Vec<ResFund>>,Error>{
    if let Some(user) = identity{
        let id = user.id().unwrap();
        let rows = sqlx::query_as::<_,Fund>("SELECT * FROM funds WHERE user_id = $1")
            .bind(id.parse::<i32>().unwrap())
            .fetch_all(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        let mut funds:Vec<ResFund> = Vec::new();
        for r in rows{
            let name_url = format!("https://fund.xueqiu.com/dj/open/fund/deriveds?codes={}",r.code);
            let name_res = reqwest::get(name_url).await.unwrap().json::<NameRes>().await.unwrap();
            if name_res.data.len()==0 {
                continue;
            }
            let url = format!("https://danjuanfunds.com/djapi/fund/nav/history/{}?page=1&size=1", r.code);
            let resp = reqwest::get(url)
                .await.unwrap()
                .json::<Res<ResData>>()
                .await.unwrap();
            if resp.data.items.len()==0 {
                continue;
            }
            let start_time = r.buy_date;
            let end_time: NaiveDate = NaiveDate::parse_from_str(&resp.data.items[0].date, "%Y-%m-%d").unwrap();
            let diff_day = (end_time - start_time).num_days();
            let new_val = Decimal::from_str(&resp.data.items[0].value).unwrap();
            let _old_val = r.price;
            let n = r.tranche;
            let total = new_val * n - r.amount;
            let unit = dec!(10000) / r.amount * total / Decimal::from_i64(diff_day).unwrap(); //(new_val - old_val) / Decimal::from_i64(diff_day).unwrap() * dec!(10000);
            let year = unit*dec!(365)/dec!(100);
            funds.push(ResFund {
                id:r.id,
                code:r.code,
                name:name_res.data[0].fd_name.to_owned(),
                buy_date:r.buy_date,
                amount:r.amount,
                total,
                unit,
                year
            });
        }
        Ok(web::Json(funds))
    }else {
        let v:Vec<ResFund> = Vec::new();
        Ok(web::Json(v))
    }
}
#[post("/updateFund")]
async fn update_fund(state: web::Data<AppState>,identity: Option<Identity>,json:web::Json<UpdateFund>)->Result<impl Responder,Error>{
    if let Some(user) = identity{
        let _id = user.id().unwrap();
        sqlx::query("UPDATE funds SET code = $1, buy_date = $2, price = $3, amount = $4, tranche = $5 WHERE id = $6")
            .bind(&json.code)
            .bind(&json.buy_date)
            .bind(&json.price)
            .bind(&json.amount)
            .bind(&json.tranche)
            .bind(&json.id)
            .execute(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        Ok("success".to_owned())
    }else {
        Ok("fail".to_owned())
    }
}

#[derive(Debug,Deserialize)]
struct ResDataItem{
    date : String, 
    // nav : String,
    // percentage : String,
    value : String
}
#[derive(Debug,Deserialize)]
struct ResData {
    // current_page : i32,
    items : Vec<ResDataItem>,
    // size : i32,
    // total_items : i32,
    // total_pages : i32
}
#[derive(Debug,Deserialize)]
struct Res<T> {
    data: T,
    // result_code: i32
}
#[derive(Deserialize)]
struct AddFund{
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal
}
#[derive(Deserialize)]
struct UpdateFund{
    pub id: i32,
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal
}
#[derive(Debug,FromRow)]
struct Fund{
    pub id:i32,
    pub code: String,
    pub buy_date: NaiveDate,
    pub price: Decimal,
    pub amount: Decimal,
    pub tranche: Decimal
}
#[derive(Serialize)]
struct ResFund{
    pub id: i32,
    pub code: String,
    pub name: String,
    pub buy_date: NaiveDate,
    pub amount: Decimal,
    pub total: Decimal,
    pub unit: Decimal,
    pub year: Decimal
}
#[derive(Deserialize)]
struct FdName{
    fd_name:String
}
#[derive(Deserialize)]
struct NameRes{
    data:Vec<FdName>
}

#[derive(Debug,Deserialize)]
struct UserNew {
    pub pwd: String,
    pub user_name: String
}
#[derive(Debug,Serialize,FromRow)]
struct User {
    pub id: i32,
    pub user_name: String
}
#[derive(Debug,Serialize,FromRow)]
struct Url{
    pub id: i32,
    pub content:String,
    pub remark:String
}
#[derive(Debug,Deserialize)]
struct AddUrl{
    pub content:String,
    pub remark:String
}
#[derive(Debug,Deserialize)]
struct LoginFormData{
    pub pwd: String,
    pub user_name: String
}
#[derive(Debug,Serialize,FromRow)]
struct LoginUser {
    pub id: i32,
    pub pwd: String,
    pub user_name: String
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres(local_uri = "postgres://user-shuttle518:{secrets.PASSWORD}@db.shuttle.rs:5432/db-shuttle518")] pool: PgPool) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
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
                .service(login)
                .service(logout)
                .service(add_user)
                .service(get_user)
                .service(add_url)
                .service(get_urls)
                .service(add_fund)
                .service(get_funds)
                .service(update_fund)
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
        ).service(
            web::scope("")
                .service(index_page)
                .service(login_page)
        );
    };

    Ok(config.into())
}
