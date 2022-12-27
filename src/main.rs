mod models;

extern crate lazy_static;
extern crate postgres;

use models::OHLCVData;
use postgres::{Client, NoTls};

use std::thread;
use std::time::Duration;
use std::error::Error;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scheduling_task = tokio::spawn(async {
        // wait for 15 minutes
        let mut loop_interval = tokio::time::interval(Duration::from_secs(60));

        loop {
            loop_interval.tick().await;
            println!("starting another period...");

            let symbol: String = "ADAUSDT".to_string();
            let inv: String = "15m".to_string();
            let res = OHLCVData::get(&symbol, &inv).await.unwrap();

            for i in res {
                println!("data here: {:?}", i);
                thread::spawn(move || { prepare_database(i).ok() });
            }
        }
    });

    scheduling_task.await;
    
    Ok(())
}

fn prepare_database(data: OHLCVData) -> Result<(), postgres::Error> {
    // load environment variables
    dotenv().ok();
    let posgres_addr = env::var("DATABASE_URL").expect("ERROR: Database URL is undefined.");
    println!("database address: {:?}", posgres_addr);

    let mut client = Client::connect(&posgres_addr.to_string(), NoTls)?;

    // create table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS testing (
            id                  SERIAL PRIMARY KEY,
            open_time           BIGINT,
            open                REAL,
            high                REAL,
            low                 REAL,
            close               REAL,
            volume              REAL,
            closetime           BIGINT,
            quote_asset_volume  REAL,
            num_of_trades       INTEGER,
            taker_by_quote      REAL,
            taker_buy_quote     REAL,
            ignore              INTEGER
        );
    ");

    println!("Try to insert\n");

    client.execute(
        "INSERT INTO testing (
            open_time,
            open,high,
            low,
            close,
            volume,
            closetime,
            quote_asset_volume,
            num_of_trades,
            taker_by_quote,
            taker_buy_quote,
            ignore) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        &[
            &(data.opentime as i64),
            &data.open,
            &data.high,
            &data.low,
            &data.close,
            &data.volume,
            &(data.closetime as i64),
            &data.quote_asset_volume,
            &(data.num_of_trades as i32),
            &data.taker_by_quote,
            &data.taker_buy_quote,
            &0_i32
        ]
    )?;

    println!("Inserted into database");

    Ok(())
}
