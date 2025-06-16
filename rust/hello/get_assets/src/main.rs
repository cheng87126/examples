use std::{env, io::Write};
use std::fs::File;
use std::path::Path;
use scraper::{ Html, Selector };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match get_dir_name() {
        Some(dir) => {
            println!("Temporary directory: {:?}", dir);
            let url = "".to_string(); // todo:need url
            let body = get_page(url).await;
            println!("body = {body:?}");
            if let Ok(html) = body {
                let document = Html::parse_document(&html);
                let selector = Selector::parse(".book-post ul a").unwrap();
                for element in document.select(&selector) {
                    if let Some(href) = element.value().attr("href") {
                        let img = format!("",href); // todo:need url
                        println!("{}",img);
                        
                        let target = img;
                        let response = reqwest::get(target).await?;

                        let fname = response
                            .url()
                            .path_segments()
                            .and_then(|segments| segments.last())
                            .and_then(|name| if name.is_empty() { None } else { Some(name) })
                            .unwrap_or("tmp.bin");

                        let mut name = "./assets/".to_string();
                        name += fname;
                        let path = Path::new(&name);
                        let mut dest = File::create(path)?;
                        let content =  response.bytes().await?;
                        dest.write_all(&content)?;
                    }
                }
            }
        },
        None => {}
    }
    Ok(())
}

async fn get_page(url:String) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(body)
}

fn get_dir_name() -> Option<String> {
    let path = env::current_dir().ok()?;
    path.file_name()?.to_str().map(|s| s.to_string())
}