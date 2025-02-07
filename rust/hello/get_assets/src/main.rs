use std::env;

#[tokio::main]
async fn main() {
    match get_dir_name() {
        Some(dir) => {
            println!("Temporary directory: {:?}", dir);
            let url = "https://www.rust-lang.org".to_string();
            let body = get_page(url).await;
            println!("body = {body:?}");
        },
        None => {}
    }
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