use std::io::Cursor;
use actix_web::{
    get,
    web::{self},
    Responder,
    HttpResponse
};
use serde::Deserialize;
use image::{ load_from_memory, imageops::{resize,FilterType} };

#[derive(Debug,Deserialize)]
struct ImageQuery{
    pub url:String
}

#[get("/image")]
pub async fn process_image(image_query: web::Query<ImageQuery>) -> impl Responder {
    println!("{:?}",image_query);
    let resp = reqwest::get(&image_query.url).await.unwrap();
    let data = resp.bytes().await.unwrap();

    let img = load_from_memory(&data).unwrap();
    let ret = resize(&img,100,100,FilterType::Nearest);
    let mut bytes: Vec<u8> = Vec::new();
    ret.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
    HttpResponse::Ok()
        // .content_type("image/png")
        .body(bytes)
}