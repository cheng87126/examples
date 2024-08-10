// use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    // tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::get(url).await?.text().await?;

    let df_csv = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .into_reader_with_file_handle(Cursor::new(data))
        // .try_into_reader_with_file_path(Some(url.into()))?
        .finish()?;
    println!("{}", df_csv);
    // 使用 polars 直接请求
    // let df = CsvReader::new(Cursor::new(data))
    //     .infer_schema(Some(16))
    //     .finish()?;
    
    // let df = CsvReader::new(Cursor::new(data)).finish()?;
    // let c = col("new_deaths").gt(500);
    // let filtered = df.filter(col("new_deaths").gt(500))?;

    // println!(
    //     "{:?}",
    //     filtered.select((
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ))
    // );

    Ok(())
}
