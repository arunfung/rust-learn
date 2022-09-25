use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::get(url).await?.text().await?;

    // 使用 polars 直接请求
    let df = CsvReader::new(Cursor::new(data)).infer_schema(Some(16)).finish()?;
    // let filtered = df.lazy().filter(col("new_deaths").gt(500))
    //     .select(["location", "total_cases", "new_cases", "total_deaths", "new_deaths"])
    //     .collect()?;
    let mask = df.column("new_deaths")?.gt(350)?;
    let filtered = df.filter(&mask)?;
    let res = filtered.select(["location", "total_cases", "new_cases", "total_deaths", "new_deaths"])?;
    println!("{:?}", res);

    Ok(())
}