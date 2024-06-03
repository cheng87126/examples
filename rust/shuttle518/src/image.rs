use actix_web::{
    get,
    web::{self},
    Responder,
    HttpResponse
};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct ImageQuery{
    pub url:String
}

#[get("/image")]
pub async fn process_image(image_query: web::Query<ImageQuery>) -> impl Responder {
    println!("{:?}",image_query);
    let resp = reqwest::get(&image_query.url).await.unwrap();
    let data = resp.bytes().await.unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(data)
}