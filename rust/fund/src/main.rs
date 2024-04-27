// use std::collections::HashMap;
use std::{io, fs};
use reqwest;
use tokio;
use serde::{Deserialize};
use chrono::{NaiveDate};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("./test.txt").expect("Should have been able to read the file");
    for row in contents.split("\r\n") {
        let mut cols = row.split(" ");
        let date = cols.next().unwrap();
        let code = cols.next().unwrap();
        let name = cols.next().unwrap();
        let amount = cols.next().unwrap();
        let nums = cols.next().unwrap();
        let value = cols.next().unwrap();
        // if let Some(code) = cols.nth(1) {
        let url = format!("https://danjuanfunds.com/djapi/fund/nav/history/{}?page=1&size=1", code); 
        let resp = reqwest::get(url)
            .await?
            .json::<Res<ResData>>()
            .await?;
        // println!("{}",resp.data.items[0].value);
        let start_time: NaiveDate = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        let end_time: NaiveDate = NaiveDate::parse_from_str(&resp.data.items[0].date, "%Y-%m-%d").unwrap();
        let diff_day = (end_time - start_time).num_days();
        let new_val:f64 = resp.data.items[0].value.parse().unwrap();
        let old_val:f64 = value.parse().unwrap();
        let n:f64 = nums.parse().unwrap();
        let a = 10000 as f64 / amount.parse::<f64>().unwrap();
        let ret = (new_val - old_val) * n / diff_day as f64 * a;
        println!("{} {}  {}  {}",end_time, name, code, ret);
        // }
    }
    // let resp = reqwest::get("https://danjuanfunds.com/djapi/fund/nav/history/018978?page=1&size=1")
    //     .await?
    //     .json::<Res<ResData>>()
    //     .await?;
    // println!("{resp:#?}");
    let mut guess = String::new();
    println!("按任意键退出");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    Ok(())
}
