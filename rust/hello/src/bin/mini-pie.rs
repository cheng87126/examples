use std::env;

enum Methods{
    GET,
    POST,
    PUT,
    DELETe
}
#[tokio::main]
async fn main(){
    // [flags] [METHOD] URL [ITEM [ITEM]]
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    // dbg!(args);
    if args[1] == "get" {
        // let body = reqwest::blocking::get("https://www.rust-lang.org")?
        //     .query(&[("foo", "a"), ("foo", "b")])?
        //     .text()?;

        get(&args[2],&get_query(&args)).await.unwrap();
    }
}
fn get_query(args:&Vec<String>)->Vec<(String,String)>{
    let mut query:Vec<(String,String)> = Vec::new();
    for (idx,val) in args.iter().enumerate() {
        if idx>2 {
            let mut arr = val.split("=");
            query.push((arr.next().unwrap().to_owned(),arr.next().unwrap().to_owned()));
        }
    }

    query
}

async fn get(url:&str,query:&Vec<(String,String)>)->Result<(),reqwest::Error>{
    let client = reqwest::Client::new();
    let res = client.get(url)
        .query(query)
        .send()
        .await?;
    let header = res.headers();
    for (k,v) in header {
        println!("{}:{:?}",k,v);
    }
    let body = res.text().await?;
    println!("{body}");
    Ok(())
}
