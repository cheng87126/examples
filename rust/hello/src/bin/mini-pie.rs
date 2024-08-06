use std::{collections::HashMap, env};
use colored::Colorize;
use clap::{Parser, Subcommand, Args};

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Methods>
}
#[derive(Subcommand,Debug)]
enum Methods{
    GET(CliArgs),
    POST(CliArgs),
    PUT,
    DELETe
}
#[derive(Args,Debug)]
struct CliArgs{
    url:String,
    body:Vec<String>
}

#[tokio::main]
async fn main(){
    let cli = Cli::parse();
    println!("{:?}",cli);
    // [flags] [METHOD] URL [ITEM [ITEM]]
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    if args[1] == "get" {
        // let body = reqwest::blocking::get("https://www.rust-lang.org")?
        //     .query(&[("foo", "a"), ("foo", "b")])?
        //     .text()?;

        get(&args[2],&get_query(&args)).await.unwrap();
    }
    if args[1] == "post" {
        post(&args[2],&get_query(&args)).await.unwrap();
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
    print_body(body);
    Ok(())
}
async fn post(url:&str,data:&[(String,String)])->Result<(),reqwest::Error>{
    let mut map = HashMap::new();
    for (k,v) in data{
        map.insert(k, v);
    }
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&map)
        .send()
        .await?;
    let body = res.text().await?;
    print_body(body);
    Ok(())
}

fn print_body(str:String){
    println!("{}",str.cyan());
}