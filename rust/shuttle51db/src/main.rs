use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    println!("{:?}", data.pool);
    let ret = sqlx::query_as::<_, Todos>("SELECT * FROM todos")
        .fetch_one(&data.pool)
        .await;
    if let Ok(row) = ret {
        println!("{:?}", row);
    } else {
        println!("not found");
    }
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Clone, Debug)]
struct AppState {
    pool: Pool<Postgres>,
}
#[derive(Debug, sqlx::FromRow)]
struct Todos {
    id: i32,
    node: String,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_str = std::fs::read_to_string(".env").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
